use core::fmt::Debug;

use bytemuck::{AnyBitPattern, Pod, Zeroable};

/// Basic 16-bit RGB Color.
///
/// The red, green and blue channels each take up 5, 6 and 5 bits respectively.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ColorRGB {
	pub rgb: u16,
}

/// Basic 24-bit RGBA Color.
///
/// The red, green and blue channels each take up 5, 6 and 5 bits respectively.
/// The alpha channel takes a full byte.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ColorRGBA {
	pub rgb: ColorRGB,
	pub a:   u8,
}

/// Basic 8-bit grayscale Color.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ColorGrayscale {
	pub value: u8,
}

/// Trait implemented by values that can be stored inside of textures, usually colors.
pub trait AnyColor: AnyBitPattern + Debug + PartialEq {}
impl<T: AnyBitPattern + Debug + PartialEq> AnyColor for T {}

pub mod grayscale;
pub mod rgb;
pub mod rgba;

pub mod blend;
pub use blend::Blend;

pub mod palette {
	use super::ColorRGB;

	// Adaptation of Sweetie-16 [https://lospec.com/palette-list/sweetie-16]

	pub const RGB_BLACK: ColorRGB = ColorRGB::unpack(0x181c28);
	pub const RGB_PURPLE: ColorRGB = ColorRGB::unpack(0x582858);
	pub const RGB_RED: ColorRGB = ColorRGB::unpack(0xb03c50);
	pub const RGB_ORANGE: ColorRGB = ColorRGB::unpack(0xe87c58);
	pub const RGB_YELLOW: ColorRGB = ColorRGB::unpack(0xf8cc78);
	pub const RGB_LIME: ColorRGB = ColorRGB::unpack(0xa8f070);
	pub const RGB_GREEN: ColorRGB = ColorRGB::unpack(0x38b868);
	pub const RGB_TEAL: ColorRGB = ColorRGB::unpack(0x287078);
	pub const RGB_DARK_BLUE: ColorRGB = ColorRGB::unpack(0x283870);
	pub const RGB_BLUE: ColorRGB = ColorRGB::unpack(0x385cc8);
	pub const RGB_SKY_BLUE: ColorRGB = ColorRGB::unpack(0x40a8f8);
	pub const RGB_AQUA: ColorRGB = ColorRGB::unpack(0x70f0f8);
	pub const RGB_WHITE: ColorRGB = ColorRGB::unpack(0xf8f8f8);
	pub const RGB_LIGHT_GRAY: ColorRGB = ColorRGB::unpack(0x90b0c0);
	pub const RGB_MEDIUM_GRAY: ColorRGB = ColorRGB::unpack(0x586c88);
	pub const RGB_DARK_GRAY: ColorRGB = ColorRGB::unpack(0x303c58);
}
