use heapless::Deque;
use nadk::utils::KeyboardState;

use crate::{AppState, Key};

/// Cutekit's input record for recent inputs
#[derive(Clone, Copy, Debug)]
pub enum Input {
	/// A record for a pressed key at the given time
	Pressed { key: Key, timestamp: f32 },
	/// A record for a released key at the given time
	Released { key: Key, timestamp: f32 },
}

/// Cutekit's input manager, used by the global context and root functions
#[derive(Clone, Debug)]
pub struct InputManager<const N: usize = 16> {
	keyboard:     KeyboardState,
	input_record: Deque<Input, N>,
}

impl<const N: usize> InputManager<N> {
	pub const fn init() -> Self {
		InputManager {
			keyboard:     KeyboardState::new(),
			input_record: Deque::new(),
		}
	}

	pub fn update(&mut self) {
		self.keyboard.update();

		let now = AppState::get().frame_time;

		let enqueue = |s: &mut Self, v| {
			if s.input_record.len() == s.input_record.capacity() {
				s.input_record.pop_back();
			}
			s.input_record.push_front(v).unwrap();
		};

		for key in enum_iterator::all::<Key>() {
			if self.key_just_pressed(key) {
				enqueue(self, Input::Pressed { key, timestamp: now });
			}
			if self.key_just_released(key) {
				enqueue(self, Input::Released { key, timestamp: now });
			}
		}
	}

	pub fn key_just_pressed(&self, key: Key) -> bool { self.keyboard.key_just_pressed(key) }

	pub fn key_just_released(&self, key: Key) -> bool { self.keyboard.key_just_released(key) }

	pub fn key_down(&self, key: Key) -> bool { self.keyboard.key_down(key) }

	pub fn key_impulsed(&self, key: Key) -> bool {
		if !self.key_down(key) {
			return false;
		}

		let input = self.input_record.iter().find(|x| match x {
			Input::Pressed { key: k, .. } => *k == key,
			Input::Released { .. } => false,
		});

		if let Some(Input::Pressed { timestamp, .. }) = input {
			let state = AppState::get();
			let delay = state.frame_time - timestamp - 0.5;

			return delay >= 0.0
				&& (state.delta_time >= 0.06 || delay % 0.06 < delay - state.delta_time);
		}
		return false;
	}

	pub fn get_recent_inputs(&mut self) -> &[Input] { self.input_record.make_contiguous() }
}
