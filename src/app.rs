use crate::fits::FitsImage;
use crate::model::{
    JobState, LogEntry, Screen, SirilModel, TelescopeDevice, TransferModel, WeatherSnapshot,
};
use crate::services::discovery::{self, DiscoveryEvent, DiscoveryHandle};
use crate::services::media::{self, MediaEvent, MediaHandle};
use crate::services::simulator;
use crate::services::siril::{self, SirilEvent, SirilHandle};
use crate::services::transfer::{self, TransferEvent, TransferHandle};
use crate::services::weather::{self, WeatherHandle};
use crate::theme;
use eframe::egui::{self, Align, Color32, Layout, RichText, ScrollArea, Sense, Stroke, Vec2};
use std::collections::VecDeque;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct LiveSlot {
    handle: Option<MediaHandle>,
    texture: Option<egui::TextureHandle>,
    status: String,
    ip: Option<IpAddr>,
}

pub struct RSeestarApp {
    screen: Screen,
    night_vision: bool,
    devices: Vec<TelescopeDevice>,
    discovery_handle: Option<DiscoveryHandle>,
    transfer: TransferModel,
    transfer_handle: Option<TransferHandle>,
    weather: WeatherSnapshot,
    weather_handle: Option<WeatherHandle>,
    siril: SirilModel,
    siril_handle: Option<SirilHandle>,
    log: VecDeque<LogEntry>,
    siril_log: VecDeque<LogEntry>,
    manual_ip: String,
    show_manual: bool,
    show_streams: bool,
    live_slots: [LiveSlot; 4],
    simulator_active: bool,
    fits_path: String,
    fits_image: Option<FitsImage>,
    fits_texture: Option<egui::TextureHandle>,
    fits_error: Option<String>,
    fits_stretch: f32,
}

impl RSeestarApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx, false);
        let mut app = Self {
            screen: Screen::Landing,
            night_vision: false,
            devices: Vec::new(),
            discovery_handle: None,
            transfer: TransferModel::default(),
            transfer_handle: None,
            weather: WeatherSnapshot {
                status: "Awaiting query initiation...".into(),
                ..Default::default()
            },
            weather_handle: None,
            siril: SirilModel::default(),
            siril_handle: None,
            log: VecDeque::new(),
            siril_log: VecDeque::new(),
            manual_ip: String::new(),
            show_manual: false,
            show_streams: true,
            live_slots: std::array::from_fn(|_| LiveSlot::default()),
            simulator_active: false,
            fits_path: String::new(),
            fits_image: None,
            fits_texture: None,
            fits_error: None,
            fits_stretch: 0.0,
        };
        app.log("[SYSTEM] Native RSeestar application initialized.");
        app
    }

    fn log(&mut self, message: impl Into<String>) {
        self.log.push_back(LogEntry::new(message));
        while self.log.len() > 500 {
            self.log.pop_front();
        }
    }

    fn log_siril(&mut self, message: impl Into<String>) {
        self.siril_log.push_back(LogEntry::new(message));
        while self.siril_log.len() > 500 {
            self.siril_log.pop_front();
        }
    }

    fn navigate(&mut self, screen: Screen) {
        self.screen = screen;
        if screen == Screen::Weather
            && self.weather_handle.is_none()
            && self.weather.location.is_empty()
        {
            self.start_weather_fetch();
        }
        let name = match screen {
            Screen::Landing => "Landing",
            Screen::Portal => "Navigation Portal",
            Screen::Weather => "Weather Page",
            Screen::Transfer => "Scanning and Transfer Tool",
            Screen::Stacking => "Siril CLI Stacking",
        };
        self.log(format!("[UI] Navigated to {name}."));
    }

    fn toggle_night_vision(&mut self, ctx: &egui::Context) {
        self.night_vision = !self.night_vision;
        theme::apply(ctx, self.night_vision);
    }

    fn header_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if self.screen != Screen::Landing
                && self.screen != Screen::Portal
                && ui.small_button("← Back to Navigation Portal").clicked()
            {
                self.navigate(Screen::Portal);
            }
            if ui.small_button("☾ Toggle Night Vision (Red UI)").clicked() {
                self.toggle_night_vision(ctx);
            }
        });
    }

    fn title(&self, ui: &mut egui::Ui, subtitle: &str) {
        let accent = if self.night_vision {
            theme::RED
        } else {
            theme::CYAN
        };
        ui.vertical_centered(|ui| {
            ui.add_space(2.0);
            ui.label(
                RichText::new("NICKTONKS_ASTROPHOTOGRAPHY")
                    .heading()
                    .strong()
                    .color(accent),
            );
            ui.label(
                RichText::new(subtitle)
                    .monospace()
                    .small()
                    .color(theme::AMBER),
            );
            ui.add_space(3.0);
        });
        ui.separator();
    }

    fn section<R>(
        night_vision: bool,
        ui: &mut egui::Ui,
        title: &str,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        theme::panel_frame(night_vision)
            .show(ui, |ui| {
                ui.label(
                    RichText::new(title)
                        .monospace()
                        .strong()
                        .color(theme::AMBER),
                );
                ui.add_space(2.0);
                add_contents(ui)
            })
            .inner
    }

    fn landing(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.header_controls(ui, ctx);
        self.title(
            ui,
            "Seestar Station Mode File Organiser, Transfer, Live View & Stacking v1.0",
        );

        Self::section(self.night_vision, ui, "SUPPORTED CONNECTION MODES", |ui| {
            ui.columns(2, |columns| {
                card(
                    &mut columns[0],
                    "Home Wi-Fi (Station Mode)",
                    "PC and Seestar connect through the local router. Addresses may be assigned dynamically by DHCP.",
                );
                card(
                    &mut columns[1],
                    "Direct Seestar Wi-Fi",
                    "PC connects directly to the telescope hotspot, typically using the address 10.0.0.1.",
                );
            });
        });
        Self::section(self.night_vision, ui, "DEVICE DISCOVERY OPTIONS", |ui| {
            ui.columns(2, |columns| {
                card(
                    &mut columns[0],
                    "Automatic Network Scan",
                    "Searches the active local subnet and validates candidate Seestar services.",
                );
                card(
                    &mut columns[1],
                    "Manual IP Address",
                    "Validates a known telescope address for direct Wi-Fi or reserved DHCP setups.",
                );
            });
        });
        Self::section(
            self.night_vision,
            ui,
            "AUTOMATED DESTINATION FOLDER HIERARCHY",
            |ui| {
                ui.label("Downloaded files are organized by target object and file type:");
                ui.add_space(3.0);
                ui.monospace("[Destination]\\[Target]\\Lights   (.fit/.fits)");
                ui.monospace("[Destination]\\[Target]\\Video    (.avi/.mp4)");
                ui.monospace("[Destination]\\[Target]\\JPEGS    (.jpg/.jpeg)");
                ui.monospace("[Destination]\\Seestar_1.log");
            },
        );
        Self::section(
            self.night_vision,
            ui,
            "LIVE VIEW & SIRIL PROCESSING",
            |ui| {
                ui.label("Up to four telescope RTSP feeds can be monitored at once. Siril CLI performs calibration, plate solving, registration, and stacking; the integrated viewer previews FITS results.");
            },
        );
        Self::section(
            self.night_vision,
            ui,
            "IMPORTANT TOOL DISCLOSURES & PROTOCOLS",
            |ui| {
                ui.label("Transfers are restartable and skip verified existing files. RSeestar does not alter Windows guest-authentication registry policy during normal operation.");
            },
        );

        ui.vertical_centered(|ui| {
            ui.add_space(4.0);
            if ui
                .add_sized(
                    [430.0, 32.0],
                    egui::Button::new("Continue to Navigation Portal →")
                        .fill(Color32::from_rgb(13, 92, 70))
                        .stroke(Stroke::new(1.0, theme::GREEN)),
                )
                .clicked()
            {
                self.navigate(Screen::Portal);
            }
        });
    }

    fn portal(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.header_controls(ui, ctx);
        self.title(ui, "Navigation Portal — Select Destination Module");
        ui.add_space(8.0);
        for (label, description, screen) in [
            (
                "☁  1. Weather Page",
                "Live observatory conditions and forecast",
                Screen::Weather,
            ),
            (
                "▣  2. Scanning and Transfer Tool",
                "Discovery, file ingest, and live view",
                Screen::Transfer,
            ),
            (
                "✦  3. Siril CLI Stacking",
                "Advanced stacking and FITS preview",
                Screen::Stacking,
            ),
        ] {
            let response = theme::panel_frame(self.night_vision).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(label).monospace().strong().color(
                            if self.night_vision {
                                theme::RED
                            } else {
                                theme::CYAN
                            },
                        ));
                        ui.label(RichText::new(description).small().color(theme::MUTED));
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Open →").strong());
                    });
                });
            });
            if response.response.interact(Sense::click()).clicked() {
                self.navigate(screen);
            }
            ui.add_space(3.0);
        }
    }

    fn transfer_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.header_controls(ui, ctx);
        self.title(
            ui,
            "Seestar Station Mode File Organiser, Transfer, Live View & Stacking v1.0",
        );

        Self::section(
            self.night_vision,
            ui,
            "1. Telescope Node Discovery:",
            |ui| {
                ui.columns(2, |columns| {
                    if columns[0].button("Run Automatic Scan").clicked() {
                        self.start_discovery();
                    }
                    if columns[1].button("Add Manual IPs").clicked() {
                        self.show_manual = !self.show_manual;
                    }
                });
                if self.show_manual {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("IP:");
                        ui.text_edit_singleline(&mut self.manual_ip);
                        if ui.button("Add IP").clicked() {
                            self.add_manual_device();
                        }
                    });
                }
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&self.transfer.status)
                            .monospace()
                            .small()
                            .color(theme::AMBER),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button("Load Test Nodes").clicked() {
                            self.load_simulator();
                        }
                    });
                });
            },
        );

        Self::section(
            self.night_vision,
            ui,
            "2. Active Telescope Connection Details:",
            |ui| {
                if self.devices.is_empty() {
                    ui.label(
                        RichText::new("Devices will connect once discovery is complete.")
                            .color(theme::MUTED),
                    );
                } else {
                    let devices = self.devices.clone();
                    for (index, device) in devices.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("◉ [ONLINE_NODE_{}]", index + 1))
                                    .monospace()
                                    .color(theme::GREEN),
                            );
                            ui.label(device.ip.to_string());
                            ui.label(
                                RichText::new(&device.share_path)
                                    .monospace()
                                    .small()
                                    .color(theme::CYAN),
                            );
                            if device.simulated {
                                ui.label(RichText::new("SIMULATED").small().color(theme::AMBER));
                            }
                        });
                    }
                }
            },
        );

        Self::section(
            self.night_vision,
            ui,
            "3. Connected Device Live Stream Monitors:",
            |ui| {
                ui.horizontal(|ui| {
                    if ui.small_button("Stop All Feeds").clicked() {
                        self.stop_all_streams();
                        self.log("[RTSP_STOP] All live feeds stopped.");
                    }
                    if ui
                        .small_button(if self.show_streams {
                            "[-] Minimize"
                        } else {
                            "[+] Expand"
                        })
                        .clicked()
                    {
                        self.show_streams = !self.show_streams;
                    }
                });
                if self.show_streams {
                    ui.add_space(4.0);
                    egui::Grid::new("stream_grid")
                        .num_columns(2)
                        .spacing([5.0, 5.0])
                        .show(ui, |ui| {
                            for slot in 0..4 {
                                self.stream_card(ui, slot);
                                if slot % 2 == 1 {
                                    ui.end_row();
                                }
                            }
                        });
                }
            },
        );

        Self::section(self.night_vision, ui, "4. Save Data Directory:", |ui| {
            let mut destination = self.transfer.destination.to_string_lossy().into_owned();
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut destination).desired_width(f32::INFINITY));
                if ui.button("Use Current Folder").clicked() {
                    destination = std::env::current_dir()
                        .unwrap_or_default()
                        .display()
                        .to_string();
                }
            });
            self.transfer.destination = PathBuf::from(destination);
        });

        Self::section(self.night_vision, ui, "5. File Types to be Saved:", |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.transfer.selection.fits, "FITS Data (.fit/.fits)");
                ui.checkbox(&mut self.transfer.selection.video, "Video (.avi/.mp4)");
                ui.checkbox(
                    &mut self.transfer.selection.images,
                    "Snapshots (.jpg/.jpeg)",
                );
            });
        });

        Self::section(self.night_vision, ui, "6. Data Transfer Control:", |ui| {
            ui.columns(3, |columns| {
                if columns[0].button("Start Sync").clicked() {
                    self.start_transfer();
                }
                if columns[1].button("Pause Sync").clicked() {
                    self.pause_transfer();
                }
                if columns[2].button("Resume Sync").clicked() {
                    self.resume_transfer();
                }
            });
            if self.transfer.state != JobState::Idle {
                ui.add(
                    egui::ProgressBar::new(self.transfer.progress)
                        .show_percentage()
                        .text(self.transfer.state.to_string()),
                );
            }
        });

        Self::section(
            self.night_vision,
            ui,
            "7. Observatory Transfer Log:",
            |ui| {
                log_view(ui, &self.log, 92.0);
            },
        );
        ui.vertical_centered(|ui| {
            if ui.button("Stacking and Preview Page").clicked() {
                self.navigate(Screen::Stacking);
            }
        });
    }

    fn stream_card(&self, ui: &mut egui::Ui, slot: usize) {
        let available = ui.available_width();
        let width = (available - 8.0) / 2.0;
        let device = self.devices.get(slot);
        egui::Frame::new()
            .fill(Color32::BLACK)
            .stroke(Stroke::new(1.0, theme::BORDER))
            .corner_radius(4)
            .inner_margin(5)
            .show(ui, |ui| {
                ui.set_min_width(width.max(180.0));
                let title = device
                    .map(|d| format!("SLOT {} [{}]", slot + 1, d.ip))
                    .unwrap_or_else(|| format!("SLOT {} [IDLE]", slot + 1));
                ui.label(RichText::new(title).monospace().small().color(theme::AMBER));
                let size = Vec2::new(width.max(180.0), 105.0);
                if let Some(texture) = &self.live_slots[slot].texture {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(size));
                } else {
                    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, Color32::BLACK);
                    ui.painter().rect_stroke(
                        rect,
                        2.0,
                        Stroke::new(
                            1.0,
                            if self.night_vision {
                                theme::RED
                            } else {
                                theme::CYAN
                            },
                        ),
                        egui::StrokeKind::Inside,
                    );
                    let message = if device.is_some() {
                        &self.live_slots[slot].status
                    } else {
                        "Waiting for Node…"
                    };
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        message,
                        egui::FontId::monospace(9.5),
                        theme::MUTED,
                    );
                }
            });
    }

    fn weather_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.header_controls(ui, ctx);
        self.title(ui, "Scientific Observatory Weather & Seeing Dashboard");
        Self::section(
            self.night_vision,
            ui,
            "Live Observatory Conditions & Sky Telemetry:",
            |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&self.weather.status)
                            .monospace()
                            .small()
                            .color(theme::AMBER),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button("Refresh Weather").clicked() {
                            self.start_weather_fetch();
                        }
                    });
                });
                ui.add_space(4.0);
                egui::Grid::new("weather_cards")
                    .num_columns(3)
                    .spacing([5.0, 5.0])
                    .show(ui, |ui| {
                        metric(
                            ui,
                            "Detected Location",
                            or_dash(&self.weather.location),
                            "IP Geolocation",
                        );
                        metric(
                            ui,
                            "Cloud Cover",
                            option_unit(self.weather.cloud_percent, "%"),
                            "Atmospheric Cover",
                        );
                        metric(
                            ui,
                            "Sky Quality",
                            "Not calculated".into(),
                            "Requires location model",
                        );
                        ui.end_row();
                        metric(
                            ui,
                            "Ambient Temperature",
                            self.weather
                                .temperature_c
                                .map(|v| format!("{v:.1} °C"))
                                .unwrap_or_else(|| "--".into()),
                            "Surface Air Temp",
                        );
                        metric(
                            ui,
                            "Relative Humidity",
                            option_unit(self.weather.humidity_percent, "%"),
                            "Dew Point Risk",
                        );
                        metric(
                            ui,
                            "Wind & Visibility",
                            self.weather
                                .wind_kph
                                .map(|v| format!("{v:.1} km/h"))
                                .unwrap_or_else(|| "--".into()),
                            &self
                                .weather
                                .visibility_km
                                .map(|v| format!("Visibility: {v:.1} km"))
                                .unwrap_or_else(|| "Visibility: --".into()),
                        );
                        ui.end_row();
                    });
            },
        );
        Self::section(
            self.night_vision,
            ui,
            "Hourly Cloud Cover & Weather Trend (Next 24 Hours):",
            |ui| {
                if self.weather.hourly.is_empty() {
                    ui.label(
                        RichText::new("Awaiting hourly telemetry…")
                            .monospace()
                            .color(theme::MUTED),
                    );
                } else {
                    ScrollArea::horizontal().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for hour in &self.weather.hourly {
                                let temperature = hour
                                    .temperature_c
                                    .map(|value| format!("{value:.1} °C"))
                                    .unwrap_or_else(|| "--".into());
                                let cloud = hour
                                    .cloud_percent
                                    .map(|value| format!("Cloud: {value}%"))
                                    .unwrap_or_else(|| "Cloud: --".into());
                                metric(ui, &hour.time, temperature, &cloud);
                            }
                        });
                    });
                }
            },
        );
        Self::section(
            self.night_vision,
            ui,
            "7-Day Atmospheric & Forecast Trend:",
            |ui| {
                if self.weather.daily.is_empty() {
                    ui.label(
                        RichText::new("Awaiting forecast telemetry…")
                            .monospace()
                            .color(theme::MUTED),
                    );
                } else {
                    ScrollArea::horizontal().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for day in &self.weather.daily {
                                let temperatures = match (day.maximum_c, day.minimum_c) {
                                    (Some(maximum), Some(minimum)) => {
                                        format!("{maximum:.1} / {minimum:.1} °C")
                                    }
                                    _ => "--".into(),
                                };
                                let rain = day
                                    .rain_percent
                                    .map(|value| format!("Rain: {value}%"))
                                    .unwrap_or_else(|| "Rain: --".into());
                                metric(ui, &day.date, temperatures, &rain);
                            }
                        });
                    });
                }
            },
        );
    }

    fn stacking_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.header_controls(ui, ctx);
        self.title(ui, "Siril CLI Advanced Stacking Workspace");
        Self::section(self.night_vision, ui, "1. Siril Executable Path:", |ui| {
            let mut path = self.siril.executable.to_string_lossy().into_owned();
            ui.add(egui::TextEdit::singleline(&mut path).desired_width(f32::INFINITY));
            self.siril.executable = PathBuf::from(path);
        });
        Self::section(self.night_vision, ui, "2. Select Target Folder:", |ui| {
            ui.label(RichText::new("Select the top-level target folder, for example M71_Subs rather than M71_Subs\\Lights.").small());
            let mut path = self.siril.working_directory.to_string_lossy().into_owned();
            if ui
                .add(egui::TextEdit::singleline(&mut path).desired_width(f32::INFINITY))
                .lost_focus()
            {
                self.siril.target_name = target_name_from_path(Path::new(&path));
            }
            self.siril.working_directory = PathBuf::from(path);
        });
        Self::section(self.night_vision, ui, "3. Target Name:", |ui| {
            ui.text_edit_singleline(&mut self.siril.target_name);
        });
        Self::section(self.night_vision, ui, "4. Execution Control:", |ui| {
            ui.columns(2, |columns| {
                if columns[0].button("Run Siril Stacking").clicked() {
                    self.start_siril();
                }
                if columns[1].button("Stop Siril Stacking").clicked() {
                    self.stop_siril();
                }
            });
        });
        Self::section(self.night_vision, ui, "5. Siril Execution Log:", |ui| {
            log_view(ui, &self.siril_log, 82.0);
        });
        Self::section(
            self.night_vision,
            ui,
            "6. Stacked FITS Preview (Not a full resolution preview):",
            |ui| {
                ui.label(
                    RichText::new("Select any astronomy .fit/.fits image or stack to preview:")
                        .strong(),
                );
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fits_path)
                            .desired_width(f32::INFINITY)
                            .hint_text("Path to FITS file…"),
                    );
                    if ui.button("Load FITS").clicked() {
                        self.load_fits(ctx);
                    }
                });
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(self.fits_stretch == 0.0, "Original View")
                        .clicked()
                    {
                        self.fits_stretch = 0.0;
                        self.refresh_fits_texture(ctx);
                    }
                    if ui
                        .selectable_label(self.fits_stretch > 0.0, "Auto-Stretch")
                        .clicked()
                    {
                        self.fits_stretch = 6.0;
                        self.refresh_fits_texture(ctx);
                    }
                    if self.fits_stretch > 0.0 {
                        let changed = ui
                            .add(
                                egui::Slider::new(&mut self.fits_stretch, 1.0..=10.0)
                                    .step_by(0.5)
                                    .text("Intensity"),
                            )
                            .changed();
                        if changed {
                            self.refresh_fits_texture(ctx);
                        }
                    }
                });
                if let Some(error) = &self.fits_error {
                    ui.colored_label(theme::RED, error);
                }
                if let Some(image) = &self.fits_image {
                    let object = image
                        .headers
                        .get("OBJECT")
                        .map(String::as_str)
                        .unwrap_or("Unknown");
                    ui.label(
                        RichText::new(format!(
                            "Object: {object} | Dimensions: {} × {} | BITPIX: {} | BSCALE: {} | BZERO: {} | Header cards: {}",
                            image.width,
                            image.height,
                            image.bitpix,
                            image.bscale,
                            image.bzero,
                            image.headers.len()
                        ))
                        .monospace()
                        .small()
                        .color(theme::BLUE),
                    );
                }
                if let Some(texture) = &self.fits_texture {
                    let available = ui.available_width().min(600.0);
                    let size = texture.size_vec2();
                    let scale = (available / size.x).min(420.0 / size.y).min(1.0);
                    ScrollArea::both().max_height(430.0).show(ui, |ui| {
                        ui.image((texture.id(), size * scale));
                    });
                }
            },
        );
    }

    fn start_discovery(&mut self) {
        if let Some(handle) = self.discovery_handle.take() {
            handle.cancel();
        }
        self.devices.clear();
        self.stop_all_streams();
        self.simulator_active = false;
        self.transfer.status = "SCAN STATUS: DISCOVERY STARTING…".into();
        self.log("[SUBNET_SWEEP] Automatic discovery requested.");
        self.discovery_handle = Some(discovery::start());
    }

    fn load_simulator(&mut self) {
        match simulator::provision(&simulator::default_root()) {
            Ok(devices) => {
                self.devices = devices;
                self.simulator_active = true;
                self.transfer.status =
                    format!("SCAN STATUS: SIMULATION ({} ACTIVE)", self.devices.len());
                self.log("[SIMULATOR] Loaded deterministic Seestar test nodes and fixtures.");
                self.sync_streams();
            }
            Err(error) => self.log(format!("[ERROR] Could not provision simulator: {error}")),
        }
    }

    fn add_manual_device(&mut self) {
        match self.manual_ip.trim().parse::<IpAddr>() {
            Ok(ip) if self.devices.iter().any(|device| device.ip == ip) => {
                self.log(format!("[WARNING] IP already present: {ip}"));
            }
            Ok(ip) if discovery::validate(ip) => {
                self.devices
                    .push(TelescopeDevice::new(format!("Seestar-{ip}"), ip, false));
                self.transfer.status =
                    format!("SCAN STATUS: MANUAL CONFIG ({} ACTIVE)", self.devices.len());
                self.log(format!(
                    "[MANUAL_NODE] Added IP pending service validation: {ip}"
                ));
                self.manual_ip.clear();
                self.sync_streams();
            }
            Ok(ip) => self.log(format!(
                "[REJECTED] No SMB or RTSP Seestar service responded at {ip}."
            )),
            Err(_) => self.log("[ERROR] Manual IP address is invalid."),
        }
    }

    fn start_transfer(&mut self) {
        if self.devices.is_empty() {
            self.log("[ERROR] No connected Seestar devices found.");
            self.transfer.state = JobState::Failed;
        } else if self.transfer.destination.as_os_str().is_empty() {
            self.log("[ERROR] Select a destination directory first.");
            self.transfer.state = JobState::Failed;
        } else if !self.transfer.selection.any() {
            self.log("[ERROR] Select at least one file type to transfer.");
            self.transfer.state = JobState::Failed;
        } else {
            self.transfer.state = JobState::Running;
            self.transfer.progress = 0.0;
            self.transfer_handle = Some(transfer::start(
                self.devices.clone(),
                self.transfer.destination.clone(),
                self.transfer.selection,
            ));
        }
    }

    fn pause_transfer(&mut self) {
        if self.transfer.state == JobState::Running {
            if let Some(handle) = &self.transfer_handle {
                handle.pause();
            }
            self.transfer.state = JobState::Paused;
            self.log("[TRANSFER_PAUSED] Sync process suspended by operator.");
        } else {
            self.log("[TRANSFER] No active transfer process to pause.");
        }
    }

    fn resume_transfer(&mut self) {
        if self.transfer.state == JobState::Paused {
            if let Some(handle) = &self.transfer_handle {
                handle.resume();
            }
            self.transfer.state = JobState::Running;
            self.log("[TRANSFER] Resuming active file copy routine.");
        } else {
            self.log("[TRANSFER] No paused transfer is available to resume.");
        }
    }

    fn start_siril(&mut self) {
        if !self.siril.executable.is_file() {
            self.siril.state = JobState::Failed;
            self.log_siril(format!(
                "[ERROR] Siril executable not found: {}",
                self.siril.executable.display()
            ));
        } else if !self.siril.working_directory.is_dir() {
            self.siril.state = JobState::Failed;
            self.log_siril("[ERROR] Select a valid target folder containing a Lights directory.");
        } else {
            self.log_siril("=================================================");
            self.log_siril("[SIRIL_INIT] Generating and executing mosaic stacking script…");
            self.log_siril("=================================================");
            match siril::start(
                self.siril.executable.clone(),
                self.siril.working_directory.clone(),
                self.siril.target_name.clone(),
            ) {
                Ok(handle) => {
                    self.siril.state = JobState::Running;
                    self.siril_handle = Some(handle);
                }
                Err(error) => {
                    self.siril.state = JobState::Failed;
                    self.log_siril(format!("[ERROR] Could not launch Siril: {error}"));
                }
            }
        }
    }

    fn stop_siril(&mut self) {
        if let Some(handle) = &self.siril_handle {
            self.siril.state = JobState::Stopping;
            handle.cancel();
            self.log_siril("[SIRIL] Stop requested for the active Siril process.");
        } else {
            self.log_siril("[SIRIL] No active stacking process to stop.");
        }
    }

    fn load_fits(&mut self, ctx: &egui::Context) {
        self.fits_error = None;
        match fs::read(self.fits_path.trim()) {
            Ok(bytes) => match FitsImage::parse(&bytes) {
                Ok(image) => {
                    self.log_siril(format!(
                        "[FITS] Loaded {} × {} image.",
                        image.width, image.height
                    ));
                    self.fits_image = Some(image);
                    self.refresh_fits_texture(ctx);
                }
                Err(error) => self.fits_error = Some(error.to_string()),
            },
            Err(error) => self.fits_error = Some(format!("Could not read FITS file: {error}")),
        }
    }

    fn refresh_fits_texture(&mut self, ctx: &egui::Context) {
        let Some(image) = &self.fits_image else {
            return;
        };
        let rgba = image.preview_rgba(self.fits_stretch);
        let color = egui::ColorImage::from_rgba_unmultiplied([image.width, image.height], &rgba);
        self.fits_texture =
            Some(ctx.load_texture("fits-preview", color, egui::TextureOptions::LINEAR));
    }

    fn poll_transfer(&mut self, ctx: &egui::Context) {
        let mut events = Vec::new();
        if let Some(handle) = &self.transfer_handle {
            while let Ok(event) = handle.try_recv() {
                events.push(event);
            }
        }
        let mut finished = false;
        for event in events {
            match event {
                TransferEvent::Log(message) => self.log(message),
                TransferEvent::Progress { completed, total } => {
                    self.transfer.progress = if total == 0 {
                        1.0
                    } else {
                        completed as f32 / total as f32
                    };
                }
                TransferEvent::Finished => {
                    self.transfer.state = JobState::Complete;
                    self.transfer.progress = 1.0;
                    self.log("=================================================");
                    self.log("[TRANSFER_COMPLETE] Ingest sequence finished.");
                    self.log("=================================================");
                    finished = true;
                }
                TransferEvent::Cancelled => {
                    self.transfer.state = JobState::Idle;
                    self.log("[TRANSFER_STOPPED] Sync process cancelled.");
                    finished = true;
                }
                TransferEvent::Failed(message) => {
                    self.transfer.state = JobState::Failed;
                    self.log(format!("[ERROR] Transfer failed: {message}"));
                    finished = true;
                }
            }
        }
        if finished {
            self.transfer_handle = None;
        } else if self.transfer_handle.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
    }

    fn poll_discovery(&mut self, ctx: &egui::Context) {
        let mut events = Vec::new();
        if let Some(handle) = &self.discovery_handle {
            while let Ok(event) = handle.try_recv() {
                events.push(event);
            }
        }
        let mut finished = false;
        for event in events {
            match event {
                DiscoveryEvent::Status(status) => {
                    self.transfer.status = format!("SCAN STATUS: {}", status.to_uppercase());
                    self.log(format!("[SCAN] {status}"));
                }
                DiscoveryEvent::Found(device) => {
                    self.log(format!("[FOUND] Seestar service candidate - {}", device.ip));
                    self.devices.push(device);
                    self.sync_streams();
                }
                DiscoveryEvent::Finished { checked } => {
                    self.transfer.status = if self.devices.is_empty() {
                        "SCAN STATUS: NO TELESCOPE NODES DISCOVERED".into()
                    } else {
                        format!(
                            "SCAN STATUS: SWEEP COMPLETE ({} ACTIVE)",
                            self.devices.len()
                        )
                    };
                    self.log(format!(
                        "[SWEEP_COMPLETE] Checked {checked} addresses; telescope candidates: {}",
                        self.devices.len()
                    ));
                    finished = true;
                }
                DiscoveryEvent::Failed(message) => {
                    self.transfer.status = "SCAN STATUS: DISCOVERY FAILED".into();
                    self.log(format!("[ERROR] {message}"));
                    finished = true;
                }
            }
        }
        if finished {
            self.discovery_handle = None;
        } else if self.discovery_handle.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }

    fn poll_siril(&mut self, ctx: &egui::Context) {
        let mut events = Vec::new();
        if let Some(handle) = &self.siril_handle {
            while let Ok(event) = handle.try_recv() {
                events.push(event);
            }
        }
        let mut finished = false;
        for event in events {
            match event {
                SirilEvent::Log(message) => self.log_siril(message),
                SirilEvent::Finished { output } => {
                    self.siril.state = JobState::Complete;
                    self.log_siril(format!(
                        "[SUCCESS] Stacking complete! Final image saved to: {}",
                        output.display()
                    ));
                    self.fits_path = output.display().to_string();
                    finished = true;
                }
                SirilEvent::Failed(message) => {
                    self.siril.state = JobState::Failed;
                    self.log_siril(format!("[ERROR] {message}"));
                    finished = true;
                }
                SirilEvent::Cancelled => {
                    self.siril.state = JobState::Idle;
                    self.log_siril("[SIRIL] Stacking process stopped by operator.");
                    finished = true;
                }
            }
        }
        if finished {
            self.siril_handle = None;
        } else if self.siril_handle.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }

    fn start_weather_fetch(&mut self) {
        self.weather.status = "Querying live atmospheric telemetry and forecast…".into();
        self.weather_handle = Some(weather::fetch());
        self.log("[WEATHER] Initializing geolocation and atmospheric condition query.");
    }

    fn poll_weather(&mut self, ctx: &egui::Context) {
        let result = self
            .weather_handle
            .as_ref()
            .and_then(|handle| handle.try_recv().ok());
        if let Some(result) = result {
            match result {
                Ok(snapshot) => {
                    self.log(format!(
                        "[WEATHER] Observatory telemetry loaded for {}.",
                        snapshot.location
                    ));
                    self.weather = snapshot;
                }
                Err(error) => {
                    self.weather.status = format!("Weather query failed: {error}");
                    self.log(format!("[WEATHER] {error}"));
                }
            }
            self.weather_handle = None;
        } else if self.weather_handle.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }

    fn sync_streams(&mut self) {
        self.stop_all_streams();
        for (index, device) in self.devices.iter().take(4).enumerate() {
            self.live_slots[index].ip = Some(device.ip);
            if device.simulated {
                self.live_slots[index].status = "SIMULATED RTSP TEST PATTERN".into();
            } else {
                self.live_slots[index].status = "CONNECTING TO LIVE FEED…".into();
                self.live_slots[index].handle = Some(media::start(device.rtsp_url.clone()));
            }
        }
    }

    fn stop_all_streams(&mut self) {
        for slot in &mut self.live_slots {
            if let Some(handle) = slot.handle.take() {
                handle.stop();
            }
            slot.texture = None;
            slot.ip = None;
            slot.status = "Waiting for Node…".into();
        }
    }

    fn poll_media(&mut self, ctx: &egui::Context) {
        for (index, slot) in self.live_slots.iter_mut().enumerate() {
            let mut events = Vec::new();
            if let Some(handle) = &slot.handle {
                while let Ok(event) = handle.try_recv() {
                    events.push(event);
                }
            }
            for event in events {
                match event {
                    MediaEvent::Status(status) => slot.status = status,
                    MediaEvent::Frame {
                        width,
                        height,
                        rgba,
                    } => {
                        let image =
                            egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba);
                        if let Some(texture) = slot.texture.as_mut() {
                            texture.set(image, egui::TextureOptions::LINEAR);
                        } else {
                            slot.texture = Some(ctx.load_texture(
                                format!("rtsp-slot-{index}"),
                                image,
                                egui::TextureOptions::LINEAR,
                            ));
                        }
                    }
                    MediaEvent::Failed(message) => {
                        slot.status = message;
                        slot.handle = None;
                    }
                    MediaEvent::Stopped => {
                        slot.status = "Feed stopped".into();
                        slot.handle = None;
                    }
                }
            }
            if slot.handle.is_some() {
                ctx.request_repaint_after(std::time::Duration::from_millis(16));
            }
        }
    }
}

impl eframe::App for RSeestarApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll_discovery(&ctx);
        self.poll_transfer(&ctx);
        self.poll_siril(&ctx);
        self.poll_weather(&ctx);
        self.poll_media(&ctx);
        draw_grid_background(ui, self.night_vision);
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(900.0);
                match self.screen {
                    Screen::Landing => self.landing(ui, &ctx),
                    Screen::Portal => self.portal(ui, &ctx),
                    Screen::Weather => self.weather_page(ui, &ctx),
                    Screen::Transfer => self.transfer_page(ui, &ctx),
                    Screen::Stacking => self.stacking_page(ui, &ctx),
                }
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("RSeestar native compatibility build")
                            .small()
                            .color(theme::MUTED),
                    );
                });
            });
    }
}

fn draw_grid_background(ui: &egui::Ui, night_vision: bool) {
    let rect = ui.max_rect();
    let color = if night_vision {
        Color32::from_rgb(24, 2, 2)
    } else {
        Color32::from_rgb(12, 20, 35)
    };
    let painter = ui.painter();
    let step = 24.0;
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            Stroke::new(0.5, color),
        );
        x += step;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            Stroke::new(0.5, color),
        );
        y += step;
    }
}

fn card(ui: &mut egui::Ui, title: &str, body: &str) {
    egui::Frame::new()
        .fill(theme::PANEL_DARK)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .corner_radius(4)
        .inner_margin(6)
        .show(ui, |ui| {
            ui.label(RichText::new(title).strong().color(theme::AMBER));
            ui.label(RichText::new(body).small().color(theme::TEXT));
        });
}

fn metric(ui: &mut egui::Ui, title: &str, value: String, subtitle: &str) {
    egui::Frame::new()
        .fill(theme::PANEL_DARK)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .corner_radius(4)
        .inner_margin(6)
        .show(ui, |ui| {
            ui.set_min_width(180.0);
            ui.label(RichText::new(title).monospace().small().color(theme::AMBER));
            ui.label(RichText::new(value).strong().color(theme::CYAN));
            ui.label(RichText::new(subtitle).small().color(theme::MUTED));
        });
}

fn log_view(ui: &mut egui::Ui, entries: &VecDeque<LogEntry>, height: f32) {
    egui::Frame::new()
        .fill(theme::BG)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .inner_margin(4)
        .show(ui, |ui| {
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .max_height(height)
                .min_scrolled_height(height)
                .show(ui, |ui| {
                    for entry in entries {
                        let _ = entry.at;
                        ui.label(
                            RichText::new(&entry.message)
                                .monospace()
                                .small()
                                .color(theme::BLUE),
                        );
                    }
                });
        });
}

fn target_name_from_path(path: &Path) -> String {
    let ignored = ["lights", "video", "jpegs", "subfolder"];
    let path_text = path.to_string_lossy();
    let mut parts: Vec<&str> = path_text
        .trim_end_matches(['\\', '/'])
        .split(['\\', '/'])
        .filter(|part| !part.is_empty())
        .collect();
    if parts.last().is_some_and(|name| {
        ignored
            .iter()
            .any(|ignored| name.eq_ignore_ascii_case(ignored))
    }) {
        parts.pop();
    }
    let name = parts.last().copied().unwrap_or("Target");
    if name.to_ascii_lowercase().ends_with("_subs") {
        name[..name.len() - 5].to_string()
    } else {
        name.to_string()
    }
}

fn option_unit(value: Option<u8>, unit: &str) -> String {
    value
        .map(|value| format!("{value} {unit}"))
        .unwrap_or_else(|| "--".into())
}

fn or_dash(value: &str) -> String {
    if value.is_empty() {
        "--".into()
    } else {
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_name_uses_parent_for_lights() {
        assert_eq!(
            target_name_from_path(Path::new(r"C:\Data\M71_Subs\Lights")),
            "M71"
        );
    }

    #[test]
    fn target_name_uses_selected_folder() {
        assert_eq!(target_name_from_path(Path::new(r"C:\Data\M42")), "M42");
    }
}
