use std::io;

use ratatui::{
	backend::CrosstermBackend,
	Terminal,
};

use crate::{
	event::{
		Event,
		EventHandler,
	},
	state::{
		Result,
		State,
	},
	tui::Tui,
};

pub mod event;
pub mod state;
pub mod tui;
pub mod ui;
pub mod util;

fn main() -> Result<()> {
	// Create an application state.
	let mut state = State::new();

	// Initialize the terminal user interface.
	let backend = CrosstermBackend::new(io::stderr());
	let terminal = Terminal::new(backend)?;
	let events = EventHandler::new(250);
	let mut tui = Tui::new(terminal, events);
	tui.init()?;

	// Start the main loop.
	while state.running {
		// Render the user interface.
		tui.draw(&mut state)?;
		// Handle events.
		match tui.events.next()? {
			Event::Tick => state.tick(),
			Event::Key(key_event) => event::handle_key_events(
				key_event,
				tui.events.sender.clone(),
				&mut state,
			)?,
			Event::Mouse(mouse_event) => event::handle_mouse_events(
				mouse_event,
				tui.events.sender.clone(),
				&mut state,
			)?,
			Event::Resize(_, _) => {
				tui.events.sender.clone().send(Event::RenderMarkdown)?
			}
			Event::Generate(i) => {
				state.selected_config = i;
				state.changelog = match util::run_git_cliff(&[
					"-c".into(),
					state.configs[state.selected_config].file.to_string(),
					"-u".into(),
					"--no-exec".into(),
				]) {
					Ok(v) => v,
					Err(e) => e.to_string(),
				};
				tui.events.sender.clone().send(Event::RenderMarkdown)?;
			}
			Event::RenderMarkdown => {
				state.markdown = Some(md_tui::parser::parse_markdown(
					None,
					&state.changelog,
					state.markdown_area.width,
				));
			}
		}
	}

	// Exit the user interface.
	tui.exit()?;
	Ok(())
}
