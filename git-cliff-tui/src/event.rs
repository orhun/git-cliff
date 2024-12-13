use crate::state::{
	Result,
	State,
};
use copypasta::ClipboardProvider;
use ratatui::crossterm::event::{
	self,
	Event as CrosstermEvent,
	KeyCode,
	KeyEvent,
	KeyEventKind,
	KeyModifiers,
	MouseEvent,
};
use std::sync::mpsc;
use std::thread;
use std::time::{
	Duration,
	Instant,
};

/// Terminal events.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
	/// Terminal tick.
	Tick,
	/// Key press.
	Key(KeyEvent),
	/// Mouse click/scroll.
	Mouse(MouseEvent),
	/// Terminal resize.
	Resize(u16, u16),
	/// Generate changelog.
	Generate(bool),
	/// Quit the application.
	Quit,
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
	/// Event sender channel.
	pub sender:   mpsc::Sender<Event>,
	/// Event receiver channel.
	pub receiver: mpsc::Receiver<Event>,
	/// Event handler thread.
	handler:      thread::JoinHandle<()>,
}

impl EventHandler {
	/// Constructs a new instance of [`EventHandler`].
	pub fn new(tick_rate: u64) -> Self {
		let tick_rate = Duration::from_millis(tick_rate);
		let (sender, receiver) = mpsc::channel();
		let handler = {
			let sender = sender.clone();
			thread::spawn(move || {
				let mut last_tick = Instant::now();
				loop {
					let timeout = tick_rate
						.checked_sub(last_tick.elapsed())
						.unwrap_or(tick_rate);

					if event::poll(timeout).expect("failed to poll new events") {
						match event::read().expect("unable to read event") {
							CrosstermEvent::Key(e) => {
								if e.kind == KeyEventKind::Press {
									sender.send(Event::Key(e))
								} else {
									Ok(())
								}
							}
							CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
							CrosstermEvent::Resize(w, h) => {
								sender.send(Event::Resize(w, h))
							}
							CrosstermEvent::FocusGained => Ok(()),
							CrosstermEvent::FocusLost => Ok(()),
							CrosstermEvent::Paste(_) => unimplemented!(),
						}
						.expect("failed to send terminal event")
					}

					if last_tick.elapsed() >= tick_rate {
						let _ = sender.send(Event::Tick);
						last_tick = Instant::now();
					}
				}
			})
		};
		Self {
			sender,
			receiver,
			handler,
		}
	}
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
			} else {
				if let Some(clipboard) = &mut state.clipboard {
					if let Err(e) = clipboard.set_contents(state.contents.clone()) {
						return Err(format!(
							"Failed to set clipboard contents: {e}"
						)
						.into());
					}
				}
			}
		}
		KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
			state.list_state.select_previous();
			sender.send(Event::Generate(false))?;
		}
		KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
			state.list_state.select_next();
			sender.send(Event::Generate(false))?;
		}
		KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
			state.scroll_index = state.scroll_index.saturating_sub(1);
		}
		KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
			state.scroll_index = state.scroll_index.saturating_add(1);

			state.args.latest = !state.args.latest;
			sender.send(Event::Generate(true))?;
		}
		KeyCode::Enter => sender.send(Event::Generate(false))?,
		KeyCode::Char('u') | KeyCode::Char('U') => {
			state.args.unreleased = !state.args.unreleased;
			sender.send(Event::Generate(true))?;
		}
		_ => {}
	}
	Ok(())
}
