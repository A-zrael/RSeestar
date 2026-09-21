use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
#[derive(Debug)]
pub enum MediaEvent {
    Status(String),
    Frame {
        width: usize,
        height: usize,
        rgba: Vec<u8>,
    },
    Failed(String),
    Stopped,
}

pub struct MediaHandle {
    cancelled: Arc<AtomicBool>,
    receiver: mpsc::Receiver<MediaEvent>,
}

impl MediaHandle {
    pub fn stop(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn try_recv(&self) -> Result<MediaEvent, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

impl Drop for MediaHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn start(url: String) -> MediaHandle {
    let cancelled = Arc::new(AtomicBool::new(false));
    let worker_cancelled = Arc::clone(&cancelled);
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || platform::run(url, worker_cancelled, sender));
    MediaHandle {
        cancelled,
        receiver,
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use super::MediaEvent;
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, mpsc};

    pub fn run(_url: String, _cancelled: Arc<AtomicBool>, sender: mpsc::Sender<MediaEvent>) {
        let _ = sender.send(MediaEvent::Failed(
            "live RTSP playback uses Windows Media Foundation and is available in the Windows build"
                .into(),
        ));
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use super::MediaEvent;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, mpsc};
    use windows::Win32::Media::MediaFoundation::*;
    use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize};
    use windows::core::HSTRING;

    struct MediaFoundation;

    impl MediaFoundation {
        unsafe fn start() -> windows::core::Result<Self> {
            unsafe {
                CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
                if let Err(error) = MFStartup(MF_VERSION, MFSTARTUP_FULL) {
                    CoUninitialize();
                    return Err(error);
                }
            }
            Ok(Self)
        }
    }

    impl Drop for MediaFoundation {
        fn drop(&mut self) {
            unsafe {
                let _ = MFShutdown();
                CoUninitialize();
            }
        }
    }

    pub fn run(url: String, cancelled: Arc<AtomicBool>, sender: mpsc::Sender<MediaEvent>) {
        let result = unsafe { run_inner(&url, &cancelled, &sender) };
        match result {
            Ok(()) => {
                let _ = sender.send(MediaEvent::Stopped);
            }
            Err(error) => {
                let _ = sender.send(MediaEvent::Failed(format!(
                    "Windows Media Foundation stream error: {error}"
                )));
            }
        }
    }

    unsafe fn run_inner(
        url: &str,
        cancelled: &AtomicBool,
        sender: &mpsc::Sender<MediaEvent>,
    ) -> windows::core::Result<()> {
        let _foundation = unsafe { MediaFoundation::start()? };
        let _ = sender.send(MediaEvent::Status("Connecting to RTSP feed…".into()));
        let url = HSTRING::from(url);
        let reader = unsafe { MFCreateSourceReaderFromURL(&url, None)? };
        let stream = MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32;
        unsafe {
            reader.SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS.0 as u32, false)?;
            reader.SetStreamSelection(stream, true)?;
            let media_type = MFCreateMediaType()?;
            media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
            media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32)?;
            reader.SetCurrentMediaType(stream, None, &media_type)?;
        }
        let current_type = unsafe { reader.GetCurrentMediaType(stream)? };
        let frame_size = unsafe { current_type.GetUINT64(&MF_MT_FRAME_SIZE)? };
        let width = (frame_size >> 32) as usize;
        let height = (frame_size & 0xffff_ffff) as usize;
        if width == 0 || height == 0 {
            return Err(windows::core::Error::from_hresult(windows::core::HRESULT(
                0x80070057_u32 as i32,
            )));
        }
        let _ = sender.send(MediaEvent::Status(format!("Live — {width} × {height}")));
        while !cancelled.load(Ordering::Acquire) {
            let mut flags = 0_u32;
            let mut sample = None;
            unsafe {
                reader.ReadSample(stream, 0, None, Some(&mut flags), None, Some(&mut sample))?;
            }
            if flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 {
                break;
            }
            let Some(sample) = sample else { continue };
            let buffer = unsafe { sample.ConvertToContiguousBuffer()? };
            let mut pointer = std::ptr::null_mut();
            let mut length = 0_u32;
            unsafe { buffer.Lock(&mut pointer, None, Some(&mut length))? };
            let required = width.saturating_mul(height).saturating_mul(4);
            if !pointer.is_null() && length as usize >= required {
                let bgra = unsafe { std::slice::from_raw_parts(pointer, required) };
                let mut rgba = vec![0_u8; required];
                for (source, target) in bgra.chunks_exact(4).zip(rgba.chunks_exact_mut(4)) {
                    target.copy_from_slice(&[source[2], source[1], source[0], 255]);
                }
                let _ = sender.send(MediaEvent::Frame {
                    width,
                    height,
                    rgba,
                });
            }
            unsafe { buffer.Unlock()? };
        }
        unsafe { reader.Flush(stream)? };
        Ok(())
    }
}
