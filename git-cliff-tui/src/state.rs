use copypasta::ClipboardContext;
use git_cliff::args::Args;
use git_cliff::core::changelog::Changelog;
use git_cliff::core::embed::BuiltinConfig;
use ratatui::widgets::ListState;
use std::error;
use throbber_widgets_tui::ThrobberState;

/// Application result type.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application state.
pub struct State<'a> {
	/// git-cliff arguments.
	pub args:            Args,
	/// Built-in configuration files.
	pub builtin_configs: Vec<String>,
	/// The state of the list.
	pub list_state:      ListState,
	/// Changelog object.
	pub changelog:       Option<Changelog<'a>>,
	/// Changelog contents.
	pub contents:        String,
	/// Scroll index.
	pub scroll_index:    usize,
	/// Clipboard context.
	pub clipboard:       Option<ClipboardContext>,
	/// Throbber state.
	pub throbber_state:  ThrobberState,
	/// Is generating?
	pub is_generating:   bool,
	/// Is the sidebar toggled?
	pub is_toggled:      bool,
	/// Autoload changes.
	pub autoload:        bool,
	/// Is the application running?
	pub is_running:      bool,
}

impl State<'_> {
	/// Constructs a new instance.
	pub fn new(args: Args) -> Result<Self> {
		let configs = BuiltinConfig::iter().map(|file| file.to_string()).collect();
		Ok(Self {
			args,
			builtin_configs: configs,
			list_state: {
				let mut list_state = ListState::default();
				list_state.select_first();
				list_state
			},
			changelog: None,
			contents: String::new(),
			scroll_index: 0,
			throbber_state: ThrobberState::default(),
			clipboard: match ClipboardContext::new() {
				Ok(ctx) => Some(ctx),
				Err(e) => {
					eprintln!("Failed to initialize clipboard: {e}");
					None
				}
			},
			is_generating: false,
			is_toggled: true,
			autoload: true,
			is_running: true,
		})
	}

	/// Generates the changelog.
	///
	/// This runs once the application starts for fetching the remote data if
	/// necessary.
	pub fn get_changelog_data(&mut self) -> Result<()> {
		self.changelog = Some(git_cliff::generate_changelog(&mut self.args)?);
		Ok(())
	}

	/// Returns the changelog contents.
	pub fn generate_changelog(&mut self) -> Result<Option<String>> {
		if let Some(changelog) = &self.changelog {
			let config = BuiltinConfig::parse(
				self.builtin_configs[self.list_state.selected().unwrap_or_default()]
					.clone(),
			)?
			.0;
			let mut changelog =
				Changelog::new(changelog.releases.clone(), config.clone())?;
			changelog.add_remote_context()?;
			let mut output = Vec::new();
			git_cliff::write_changelog(
				self.args.clone(),
				changelog.clone(),
				&mut output,
			)?;
			let contents = String::from_utf8(output)?;
			self.changelog = Some(changelog);
			Ok(Some(contents))
		} else {
			Ok(None)
		}
	}

	/// Processes the changelog contents.
	pub fn process_changelog(&mut self) -> Result<()> {
		if let Some(contents) = &self.generate_changelog()? {
			self.contents = contents.clone();
		}
		Ok(())
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
