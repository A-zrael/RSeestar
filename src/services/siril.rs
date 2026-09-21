use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum SirilEvent {
    Log(String),
    Finished { output: PathBuf },
    Failed(String),
    Cancelled,
}

pub struct SirilHandle {
    child: Arc<Mutex<Option<Child>>>,
    receiver: mpsc::Receiver<SirilEvent>,
}

impl SirilHandle {
    pub fn cancel(&self) {
        if let Ok(mut child) = self.child.lock()
            && let Some(child) = child.as_mut()
        {
            let _ = child.kill();
        }
    }

    pub fn try_recv(&self) -> Result<SirilEvent, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

impl Drop for SirilHandle {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub fn start(executable: PathBuf, work_dir: PathBuf, target: String) -> io::Result<SirilHandle> {
    let target = sanitize_target(&target);
    let timestamp = timestamp_utc(SystemTime::now());
    let output_stem = format!("Stacked-{target}-{timestamp}");
    let output_path = work_dir.join(format!("{output_stem}.fit"));
    let script_path = std::env::temp_dir().join(format!(
        "rseestar-siril-{}-{}.ssf",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    fs::write(&script_path, build_script(&output_stem))?;

    let mut process = Command::new(executable)
        .arg("-s")
        .arg(&script_path)
        .arg("-d")
        .arg(&work_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = process.stdout.take();
    let stderr = process.stderr.take();
    let child = Arc::new(Mutex::new(Some(process)));
    let worker_child = Arc::clone(&child);
    let (sender, receiver) = mpsc::channel();

    if let Some(stdout) = stdout {
        spawn_log_reader(stdout, sender.clone());
    }
    if let Some(stderr) = stderr {
        spawn_log_reader(stderr, sender.clone());
    }

    thread::spawn(move || {
        let _ = sender.send(SirilEvent::Log("[SIRIL] Launching CLI process…".into()));
        loop {
            let status = {
                let mut guard = match worker_child.lock() {
                    Ok(guard) => guard,
                    Err(_) => {
                        let _ = sender
                            .send(SirilEvent::Failed("Siril process lock was poisoned".into()));
                        return;
                    }
                };
                match guard
                    .as_mut()
                    .and_then(|child| child.try_wait().ok())
                    .flatten()
                {
                    Some(status) => {
                        guard.take();
                        Some(status)
                    }
                    None => None,
                }
            };
            if let Some(status) = status {
                let _ = fs::remove_file(&script_path);
                if status.success() && output_path.is_file() {
                    let _ = sender.send(SirilEvent::Finished {
                        output: output_path,
                    });
                } else if status.success() {
                    let _ = sender.send(SirilEvent::Failed(format!(
                        "Siril exited successfully but did not create {}",
                        output_path.display()
                    )));
                } else if status.code().is_none() {
                    let _ = sender.send(SirilEvent::Cancelled);
                } else {
                    let _ = sender.send(SirilEvent::Failed(format!("Siril exited with {status}")));
                }
                return;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    Ok(SirilHandle { child, receiver })
}

fn spawn_log_reader(stream: impl Read + Send + 'static, sender: mpsc::Sender<SirilEvent>) {
    thread::spawn(move || {
        for line in BufReader::new(stream).lines().map_while(Result::ok) {
            let _ = sender.send(SirilEvent::Log(line));
        }
    });
}

fn build_script(output_stem: &str) -> String {
    format!(
        "requires 1.3.6\n\
         cd lights\n\
         link light -out=../process\n\
         cd ../process\n\
         calibrate light -debayer\n\
         seqplatesolve pp_light -force -nocache\n\
         seqapplyreg pp_light -filter-round=2.5k -framing=max\n\
         stack r_pp_light rej 3 3 -norm=addscale -output_norm -rgb_equal -out=result\n\
         load result\n\
         save ../{output_stem}\n\
         close\n"
    )
}

fn sanitize_target(target: &str) -> String {
    let sanitized: String = target
        .trim()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if sanitized.is_empty() {
        "Target".into()
    } else {
        sanitized
    }
}

fn timestamp_utc(time: SystemTime) -> String {
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let days = seconds.div_euclid(86_400);
    let day_seconds = seconds.rem_euclid(86_400);
    let hour = day_seconds / 3_600;
    let minute = day_seconds % 3_600 / 60;
    let second = day_seconds % 60;
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}{month:02}{day:02}_{hour:02}{minute:02}{second:02}")
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_preserves_reference_pipeline() {
        let script = build_script("Stacked-M42-20260921_120000");
        assert!(script.contains("seqplatesolve pp_light -force -nocache"));
        assert!(script.contains("stack r_pp_light rej 3 3 -norm=addscale -output_norm -rgb_equal"));
        assert!(script.contains("save ../Stacked-M42-20260921_120000"));
    }

    #[test]
    fn timestamp_matches_unix_epoch() {
        assert_eq!(timestamp_utc(UNIX_EPOCH), "19700101_000000");
    }

    #[test]
    fn target_is_safe_for_siril_script() {
        assert_eq!(sanitize_target(" M 42/Orion "), "M_42_Orion");
    }
}
