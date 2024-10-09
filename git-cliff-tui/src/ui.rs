use crate::state::State;
use md_tui::nodes::root::Component;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Layout,
		Margin,
		Rect,
	},
	style::{
		Color,
		Modifier,
		Style,
		Stylize,
	},
	text::{
		Line,
		Span,
	},
	widgets::{
		Block,
		BorderType,
		Paragraph,
		Scrollbar,
		ScrollbarOrientation,
		ScrollbarState,
		Wrap,
	},
	Frame,
};

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
	("⏎ ", "Generate Changelog"),
	("↕ ↔ ", "Scroll"),
	("t", "Toggle"),
	("q", "Quit"),
];

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
	frame.render_widget(
		Block::new()
			.title_top(format!("{} ⛰️", env!("CARGO_PKG_NAME")).bold())
			.title_alignment(Alignment::Center),
		frame.area(),
	);

	let rects = Layout::vertical([Constraint::Percentage(100), Constraint::Min(3)])
		.margin(1)
		.split(frame.area());
	render_key_bindings(frame, rects[1]);

	let rects = Layout::horizontal([
		Constraint::Min(
			state.is_toggled as u16 *
				state
					.configs
					.iter()
					.map(|c| c.file.len() as u16)
					.map(|c| c + 6)
					.max()
					.unwrap_or_default(),
		),
		Constraint::Percentage(100),
	])
	.split(rects[0]);
	render_list(state, frame, rects[0]);
	if state.error.is_some() {
		render_error(state, frame, rects[1]);
	} else {
		render_changelog(state, frame, rects[1]);
	}
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
			.title_top("|Config|".yellow())
			.title_alignment(Alignment::Center)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
		rect,
	);
	if !state.configs.is_empty() {
		let item_count = ((rect.height - 2) / 3) as usize;
		let start_offset = (state.selected_index + 1).saturating_sub(item_count);
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
						let mut style = Style::new().fg(Color::Rgb(100, 100, 100));
						if config.is_hovered {
							style = style.yellow()
						} else if state.selected_index == i + start_offset {
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
		if state.configs.len() * 3 > rect.height as usize - 2 {
			frame.render_stateful_widget(
				Scrollbar::new(ScrollbarOrientation::VerticalRight)
					.begin_symbol(Some("↑"))
					.end_symbol(Some("↓")),
				rect.inner(Margin {
					vertical:   1,
					horizontal: 0,
				}),
				&mut ScrollbarState::new(item_count).position(state.selected_index),
			);
		}
	}
}

fn render_changelog(state: &mut State, frame: &mut Frame, rect: Rect) {
	state.markdown.area = rect.inner(Margin {
		horizontal: 1,
		vertical:   1,
	});
	frame.render_widget(
		Block::bordered()
			.title_top("|Changelog|".yellow().into_left_aligned_line())
			.title_bottom(
				Line::from(if state.is_generating {
					vec![
						"|".fg(Color::Rgb(100, 100, 100)),
						"> Generating...".white().into(),
						"|".fg(Color::Rgb(100, 100, 100)),
					]
				} else if state.markdown.component.is_some() {
					vec![
						"|".fg(Color::Rgb(100, 100, 100)),
						state.configs[state.markdown.config_index]
							.file
							.clone()
							.white()
							.italic(),
						"|".fg(Color::Rgb(100, 100, 100)),
						" |".fg(Color::Rgb(100, 100, 100)),
						if state.autoload {
							"a".green().bold()
						} else {
							"a".red().bold()
						},
						"utoload".white(),
						"|".fg(Color::Rgb(100, 100, 100)),
						" |".fg(Color::Rgb(100, 100, 100)),
						"c".yellow().bold(),
						"opy".white(),
						"|".fg(Color::Rgb(100, 100, 100)),
						" |".fg(Color::Rgb(100, 100, 100)),
						if state.args.unreleased {
							"u".green().bold()
						} else {
							"u".red().bold()
						},
						"nreleased".white(),
						"|".fg(Color::Rgb(100, 100, 100)),
						" |".fg(Color::Rgb(100, 100, 100)),
						if state.args.latest {
							"l".green().bold()
						} else {
							"l".red().bold()
						},
						"atest".white(),
						"|".fg(Color::Rgb(100, 100, 100)),
					]
				} else {
					vec![
						"|".fg(Color::Rgb(100, 100, 100)),
						"Select config to start".white().into(),
						"|".fg(Color::Rgb(100, 100, 100)),
					]
				})
				.left_aligned(),
			)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
		rect,
	);
	if let Some(component) = &mut state.markdown.component {
		let mut height = 2;
		for child in component.children() {
			if let Component::TextComponent(c) = child {
				let mut c = c.clone();
				c.set_scroll_offset(state.markdown.scroll_index);
				c.set_y_offset(height);
				height += c.height();
				if c.height() + c.scroll_offset() + 1 < height {
					frame.render_widget(c.clone(), state.markdown.area);
				}
			}
		}
		frame.render_stateful_widget(
			Scrollbar::new(ScrollbarOrientation::VerticalRight)
				.begin_symbol(Some("↑"))
				.end_symbol(Some("↓")),
			rect.inner(Margin {
				vertical:   1,
				horizontal: 0,
			}),
			&mut ScrollbarState::new(component.height() as usize)
				.position(state.markdown.scroll_index as usize),
		);
	}

	if state.is_generating {
		let throbber_area = Rect::new(
			rect.left().saturating_add(2),
			rect.bottom().saturating_sub(1),
			1,
			1,
		);
		frame.render_stateful_widget(
			throbber_widgets_tui::Throbber::default()
				.style(Style::default().fg(Color::Yellow))
				.throbber_style(
					Style::default()
						.fg(Color::Yellow)
						.add_modifier(Modifier::BOLD),
				)
				.throbber_set(throbber_widgets_tui::BLACK_CIRCLE)
				.use_type(throbber_widgets_tui::WhichUse::Spin),
			throbber_area,
			&mut state.throbber_state,
		);
	}
}

fn render_error(state: &mut State, frame: &mut Frame, rect: Rect) {
	if let Some(error) = &state.error {
		frame.render_widget(
			Block::bordered()
				.title_top("|Error|".red().into_centered_line())
				.border_type(BorderType::Rounded)
				.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
			rect,
		);
		frame.render_widget(
			Paragraph::new(Line::from(error.clone()))
				.alignment(Alignment::Center)
				.wrap(Wrap { trim: false }),
			rect.inner(Margin {
				horizontal: 1,
				vertical:   1,
			}),
		);
	}
}
