use std::sync::Arc;

use crate::desktop::{
    grpc::{DesktopBackendApi, MockBackendClient},
    playback::QueueState,
    screens::Screen,
};

pub struct AppState {
    pub current_screen: Screen,
    pub queue: QueueState,
    pub import_url: String,
    pub status_message: String,
    pub backend: Arc<dyn DesktopBackendApi>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut queue = QueueState::default();
        queue.seed_demo();

        Self {
            current_screen: Screen::Home,
            queue,
            import_url: String::new(),
            status_message: "Ready".to_owned(),
            backend: Arc::new(MockBackendClient::default()),
        }
    }
}
