use eframe::egui::{self, CentralPanel, RichText};

use crate::desktop::{
    components::{bottom_playback_bar, playlist_import_controls, right_queue_panel, sidebar},
    grpc::ImportPlaylistRequest,
    screens::Screen,
    state::AppState,
    theme::{apply_theme, ThemePalette},
};

pub struct LoFonicDesktopApp {
    state: AppState,
    palette: ThemePalette,
}

impl Default for LoFonicDesktopApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            palette: ThemePalette::default(),
        }
    }
}

impl eframe::App for LoFonicDesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx, self.palette);

        sidebar(ctx, &mut self.state, self.palette);
        right_queue_panel(ctx, &mut self.state, self.palette);
        bottom_playback_bar(ctx, &mut self.state, self.palette);

        CentralPanel::default().show(ctx, |ui| {
            render_screen(ui, &mut self.state, self.palette);
        });
    }
}

fn render_screen(ui: &mut egui::Ui, state: &mut AppState, palette: ThemePalette) {
    ui.heading(
        RichText::new(format!("{} Screen", state.current_screen.label())).color(palette.accent),
    );
    ui.add_space(8.0);
    ui.label(RichText::new("MVP workflow scaffold").color(palette.text_muted));
    ui.add_space(4.0);

    match state.current_screen {
        Screen::Home => {
            ui.label("Recent tracks, recommendations, continue listening.");
            playlist_import_controls(ui, state);
            if ui.button("Import playlist").clicked() {
                if state.import_url.trim().is_empty() {
                    state.status_message = "Provide playlist URL first".to_owned();
                } else {
                    let result = state.backend.import_playlist(ImportPlaylistRequest {
                        url: state.import_url.clone(),
                    });
                    state.status_message = match result {
                        Ok(res) => format!("Import started: {}", res.job_id),
                        Err(err) => format!("Import failed: {err}"),
                    };
                }
            }
        }
        Screen::Library => match state.backend.library_tracks() {
            Ok(tracks) => {
                for track in tracks {
                    ui.label(format!("{} — {} ({}s)", track.artist, track.title, track.duration_sec));
                }
            }
            Err(err) => {
                ui.label(format!("Could not load library: {err}"));
            }
        },
        Screen::Artists => {
            ui.label("Artist image, albums, top tracks and metadata.");
        }
        Screen::Albums => {
            ui.label("Large album cover, tracks, waveform preview and metadata.");
        }
        Screen::Playlists => {
            ui.label("User playlists and imported lists.");
        }
        Screen::Downloads => match state.backend.download_jobs() {
            Ok(jobs) => {
                for job in jobs {
                    ui.label(format!("{}: {:.0}% ({})", job.id, job.progress_percent, job.state));
                }
            }
            Err(err) => {
                ui.label(format!("Could not load jobs: {err}"));
            }
        },
        Screen::Tasks => {
            ui.label("Background tasks: import, download, normalize, index.");
            let active_job = state
                .status_message
                .strip_prefix("Import started: ")
                .unwrap_or("job-demo");
            match state.backend.pipeline_for_job(active_job) {
                Ok(steps) => {
                    for step in steps {
                        ui.label(format!("{}: {}", step.step, step.status));
                    }
                }
                Err(err) => {
                    ui.label(format!("Could not load pipeline: {err}"));
                }
            }
        }
        Screen::Settings => {
            ui.label("Audio quality, server endpoint, theme and account settings.");
        }
    }
}
