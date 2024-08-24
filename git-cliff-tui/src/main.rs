use std::{
	sync::mpsc,
	time::Duration,
};

use crate::{
	event::Event,
	state::{
		Result,
		State,
	},
};
use ratatui::crossterm::event::Event as CrosstermEvent;

pub mod effect;
pub mod event;
pub mod logo;
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
	let (sender, receiver) = mpsc::channel::<Event>();
	let mut state = State::new(args.clone(), sender.clone())?;

	// Initialize the terminal user interface.
	let mut terminal = ratatui::init();

	// Start the main loop.
	loop {
		terminal.draw(|frame| ui::render(&mut state, frame))?;
		if let Ok(event) = receiver.try_recv() {
			match event {
				Event::Generate => {
					state.is_generating = true;
					state.border_effect = None;
					state.generate_changelog()?;
				}
				Event::UpdateChangelog(contents) => {
					state.is_generating = false;
					state.border_effect = None;
					state.configs = contents;
				}
				Event::Quit => break,
			}
		}
		if ratatui::crossterm::event::poll(Duration::from_millis(16))? {
			let event = ratatui::crossterm::event::read()?;
			if let CrosstermEvent::Key(key_event) = event {
   					event::handle_key_events(key_event, sender.clone(), &mut state)?;
   				}
		}
	}

	ratatui::restore();
	Ok(())
}
