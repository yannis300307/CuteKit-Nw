use glam::U16Vec3;

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
		let bg_rgb = U16Vec3::from_array(bg.rgb());
		let fg_rgb = U16Vec3::from_array(fg.rgb());
		let fg_a = fg.a as u16;
		let bg_a = 256 - fg_a;

		ColorRGB::from_slice(&((bg_rgb * bg_a + fg_rgb * fg_a) / 256).to_array())
	}
}

impl Blend<ColorRGBA> for ColorRGBA {
	fn blend(bg: ColorRGBA, fg: ColorRGBA) -> ColorRGBA {
		match fg.a {
			255 => return fg,
			0 => return bg,
			_ => {}
		}
		let bg_rgb = U16Vec3::from_array(bg.rgb());
		let fg_rgb = U16Vec3::from_array(fg.rgb());
		let bg_a = bg.a as u16;
		let fg_a = fg.a as u16;

		let nbga = bg_a * (255 - fg_a) / 255;
		let blend = fg_a + nbga;

		ColorRGBA::from_slice(
			&((bg_rgb * nbga + fg_rgb * fg_a) / blend)
				.extend(blend / 255)
				.to_array(),
		)
	}
}
