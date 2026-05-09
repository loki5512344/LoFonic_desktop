use egui::{Color32, ProgressBar, RichText, Ui};

use crate::desktop::playback::TrackSummary;

pub fn audio_metadata_strip(ui: &mut Ui, track: &TrackSummary, text_color: Color32) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(&track.format).color(text_color));
        ui.separator();
        ui.label(
            RichText::new(format!("24bit/{}kHz", track.sample_rate_hz / 1000)).color(text_color),
        );
        ui.separator();
        ui.label(RichText::new(format!("{} kbps", track.bitrate_kbps)).color(text_color));
        ui.separator();
        ui.label(RichText::new(format!("{:.1} LUFS", track.lufs)).color(text_color));
    });
}

pub fn waveform_placeholder(ui: &mut Ui, progress: f32, accent: Color32) {
    ui.add(
        ProgressBar::new(progress.clamp(0.0, 1.0))
            .fill(accent)
            .text("Waveform / progress"),
    );
}
