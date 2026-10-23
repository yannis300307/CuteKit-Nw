use bytemuck::{Pod, Zeroable};

use crate::Color;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ColorAlpha {
	pub rgb:   Color,
	pub alpha: u8,
}

impl ColorAlpha {
	#[inline]
	pub const fn new(r: u16, g: u16, b: u16, alpha: u16) -> Self {
		Self {
			rgb:   Color::new(r, g, b),
			alpha: alpha as u8,
		}
	}

	#[inline]
	pub const fn from_rgba888(r: u16, g: u16, b: u16, a: u16) -> Self {
		Self {
			rgb:   Color::from_rgb888(r, g, b),
			alpha: a as u8,
		}
	}

	#[inline]
	pub const fn from_color(color: Color) -> Self {
		Self {
			rgb:   color,
			alpha: 255,
		}
	}

	#[inline]
	pub const fn get_components(&self) -> (u16, u16, u16, u16) {
		let rgb = self.rgb;
		let (r, g, b) = rgb.get_components();
		(r, g, b, self.alpha as u16)
	}
}

impl From<Color> for ColorAlpha {
	#[inline]
	fn from(color: Color) -> Self { Self::from_color(color) }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ColorGrayscale {
	pub value: u8,
}

impl ColorGrayscale {
	#[inline]
	pub const fn new(value: u8) -> Self { Self { value } }

	#[inline]
	pub const fn to_color(self) -> Color {
		Color::from_rgb888(self.value as u16, self.value as u16, self.value as u16)
	}
}

pub mod blend;
pub use blend::Blend;
