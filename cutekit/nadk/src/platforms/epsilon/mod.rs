pub mod display;
#[cfg(target_os = "none")]
pub mod hardware;
#[cfg(not(target_os = "none"))]
pub mod simulator;

pub struct Epsilon;
impl Epsilon {
	pub const SCREEN_HEIGHT: u16 = 240;
	pub const SCREEN_WIDTH: u16 = 320;

	pub fn set_brightness(brightness: u8) {
		unsafe {
			eadk_backlight_set_brightness(brightness);
		}
	}

	pub fn get_brightness() -> u8 { unsafe { eadk_backlight_brightness() } }

	pub fn scan_keyboard() -> u64 { unsafe { eadk_keyboard_scan() } }

	pub fn get_random_u32() -> u32 { unsafe { eadk_random() } }

	pub fn wait_ms(delay: u32) {
		unsafe {
			eadk_timing_msleep(delay);
		}
	}

	pub fn get_current_time_ms() -> u64 { unsafe { eadk_timing_millis() } }
}

unsafe extern "C" {
	fn eadk_backlight_set_brightness(brightness: u8);
	fn eadk_backlight_brightness() -> u8;

	fn eadk_keyboard_scan() -> u64;

	fn eadk_random() -> u32;

	fn eadk_timing_msleep(delay: u32);
	fn eadk_timing_millis() -> u64;
}
