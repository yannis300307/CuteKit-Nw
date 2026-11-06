use core::ffi::{CStr, c_char};

use crate::{Color565, ScreenPoint, ScreenRect, epsilon::Epsilon};

impl Epsilon {
	pub fn push_rect(rect: ScreenRect, pixels: &[Color565]) {
		unsafe {
			eadk_display_push_rect(rect, pixels.as_ptr());
		}
	}

	pub fn pull_rect(rect: ScreenRect, data: &mut [Color565]) {
		unsafe {
			eadk_display_pull_rect(rect, data.as_mut_ptr());
		}
	}

	pub fn push_rect_uniform(rect: ScreenRect, color: Color565) {
		unsafe {
			eadk_display_push_rect_uniform(rect, color);
		}
	}

	pub fn wait_for_vblank() {
		unsafe {
			eadk_display_wait_for_vblank();
		}
	}

	pub fn draw_string(
		text: &CStr,
		point: ScreenPoint,
		large_font: bool,
		text_color: Color565,
		background_color: Color565,
	) {
		unsafe {
			eadk_display_draw_string(
				text.as_ptr(),
				point,
				large_font,
				text_color,
				background_color,
			)
		}
	}
}

unsafe extern "C" {
	fn eadk_display_push_rect_uniform(rect: ScreenRect, color: Color565);
	fn eadk_display_push_rect(rect: ScreenRect, color: *const Color565);
	fn eadk_display_wait_for_vblank();
	fn eadk_display_pull_rect(rect: ScreenRect, color: *mut Color565);
	fn eadk_display_draw_string(
		text: *const c_char,
		point: ScreenPoint,
		large_font: bool,
		text_color: Color565,
		background_color: Color565,
	);
}
