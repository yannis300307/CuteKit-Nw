use core::ops::{AddAssign, SubAssign};

use crate::{
	Rect,
	Vec2,
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

		let start = rect.start.zip_map(&clip.start, |a, b| a.max(b));
		let end = rect.end.zip_map(&clip.end, |a, b| a.min(b));

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let tx_clip = sprite.texture.get_clip().ordered_on(sprite.rect);
		let size = rect.size().cast::<f32>();
		let tx_size = tx_clip.size().cast::<f32>();

		let step = tx_size.component_div(&size);

		let tx_start = Vec2::new(0, 1).map(|i: usize| {
			let tx_start = tx_clip.start[i] as f32;
			let clipped = (start[i] - rect.start[i]) as f32;
			let offset = f32::from(tx_clip.start[i] > tx_clip.end[i]);

			tx_start + clipped * step[i] - offset
		});

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

impl<TX: AnyTexture + Sampler> AddAssign<Vec2<i16>> for DrawSprite<'_, TX> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.rect += rhs; }
}

impl<TX: AnyTexture + Sampler> SubAssign<Vec2<i16>> for DrawSprite<'_, TX> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.rect -= rhs; }
}

impl_add_sub!(DrawSprite<'a, TX: AnyTexture + Sampler> for Vec2<i16>);

impl<TX: AnyTexture + Sampler + Copy> DrawRenderable for DrawSprite<'_, TX> {}
