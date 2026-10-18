use nadk::display::{ScreenRect, push_rect};

use crate::{
	Color,
	SCREEN_HEIGHT,
	SCREEN_TILE_HEIGHT,
	SCREEN_TILE_WIDTH,
	Vec2,
	drawing::{Drawable, TileBuffer},
	inputs::InputManager,
};

#[readonly::make]
pub struct Cutekit {
	pub frame_time: f32,
	pub delta_time: f32,
	pub inputs:     InputManager,
}

impl Cutekit {
	const TARGET_DELTA_TIME: f32 = 0.05;

	fn get_time() -> f32 { nadk::time::get_current_time_millis() as f32 / 1000.0 }

	pub fn init() -> Self {
		Self {
			delta_time: Self::TARGET_DELTA_TIME,
			frame_time: Self::get_time() - Self::TARGET_DELTA_TIME,

			inputs: InputManager::init(),
		}
	}

	pub fn update(&mut self) {
		let now = Self::get_time();
		self.delta_time = now - self.frame_time;
		self.frame_time = now;

		self.inputs.update(self.delta_time);
	}

	pub fn render<'a, const N: usize>(
		&mut self,
		clear_color: Color,
		mut pipelines: [&'a mut dyn Drawable; N],
	) {
		for y in (0..SCREEN_HEIGHT).step_by(SCREEN_TILE_HEIGHT as usize) {
			let mut tile_buffer = TileBuffer::new(
				Vec2::new(SCREEN_TILE_WIDTH, SCREEN_TILE_HEIGHT),
				clear_color,
			);

			for pipeline in pipelines.iter_mut() {
				pipeline.draw(&mut tile_buffer, Vec2::new(0, y));
			}

			push_rect(
				ScreenRect {
					x:      0,
					y:      y as u16,
					width:  SCREEN_TILE_WIDTH as u16,
					height: SCREEN_TILE_HEIGHT as u16,
				},
				&tile_buffer.data,
			);
		}
	}
}
