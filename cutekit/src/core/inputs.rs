use enum_iterator::all;
use nadk::{
	keyboard::KeyboardState,
	{self},
};

use crate::Key;

pub struct InputManager {
	down:          KeyboardState,
	just_pressed:  KeyboardState,
	just_released: KeyboardState,
	last_pressed:  Option<Key>,
	impulsed_key:  Option<Key>,
	impulse_timer: f32,
}

impl InputManager {
	pub fn init() -> Self {
		InputManager {
			down:          KeyboardState::default(),
			just_pressed:  KeyboardState::default(),
			just_released: KeyboardState::default(),

			last_pressed:  None,
			impulsed_key:  None,
			impulse_timer: 0.0,
		}
	}

	pub fn refresh(&mut self) {
		let next = nadk::keyboard::KeyboardState::scan();

		self.just_pressed = next.get_just_pressed(self.down);
		self.just_released = next.get_just_released(self.down);
		self.down = next;
	}

	pub fn update(&mut self, delta_time: f32) {
		self.refresh();

		if let Some(key) = self.get_last_pressed() {
			self.last_pressed = Some(key);
			self.impulse_timer = 0.5;
		}

		if let Some(key) = self.last_pressed {
			if self.key_down(key) {
				self.impulse_timer -= delta_time;

				if self.impulse_timer < 0.0 {
					self.impulse_timer += 0.06;
					self.impulsed_key = self.last_pressed;
				}
			} else {
				self.last_pressed = None;
				self.impulse_timer = 0.0;
			}
		}
	}

	pub fn get_last_pressed(&self) -> Option<Key> {
		for key in all::<Key>() {
			if self.key_just_pressed(key) {
				return Some(key);
			}
		}
		None
	}

	pub fn key_impulsed(&self, key: Key) -> bool { self.impulsed_key == Some(key) }

	pub fn key_just_pressed(&self, key: Key) -> bool { self.just_pressed.key_down(key) }

	pub fn key_just_released(&self, key: Key) -> bool { self.just_released.key_down(key) }

	pub fn key_down(&self, key: Key) -> bool { self.down.key_down(key) }

	pub fn wait_delay_or_ok(&mut self, delay_ms: usize) {
		while self.key_down(Key::Ok) {
			self.refresh();
			nadk::time::wait_milliseconds(50);
		}
		for _ in 0..delay_ms / 50 {
			self.refresh();
			if self.key_just_pressed(Key::Ok) {
				break;
			}
			nadk::time::wait_milliseconds(50);
		}
	}
}
