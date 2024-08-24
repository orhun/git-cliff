use crate::state::State;
use ratatui::{
	layout::{Alignment, Constraint, Layout, Rect},
	style::{Color, Style, Stylize},
	text::{Line, Span},
	widgets::{Block, BorderType, Paragraph},
	Frame,
};

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
	("⏎ ", "Generate Changelog"),
	("↕ ↔ ", "Scroll"),
	("q", "Quit"),
];

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
	frame.render_widget(
		Block::new()
			.title_top(env!("CARGO_PKG_NAME").bold())
			.title_alignment(Alignment::Center),
		frame.area(),
	);
	let rects = Layout::vertical([Constraint::Percentage(100), Constraint::Min(3)])
		.margin(1)
		.split(frame.area());
	render_key_bindings(frame, rects[1]);

	let rects =
		Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
			.split(rects[0]);
	render_list(state, frame, rects[0]);
}

fn render_key_bindings(frame: &mut Frame, rect: Rect) {
	frame.render_widget(
		Paragraph::new(
			Line::default()
				.spans(
					KEY_BINDINGS
						.iter()
						.flat_map(|(key, desc)| {
							vec![
								"<".fg(Color::Rgb(100, 100, 100)),
								key.yellow(),
								": ".fg(Color::Rgb(100, 100, 100)),
								Span::from(*desc),
								"> ".fg(Color::Rgb(100, 100, 100)),
							]
						})
						.collect::<Vec<Span>>(),
				)
				.alignment(Alignment::Center),
		)
		.block(
			Block::bordered()
				.title_bottom(Line::from(format!("|{}|", env!("CARGO_PKG_VERSION"))))
				.title_alignment(Alignment::Right)
				.border_type(BorderType::Rounded)
				.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
		),
		rect,
	);
}

fn render_list(state: &mut State, frame: &mut Frame, rect: Rect) {
	frame.render_widget(
		Block::bordered()
			.title_top("|Packages|".yellow())
			.title_alignment(Alignment::Center)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
		rect,
	);
	if !state.configs.is_empty() {
		let item_count = ((rect.height - 2) / 3) as usize;
		let start_offset = (state.selected_config + 1).saturating_sub(item_count);
		let rects = Layout::vertical([Constraint::Min(2)].repeat(item_count))
			.margin(1)
			.split(rect);
		for (i, config) in state
			.configs
			.iter_mut()
			.skip(start_offset)
			.take(item_count)
			.enumerate()
		{
			config.area = rects[i];
			frame.render_widget(
				Block::bordered()
					.border_type(BorderType::Rounded)
					.border_style({
						let mut style = Style::new().fg(Color::White);
						if config.is_hovered {
							style = style.yellow()
						} else if state.selected_config == i + start_offset {
							style = style.yellow();
						}
						style
					}),
				rects[i],
			);
			let item = Layout::horizontal([
				Constraint::Min(1),
				Constraint::Percentage(100),
			])
			.margin(1)
			.split(rects[i]);

			frame.render_widget(
				Paragraph::new(Line::from(config.file.clone())),
				item[1],
			);
		}
	}
}
