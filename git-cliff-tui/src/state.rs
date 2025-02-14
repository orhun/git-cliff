use copypasta::ClipboardContext;
use git_cliff::args::Args;
use git_cliff::core::changelog::Changelog;
use git_cliff::core::embed::BuiltinConfig;
use ratatui::widgets::ListState;
use std::sync::mpsc;
use std::{
	error,
	thread,
};
use tachyonfx::Effect;

use crate::event::Event;
use crate::logo::Logo;

/// Application result type.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
	pub name:     String,
	pub contents: String,
}

/// Application state.
pub struct State {
	/// git-cliff arguments.
	pub args:          Args,
	/// Built-in configuration files.
	pub configs:       Vec<Config>,
	/// The state of the list.
	pub list_state:    ListState,
	/// Event sender.
	pub sender:        mpsc::Sender<Event>,
	/// Scroll index.
	pub scroll_index:  usize,
	/// Clipboard context.
	pub clipboard:     Option<ClipboardContext>,
	/// Is generating?
	pub is_generating: bool,
	/// Logo widget.
	pub logo:          Logo,
	/// Border effect.
	pub border_effect: Option<Effect>,
	/// Is the config list toggled?
	pub toggle:        bool,
}

impl State {
	/// Constructs a new instance.
	pub fn new(args: Args, sender: mpsc::Sender<Event>) -> Result<Self> {
		let configs = BuiltinConfig::iter()
			.map(|file| Config {
				name:     file.to_string(),
				contents: String::new(),
			})
			.collect();
		let state = Self {
			configs,
			list_state: {
				let mut list_state = ListState::default();
				list_state.select_first();
				list_state
			},
			sender,
			scroll_index: 0,
			clipboard: match ClipboardContext::new() {
				Ok(ctx) => Some(ctx),
				Err(e) => {
					eprintln!("Failed to initialize clipboard: {e}");
					None
				}
			},
			is_generating: true,
			logo: Logo::default(),
			args,
			border_effect: None,
			toggle: false,
		};
		state.generate_changelog()?;
		Ok(state)
	}

	/// Returns the changelog contents.
	pub fn generate_changelog(&self) -> Result<()> {
		let sender = self.sender.clone();
		let args = self.args.clone();
		let mut configs = self.configs.clone();
		thread::spawn(move || {
			let run = || -> Result<()> {
				let releases = git_cliff::run(args.clone())?.releases;
				for config in configs.iter_mut() {
					let builtin_config =
						BuiltinConfig::parse(config.name.clone())?.0;
					let mut changelog =
						Changelog::new(releases.clone(), builtin_config)?;
					changelog.add_remote_context()?;
					let mut output = Vec::new();
					git_cliff::write_changelog(
						args.clone(),
						changelog.clone(),
						&mut output,
					)?;
					config.contents = String::from_utf8(output)?;
				}
				sender.send(Event::UpdateChangelog(configs))?;
				Ok(())
			};
			run().expect("failed to generate changelog");
		});

		Ok(())
	}
}
