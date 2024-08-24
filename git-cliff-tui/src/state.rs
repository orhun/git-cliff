use std::error;

/// Application result type.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application state.
#[derive(Debug)]
pub struct State {
	/// Is the application running?
	pub running: bool,
}

impl Default for State {
	fn default() -> Self {
		Self { running: true }
	}
}

impl State {
	/// Constructs a new instance.
	pub fn new() -> Self {
		Self::default()
	}

	/// Handles the tick event of the terminal.
	pub fn tick(&self) {}

	/// Set running to false to quit the application.
	pub fn quit(&mut self) {
		self.running = false;
	}
}
