use bytemuck::{Pod, Zeroable};

/// The color format of the screen. Encoded with 16 bits (2 bytes).
/// 5 bits are used for red, 6 for green and 5 for blue.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Zeroable, Pod)]
pub struct Color565(u16);

impl Color565 {
	/// Create a new Color565 using standard RGB-24 components.
	#[inline]
	pub const fn new(r: u16, g: u16, b: u16) -> Color565 {
		Color565(((r & 0b11111000) << 8) | ((g & 0b11111100) << 3) | ((b & 0b11111000) >> 3))
	}

	/// Extract the color's red, green and blue components into standard 8-bits values.
	#[inline]
	pub const fn components(&self) -> [u16; 3] {
		[
			self.0 >> 8 & 0b11111000,
			self.0 >> 3 & 0b11111100,
			self.0 << 3 & 0b11111000,
		]
	}
}

/// A rectangle on the screen defined by its top left corner coordinates and its size.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
	pub x:      u16,
	pub y:      u16,
	pub width:  u16,
	pub height: u16,
}

/// A point on the screen defined by its coordinates.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenPoint {
	pub x: u16,
	pub y: u16,
}
