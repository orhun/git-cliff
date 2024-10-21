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
	MouseButton,
	MouseEvent,
	MouseEventKind,
};
use ratatui::layout::Position;
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
	Generate,
	/// Generate changelog when the file changes.
	AutoGenerate,
	/// Render markdown.
	RenderMarkdown(String),
	/// Error event.
	Error(String),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
	/// Event sender channel.
	pub sender: mpsc::Sender<Event>,
	/// Event receiver channel.
	receiver:   mpsc::Receiver<Event>,
	/// Event handler thread.
	handler:    thread::JoinHandle<()>,
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

	/// Receive the next event from the handler thread.
	///
	/// This function will always block the current thread if
	/// there is no data available and it's possible for more data to be sent.
	pub fn next(&self) -> Result<Event> {
		Ok(self.receiver.recv()?)
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
			state.quit();
		}
		KeyCode::Char('c') | KeyCode::Char('C') => {
			if key_event.modifiers == KeyModifiers::CONTROL {
				state.quit();
			} else if let Some(clipboard) = &mut state.clipboard {
				if let Err(e) = clipboard.set_contents(state.changelog.clone()) {
					eprintln!("Failed to set clipboard contents: {e}");
				}
			}
		}
		KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
			state.selected_index = if state.selected_index == 0 {
				state.configs.len() - 1
			} else {
				state.selected_index - 1
			}
		}
		KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
			state.selected_index = if state.selected_index >= state.configs.len() - 1
			{
				0
			} else {
				state.selected_index + 1
			}
		}
		KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
			state.markdown.scroll_index =
				state.markdown.scroll_index.saturating_sub(1);
		}
		KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
			if key_event.modifiers == KeyModifiers::CONTROL {
				state.markdown.scroll_index =
					state.markdown.scroll_index.saturating_add(1);
			} else {
				state.args.latest = !state.args.latest;
				sender.send(Event::Generate)?;
			}
		}
		KeyCode::Enter => {
			state.markdown.config_index = state.selected_index;
			sender.send(Event::Generate)?
		}
		KeyCode::Char('a') | KeyCode::Char('A') => {
			state.autoload = !state.autoload;
		}
		KeyCode::Char('t') | KeyCode::Char('T') => {
			state.is_toggled = !state.is_toggled;
		}
		KeyCode::Char('u') | KeyCode::Char('U') => {
			state.args.unreleased = !state.args.unreleased;
			sender.send(Event::Generate)?;
		}
		_ => {}
	}
	Ok(())
}

/// Handles the mouse events and updates the state.
pub(crate) fn handle_mouse_events(
	mouse_event: MouseEvent,
	sender: mpsc::Sender<Event>,
	state: &mut State,
) -> Result<()> {
	match mouse_event.kind {
		MouseEventKind::Moved => {
			let position = Position::new(mouse_event.column, mouse_event.row);
			state.configs.iter_mut().for_each(|config| {
				config.is_hovered = config.area.contains(position);
			})
		}
		MouseEventKind::Down(MouseButton::Left) => {
			if let Some(i) = state.configs.iter().position(|p| p.is_hovered) {
				state.selected_index = i;
				sender.send(Event::Generate)?;
			}
		}
		MouseEventKind::ScrollUp => {
			state.markdown.scroll_index =
				state.markdown.scroll_index.saturating_sub(1);
		}
		MouseEventKind::ScrollDown => {
			state.markdown.scroll_index =
				state.markdown.scroll_index.saturating_add(1);
		}
		_ => {}
	}
	Ok(())
}
