use bytemuck::{Pod, Zeroable};

use crate::Color;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ColorAlpha {
	pub rgb:   Color,
	pub alpha: u8,
}

impl ColorAlpha {
	pub const fn new(r: u16, g: u16, b: u16, alpha: u16) -> Self {
		Self {
			rgb:   Color::new(r, g, b),
			alpha: alpha as u8,
		}
	}

	pub const fn from_rgba888(r: u16, g: u16, b: u16, a: u16) -> Self {
		Self {
			rgb:   Color::from_rgb888(r, g, b),
			alpha: a as u8,
		}
	}

	pub const fn from_color(color: Color) -> Self {
		Self {
			rgb:   color,
			alpha: 255,
		}
	}

	pub const fn get_components(&self) -> (u16, u16, u16, u16) {
		let rgb = self.rgb;
		let (r, g, b) = rgb.get_components();
		(r, g, b, self.alpha as u16)
	}
}

impl From<Color> for ColorAlpha {
	fn from(color: Color) -> Self { Self::from_color(color) }
}

pub trait Blendable<T> {
	fn blend(self, fg: T) -> Self;
	fn blend_unchecked(self, fg: T) -> Self;
}

impl Blendable<Color> for Color {
	fn blend_unchecked(self, fg: Color) -> Color { fg }

	fn blend(self, fg: Color) -> Color { fg }
}

impl Blendable<ColorAlpha> for Color {
	fn blend_unchecked(self, fg: ColorAlpha) -> Color {
		let (bg_r, bg_g, bg_b) = self.get_components();
		let (fg_r, fg_g, fg_b, fg_a) = fg.get_components();
		let bg_a = 255 - fg_a;

		Color::new(
			(bg_r * bg_a + fg_r * fg_a) / 255,
			(bg_g * bg_a + fg_g * fg_a) / 255,
			(bg_b * bg_a + fg_b * fg_a) / 255,
		)
	}

	fn blend(self, fg: ColorAlpha) -> Color {
		if fg.alpha == 255 {
			return fg.rgb;
		}
		if fg.alpha == 0 {
			return self;
		}
		self.blend_unchecked(fg)
	}
}

impl Blendable<Color> for ColorAlpha {
	fn blend_unchecked(self, fg: Color) -> ColorAlpha { ColorAlpha::from_color(fg) }

	fn blend(self, fg: Color) -> ColorAlpha { ColorAlpha::from_color(fg) }
}

impl Blendable<ColorAlpha> for ColorAlpha {
	fn blend_unchecked(self, fg: ColorAlpha) -> ColorAlpha {
		let (bg_r, bg_g, bg_b, bg_a) = self.get_components();
		let (fg_r, fg_g, fg_b, fg_a) = fg.get_components();

		let nbga = bg_a * (255 - fg_a) / 255;
		let blend = fg_a + nbga;

		ColorAlpha::new(
			(bg_r * nbga + fg_r * fg_a) / blend,
			(bg_g * nbga + fg_g * fg_a) / blend,
			(bg_b * nbga + fg_b * fg_a) / blend,
			blend / 255,
		)
	}

	fn blend(self, fg: ColorAlpha) -> ColorAlpha {
		if fg.alpha == 255 {
			return fg;
		}
		if fg.alpha == 0 {
			return self;
		}
		self.blend_unchecked(fg)
	}
}
