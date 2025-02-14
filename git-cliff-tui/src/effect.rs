use std::time::Instant;

use ratatui::{
	layout::Rect,
	style::Color,
};
use tachyonfx::{
	fx,
	Effect,
	HslConvertable,
};

use tachyonfx::Interpolatable;

pub trait IndexResolver<T: Clone> {
	fn resolve(idx: usize, data: &[T]) -> &T;
}

#[derive(Clone, Debug)]
pub struct ColorCycle<T: IndexResolver<Color>> {
	colors:  Vec<Color>,
	_marker: std::marker::PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct PingPongCycle;

impl IndexResolver<Color> for PingPongCycle {
	fn resolve(idx: usize, data: &[Color]) -> &Color {
		let dbl_idx = idx % (2 * data.len());
		let final_index = if dbl_idx < data.len() {
			dbl_idx
		} else {
			2 * data.len() - 1 - dbl_idx
		};

		data.get(final_index)
			.expect("ColorCycle: index out of bounds")
	}
}

pub type PingPongColorCycle = ColorCycle<PingPongCycle>;

#[derive(Clone, Debug)]
pub struct RepeatingCycle;

impl IndexResolver<Color> for RepeatingCycle {
	fn resolve(idx: usize, data: &[Color]) -> &Color {
		data.get(idx % data.len())
			.expect("ColorCycle: index out of bounds")
	}
}

pub type RepeatingColorCycle = ColorCycle<RepeatingCycle>;

impl<T> ColorCycle<T>
where
	T: IndexResolver<Color>,
{
	pub fn new(initial_color: Color, colors: &[(usize, Color)]) -> Self {
		let mut gradient = vec![initial_color];
		colors
			.iter()
			.fold((0, initial_color), |(_, prev_color), (len, color)| {
				(0..=*len).for_each(|i| {
					let color = prev_color.lerp(color, i as f32 / *len as f32);
					gradient.push(color);
				});
				gradient.push(*color);
				(*len, *color)
			});

		Self {
			colors:  gradient,
			_marker: std::marker::PhantomData,
		}
	}

	pub fn color_at(&self, idx: usize) -> &Color {
		T::resolve(idx, &self.colors)
	}
}

/// Creates a repeating color cycle based on a base color.
///
/// # Arguments
/// * `base_color` - Primary color to derive the cycle from
/// * `length_multiplier` - Factor to adjust the cycle length
///
/// # Returns
/// A ColorCycle instance with derived colors and adjusted steps.
fn create_color_cycle(
	base_color: Color,
	length_multiplier: usize,
) -> ColorCycle<RepeatingCycle> {
	let color_step: usize = 7 * length_multiplier;

	let (h, s, l) = base_color.to_hsl();

	let color_l = Color::from_hsl(h, s, 80.0);
	let color_d = Color::from_hsl(h, s, 40.0);

	RepeatingColorCycle::new(base_color, &[
		(4 * length_multiplier, color_d),
		(2 * length_multiplier, color_l),
		(
			4 * length_multiplier,
			Color::from_hsl((h - 25.0) % 360.0, s, (l + 10.0).min(100.0)),
		),
		(
			color_step,
			Color::from_hsl(h, (s - 20.0).max(0.0), (l + 10.0).min(100.0)),
		),
		(
			color_step,
			Color::from_hsl((h + 25.0) % 360.0, s, (l + 10.0).min(100.0)),
		),
		(
			color_step,
			Color::from_hsl(h, (s + 20.0).max(0.0), (l + 10.0).min(100.0)),
		),
	])
}

/// Creates an animated border effect using color cycling.
///
/// # Arguments
/// * `base_color` - The primary color to base the cycling effect on
/// * `area` - The rectangular area where the effect should be rendered
///
/// # Returns
///
/// An Effect that animates a border around the specified area using cycled
/// colors
pub fn create_border_effect(
	base_color: Color,
	speed: f32,
	length: usize,
	area: Rect,
) -> Effect {
	let color_cycle = create_color_cycle(base_color, length);

	let effect =
		fx::effect_fn_buf(Instant::now(), u32::MAX, move |started_at, ctx, buf| {
			let elapsed = started_at.elapsed().as_secs_f32();

			// speed n cells/s
			let idx = (elapsed * speed) as usize;

			let area = ctx.area;

			let mut update_cell = |(x, y): (u16, u16), idx: usize| {
				if let Some(cell) = buf.cell_mut((x, y)) {
					cell.set_fg(*color_cycle.color_at(idx));
				}
			};

			(area.x..area.right()).enumerate().for_each(|(i, x)| {
				update_cell((x, area.y), idx + i);
			});

			let cell_idx_offset = area.width as usize;
			(area.y + 1..area.bottom() - 1)
				.enumerate()
				.for_each(|(i, y)| {
					update_cell((area.right() - 1, y), idx + i + cell_idx_offset);
				});

			let cell_idx_offset =
				cell_idx_offset + area.height.saturating_sub(2) as usize;
			(area.x..area.right()).rev().enumerate().for_each(|(i, x)| {
				update_cell((x, area.bottom() - 1), idx + i + cell_idx_offset);
			});

			let cell_idx_offset = cell_idx_offset + area.width as usize;
			(area.y + 1..area.bottom())
				.rev()
				.enumerate()
				.for_each(|(i, y)| {
					update_cell((area.x, y), idx + i + cell_idx_offset);
				});
		});

	effect.with_area(area)
}
