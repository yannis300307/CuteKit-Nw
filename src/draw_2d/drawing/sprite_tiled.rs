use core::ops::{AddAssign, SubAssign};

use crate::{
	Rect,
	Vec2,
	color::Blend,
	draw_2d::DrawSprite,
	drawing::{Draw, DrawRenderable, Sampler},
	mods::draw_2d::drawing::impl_add_sub,
	textures::{AnyTexture, AnyTextureMut},
};

/// Draw operation to render a texture, tiled over a given area.
///
/// The `rect` field can be unordered,
/// the texture will be repeated across to match its area.\
/// The `tile` field represents the area taken by a single texture,
/// negative values will flip the texture on that axis.
///
/// Only the texture's clip area will be drawn,
/// it can be unordered, in which case the texture will be flipped on that axis.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawSpriteTiled<'a, TX: AnyTexture + Sampler> {
	pub tile:    Rect,
	pub rect:    Rect,
	pub texture: &'a TX,
}

impl<DST: AnyTextureMut, TX: AnyTexture + Sampler> Draw<DrawSpriteTiled<'_, TX>> for DST
where
	<TX as Sampler>::SampleColor: Blend<DST::Pixel>,
{
	fn draw(&mut self, sprite: &DrawSpriteTiled<TX>) {
		let clip = self.get_clip();
		let rect = sprite.rect.ordered();

		let start = rect.start.zip_map(&clip.start, |a, b| a.max(b));
		let end = rect.end.zip_map(&clip.end, |a, b| a.min(b));

		let tile_size = sprite.tile.size();

		if start.x >= end.x || start.y >= end.y || tile_size.x == 0 || tile_size.y == 0 {
			return;
		}

		let abs_ts = tile_size.abs();
		let ord_ts = tile_size.zip_map(&sprite.rect.size(), |a, b| a * b.signum());

		let offset = (sprite.rect.start - start + sprite.tile.start)
			.zip_map(&abs_ts, |a, b| a.rem_euclid(b));

		let tstart = start + offset - ord_ts.map(|x| x.max(0));
		let tend = end + offset - ord_ts.map(|x| x.min(0));

		self.clip(rect);
		for y in (tstart.y..tend.y).step_by(abs_ts.y as usize) {
			for x in (tstart.x..tend.x).step_by(abs_ts.x as usize) {
				self.draw(&DrawSprite {
					rect:    Rect::from_size(Vec2::new(x, y), ord_ts),
					texture: sprite.texture,
				});
			}
		}
		self.clip(clip);
	}
}

impl<TX: AnyTexture + Sampler> AddAssign<Vec2<i16>> for DrawSpriteTiled<'_, TX> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.rect += rhs; }
}

impl<TX: AnyTexture + Sampler> SubAssign<Vec2<i16>> for DrawSpriteTiled<'_, TX> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.rect -= rhs; }
}

impl_add_sub!(DrawSpriteTiled<'a, TX: AnyTexture + Sampler> for Vec2<i16>);

impl<TX: AnyTexture + Sampler + Copy> DrawRenderable for DrawSpriteTiled<'_, TX> {}
