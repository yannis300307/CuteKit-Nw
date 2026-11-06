use core::ffi::{CStr, c_char};

use crate::{Color565, ScreenPoint, ScreenRect, upsilon::Upsilon};

impl Upsilon {
	pub fn push_rect(rect: ScreenRect, pixels: &[Color565]) {
		unsafe {
			extapp_pushRect(rect.x, rect.y, rect.width, rect.height, pixels.as_ptr());
		}
	}

	pub fn pull_rect(rect: ScreenRect, data: &mut [Color565]) {
		unsafe {
			extapp_pullRect(rect.x, rect.y, rect.width, rect.height, data.as_mut_ptr());
		}
	}

	pub fn push_rect_uniform(rect: ScreenRect, color: Color565) {
		unsafe {
			extapp_pushRectUniform(rect.x, rect.y, rect.width, rect.height, color);
		}
	}

	pub fn wait_for_vblank() {
		unsafe {
			extapp_waitForVBlank();
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
			(if large_font {
				extapp_drawTextLarge
			} else {
				extapp_drawTextSmall
			})(
				text.as_ptr(),
				point.x,
				point.y,
				text_color,
				background_color,
				false,
			)
		}
	}
}
unsafe extern "C" {
	fn extapp_pushRectUniform(x: u16, y: u16, w: u16, h: u16, color: Color565);
	fn extapp_pushRect(x: u16, y: u16, w: u16, h: u16, color: *const Color565);
	fn extapp_waitForVBlank() -> bool;
	fn extapp_pullRect(x: u16, y: u16, w: u16, h: u16, color: *mut Color565);
	fn extapp_drawTextLarge(
		text: *const c_char,
		x: u16,
		y: u16,
		text_color: Color565,
		background_color: Color565,
		fake: bool,
	);
	fn extapp_drawTextSmall(
		text: *const c_char,
		x: u16,
		y: u16,
		text_color: Color565,
		background_color: Color565,
		fake: bool,
	);
}
