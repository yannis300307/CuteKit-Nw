use crate::inputs::InputManager;

/// Struct used by Cutekit's global context to store its data.
#[derive(Debug)]
pub struct AppState {
	pub frame_time: f32,
	pub delta_time: f32,
	pub inputs:     &'static mut InputManager,
}

/// Cutekit's global context, used by some root functions.
static mut GLOBAL_APP_STATE: AppState = AppState {
	frame_time: 0.0,
	delta_time: 0.0,
	inputs:     &mut InputManager::init(),
};

impl AppState {
	/// Returns a mutable reference to GLOBAL_APP_STATE
	pub fn get_mut() -> &'static mut Self {
		unsafe {
			#[allow(static_mut_refs)]
			&mut GLOBAL_APP_STATE
		}
	}

	/// Returns a reference to GLOBAL_APP_STATE
	pub fn get() -> &'static Self {
		unsafe {
			#[allow(static_mut_refs)]
			&GLOBAL_APP_STATE
		}
	}
}
