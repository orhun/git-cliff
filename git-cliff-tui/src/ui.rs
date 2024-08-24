use crate::state::State;
use ratatui::{
	layout::Alignment,
	style::{Color, Style},
	widgets::{Block, BorderType, Paragraph},
	Frame,
};

/// Renders the user interface widgets.
pub fn render(_state: &mut State, frame: &mut Frame) {
	frame.render_widget(
		Paragraph::new(format!(
			"This is a tui template.\nPress `Esc`, `Ctrl-C` or `q` to stop \
			 running.\nPress left and right to increment and decrement the counter \
			 respectively.",
		))
		.block(
			Block::bordered()
				.title("Template")
				.title_alignment(Alignment::Center)
				.border_type(BorderType::Rounded),
		)
		.style(Style::default().fg(Color::Cyan).bg(Color::Black))
		.centered(),
		frame.area(),
	)
}
