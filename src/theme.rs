use eframe::egui::{self, Color32, CornerRadius, FontFamily, FontId, Stroke, TextStyle, Vec2};
use std::collections::BTreeMap;

pub const BG: Color32 = Color32::from_rgb(3, 5, 11);
pub const PANEL: Color32 = Color32::from_rgb(9, 14, 26);
pub const PANEL_DARK: Color32 = Color32::from_rgb(5, 9, 17);
pub const BORDER: Color32 = Color32::from_rgb(30, 45, 74);
pub const CYAN: Color32 = Color32::from_rgb(0, 240, 255);
pub const BLUE: Color32 = Color32::from_rgb(56, 189, 248);
pub const AMBER: Color32 = Color32::from_rgb(255, 200, 87);
pub const TEXT: Color32 = Color32::from_rgb(241, 245, 249);
pub const MUTED: Color32 = Color32::from_rgb(148, 163, 184);
pub const GREEN: Color32 = Color32::from_rgb(16, 185, 129);
pub const RED: Color32 = Color32::from_rgb(239, 68, 68);

pub fn apply(ctx: &egui::Context, night_vision: bool) {
    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();
    style.spacing.item_spacing = Vec2::new(6.0, 5.0);
    style.spacing.button_padding = Vec2::new(9.0, 5.0);
    style.visuals.dark_mode = true;
    style.visuals.panel_fill = if night_vision {
        Color32::from_rgb(10, 0, 0)
    } else {
        BG
    };
    style.visuals.window_fill = if night_vision {
        Color32::from_rgb(15, 0, 0)
    } else {
        PANEL
    };
    style.visuals.extreme_bg_color = if night_vision {
        Color32::from_rgb(4, 0, 0)
    } else {
        PANEL_DARK
    };
    style.visuals.widgets.noninteractive.bg_fill = style.visuals.window_fill;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.inactive.bg_fill = if night_vision {
        Color32::from_rgb(45, 3, 3)
    } else {
        Color32::from_rgb(17, 27, 48)
    };
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    style.visuals.widgets.hovered.bg_fill = if night_vision {
        Color32::from_rgb(75, 4, 4)
    } else {
        Color32::from_rgb(26, 38, 63)
    };
    style.visuals.widgets.hovered.bg_stroke =
        Stroke::new(1.0, if night_vision { RED } else { CYAN });
    style.visuals.widgets.hovered.fg_stroke =
        Stroke::new(1.0, if night_vision { RED } else { CYAN });
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(13, 92, 70);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, GREEN);
    style.visuals.selection.bg_fill = Color32::from_rgb(13, 92, 70);
    style.visuals.selection.stroke = Stroke::new(1.0, TEXT);
    style.visuals.window_corner_radius = CornerRadius::same(5);
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);
    style.visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    style.visuals.widgets.hovered.corner_radius = CornerRadius::same(4);
    style.visuals.widgets.active.corner_radius = CornerRadius::same(4);
    style.text_styles = BTreeMap::from([
        (
            TextStyle::Small,
            FontId::new(10.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(11.5, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(11.0, FontFamily::Proportional),
        ),
        (TextStyle::Heading, FontId::new(14.0, FontFamily::Monospace)),
        (
            TextStyle::Monospace,
            FontId::new(10.5, FontFamily::Monospace),
        ),
    ]);
    ctx.set_style_of(egui::Theme::Dark, style);
}

pub fn panel_frame(night_vision: bool) -> egui::Frame {
    egui::Frame::new()
        .fill(if night_vision {
            Color32::from_rgb(15, 0, 0)
        } else {
            PANEL
        })
        .stroke(Stroke::new(
            1.0,
            if night_vision {
                Color32::from_rgb(90, 8, 8)
            } else {
                BORDER
            },
        ))
        .corner_radius(CornerRadius::same(5))
        .inner_margin(egui::Margin::symmetric(8, 7))
}
