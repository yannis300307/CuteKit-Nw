use core::{ffi::CStr, panic::PanicInfo};

use crate::{Color565, Platform, ScreenPoint, ScreenRect};

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
	Platform::push_rect_uniform(
		ScreenRect {
			x:      0,
			y:      0,
			width:  Platform::SCREEN_WIDTH,
			height: Platform::SCREEN_HEIGHT,
		},
		Color565::new(255, 0, 0),
	);

	Platform::draw_string(
		c"Oops :(",
		ScreenPoint { x: 10, y: 10 },
		true,
		Color565::new(0, 0, 0),
		Color565::new(255, 0, 0),
	);

	if let Some(loc) = panic.location() {
		Platform::draw_string(
			loc.file_as_c_str(),
			ScreenPoint { x: 10, y: 30 },
			false,
			Color565::new(0, 0, 0),
			Color565::new(255, 0, 0),
		);
	}

	if let Some(msg) = panic.message().as_str() {
		let mut start = 0;
		let mut y = 50;

		while start < msg.len() && y < Platform::SCREEN_HEIGHT {
			let l1 = &msg[start..];
			let line = &l1[..42.min(l1.len())];
			let end = line.find('\n').or(line.rfind(' ')).unwrap_or(line.len());
			let bytes = line[..end].as_bytes();

			let buffer: [u8; 43] = core::array::from_fn(|i| *bytes.get(i).unwrap_or(&0));
			unsafe {
				Platform::draw_string(
					CStr::from_bytes_with_nul_unchecked(&buffer),
					ScreenPoint { x: 10, y },
					false,
					Color565::new(0, 0, 0),
					Color565::new(255, 0, 0),
				);
			}
			start = end + 1;
			y += 20;
		}
	}

	loop {
		Platform::wait_ms(50);
	}
}
