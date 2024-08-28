use git_cliff_core::embed::BuiltinConfig;
use md_tui::nodes::root::ComponentRoot;
use ratatui::layout::Rect;
use std::error;

/// Application result type.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Changelog configuration.
#[derive(Debug, Default)]
pub struct Config {
	/// Name/path of the configuration.
	pub file:       String,
	/// Widget area.
	pub area:       Rect,
	/// Is the widget hovered?
	pub is_hovered: bool,
}

/// Markdown content.
#[derive(Default)]
pub struct Markdown {
	/// Rendered component.
	pub component:    Option<ComponentRoot>,
	/// Widget area.
	pub area:         Rect,
	/// Selected config index.
	pub config_index: usize,
}

/// Is the application running?
/// Application state.
pub struct State {
	/// Is the application running?
	pub running:         bool,
	/// Configuration files.
	pub configs:         Vec<Config>,
	/// Index of the selected configuration.
	pub selected_config: usize,
	/// Changelog contents.
	pub changelog:       String,
	/// Rendered markdown.
	pub markdown:        Markdown,
	/// Autoload changes.
	pub autoload:        bool,
}

impl Default for State {
	fn default() -> Self {
		let configs = BuiltinConfig::iter()
			.map(|file| Config {
				file:       file.to_string(),
				area:       Rect::default(),
				is_hovered: false,
			})
			.collect();
		Self {
			running: true,
			configs,
			selected_config: 0,
			changelog: String::new(),
			markdown: Markdown::default(),
			autoload: true,
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
