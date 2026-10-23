use core::array;

use nadk::display::{ScreenRect, push_rect};

use crate::{
	Color,
	GlobalState,
	SCREEN_HEIGHT,
	SCREEN_TILE_HEIGHT,
	SCREEN_TILE_WIDTH,
	Vec2,
	drawing::DrawableMut,
	inputs::InputManager,
	textures::TextureMut,
};

#[readonly::make]
pub struct Cutekit {
	pub delta_time: f32,
	pub inputs:     InputManager,
}

impl Cutekit {
	const TARGET_DELTA_TIME: f32 = 0.05;

	fn get_time() -> f32 { nadk::time::get_current_time_millis() as f32 / 1000.0 }

	fn init() -> Self {
		{
			let state = GlobalState::lock();
			state.frame_time = Self::get_time() - Self::TARGET_DELTA_TIME;
			GlobalState::unlock();
		}
		Self {
			delta_time: Self::TARGET_DELTA_TIME,
			inputs:     InputManager::init(),
		}
	}

	pub fn update(&mut self) {
		let now = Self::get_time();
		{
			let state = GlobalState::lock();
			self.delta_time = now - state.frame_time;
			state.frame_time = now;
			GlobalState::unlock();
		}

		self.inputs.update(self.delta_time);
	}

	pub fn render(&mut self, clear_color: Color, pipelines: &mut [&mut dyn DrawableMut]) {
		for y in (0..SCREEN_HEIGHT).step_by(SCREEN_TILE_HEIGHT as usize) {
			let mut buffer_data = array::repeat::<
				Color,
				{ SCREEN_TILE_WIDTH as usize * SCREEN_TILE_HEIGHT as usize },
			>(clear_color);
			let mut tile_buffer = TextureMut::from_data(
				Vec2::new(SCREEN_TILE_WIDTH, SCREEN_TILE_HEIGHT),
				&mut buffer_data,
			);

			for pipeline in pipelines.iter_mut() {
				pipeline.draw_mut(&mut tile_buffer, Vec2::new(0, y));
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

pub fn init() -> Cutekit {
	if cfg!(feature = "handle_nadk") {
		#[cfg(target_os = "none")]
		nadk::init_heap();
		nadk::utils::wait_ok_released();
	}

	Cutekit::init()
}
