use crate::state::State;
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
	text::Line,
	widgets::{
		Block,
		BorderType,
		List,
		ListItem,
		Scrollbar,
		ScrollbarOrientation,
		ScrollbarState,
	},
	Frame,
};

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
	frame.render_widget(
		Block::new().style(Style::default().bg(Color::Rgb(27, 27, 27))),
		frame.area(),
	);

	frame.render_widget(
		Block::new()
			.title_top(format!("{} ⛰️", env!("CARGO_PKG_NAME")).bold())
			.title_alignment(Alignment::Center),
		frame.area(),
	);

	let rects = Layout::horizontal([
		Constraint::Min(
			state
				.builtin_configs
				.iter()
				.map(|c| c.len() as u16)
				.map(|c| c + 6)
				.max()
				.unwrap_or_default(),
		),
		Constraint::Percentage(100),
	])
	.split(frame.area());
	render_list(state, frame, rects[0]);
	render_changelog(state, frame, rects[1]);

	if !state.logo.is_rendered {
		render_logo(state, frame);
	}
}

fn render_logo(state: &mut State, frame: &mut Frame) {
	let logo_area = Rect::new(
		frame.area().width / 2 - state.logo.width / 2,
		frame.area().height / 2 - state.logo.height / 2,
		state.logo.width,
		state.logo.height,
	);
	frame.render_widget(&mut state.logo, logo_area);
}

fn render_list(state: &mut State, frame: &mut Frame, area: Rect) {
	if !state.builtin_configs.is_empty() {
		let items = state
			.builtin_configs
			.iter()
			.map(|c| ListItem::new(c.to_string()))
			.collect::<Vec<ListItem>>();
		let list = List::new(items)
			.block(
				Block::bordered()
					.title_top("|Config|".yellow())
					.title_alignment(Alignment::Center)
					.border_type(BorderType::Rounded)
					.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
			)
			.style(Style::new().white())
			.highlight_style(Style::new().reversed());
		frame.render_stateful_widget(list, area, &mut state.list_state);
		frame.render_stateful_widget(
			Scrollbar::new(ScrollbarOrientation::VerticalRight)
				.begin_symbol(Some("↑"))
				.end_symbol(Some("↓")),
			area.inner(Margin {
				vertical:   1,
				horizontal: 0,
			}),
			&mut ScrollbarState::new(state.builtin_configs.len())
				.position(state.list_state.selected().unwrap_or_default()),
		);
	}
}

fn render_changelog(state: &mut State, frame: &mut Frame, area: Rect) {
	frame.render_widget(
		Block::bordered()
			.title_top("|Changelog|".yellow().into_left_aligned_line())
			.title_bottom(
				Line::from(if state.is_generating {
					vec![
						"|".fg(Color::Rgb(100, 100, 100)),
						"> Generating...".white(),
						"|".fg(Color::Rgb(100, 100, 100)),
					]
				} else if !state.contents.is_empty() {
					vec![
						"|".fg(Color::Rgb(100, 100, 100)),
						state
							.list_state
							.selected()
							.map(|i| state.builtin_configs[i].clone())
							.unwrap_or_default()
							.white()
							.italic(),
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
						"Select config to start".white(),
						"|".fg(Color::Rgb(100, 100, 100)),
					]
				})
				.left_aligned(),
			)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
		area,
	);
	frame.render_widget(
		tui_markdown::from_str(
			&state
				.contents
				.lines()
				.skip(state.scroll_index)
				.collect::<Vec<&str>>()
				.join("\n"),
		),
		area.inner(Margin {
			horizontal: 1,
			vertical:   1,
		}),
	);

	frame.render_stateful_widget(
		Scrollbar::new(ScrollbarOrientation::VerticalRight)
			.begin_symbol(Some("↑"))
			.end_symbol(Some("↓")),
		area.inner(Margin {
			vertical:   1,
			horizontal: 0,
		}),
		&mut ScrollbarState::new(state.contents.len()).position(state.scroll_index),
	);

	if state.is_generating {
		let throbber_area = Rect::new(
			area.left().saturating_add(2),
			area.bottom().saturating_sub(1),
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