use core::array;

use glam::i16vec2;

use crate::{
	AppState,
	ColorRGB,
	Key,
	SCREEN_HEIGHT,
	SCREEN_TILE_HEIGHT,
	SCREEN_TILE_WIDTH,
	drawing::Renderable,
	inputs::Input,
	textures::TextureMut,
};

/// Initializes Cutekit and nadk.
///
/// This function initializes the heap
/// and makes sure the application is ready to handle further instructions.
///
/// This function is not necessary for the good functioning of Cutekit.
pub fn init() {
	nadk::init();

	let state = AppState::get_mut();

	while state.inputs.key_down(Key::Ok) {
		nadk::wait_ms(50);
		state.inputs.update();
	}
	state.frame_time = nadk::get_current_time_ms() as f32 / 1000.0;
}

/// Updates the application.
///
/// This function will update inputs and timing-related elements,
/// and indicates the beginning of a new 'frame'.
///
/// This function should be called once at the beginning of your update loop.
pub fn update() {
	let now = nadk::get_current_time_ms() as f32 / 1000.0;

	let state = AppState::get_mut();
	let dt = now - state.frame_time;

	state.delta_time = dt;
	state.frame_time = now;
	state.inputs.update();
}

/// Gets the time in seconds elapsed between this frame and the last.
pub fn delta_time() -> f32 { AppState::get().delta_time }

/// Gets the estimated FPS of the application.
///
/// Equivalent to `1.0 / delta_time`
pub fn fps() -> f32 { 1.0 / delta_time() }

/// Get the time in seconds when the last update occured.
pub fn frame_time() -> f32 { AppState::get().frame_time }

/// Returns true if the given key has just been pressed this frame
pub fn is_key_just_pressed(key: Key) -> bool { AppState::get().inputs.key_just_pressed(key) }

/// Returns true if the given key has just been released this frame
pub fn is_key_just_released(key: Key) -> bool { AppState::get().inputs.key_just_released(key) }

/// Returns true if the given key is currently being pressed this frame
pub fn is_key_down(key: Key) -> bool { AppState::get().inputs.key_down(key) }

/// Returns true if the given key is currently being pressed this frame,
/// following a text-like behavior
/// (true when just pressed and every 0.06 seconds after 0.5 seconds)
pub fn is_key_impulsed(key: Key) -> bool { AppState::get().inputs.key_impulsed(key) }

/// Returns a slice of recently recorded key presses and releases
/// along with the timestamp corresponding to when each of them happened.
pub fn get_recent_inputs<'a>() -> &'a [Input] { AppState::get_mut().inputs.get_recent_inputs() }

/// Renders the given drawable elements on the screen,
/// clearing it with the given color beforehand.
///
/// See also the `Pipeline` and `Draw...` structs from the `draw_2d` module
pub fn render(clear_color: ColorRGB, pipelines: &mut [&mut dyn Renderable]) {
	for y in (0..SCREEN_HEIGHT).step_by(SCREEN_TILE_HEIGHT as usize) {
		let pos = i16vec2(0, y);

		let mut buffer_data = array::repeat::<
			ColorRGB,
			{ SCREEN_TILE_WIDTH as usize * SCREEN_TILE_HEIGHT as usize },
		>(clear_color);

		let mut tile_buffer = TextureMut::from_data(
			i16vec2(SCREEN_TILE_WIDTH, SCREEN_TILE_HEIGHT),
			&mut buffer_data,
		);

		for pipeline in pipelines.iter_mut() {
			pipeline.render(&mut tile_buffer, pos);
		}

		nadk::push_rect(
			nadk::ScreenRect {
				x:      0,
				y:      y as u16,
				width:  SCREEN_TILE_WIDTH as u16,
				height: SCREEN_TILE_HEIGHT as u16,
			},
			ColorRGB::slice_to_nadk(tile_buffer.get_data()),
		);
	}
}
