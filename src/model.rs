use std::fmt;
use std::net::IpAddr;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Screen {
    #[default]
    Landing,
    Portal,
    Weather,
    Transfer,
    Stacking,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum JobState {
    #[default]
    Idle,
    Running,
    Paused,
    Stopping,
    Complete,
    Failed,
}

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Idle => "IDLE",
            Self::Running => "RUNNING",
            Self::Paused => "PAUSED",
            Self::Stopping => "STOPPING",
            Self::Complete => "COMPLETE",
            Self::Failed => "FAILED",
        };
        f.write_str(text)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TelescopeDevice {
    pub name: String,
    pub ip: IpAddr,
    pub share_path: String,
    pub rtsp_url: String,
    pub source_path: PathBuf,
    pub simulated: bool,
}

impl TelescopeDevice {
    pub fn new(name: impl Into<String>, ip: IpAddr, simulated: bool) -> Self {
        let share_path = format!(r"\\{}\EMMC Images\MyWorks", ip);
        Self {
            name: name.into(),
            ip,
            source_path: PathBuf::from(&share_path),
            share_path,
            rtsp_url: format!("rtsp://{ip}:4554/stream"),
            simulated,
        }
    }

    pub fn with_source_path(mut self, source_path: PathBuf) -> Self {
        self.source_path = source_path;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferSelection {
    pub fits: bool,
    pub video: bool,
    pub images: bool,
}

impl Default for TransferSelection {
    fn default() -> Self {
        Self {
            fits: true,
            video: true,
            images: true,
        }
    }
}

impl TransferSelection {
    pub fn any(self) -> bool {
        self.fits || self.video || self.images
    }
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub at: SystemTime,
    pub message: String,
}

impl LogEntry {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            at: SystemTime::now(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TransferModel {
    pub destination: PathBuf,
    pub selection: TransferSelection,
    pub state: JobState,
    pub progress: f32,
    pub status: String,
}

impl Default for TransferModel {
    fn default() -> Self {
        Self {
            destination: PathBuf::new(),
            selection: TransferSelection::default(),
            state: JobState::Idle,
            progress: 0.0,
            status: "SCAN STATUS: AWAITING OPERATOR INPUT".into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct WeatherSnapshot {
    pub location: String,
    pub temperature_c: Option<f32>,
    pub humidity_percent: Option<u8>,
    pub cloud_percent: Option<u8>,
    pub wind_kph: Option<f32>,
    pub visibility_km: Option<f32>,
    pub status: String,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
}

#[derive(Clone, Debug, Default)]
pub struct HourlyForecast {
    pub time: String,
    pub temperature_c: Option<f32>,
    pub cloud_percent: Option<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct DailyForecast {
    pub date: String,
    pub maximum_c: Option<f32>,
    pub minimum_c: Option<f32>,
    pub rain_percent: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct SirilModel {
    pub executable: PathBuf,
    pub working_directory: PathBuf,
    pub target_name: String,
    pub state: JobState,
}

impl Default for SirilModel {
    fn default() -> Self {
        Self {
            executable: PathBuf::from(r"C:\Program Files\Siril\bin\siril-cli.exe"),
            working_directory: PathBuf::new(),
            target_name: "Target".into(),
            state: JobState::Idle,
        }
    }
}
