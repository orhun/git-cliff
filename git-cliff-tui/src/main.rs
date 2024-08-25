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
pub mod util;

fn main() -> Result<()> {
	// Create an application state.
	let mut state = State::new();

	// Initialize the terminal user interface.
	let events = EventHandler::new(250);
	let mut terminal = ratatui::init();

	// Start the main loop.
	while state.running {
		// Render the user interface.
		terminal.draw(|frame| ui::render(&mut state, frame))?;
		// Handle events.
		match events.next()? {
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
				events.sender.clone().send(Event::RenderMarkdown)?;
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

	ratatui::restore();
	Ok(())
}
