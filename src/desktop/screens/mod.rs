#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Library,
    Artists,
    Albums,
    Playlists,
    Downloads,
    Tasks,
    Settings,
}

impl Screen {
    pub const ALL: [Screen; 8] = [
        Screen::Home,
        Screen::Library,
        Screen::Artists,
        Screen::Albums,
        Screen::Playlists,
        Screen::Downloads,
        Screen::Tasks,
        Screen::Settings,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Screen::Home => "Home",
            Screen::Library => "Library",
            Screen::Artists => "Artists",
            Screen::Albums => "Albums",
            Screen::Playlists => "Playlists",
            Screen::Downloads => "Downloads",
            Screen::Tasks => "Tasks",
            Screen::Settings => "Settings",
        }
    }
}
