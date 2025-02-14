use crate::state::{
	Config,
	Result,
	State,
};
use copypasta::ClipboardProvider;
use ratatui::crossterm::event::{
	KeyCode,
	KeyEvent,
	KeyModifiers,
};
use std::sync::mpsc;

/// Terminal events.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Event {
	/// Generate changelog.
	Generate,
	/// Update the changelog data.
	UpdateChangelog(Vec<Config>),
	/// Quit the application.
	Quit,
}

/// Handles the key events and updates the state of [`State`].
pub fn handle_key_events(
	key_event: KeyEvent,
	sender: mpsc::Sender<Event>,
	state: &mut State,
) -> Result<()> {
	match key_event.code {
		KeyCode::Esc | KeyCode::Char('q') => {
			sender.send(Event::Quit)?;
		}
		KeyCode::Char('c') | KeyCode::Char('C') => {
			if key_event.modifiers == KeyModifiers::CONTROL {
				sender.send(Event::Quit)?;
			} else if let Some(clipboard) = &mut state.clipboard {
				let contents = state
					.list_state
					.selected()
					.map(|i| state.configs[i].contents.clone())
					.unwrap_or_default();
				if let Err(e) = clipboard.set_contents(contents) {
					return Err(
						format!("Failed to set clipboard contents: {e}").into()
					);
				}
			}
		}
		KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
			state.list_state.select_previous();
		}
		KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
			state.list_state.select_next();
		}
		KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
			state.scroll_index = state.scroll_index.saturating_sub(1);
		}
		KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
			state.scroll_index = state.scroll_index.saturating_add(1);
			// state.args.latest = !state.args.latest;
			// sender.send(Event::Generate(true))?;
		}
		KeyCode::Enter => sender.send(Event::Generate)?,
		KeyCode::Char('u') | KeyCode::Char('U') => {
			state.args.unreleased = !state.args.unreleased;
			sender.send(Event::Generate)?;
		}
		KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Tab => {
			state.toggle = !state.toggle;
		}
		_ => {}
	}
	Ok(())
}
