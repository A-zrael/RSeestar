use crate::model::{TelescopeDevice, TransferSelection};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use smb2::{ClientConfig, SmbClient, Tree};

const RUNNING: u8 = 0;
const PAUSED: u8 = 1;
const CANCELLED: u8 = 2;

#[derive(Debug)]
pub enum TransferEvent {
    Log(String),
    Progress { completed: usize, total: usize },
    Finished,
    Cancelled,
    Failed(String),
}

pub struct TransferHandle {
    state: Arc<AtomicU8>,
    receiver: mpsc::Receiver<TransferEvent>,
}

impl TransferHandle {
    pub fn pause(&self) {
        self.state.store(PAUSED, Ordering::Release);
    }

    pub fn resume(&self) {
        self.state.store(RUNNING, Ordering::Release);
    }

    pub fn cancel(&self) {
        self.state.store(CANCELLED, Ordering::Release);
    }

    pub fn try_recv(&self) -> Result<TransferEvent, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

impl Drop for TransferHandle {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[derive(Clone)]
struct CopyItem {
    source: PathBuf,
    destination: PathBuf,
    device_index: usize,
}

#[derive(Clone)]
struct RemoteCopyItem {
    remote_path: String,
    destination: PathBuf,
    device_index: usize,
    size: u64,
}

pub fn start(
    devices: Vec<TelescopeDevice>,
    destination: PathBuf,
    selection: TransferSelection,
) -> TransferHandle {
    let state = Arc::new(AtomicU8::new(RUNNING));
    let worker_state = Arc::clone(&state);
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result = run_transfer(&devices, &destination, selection, &worker_state, &sender);
        match result {
            Ok(true) => {
                let _ = sender.send(TransferEvent::Finished);
            }
            Ok(false) => {
                let _ = sender.send(TransferEvent::Cancelled);
            }
            Err(_) if worker_state.load(Ordering::Acquire) == CANCELLED => {
                let _ = sender.send(TransferEvent::Cancelled);
            }
            Err(error) => {
                let _ = sender.send(TransferEvent::Failed(error.to_string()));
            }
        }
    });
    TransferHandle { state, receiver }
}

fn run_transfer(
    devices: &[TelescopeDevice],
    destination: &Path,
    selection: TransferSelection,
    state: &AtomicU8,
    sender: &mpsc::Sender<TransferEvent>,
) -> io::Result<bool> {
    let _ = sender.send(TransferEvent::Log(
        "=================================================".into(),
    ));
    let _ = sender.send(TransferEvent::Log(
        "[SYNC_PROTOCOL] Starting managed ingest routine…".into(),
    ));
    let _ = sender.send(TransferEvent::Log(
        "=================================================".into(),
    ));
    let mut items = Vec::new();
    let mut remote_items = Vec::new();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .map_err(io::Error::other)?;
    for (device_index, device) in devices.iter().enumerate() {
        if device.simulated || device.source_path.is_dir() {
            collect_device(
                device,
                device_index + 1,
                destination,
                selection,
                &mut items,
                sender,
            )?;
        } else {
            let mut found = runtime.block_on(collect_remote_device(
                device,
                device_index + 1,
                destination,
                selection,
                sender,
            ))?;
            remote_items.append(&mut found);
        }
    }
    let total = items.len() + remote_items.len();
    if total == 0 {
        let _ = sender.send(TransferEvent::Log(
            "[WARNING] No matching source files were found.".into(),
        ));
    }
    for (index, item) in items.iter().enumerate() {
        if !wait_until_runnable(state) {
            return Ok(false);
        }
        if let Some(parent) = item.destination.parent() {
            fs::create_dir_all(parent)?;
        }
        if is_same_file(&item.source, &item.destination)? {
            let _ = sender.send(TransferEvent::Log(format!(
                "[SKIP] Existing verified file: {}",
                item.destination.display()
            )));
        } else {
            copy_interruptible(&item.source, &item.destination, state)?;
            let _ = sender.send(TransferEvent::Log(format!(
                "[INGEST] {}",
                item.destination.display()
            )));
        }
        append_device_log(
            destination,
            item.device_index,
            &format!(
                "{} -> {}",
                item.source.display(),
                item.destination.display()
            ),
        )?;
        let _ = sender.send(TransferEvent::Progress {
            completed: index + 1,
            total,
        });
    }
    let mut completed = items.len();
    for (device_index, device) in devices.iter().enumerate() {
        let device_index = device_index + 1;
        let device_items: Vec<_> = remote_items
            .iter()
            .filter(|item| item.device_index == device_index)
            .cloned()
            .collect();
        if device_items.is_empty() {
            continue;
        }
        runtime.block_on(copy_remote_device(
            device,
            &device_items,
            destination,
            state,
            sender,
            &mut completed,
            total,
        ))?;
    }
    for (index, _) in devices.iter().enumerate() {
        let _ = sender.send(TransferEvent::Log(format!(
            "[TRANSFER] Complete for Seestar Unit {}. Log: Seestar_{}.log",
            index + 1,
            index + 1
        )));
    }
    Ok(true)
}

async fn guest_connection(device: &TelescopeDevice) -> io::Result<(SmbClient, Tree)> {
    let config = ClientConfig {
        addr: format!("{}:445", device.ip),
        timeout: Duration::from_secs(8),
        username: String::new(),
        password: String::new(),
        domain: String::new(),
        auto_reconnect: true,
        compression: false,
        dfs_enabled: false,
        ..ClientConfig::default()
    };
    let mut client = SmbClient::connect(config).await.map_err(io::Error::other)?;
    let share = client
        .connect_share("EMMC Images")
        .await
        .map_err(io::Error::other)?;
    Ok((client, share))
}

async fn collect_remote_device(
    device: &TelescopeDevice,
    device_index: usize,
    destination: &Path,
    selection: TransferSelection,
    sender: &mpsc::Sender<TransferEvent>,
) -> io::Result<Vec<RemoteCopyItem>> {
    let _ = sender.send(TransferEvent::Log(format!(
        "[SMB] Opening application-managed guest session to {} (no registry changes)…",
        device.ip
    )));
    let (mut client, mut share) = guest_connection(device).await?;
    let targets = client
        .list_directory(&mut share, "MyWorks")
        .await
        .map_err(io::Error::other)?;
    let mut items = Vec::new();
    for target in targets
        .into_iter()
        .filter(|entry| entry.is_directory && entry.name != "." && entry.name != "..")
    {
        let target_name = target.name;
        let _ = sender.send(TransferEvent::Log(format!(
            "[CATALOG] Target Object: {target_name}"
        )));
        let mut pending = vec![(format!("MyWorks/{target_name}"), String::new())];
        while let Some((remote_dir, relative_dir)) = pending.pop() {
            let entries = client
                .list_directory(&mut share, &remote_dir)
                .await
                .map_err(io::Error::other)?;
            for entry in entries
                .into_iter()
                .filter(|entry| entry.name != "." && entry.name != "..")
            {
                let remote_path = format!("{remote_dir}/{}", entry.name);
                let relative = if relative_dir.is_empty() {
                    entry.name.clone()
                } else {
                    format!("{relative_dir}/{}", entry.name)
                };
                if entry.is_directory {
                    pending.push((remote_path, relative));
                    continue;
                }
                let extension = Path::new(&entry.name)
                    .extension()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                let category = match extension.as_str() {
                    "fit" | "fits" if selection.fits => "Lights",
                    "avi" | "mp4" if selection.video => "Video",
                    "jpg" | "jpeg" if selection.images => "JPEGS",
                    _ => continue,
                };
                let relative_path = Path::new(&relative);
                let relative_path = strip_category_component(relative_path, category);
                items.push(RemoteCopyItem {
                    remote_path,
                    destination: destination
                        .join(&target_name)
                        .join(category)
                        .join(relative_path),
                    device_index,
                    size: entry.size,
                });
            }
        }
    }
    client
        .disconnect_share(&share)
        .await
        .map_err(io::Error::other)?;
    Ok(items)
}

#[allow(clippy::too_many_arguments)]
async fn copy_remote_device(
    device: &TelescopeDevice,
    items: &[RemoteCopyItem],
    destination: &Path,
    state: &AtomicU8,
    sender: &mpsc::Sender<TransferEvent>,
    completed: &mut usize,
    total: usize,
) -> io::Result<()> {
    let (mut client, share) = guest_connection(device).await?;
    for item in items {
        if !wait_until_runnable(state) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "transfer cancelled",
            ));
        }
        if let Some(parent) = item.destination.parent() {
            fs::create_dir_all(parent)?;
        }
        if item.destination.is_file() && fs::metadata(&item.destination)?.len() == item.size {
            let _ = sender.send(TransferEvent::Log(format!(
                "[SKIP] Existing verified file: {}",
                item.destination.display()
            )));
        } else {
            download_interruptible(&mut client, &share, item, state).await?;
            let _ = sender.send(TransferEvent::Log(format!(
                "[INGEST] {}",
                item.destination.display()
            )));
        }
        append_device_log(
            destination,
            item.device_index,
            &format!("{} -> {}", item.remote_path, item.destination.display()),
        )?;
        *completed += 1;
        let _ = sender.send(TransferEvent::Progress {
            completed: *completed,
            total,
        });
    }
    client
        .disconnect_share(&share)
        .await
        .map_err(io::Error::other)
}

async fn download_interruptible(
    client: &mut SmbClient,
    share: &Tree,
    item: &RemoteCopyItem,
    state: &AtomicU8,
) -> io::Result<()> {
    let temporary = item.destination.with_extension(format!(
        "{}.rseestar-part",
        item.destination
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("file")
    ));
    let mut output = File::create(&temporary)?;
    let mut download = client
        .download(share, &item.remote_path)
        .await
        .map_err(io::Error::other)?;
    while let Some(chunk) = download.next_chunk().await {
        if !wait_until_runnable(state) {
            drop(output);
            let _ = fs::remove_file(&temporary);
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "transfer cancelled",
            ));
        }
        output.write_all(&chunk.map_err(io::Error::other)?)?;
    }
    output.flush()?;
    drop(output);
    if item.destination.exists() {
        fs::remove_file(&item.destination)?;
    }
    fs::rename(temporary, &item.destination)
}

fn collect_device(
    device: &TelescopeDevice,
    device_index: usize,
    destination: &Path,
    selection: TransferSelection,
    items: &mut Vec<CopyItem>,
    sender: &mpsc::Sender<TransferEvent>,
) -> io::Result<()> {
    let source = &device.source_path;
    if !source.is_dir() {
        let _ = sender.send(TransferEvent::Log(format!(
            "[WARNING] Target path unreachable for IP: {}",
            device.ip
        )));
        return Ok(());
    }
    let mut targets: Vec<_> = fs::read_dir(source)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .collect();
    targets.sort_by_key(|entry| entry.file_name());
    for target in targets {
        let target_name = target.file_name().to_string_lossy().into_owned();
        let _ = sender.send(TransferEvent::Log(format!(
            "[CATALOG] Target Object: {target_name}"
        )));
        collect_tree(
            &target.path(),
            &target.path(),
            destination,
            &target_name,
            device_index,
            selection,
            items,
        )?;
    }
    Ok(())
}

fn collect_tree(
    root: &Path,
    current: &Path,
    destination: &Path,
    target: &str,
    device_index: usize,
    selection: TransferSelection,
    items: &mut Vec<CopyItem>,
) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(current)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_tree(
                root,
                &path,
                destination,
                target,
                device_index,
                selection,
                items,
            )?;
            continue;
        }
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let category = match extension.as_str() {
            "fit" | "fits" if selection.fits => "Lights",
            "avi" | "mp4" if selection.video => "Video",
            "jpg" | "jpeg" if selection.images => "JPEGS",
            _ => continue,
        };
        let relative = path.strip_prefix(root).unwrap_or(&path);
        let relative = strip_category_component(relative, category).to_path_buf();
        items.push(CopyItem {
            source: path,
            destination: destination.join(target).join(category).join(relative),
            device_index,
        });
    }
    Ok(())
}

fn strip_category_component<'a>(relative: &'a Path, category: &str) -> &'a Path {
    let mut components = relative.components();
    let Some(first) = components.next() else {
        return relative;
    };
    if first
        .as_os_str()
        .to_string_lossy()
        .eq_ignore_ascii_case(category)
    {
        components.as_path()
    } else {
        relative
    }
}

fn is_same_file(source: &Path, destination: &Path) -> io::Result<bool> {
    if !destination.is_file() {
        return Ok(false);
    }
    Ok(fs::metadata(source)?.len() == fs::metadata(destination)?.len())
}

fn copy_interruptible(source: &Path, destination: &Path, state: &AtomicU8) -> io::Result<()> {
    let temporary = destination.with_extension(format!(
        "{}.rseestar-part",
        destination
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or("file")
    ));
    let mut input = File::open(source)?;
    let mut output = File::create(&temporary)?;
    let mut buffer = vec![0_u8; 1024 * 1024];
    loop {
        if !wait_until_runnable(state) {
            drop(output);
            let _ = fs::remove_file(&temporary);
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "transfer cancelled",
            ));
        }
        let read = input.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        output.write_all(&buffer[..read])?;
    }
    output.flush()?;
    drop(output);
    fs::rename(temporary, destination)?;
    Ok(())
}

fn wait_until_runnable(state: &AtomicU8) -> bool {
    loop {
        match state.load(Ordering::Acquire) {
            RUNNING => return true,
            PAUSED => thread::sleep(Duration::from_millis(50)),
            CANCELLED => return false,
            _ => return false,
        }
    }
}

fn append_device_log(destination: &Path, device_index: usize, line: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination.join(format!("Seestar_{device_index}.log")))?;
    writeln!(file, "{line}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::simulator;

    #[test]
    fn simulated_transfer_creates_expected_hierarchy() {
        let root =
            std::env::temp_dir().join(format!("rseestar-transfer-test-{}", std::process::id()));
        let source = root.join("source");
        let output = root.join("output");
        fs::create_dir_all(&output).unwrap();
        let devices = simulator::provision(&source).unwrap();
        let state = AtomicU8::new(RUNNING);
        let (sender, _receiver) = mpsc::channel();
        assert!(
            run_transfer(
                &devices[..1],
                &output,
                TransferSelection::default(),
                &state,
                &sender
            )
            .unwrap()
        );
        assert!(output.join("M42/Lights/M42_0001.fit").is_file());
        assert!(output.join("M42/JPEGS/M42_preview.jpg").is_file());
        assert!(output.join("Jupiter/Video/Jupiter_capture.mp4").is_file());
        assert!(output.join("Seestar_1.log").is_file());
        let _ = fs::remove_dir_all(root);
    }
}
