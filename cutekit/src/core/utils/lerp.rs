use crate::{Color, ColorAlpha, ColorGrayscale};

fn lerp8(start: u16, end: u16, t: u16) -> u16 { (start * (256 - t) + end * t) / 256 }

pub trait Lerp: Sized {
	fn lerp8(start: Self, end: Self, t: u8) -> Self;
	fn lerp(start: Self, end: Self, t: f32) -> Self {
		Self::lerp8(start, end, (t * 255.0) as u8)
	}
}

impl Lerp for Color {
	fn lerp8(start: Self, end: Self, t: u8) -> Self {
		let (sr, sg, sb) = start.get_components();
		let (er, eg, eb) = end.get_components();
		let t16 = t as u16;
		Color::new(lerp8(sr, er, t16), lerp8(sg, eg, t16), lerp8(sb, eb, t16))
	}
}

impl Lerp for ColorAlpha {
	fn lerp8(start: Self, end: Self, t: u8) -> Self {
		let (sr, sg, sb, sa) = start.get_components();
		let (er, eg, eb, ea) = end.get_components();
		let t = t as u16;

		ColorAlpha::new(
			lerp8(sr, er, t),
			lerp8(sg, eg, t),
			lerp8(sb, eb, t),
			lerp8(sa, ea, t),
		)
	}
}

impl Lerp for ColorGrayscale {
	fn lerp8(start: Self, end: Self, t: u8) -> Self {
		ColorGrayscale::new(lerp8(start.value as u16, end.value as u16, t as u16) as u8)
	}
}
