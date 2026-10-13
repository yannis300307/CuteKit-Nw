use nadk::display::{ScreenRect, push_rect};

use crate::{
	Color,
	MAX_PIPELINES,
	SCREEN_HEIGHT,
	SCREEN_TILE_HEIGHT,
	SCREEN_TILE_WIDTH,
	ScreenPos,
	inputs::InputManager,
	rendering::{PipelineRef, TileBuffer},
};

#[readonly::make]
pub struct Cutekit<const PIPELINES: usize = MAX_PIPELINES> {
	pub frame_time: f32,
	pub delta_time: f32,
	pub inputs:     InputManager,

	pipelines: heapless::Vec<PipelineRef, PIPELINES>,
}

impl Cutekit {
	const TARGET_DELTA_TIME: f32 = 0.05;

	fn get_time() -> f32 { nadk::time::get_current_time_millis() as f32 / 1000.0 }

	pub fn init() -> Self {
		Self {
			delta_time: Self::TARGET_DELTA_TIME,
			frame_time: Self::get_time() - Self::TARGET_DELTA_TIME,

			inputs: InputManager::init(),

			pipelines: heapless::Vec::new(),
		}
	}

	pub fn update(&mut self) {
		let now = Self::get_time();
		self.delta_time = now - self.frame_time;
		self.frame_time = now;

		self.inputs.update(self.delta_time);
	}

	pub fn schedule(&mut self, plugin: PipelineRef) -> Option<usize> {
		match self.pipelines.push(plugin) {
			Ok(_) => Some(self.pipelines.len() - 1),
			Err(_) => None,
		}
	}

	pub fn render(&mut self, clear_color: Color) {
		for y in (0..SCREEN_HEIGHT).step_by(SCREEN_TILE_HEIGHT) {
			let mut tile_buffer = TileBuffer::new(ScreenPos::new(0, y as i16), clear_color);

			for pipeline in self.pipelines.iter_mut() {
				pipeline.draw(&mut tile_buffer);
			}

			push_rect(
				ScreenRect {
					x:      0,
					y:      (SCREEN_TILE_HEIGHT * y) as u16,
					width:  SCREEN_TILE_WIDTH as u16,
					height: SCREEN_TILE_HEIGHT as u16,
				},
				&tile_buffer.buffer,
			);
		}
	}
}
