use std::path::{
	Path,
	PathBuf,
};

use crate::{
	event::{
		Event,
		EventHandler,
	},
	state::{
		Config,
		Result,
		State,
	},
};

pub mod event;
pub mod state;
pub mod ui;

use git_cliff::args::{
	Args,
	Parser,
};
use notify::{
	RecursiveMode,
	Watcher,
};

fn main() -> Result<()> {
	// Parse command-line arguments.
	let args = Args::parse();

	// Create an application state.
	let mut state = State::new(args.clone())?;

	// Add default configuration file.
	if Path::new("cliff.toml").exists() {
		state.configs.insert(0, Config {
			file: "cliff.toml".into(),
			..Default::default()
		});
	}

	// Add the configuration file from the command-line arguments.
	if &args.config != &PathBuf::from("cliff.toml") {
		if args.config.exists() {
			state.configs.insert(0, Config {
				file: args.config.to_string_lossy().to_string(),
				..Default::default()
			});
		}
	}

	// Initialize the terminal user interface.
	let events = EventHandler::new(250);
	let mut terminal = ratatui::init();

	// Watch for file changes.
	let sender = events.sender.clone();
	let mut watcher =
		notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
			match res {
				Ok(event) => {
					if event.kind.is_modify() {
						sender
							.send(Event::AutoGenerate)
							.expect("failed to send event");
					}
				}
				Err(e) => panic!("watch error: {e:?}"),
			}
		})?;

	for config in state.configs.iter() {
		let path = Path::new(&config.file);
		if path.exists() {
			watcher.watch(path, RecursiveMode::NonRecursive)?;
		}
	}

	// Start the main loop.
	while state.running {
		// Render the user interface.
		terminal.draw(|frame| ui::render(&mut state, frame))?;
		// Handle events.
		let event = events.next()?;
		match event {
			Event::Tick => state.tick(),
			Event::Key(key_event) => event::handle_key_events(
				key_event,
				events.sender.clone(),
				&mut state,
			)?,
			Event::Mouse(mouse_event) => event::handle_mouse_events(
				mouse_event,
				events.sender.clone(),
				&mut state,
			)?,
			Event::Resize(_, _) => {
				events.sender.clone().send(Event::RenderMarkdown)?
			}
			Event::Generate | Event::AutoGenerate => {
				if event == Event::AutoGenerate && !state.autoload {
					continue;
				}
				let mut output = Vec::new();
				state.args.config =
					PathBuf::from(state.configs[state.selected_config].file.clone());
				git_cliff::run(state.args.clone(), &mut output)?;
				state.changelog = String::from_utf8_lossy(&output).to_string();
				events.sender.clone().send(Event::RenderMarkdown)?;
			}
			Event::RenderMarkdown => {
				state.markdown.component = Some(md_tui::parser::parse_markdown(
					None,
					&state.changelog,
					state.markdown.area.width,
				));
			}
		}
	}

	ratatui::restore();
	Ok(())
}
