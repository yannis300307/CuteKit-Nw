#[derive(Clone, Copy, Debug)]
pub struct GlobalState {
	lock:           bool,
	pub frame_time: f32,
}

static mut GLOBAL_STATE: GlobalState = GlobalState {
	lock:       false,
	frame_time: 0.0,
};

impl<'a> GlobalState {
	fn is_locked() -> bool { unsafe { GLOBAL_STATE.lock } }

	fn grab() -> &'a mut Self {
		unsafe {
			#[allow(static_mut_refs)]
			&mut GLOBAL_STATE
		}
	}

	pub fn try_get() -> Option<Self> {
		if Self::is_locked() {
			return None;
		} else {
			return Some(*Self::grab());
		}
	}

	pub fn get() -> Self { Self::try_get().expect("Get on locked Global State") }

	pub fn try_lock() -> Option<&'a mut Self> {
		if Self::is_locked() {
			return None;
		} else {
			let state = Self::grab();
			state.lock = true;
			return Some(state);
		}
	}

	pub fn lock() -> &'a mut Self { Self::try_lock().expect("Double lock on Global State") }

	pub fn try_unlock() -> bool {
		if Self::is_locked() {
			Self::grab().lock = false;
			return true;
		} else {
			return false;
		}
	}

	pub fn unlock() {
		Self::try_unlock()
			.then_some(())
			.expect("Double unlock on Global State");
	}
}
