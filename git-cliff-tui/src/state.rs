use copypasta::ClipboardContext;
use git_cliff::args::Args;
use git_cliff::core::changelog::Changelog;
use git_cliff::core::embed::BuiltinConfig;
use md_tui::nodes::root::ComponentRoot;
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use std::error;
use throbber_widgets_tui::ThrobberState;

/// Application result type.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Changelog configuration.
#[derive(Debug, Default)]
pub struct Config {
	/// Name/path of the configuration.
	pub file: String,
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
	/// Scroll index.
	pub scroll_index: u16,
}

/// Is the application running?
/// Application state.
pub struct State<'a> {
	/// git-cliff arguments.
	pub args:           Args,
	/// Is the application running?
	pub is_running:     bool,
	/// Configuration files.
	pub configs:        Vec<Config>,
	/// The state of the list.
	pub list_state:     ListState,
	/// Changelog.
	pub changelog:      Option<Changelog<'a>>,
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

impl<'a> State<'a> {
	/// Constructs a new instance.
	pub fn new(args: Args) -> Result<Self> {
		let configs = BuiltinConfig::iter()
			.map(|file| Config {
				file: file.to_string(),
			})
			.collect();
		Ok(Self {
			args,
			is_running: true,
			is_toggled: true,
			is_generating: false,
			configs,
			list_state: {
				let mut list_state = ListState::default();
				list_state.select_first();
				list_state
			},
			changelog: None,
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

	/// Generates the changelog.
	pub fn generate_changelog(&mut self) -> Result<()> {
		self.changelog = Some(git_cliff::generate_changelog(&mut self.args)?);
		Ok(())
	}

	/// Returns the changelog contents.
	pub fn get_changelog_contents(&mut self) -> Result<Option<String>> {
		if let Some(changelog) = &self.changelog {
			let config = git_cliff::core::embed::BuiltinConfig::parse(
				self.configs[self.list_state.selected().unwrap_or_default()]
					.file
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
		if let Some(contents) = &self.get_changelog_contents()? {
			self.markdown.component = Some(md_tui::parser::parse_markdown(
				None,
				&contents,
				self.markdown.area.width,
			));
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
