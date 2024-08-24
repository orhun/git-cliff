use ratatui::layout::Rect;
use std::error;

/// Application result type.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Changelog configuration.
#[derive(Debug)]
pub struct Config {
	/// Name/path of the configuration.
	pub file: String,
	/// Widget area.
	pub area: Rect,
	/// Is the widget hovered?
	pub is_hovered: bool,
}

/// Application state.
#[derive(Debug)]
pub struct State {
	/// Is the application running?
	pub running: bool,
	/// Configuration files.
	pub configs: Vec<Config>,
	/// Index of the selected configuration.
	pub selected_config: usize,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: true,
			configs: vec![
				Config {
					file: String::from("github.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
				Config {
					file: String::from("keepachangelog.toml"),
					area: Rect::default(),
					is_hovered: false,
				},
			],
			selected_config: 0,
		}
	}
}

impl State {
	/// Constructs a new instance.
	pub fn new() -> Self {
		Self::default()
	}

	/// Handles the tick event of the terminal.
	pub fn tick(&self) {}

	/// Set running to false to quit the application.
	pub fn quit(&mut self) {
		self.running = false;
	}
}
