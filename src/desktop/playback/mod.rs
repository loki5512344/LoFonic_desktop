use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSummary {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub duration_sec: u32,
    pub format: String,
    pub sample_rate_hz: u32,
    pub bitrate_kbps: u32,
    pub lufs: f32,
}

impl TrackSummary {
    pub fn demo(title: &str, artist: &str, duration_sec: u32) -> Self {
        Self {
            id: format!("{artist}-{title}"),
            title: title.to_owned(),
            artist: artist.to_owned(),
            duration_sec,
            format: "FLAC".to_owned(),
            sample_rate_hz: 96_000,
            bitrate_kbps: 1411,
            lufs: -14.2,
        }
    }
}

#[derive(Debug, Default)]
pub struct QueueState {
    pub current: Option<TrackSummary>,
    pub upcoming: Vec<TrackSummary>,
    pub repeat: bool,
    pub shuffle: bool,
    pub progress_sec: f32,
}

impl QueueState {
    pub fn seed_demo(&mut self) {
        self.current = Some(TrackSummary::demo("Midnight Tape", "LoFonic Artist", 254));
        self.upcoming = vec![
            TrackSummary::demo("Warm Circuit", "LoFonic Artist", 233),
            TrackSummary::demo("Analog Dust", "Studio Session", 276),
            TrackSummary::demo("After Hours", "Tape Ensemble", 245),
        ];
    }
}
