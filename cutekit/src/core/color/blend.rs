use crate::{Color, ColorAlpha, ColorGrayscale, textures::AnyColor};

pub trait Blend<BG: AnyColor, OUT: AnyColor = BG>: AnyColor {
	fn blend(bg: BG, fg: Self) -> OUT;
}

impl Blend<Color> for Color {
	#[inline]
	fn blend(_bg: Color, fg: Color) -> Color { fg }
}

impl Blend<Color> for ColorAlpha {
	fn blend(bg: Color, fg: ColorAlpha) -> Color {
		match fg.alpha {
			255 => return fg.rgb,
			0 => return bg,
			_ => {}
		}
		let (bg_r, bg_g, bg_b) = bg.get_components();
		let (fg_r, fg_g, fg_b, fg_a) = fg.get_components();
		let bg_a = 256 - fg_a;

		Color::new(
			(bg_r * bg_a + fg_r * fg_a) / 256,
			(bg_g * bg_a + fg_g * fg_a) / 256,
			(bg_b * bg_a + fg_b * fg_a) / 256,
		)
	}
}

impl Blend<Color> for ColorGrayscale {
	#[inline]
	fn blend(_bg: Color, fg: ColorGrayscale) -> Color { fg.to_color() }
}

impl Blend<ColorAlpha> for Color {
	#[inline]
	fn blend(_bg: ColorAlpha, fg: Color) -> ColorAlpha { ColorAlpha::from_color(fg) }
}

impl Blend<ColorAlpha> for ColorAlpha {
	fn blend(bg: ColorAlpha, fg: ColorAlpha) -> ColorAlpha {
		match fg.alpha {
			255 => return fg,
			0 => return bg,
			_ => {}
		}
		let (bg_r, bg_g, bg_b, bg_a) = bg.get_components();
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
}
