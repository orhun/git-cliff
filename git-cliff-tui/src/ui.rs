use crate::{
	effect,
	state::State,
};
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
use tachyonfx::{
	Duration,
	EffectRenderer,
};

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
	frame.render_widget(
		Block::new().style(Style::default().bg(Color::Rgb(27, 27, 27))),
		frame.area(),
	);

	let rects = Layout::horizontal([
		Constraint::Min(
			state
				.configs
				.iter()
				.map(|c| c.name.len() as u16)
				.map(|c| c + 6)
				.max()
				.unwrap_or_default(),
		),
		Constraint::Percentage(100),
	])
	.split(frame.area());
	if state.toggle {
		render_list(state, frame, rects[0]);
		render_changelog(state, frame, rects[1]);
	} else {
		render_changelog(state, frame, frame.area());
	}

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
	if !state.configs.is_empty() {
		let items = state
			.configs
			.iter()
			.map(|c| ListItem::new(c.name.to_string()))
			.collect::<Vec<ListItem>>();
		let list = List::new(items)
			.block(
				Block::bordered()
					.title_top("|Config|".yellow())
					.title_alignment(Alignment::Left)
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
			&mut ScrollbarState::new(state.configs.len())
				.position(state.list_state.selected().unwrap_or_default()),
		);
	}
}

fn render_changelog(state: &mut State, frame: &mut Frame, area: Rect) {
	let contents = state
		.list_state
		.selected()
		.map(|i| state.configs[i].contents.clone())
		.unwrap_or_default();
	frame.render_widget(
		Block::bordered()
			.title_top(
				if state.is_generating {
					"|Generating...|"
				} else {
					"|Changelog|"
				}
				.yellow()
				.into_left_aligned_line(),
			)
			.title_bottom(
				Line::from(if !contents.is_empty() {
					vec![
						"|".fg(Color::Rgb(100, 100, 100)),
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
					vec![]
				})
				.right_aligned(),
			)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
		area,
	);
	frame.render_widget(
		tui_markdown::from_str(
			&contents
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
		&mut ScrollbarState::new(contents.len()).position(state.scroll_index),
	);

	match &mut state.border_effect {
		Some(effect) => {
			frame.render_effect(effect, area, Duration::from_millis(100));
		}
		None => {
			state.border_effect = Some(if state.is_generating {
				effect::create_border_effect(Color::Rgb(205, 100, 42), 60., 1, area)
			} else {
				effect::create_border_effect(Color::Gray, 30., 3, area)
			});
		}
	}
}
