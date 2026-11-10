use core::fmt::Debug;

use enum_map::{Enum, EnumMap};
use libm::floorf;

use crate::{
	ColorGrayscale,
	ColorRGBA,
	Rect,
	Texture,
	TextureAtlas,
	Vec2,
	color::Blend,
	drawing::Draw,
	mods::draw_2d::drawing::sprite::DrawSprite,
	textures::{AnyTexture, AnyTextureMut},
};

/// Trait implemented by enums that correspond to a character set.
pub trait Charset: Enum + Copy + Debug + Default {
	fn from_char(c: char) -> Option<Self>;
}

#[path = "charsets"]
pub mod charsets {
	pub mod ascii;
	pub mod base;
	pub mod basic_text;
	pub mod numeric;
}

/// Trait implemented by fonts and objects able to render using text.
pub trait AnyFont<TXF: AnyTextureMut>: Debug {
	fn draw_string(&self, tile: &mut TXF::LtAs<'_>, pos: Vec2<i16>, str: &str);
}

/// Simple colored font.
///
/// Stores an atlas of grayscale textures,
/// and renders them by interpolating between the two stored RGBA colors.
///
/// This font has basic support for offset and variable sized glyphs.
#[derive(Debug)]
pub struct ColorFont<'a, K: Charset> {
	atlas:             TextureAtlas<'a, K, ColorGrayscale>,
	origins:           EnumMap<K, Vec2<i16>>,
	horizontal_margin: i16,
	line_height:       i16,
	glyph_color:       ColorRGBA,
	background_color:  ColorRGBA,
}

impl<'a, K: Charset> ColorFont<'a, K> {
	/// Creates a new font from the given format string.
	///
	/// With the format string `"Ab."`,
	/// the glyph slice should contain the following textures in this order:
	///
	/// uppercase A | lowercase B | dot
	/// --- | --- | ---
	///
	/// The character in the format string are determined by the enum's `from_char` method.
	///
	/// Any missing character in the format string will use the given default texture.
	///
	/// If the glyph slice's size doesn't match the string's length,
	/// or if the format string contains invalid characters, this method will panic.
	///
	/// If the given format string contains duplicate characters,
	/// the method will not panic and the latest character will be used.
	pub fn from_fmt<const N: usize>(
		horizontal_margin: i16,
		line_height: i16,
		glyph_color: ColorRGBA,
		background_color: ColorRGBA,
		fmt: &str,
		default_texture: Texture<'a, ColorGrayscale>,
		glyphs: &[Texture<'a, ColorGrayscale>; N],
		origins: EnumMap<K, Vec2<i16>>,
	) -> Self {
		let mut tx_map =
			EnumMap::<K, Texture<'a, ColorGrayscale>>::from_fn(|_| default_texture);

		let mut i = 0;
		for c in fmt.chars() {
			let v =
				K::from_char(c).expect("Font's fmt string cannot contain unmapped characters");

			tx_map[v] = glyphs[i];
			i += 1;
		}
		assert!(
			i == glyphs.len(),
			"Font's fmt string must have the same size as its texture array"
		);
		Self {
			atlas: TextureAtlas::from_map(tx_map),
			origins,
			horizontal_margin,
			line_height,
			glyph_color,
			background_color,
		}
	}
}

impl<K: Charset, TXF: AnyTextureMut> AnyFont<TXF> for ColorFont<'_, K>
where
	ColorRGBA: for<'b> Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
{
	fn draw_string(&self, tile: &mut TXF::LtAs<'_>, pos: Vec2<i16>, str: &str) {
		let oob = if self.line_height < 0 {
			|clip: Rect, y, h| y < clip.start.y + h
		} else {
			|clip: Rect, y, h| y >= clip.end.y + h
		};

		let origin = pos;
		let mut pos = pos;
		if oob(tile.get_clip(), pos.y, self.line_height) {
			return;
		}

		for c in str.chars() {
			match c {
				'\n' => {
					pos.x = origin.x;
					pos.y += self.line_height;
					if oob(tile.get_clip(), pos.y, self.line_height) {
						return;
					}
				}
				'\t' => {
					let key = K::from_char(' ').unwrap_or_default();
					let width = self.atlas.get(key).size().x * 4;
					pos.x = floorf(pos.x as f32 / width as f32 + 1.0) as i16 * width;
				}
				c => {
					let key = K::from_char(c).unwrap_or_default();
					let texture = self.atlas.get(key);
					let origin = self.origins[key];

					tile.draw(&DrawSprite {
						rect:    Rect::from_size(pos + origin, texture.get_clip().size().abs()),
						texture: &texture.with_sampler(|&texture, index, _| {
							let [sr, sg, sb, sa] = self.background_color.rgba();
							let [er, eg, eb, ea] = self.glyph_color.rgba();
							let t = texture.get_pixel_at(index).value as u16;

							ColorRGBA::new(
								(sr * (256 - t) + er * t) / 256,
								(sg * (256 - t) + eg * t) / 256,
								(sb * (256 - t) + eb * t) / 256,
								(sa * (256 - t) + ea * t) / 256,
							)
						}),
					});
					pos.x += texture.size().x + self.horizontal_margin;
				}
			}
		}
	}
}
