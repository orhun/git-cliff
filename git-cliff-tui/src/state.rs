use copypasta::ClipboardContext;
use git_cliff::args::Args;
use git_cliff::core::embed::BuiltinConfig;
use md_tui::nodes::root::ComponentRoot;
use ratatui::layout::Rect;
use std::error;
use throbber_widgets_tui::ThrobberState;

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
	/// git-cliff arguments.
	pub args:           Args,
	/// Is the application running?
	pub is_running:     bool,
	/// Configuration files.
	pub configs:        Vec<Config>,
	/// Index of the selected configuration.
	pub selected_index: usize,
	/// Changelog contents.
	pub changelog:      String,
	/// Error message.
	pub error:          Option<String>,
	/// Rendered markdown.
	pub markdown:       Markdown,
	/// Autoload changes.
	pub autoload:       bool,
	/// Clipboard context.
	pub clipboard:      Option<ClipboardContext>,
	/// Is the sidebar toggled?
	pub is_toggled:     bool,
	/// Throbber state.
	pub throbber_state: ThrobberState,
	/// Is generating?
	pub is_generating:  bool,
}

impl State {
	/// Constructs a new instance.
	pub fn new(args: Args) -> Result<Self> {
		let configs = BuiltinConfig::iter()
			.map(|file| Config {
				file:       file.to_string(),
				area:       Rect::default(),
				is_hovered: false,
			})
			.collect();
		Ok(Self {
			args,
			is_running: true,
			is_toggled: true,
			is_generating: false,
			configs,
			selected_index: 0,
			changelog: String::new(),
			error: None,
			markdown: Markdown::default(),
			autoload: true,
			throbber_state: ThrobberState::default(),
			clipboard: match ClipboardContext::new() {
				Ok(ctx) => Some(ctx),
				Err(e) => {
					eprintln!("Failed to initialize clipboard: {e}");
					None
				}
			},
		})
	}

	/// Handles the tick event of the terminal.
	pub fn tick(&mut self) {
		if self.is_generating {
			self.throbber_state.calc_next();
		}
	}

	/// Set running to false to quit the application.
	pub fn quit(&mut self) {
		self.is_running = false;
	}
}
