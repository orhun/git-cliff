use crate::{
	event::{
		Event,
		EventHandler,
	},
	state::{
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

fn main() -> Result<()> {
	// Parse command-line arguments.
	let args = Args::parse();

	// Create an application state.
	let mut state = State::new(args.clone())?;

	// Initialize the terminal user interface.
	let events = EventHandler::new(250);
	let mut terminal = ratatui::init();

	// Start the main loop.
	loop {
		terminal.draw(|frame| ui::render(&mut state, frame))?;
		let event = events.receiver.recv()?;
		match event {
			Event::Tick => state.tick(),
			Event::Key(key_event) => event::handle_key_events(
				key_event,
				events.sender.clone(),
				&mut state,
			)?,
			Event::Mouse(_) => {}
			Event::Resize(_, _) => {}
			Event::Generate(update_data) => {
				state.generate_changelog(update_data)?;
			}
			Event::Quit => break,
		}
	}

	ratatui::restore();
	Ok(())
}
