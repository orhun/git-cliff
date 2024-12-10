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

	// Get the changelog data.
	state.get_changelog_data()?;

	// Initialize the terminal user interface.
	let events = EventHandler::new(250);
	let mut terminal = ratatui::init();

	// TODO: Watch for file changes.
	//
	// let sender = events.sender.clone();
	// let mut watcher =
	// 	notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
	// 		match res {
	// 			Ok(event) => {
	// 				if event.kind.is_modify() {
	// 					sender
	// 						.send(Event::AutoGenerate)
	// 						.expect("failed to send event");
	// 				}
	// 			}
	// 			Err(e) => panic!("watch error: {e:?}"),
	// 		}
	// 	})?;

	// Start the main loop.
	while state.is_running {
		terminal.draw(|frame| ui::render(&mut state, frame))?;
		let event = events.next()?;
		match event {
			Event::Tick => state.tick(),
			Event::Key(key_event) => event::handle_key_events(
				key_event,
				events.sender.clone(),
				&mut state,
			)?,
			Event::Mouse(_) => {}
			Event::Resize(_, _) => {}
			Event::Generate | Event::AutoGenerate => {
				state.process_changelog()?;
				// if event == Event::AutoGenerate && !state.autoload {
				// 	continue;
				// }
				// let sender = events.sender.clone();
				// let args = state.args.clone();
				// state.is_generating = true;
				// state.args.config = PathBuf::from(
				// 	state.configs[state.list_state.selected().
				// unwrap_or_default()] 		.file
				// 		.clone(),
				// );
				// thread::spawn(move || {
				// 	let mut output = Vec::new();
				// 	sender
				// 		.send(match git_cliff::run(args, &mut output) {
				// 			Ok(()) => Event::RenderMarkdown(
				// 				String::from_utf8_lossy(&output).to_string(),
				// 			),
				// 			Err(e) => Event::Error(e.to_string()),
				// 		})
				// 		.expect("failed to send event");
				// });
			}
		}
	}

	ratatui::restore();
	Ok(())
}
