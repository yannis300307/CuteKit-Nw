use crate::inputs::InputManager;
use crate::nadk;

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
}
