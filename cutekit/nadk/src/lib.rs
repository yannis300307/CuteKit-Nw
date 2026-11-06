#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![allow(dead_code)]

#[cfg(not(any(feature = "epsilon", feature = "upsilon")))]
compile_error!(
	r#"No valid firmware selected!
	Add "epsilon" or "upsilon" to nadk's features"#
);

#[cfg(all(feature = "epsilon", feature = "upsilon"))]
compile_error!(
	r#"Multiple firmwares selected!
	Make sure you only have nadk's "epsilon" or "upsilon" feature enabled"#
);

#[cfg(all(feature = "upsilon", not(target_os = "none")))]
compile_error!(
	r#"Upsilon isn't compatible as a simulator!
	Use the "epsilon" feature instead for the numworks simulator"#
);

mod common {
	pub mod model;
	pub use model::*;

	cfg_select! {
		feature = "epsilon" => {
			pub mod display;
			pub mod inputs;
			pub use display::*;
			pub use inputs::*;
		}
		feature = "upsilon" => {
			pub mod display;
			pub mod inputs;
			pub mod rng;

			pub use display::*;
			pub use inputs::*;
		}
	}
	cfg_select! {
		target_os = "none" => {
			pub mod heap;
			pub mod panic;
		}
		_ => {}
	}
}
pub use common::*;

pub mod utils {
	pub mod inputs;
	pub use inputs::*;
}

mod platforms {
	#[cfg(target_os = "none")]
	pub const IS_SIMULATOR: bool = false;
	#[cfg(not(target_os = "none"))]
	pub const IS_SIMULATOR: bool = true;

	cfg_select! {
		feature = "epsilon" => {
			pub mod epsilon;
			pub type Platform = epsilon::Epsilon;
		}
		feature = "upsilon" => {
			pub mod upsilon;
			pub type Platform = upsilon::Upsilon;
		}
	}
}
use core::ffi::CStr;

pub use platforms::*;
cfg_select! {
	target_os = "none" => {
		extern crate alloc;
		use alloc::borrow::Cow;
	}
	_ => {
		use std::borrow::Cow;
	}
}

/// The width of this platform's screen in pixels.
const SCREEN_WIDTH: u16 = Platform::SCREEN_WIDTH;

/// The height of this platform's screen in pixels.
const SCREEN_HEIGHT: u16 = Platform::SCREEN_HEIGHT;

/// Initalizes the applications heap and other platform specific utilities.
#[inline]
pub fn init() { Platform::init() }

/// Sets the screen brightness to the specified value.
///
/// The value must range between [0;255].
///
/// Subject to change in future versions.
#[inline]
pub fn set_brightness(brightness: u8) { Platform::set_brightness(brightness) }

/// Gets the screen brightness.
///
/// The returned value ranges between [0;255].
///
/// Subject to change in future versions.
#[inline]
pub fn get_brightness() -> u8 { Platform::get_brightness() }

/// Scans the platform's keyboard and returns a bit mask.
///
/// Each variant of the associated key enum will have its own bit,
/// the bit will be set to 1 if that key is held.
///
/// Subject to change in future versions.
/// The `KeyboardState` utility should not change.
#[inline]
pub fn scan_keyboard() -> u64 { Platform::scan_keyboard() }

/// Returns a random integer between u32::MIN and u32::MAX.
#[inline]
pub fn get_random_u32() -> u32 { Platform::get_random_u32() }

/// Pauses the application for the given amount of milliseconds.
#[inline]
pub fn wait_ms(ms: u32) { Platform::wait_ms(ms) }

/// Gets the current milliseconds since platform's epoch
///
/// The epoch will usually correspond to the boot of the device.
///
/// Subject to change in future versions.
#[inline]
pub fn get_current_time_ms() -> u64 { Platform::get_current_time_ms() }

/// Renders a slice of pixels to a rectangular region on the screen.
///
/// The slice must be large enough to contain every pixel in the rect.
/// Pixels are arranged row-first, left to right
/// (i + 1 is the pixel on the right of i).
///
/// Panics is the position is out of screen bounds
///
/// Subject to change in future versions.
#[inline]
pub fn push_rect(rect: ScreenRect, pixels: &[Color565]) {
	assert!(
		rect.x + rect.width >= SCREEN_WIDTH || rect.y + rect.height >= SCREEN_HEIGHT,
		"Given rect is out of screen bounds"
	);
	Platform::push_rect(rect, pixels)
}

/// Fills the given slice of pixels
/// with a rectangular region on the screen's pixel data.
///
/// The slice must be large enough to contain every pixel in the rect.
/// Pixels are arranged row-first, left to right
/// (i + 1 is the pixel on the right of i).
///
/// Panics is the position is out of screen bounds
///
/// Subject to change in future versions.
#[inline]
pub fn pull_rect(rect: ScreenRect, pixels: &mut [Color565]) {
	assert!(
		rect.x + rect.width >= SCREEN_WIDTH || rect.y + rect.height >= SCREEN_HEIGHT,
		"Given rect is out of screen bounds"
	);
	Platform::pull_rect(rect, pixels)
}

/// Fills a rectangular region on the screen with a flat color.
///
/// Panics is the position is out of screen bounds
///
/// Subject to change in future versions.
#[inline]
pub fn push_rect_uniform(rect: ScreenRect, color: Color565) {
	assert!(
		rect.x + rect.width >= SCREEN_WIDTH || rect.y + rect.height >= SCREEN_HEIGHT,
		"Given rect is out of screen bounds"
	);
	Platform::push_rect_uniform(rect, color)
}

/// Waits until the physical screen has fully refreshed
#[inline]
pub fn wait_for_vblank() { Platform::wait_for_vblank() }

/// Renders text at the given position on the screen.
///
/// Panics is the position is out of screen bounds
#[inline]
pub fn draw_string(
	text: &CStr,
	pos: ScreenPoint,
	use_large_font: bool,
	color: Color565,
	background_color: Color565,
) {
	assert!(
		pos.x >= SCREEN_WIDTH || pos.y >= SCREEN_HEIGHT,
		"Given position is out of screen bounds"
	);
	Platform::draw_string(text, pos, use_large_font, color, background_color)
}

/// Writes data to the specified file.
///
/// The given C string must contain valid utf8 characters.
///
/// This function may fail from platform-specific io errors.
#[inline]
pub fn file_write(filename: &CStr, content: &[u8]) -> Result<(), ()> {
	Platform::file_write(filename, content)
}

/// Returns true if the specified file currently exists.
///
/// The given C string must contain valid utf8 characters.
#[inline]
pub fn file_exists(filename: &CStr) -> bool { Platform::file_exists(filename) }

/// Reads data from the specified file.
///
/// The resulting data may or may not be owned by the application.
/// - On common operating systems, the application will own the data.
/// - On numworks systems, the data is only borrowed.
///
/// The given C string must contain valid utf8 characters.
///
/// This function may fail from platform-specific io errors.
#[inline]
pub fn file_read(filename: &CStr) -> Result<Cow<'_, [u8]>, ()> { Platform::file_read(filename) }

/// Reads data from a specific section of the file.
///
/// The resulting data may or may not be owned by the application.
/// - On common operating systems, the application will own the data.
/// - On numworks systems, the data is only borrowed.
///
/// The given C string must contain valid utf8 characters.
///
/// This function may fail from platform-specific io errors.
#[inline]
pub fn file_read_slice(
	filename: &CStr,
	start: usize,
	length: usize,
) -> Result<Cow<'_, [u8]>, ()> {
	Platform::file_read_slice(filename, start, length)
}

/// Destroys the specified file.
///
/// The given C string must contain valid utf8 characters.
///
/// This function may fail from platform-specific io errors.
#[inline]
pub fn file_erase(filename: &CStr) -> Result<(), ()> { Platform::file_erase(filename) }

/// Fills the given slice with filenames with the given extension.
///
/// The extension refers to the section after the last dot.
///
/// The given C string must contain valid utf8 characters.
///
/// This function may fail from platform-specific io errors.
#[inline]
pub fn file_list_with_extension<const N: usize>(
	extension: &CStr,
	records: &mut [Cow<str>; N],
) -> Result<usize, ()> {
	Platform::file_list_with_extension(extension, records)
}

/// Returns the variant of the enum corresponding to this platform's model
///
/// Subject to change in future versions.
#[inline]
pub fn get_calculator_model() -> CalculatorModel { Platform::get_model() }
