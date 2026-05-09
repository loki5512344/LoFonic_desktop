use egui::{Color32, Context, FontDefinitions, FontFamily, FontId, Style, TextStyle, Visuals};

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub bg_main: Color32,
    pub bg_panel: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self {
            bg_main: Color32::from_rgb(0x12, 0x12, 0x12),
            bg_panel: Color32::from_rgb(0x1b, 0x1b, 0x1b),
            accent: Color32::from_rgb(0xff, 0x6b, 0x35),
            accent_soft: Color32::from_rgb(0xff, 0x9b, 0x54),
            text_primary: Color32::from_rgb(0xf4, 0xc8, 0x9a),
            text_secondary: Color32::from_rgb(0xc8, 0x8b, 0x5a),
            text_muted: Color32::from_rgb(0x8a, 0x6a, 0x52),
        }
    }
}

pub fn apply_theme(ctx: &Context, palette: ThemePalette) {
    let mut style: Style = (*ctx.global_style()).clone();
    style.visuals = Visuals::dark();
    style.visuals.panel_fill = palette.bg_panel;
    style.visuals.faint_bg_color = palette.bg_main;
    style.visuals.override_text_color = Some(palette.text_primary);

    style.text_styles = [
        (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
        (TextStyle::Name("title".into()), FontId::new(18.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
    ]
    .into();

    // Keep defaults but prepare for font extension.
    let fonts = FontDefinitions::default();
    ctx.set_fonts(fonts);
    ctx.set_global_style(style);
}
