#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod fits;
mod model;
mod services;
mod theme;

use app::RSeestarApp;
use eframe::egui;

fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RSeestar — Seestar Transfer & Stacking Tool")
            .with_inner_size([800.0, 980.0])
            .with_min_inner_size([720.0, 700.0]),
        renderer: if cfg!(target_os = "windows") {
            eframe::Renderer::Wgpu
        } else {
            eframe::Renderer::Glow
        },
        ..Default::default()
    };

    eframe::run_native(
        "RSeestar",
        options,
        Box::new(|cc| Ok(Box::new(RSeestarApp::new(cc)))),
    )
}

fn main() {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| show_fatal_error(&format!("{info}"))));
    let result = std::panic::catch_unwind(run);
    match result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => show_fatal_error(&format!("RSeestar could not start:\n\n{error}")),
        Err(_) => {}
    }
    std::panic::set_hook(previous_hook);
}

#[cfg(target_os = "windows")]
fn show_fatal_error(message: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
    use windows::core::HSTRING;

    let message = HSTRING::from(message);
    let title = HSTRING::from("RSeestar startup error");
    unsafe {
        MessageBoxW(None, &message, &title, MB_OK | MB_ICONERROR);
    }
}

#[cfg(not(target_os = "windows"))]
fn show_fatal_error(message: &str) {
    eprintln!("{message}");
}
