use nanorand::{Rng, SeedableRng};

use crate::{CalculatorModel, heap, rng};

pub mod display;
pub mod fs;

pub struct Upsilon;
impl Upsilon {
	pub const SCREEN_HEIGHT: u16 = 240;
	pub const SCREEN_WIDTH: u16 = 320;

	pub fn init() {
		unsafe {
			heap::get().init(_heap_base as usize, _heap_size as usize);
		}
		rng::get_mut().reseed(crate::get_current_time_ms().to_le_bytes());
	}

	pub fn set_brightness(_brightness: u8) {}

	pub fn get_brightness() -> u8 { 0 }

	pub fn scan_keyboard() -> u64 { unsafe { extapp_scanKeyboard() } }

	pub fn get_random_u32() -> u32 { rng::get_mut().generate() }

	pub fn get_current_time_ms() -> u64 { unsafe { extapp_millis() } }

	pub fn wait_ms(ms: u32) { unsafe { extapp_msleep(ms) } }

	pub fn get_model() -> CalculatorModel { CalculatorModel::Upsilon }
}

#[macro_export]
macro_rules! configure_app {
	($app_name: expr, $icon_path: expr) => {
		#[unsafe(no_mangle)]
		pub extern "C" fn extapp_main() { crate::main(); }
	};
}

unsafe extern "C" {
	pub static mut _heap_base: *mut u8;
	pub static mut _heap_size: u32;

	fn extapp_scanKeyboard() -> u64;

	fn extapp_msleep(ms: u32);
	fn extapp_millis() -> u64;
}

#[unsafe(no_mangle)]
pub extern "C" fn setjmp(_: u32) -> u32 { return 0; }

#[unsafe(no_mangle)]
pub extern "C" fn longjmp(_: u32, _: u32) {}
