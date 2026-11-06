use crate::{ColorRGB, ColorRGBA, color::AnyColor};

/// Trait implemented by texture pixel values to be drawable over other pixel types.
pub trait Blend<BG: AnyColor, OUT: AnyColor = BG>: AnyColor {
	/// Blends two colors together,
	/// usually using classic alpha blending.
	fn blend(bg: BG, fg: Self) -> OUT;
}

impl<BG: AnyColor, FG: AnyColor> Blend<BG> for FG
where
	BG: From<FG>,
{
	#[inline]
	default fn blend(_bg: BG, fg: FG) -> BG { fg.into() }
}

impl Blend<ColorRGB> for ColorRGBA {
	fn blend(bg: ColorRGB, fg: ColorRGBA) -> ColorRGB {
		match fg.a {
			255 => return fg.rgb,
			0 => return bg,
			_ => {}
		}
		let [bg_r, bg_g, bg_b] = bg.rgb();
		let [fg_r, fg_g, fg_b, fg_a] = fg.rgba();
		let bg_a = 256 - fg_a;

		ColorRGB::new(
			(bg_r * bg_a + fg_r * fg_a) / 256,
			(bg_g * bg_a + fg_g * fg_a) / 256,
			(bg_b * bg_a + fg_b * fg_a) / 256,
		)
	}
}

impl Blend<ColorRGBA> for ColorRGBA {
	fn blend(bg: ColorRGBA, fg: ColorRGBA) -> ColorRGBA {
		match fg.a {
			255 => return fg,
			0 => return bg,
			_ => {}
		}
		let [bg_r, bg_g, bg_b, bg_a] = bg.rgba();
		let [fg_r, fg_g, fg_b, fg_a] = fg.rgba();

		let nbga = bg_a * (255 - fg_a) / 255;
		let blend = fg_a + nbga;

		ColorRGBA::new(
			(bg_r * nbga + fg_r * fg_a) / blend,
			(bg_g * nbga + fg_g * fg_a) / blend,
			(bg_b * nbga + fg_b * fg_a) / blend,
			blend / 255,
		)
	}
}
