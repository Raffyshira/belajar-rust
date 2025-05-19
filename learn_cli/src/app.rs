use std::time::Instant;

#[derive(Clone, PartialEq)]
pub enum DownloadMode {
    Video,
    Audio,
}

#[derive(Clone, PartialEq)]
pub enum DownloadStatus {
    Idle,
    Downloading,
    Success,
    Failed,
}

pub struct HistoryItem {
    pub title: String,
}

pub struct App {
    pub url_input: String,
    pub mode: DownloadMode,
    pub status: DownloadStatus,
    pub show_popup: bool,
    pub popup_start: Option<Instant>,
    pub history: Vec<HistoryItem>,
}

impl App {
    pub fn new() -> Self {
        Self {
            url_input: String::new(),
            mode: DownloadMode::Audio,
            status: DownloadStatus::Idle,
            show_popup: false,
            popup_start: None,
            history: Vec::new(),
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            DownloadMode::Video => DownloadMode::Audio,
            DownloadMode::Audio => DownloadMode::Video,
        }
    }
}
