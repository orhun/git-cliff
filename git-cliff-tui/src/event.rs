use crate::state::{Result, State};
use ratatui::crossterm::event::{
	self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
	MouseEvent,
};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
	/// Terminal tick.
	Tick,
	/// Key press.
	Key(KeyEvent),
	/// Mouse click/scroll.
	Mouse(MouseEvent),
	/// Terminal resize.
	Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
	/// Event sender channel.
	sender: mpsc::Sender<Event>,
	/// Event receiver channel.
	receiver: mpsc::Receiver<Event>,
	/// Event handler thread.
	handler: thread::JoinHandle<()>,
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
						sender.send(Event::Tick).expect("failed to send tick event");
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
pub fn handle_key_events(key_event: KeyEvent, state: &mut State) -> Result<()> {
	match key_event.code {
		KeyCode::Esc | KeyCode::Char('q') => {
			state.quit();
		}
		KeyCode::Char('c') | KeyCode::Char('C') => {
			if key_event.modifiers == KeyModifiers::CONTROL {
				state.quit();
			}
		}
		_ => {}
	}
	Ok(())
}
