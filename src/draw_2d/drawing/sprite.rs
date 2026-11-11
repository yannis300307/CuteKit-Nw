use core::ops::{AddAssign, SubAssign};

use glam::{I16Vec2, Vec2};

use crate::{
	Rect,
	color::Blend,
	drawing::{Draw, DrawRenderable, Sampler},
	mods::draw_2d::drawing::impl_add_sub,
	textures::{AnyTexture, AnyTextureMut},
};

/// Draw operation to render a texture.
///
/// The `rect` field can be unordered, the texture will scale to match its area.
///
/// Only the texture's clip area will be drawn,
/// it can be unordered, in which case the texture will be flipped on that axis.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawSprite<'a, TX: AnyTexture + Sampler> {
	pub rect:    Rect,
	pub texture: &'a TX,
}

impl<DST: AnyTextureMut, TX: AnyTexture + Sampler> Draw<DrawSprite<'_, TX>> for DST
where
	<TX as Sampler>::SampleColor: Blend<DST::Pixel>,
{
	fn draw(&mut self, sprite: &DrawSprite<TX>) {
		let clip = self.get_clip();
		let rect = sprite.rect.ordered();

		let start = rect.start.max(clip.start);
		let end = rect.end.min(clip.end);

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let tx_clip = sprite.texture.get_clip().ordered_on(sprite.rect);
		let size = rect.size().as_vec2();
		let tx_size = tx_clip.size().as_vec2();

		let step = tx_size / size;

		let tx_start = {
			let tx_start = tx_clip.start.as_vec2();
			let clipped = (start - rect.start).as_vec2();
			let offset = Vec2::select(tx_clip.is_ordered(), Vec2::ZERO, Vec2::ONE);

			tx_start + clipped * step - offset
		};

		let stride = self.stride() as usize;
		let tx_stride = sprite.texture.stride() as usize;

		let (start_y, end_y) = (start.y as usize, end.y as usize);
		let (start_x, end_x) = (start.x as usize, end.x as usize);

		let mut uv = tx_start;
		for row in (start_y * stride..end_y * stride).step_by(stride) {
			let tx_row = uv.y as usize * tx_stride;
			uv.x = tx_start.x;

			for index in row + start_x..row + end_x {
				let tx_index = tx_row + uv.x as usize;
				self.draw_pixel_at(index, sprite.texture.sample_pixel(tx_index, uv));
				uv.x += step.x;
			}
			uv.y += step.y;
		}
	}
}

impl<TX: AnyTexture + Sampler> AddAssign<I16Vec2> for DrawSprite<'_, TX> {
	fn add_assign(&mut self, rhs: I16Vec2) { self.rect += rhs; }
}

impl<TX: AnyTexture + Sampler> SubAssign<I16Vec2> for DrawSprite<'_, TX> {
	fn sub_assign(&mut self, rhs: I16Vec2) { self.rect -= rhs; }
}

impl_add_sub!(DrawSprite<'a, TX: AnyTexture + Sampler> for I16Vec2);

impl<TX: AnyTexture + Sampler + Copy> DrawRenderable for DrawSprite<'_, TX> {}
