use egui::{Button, RichText, SidePanel, TopBottomPanel, Ui};

use crate::desktop::{
    screens::Screen,
    state::AppState,
    theme::ThemePalette,
    widgets::{audio_metadata_strip, waveform_placeholder},
};

pub fn sidebar(ctx: &egui::Context, app_state: &mut AppState, palette: ThemePalette) {
    SidePanel::left("sidebar")
        .min_width(220.0)
        .max_width(260.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("LoFonic").color(palette.accent));
            ui.add_space(8.0);
            for screen in Screen::ALL {
                let selected = app_state.current_screen == screen;
                let button = Button::new(screen.label())
                    .fill(if selected {
                        palette.accent
                    } else {
                        palette.bg_panel
                    })
                    .corner_radius(10);
                if ui.add(button).clicked() {
                    app_state.current_screen = screen;
                }
            }
        });
}

pub fn right_queue_panel(ctx: &egui::Context, app_state: &mut AppState, palette: ThemePalette) {
    SidePanel::right("queue")
        .default_width(280.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Queue").color(palette.accent_soft));
            ui.horizontal(|ui| {
                ui.checkbox(&mut app_state.queue.repeat, "Repeat");
                ui.checkbox(&mut app_state.queue.shuffle, "Shuffle");
            });
            ui.separator();
            if app_state.queue.upcoming.is_empty() {
                ui.label("No upcoming tracks");
            } else {
                for track in &app_state.queue.upcoming {
                    ui.label(format!(
                        "{} - {} ({}s)",
                        track.artist, track.title, track.duration_sec
                    ));
                }
            }
        });
}

pub fn bottom_playback_bar(ctx: &egui::Context, app_state: &mut AppState, palette: ThemePalette) {
    TopBottomPanel::bottom("playback_bar").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let has_current = app_state.queue.current.is_some();
            if ui.button("⏮").clicked() && has_current {
                app_state.status_message = "Previous track".to_owned();
            }
            if ui.button("⏯").clicked() && has_current {
                app_state.status_message = "Play/Pause toggled".to_owned();
            }
            if ui.button("⏭").clicked() && has_current {
                app_state.status_message = "Next track".to_owned();
            }
            ui.separator();
            ui.label(RichText::new(&app_state.status_message).color(palette.text_secondary));
        });
        ui.add_space(4.0);

        if let Some(track) = app_state.queue.current.as_ref() {
            ui.label(RichText::new(format!("{} — {}", track.title, track.artist)).strong());
            waveform_placeholder(
                ui,
                app_state.queue.progress_sec / (track.duration_sec as f32).max(1.0),
                palette.accent,
            );
            audio_metadata_strip(ui, track, palette.text_secondary);
        } else {
            ui.label("Nothing is playing");
        }
    });
}

pub fn playlist_import_controls(ui: &mut Ui, app_state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Playlist URL:");
        ui.text_edit_singleline(&mut app_state.import_url);
    });
}
