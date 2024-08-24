use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
	event::{Event, EventHandler},
	state::{Result, State},
	tui::Tui,
};

pub mod event;
pub mod state;
pub mod tui;
pub mod ui;

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
			Event::Key(key_event) => {
				event::handle_key_events(key_event, &mut state)?
			}
			Event::Mouse(_) => {}
			Event::Resize(_, _) => {}
		}
	}

	// Exit the user interface.
	tui.exit()?;
	Ok(())
}
