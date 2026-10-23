use core::fmt::Debug;

use enum_map::{Enum, EnumMap};
use libm::floorf;

use crate::{
	Color,
	ColorAlpha,
	ColorGrayscale,
	OrdRect,
	Texture,
	TextureAtlas,
	TextureMut,
	Vec2,
	color::Blend,
	textures::AnyColor,
	utils::lerp::Lerp,
};

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

#[derive(Debug)]
pub struct ColorFont<'a, K: Charset> {
	atlas:             TextureAtlas<'a, K, ColorGrayscale>,
	origins:           EnumMap<K, Vec2<i16>>,
	horizontal_margin: i16,
	line_height:       i16,
	glyph_color:       ColorAlpha,
	background_color:  ColorAlpha,
}

pub trait AnyFont<T: AnyColor = Color>: Debug {
	fn draw_string(&self, tile: &mut TextureMut<T>, pos: Vec2<i16>, str: &str);
}

impl<'a, K: Charset> ColorFont<'a, K> {
	pub fn from_fmt<const N: usize>(
		horizontal_margin: i16,
		line_height: i16,
		glyph_color: ColorAlpha,
		background_color: ColorAlpha,
		fmt: &str,
		default: (Texture<'a, ColorGrayscale>, i16, i16),
		glyphs: [(Texture<'a, ColorGrayscale>, i16, i16); N],
	) -> Self {
		let mut tx_map = EnumMap::<K, Texture<'a, ColorGrayscale>>::from_fn(|_| default.0);
		let mut origins = EnumMap::<K, Vec2<i16>>::from_fn(|_| Vec2::new(default.1, default.2));

		let mut i = 0;
		for c in fmt.chars() {
			let v =
				K::from_char(c).expect("Font's fmt string cannot contain unmapped characters");

			(tx_map[v], origins[v].x, origins[v].y) = glyphs[i];
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

	pub fn from_fmt_monospace<const N: usize>(
		horizontal_margin: i16,
		line_height: i16,
		glyph_color: ColorAlpha,
		background_color: ColorAlpha,
		fmt: &str,
		default: (Texture<'a, ColorGrayscale>, i16, i16),
		glyphs: [Texture<'a, ColorGrayscale>; N],
	) -> Self {
		let mut tx_map = EnumMap::<K, Texture<'a, ColorGrayscale>>::from_fn(|_| default.0);
		let origins = EnumMap::<K, Vec2<i16>>::from_fn(|_| Vec2::new(default.1, default.2));

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

impl<'a, K: Charset, T: AnyColor> AnyFont<T> for ColorFont<'a, K>
where
	ColorAlpha: Blend<T>,
{
	fn draw_string(&self, tile: &mut TextureMut<T>, pos: Vec2<i16>, str: &str) {
		let oob = if self.line_height < 0 {
			|tile: &TextureMut<T>, y, h| y < tile.clip.0.y + h
		} else {
			|tile: &TextureMut<T>, y, h| y >= tile.clip.1.y + h
		};

		let origin = pos;
		let mut pos = pos;
		if oob(tile, pos.y, self.line_height) {
			return;
		}

		for c in str.chars() {
			match c {
				'\n' => {
					pos.x = origin.x;
					pos.y += self.line_height;
					if oob(tile, pos.y, self.line_height) {
						return;
					}
				}
				'\t' => {
					let key = K::from_char(' ').unwrap_or_default();
					let width = self.atlas.get(key).size.x * 4;
					pos.x = floorf(pos.x as f32 / width as f32 + 1.0) as i16 * width;
				}
				c => {
					let key = K::from_char(c).unwrap_or_default();
					let texture = self.atlas.get(key);
					let origin = self.origins[key];

					tile.draw_sprite_with_sampler(
						OrdRect::from_size(pos + origin, texture.clip.size().abs()),
						texture,
						|col, _| {
							ColorAlpha::lerp8(
								self.background_color,
								self.glyph_color,
								col.value,
							)
						},
					);
					pos.x += texture.size.x + self.horizontal_margin;
				}
			}
		}
	}
}
