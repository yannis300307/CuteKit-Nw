use core::ops::{AddAssign, SubAssign};

use crate::{
	CutekitMatrix,
	Rect,
	color::Blend,
	drawing::{Draw, DrawRenderable, Sampler},
	mods::draw_2d::drawing::impl_add_sub,
	textures::{AnyTexture, AnyTextureMut},
};

// UNUSED, UV mapping is a pain to get right, kinda bloat,
// and I don't think the performance benefit is that high

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpriteTiled<'a, TX: AnyTexture + Sampler> {
	pub tile:    Rect,
	pub rect:    Rect,
	pub texture: &'a TX,
}

impl<DST: AnyTextureMut, TX: AnyTexture + Sampler> Draw<SpriteTiled<'_, TX>> for DST
where
	<TX as Sampler>::SampleColor: Blend<DST::Pixel>,
{
	fn draw(&mut self, sprite: &SpriteTiled<TX>) {
		let clip = self.get_clip();
		let rect = sprite.rect.ordered();

		let start = rect.start.max_with(clip.start);
		let end = rect.end.min_with(clip.end);

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let tile = sprite.tile.ordered();
		let tx_uclip = sprite.texture.get_clip().ordered_on(sprite.tile);
		let tx_clip = tx_uclip.ordered_on(sprite.rect);

		let tile_size = tile.size().cast::<f32>();
		let tx_size = tx_clip.size().cast::<f32>();

		let stride = self.stride() as usize;
		let tx_stride = sprite.texture.stride() as usize;

		let step = tx_size.component_div(&tile_size);

		let wrap_p = |x, start, end| if x >= end { x - (end - start) } else { x };
		let wrap_n = |x, start, end| if x < end { x - (end - start) } else { x };

		let wrap_u = if step.x > 0.0 { wrap_p } else { wrap_n };
		let wrap_v = if step.y > 0.0 { wrap_p } else { wrap_n };
		let (wrap_u_start, wrap_u_end) = (tx_clip.start.x as f32, tx_clip.end.x as f32);
		let (wrap_v_start, wrap_v_end) = (tx_clip.start.y as f32, tx_clip.end.y as f32);

		let (start_y, end_y) = (start.y as usize, end.y as usize);
		let (start_x, end_x) = (start.x as usize, end.x as usize);

		let tx_start = Vec2::from_fn(|i, _| {
			let tx_init =
				(tx_uclip.start[i] + tile.start[i].rem_euclid(tx_size[i] as i16)) as f32;
			let tx_start = tx_uclip.start[i] as f32;
			let clipped = (start[i] - sprite.rect.start[i]) as f32;
			let offset = f32::from(tx_clip.start[i] > tx_clip.end[i]);

			let r = ((tx_init + clipped) * step[i] - offset - tx_start) % tx_size[i];
			(if r == 0.0 { tx_size[i] } else { r }) + tx_start
		});

		let mut uv = tx_start;
		for row in (start_y * stride..end_y * stride).step_by(stride) {
			let tx_row = uv.y as usize * tx_stride;
			uv.x = tx_start.x;

			for index in row + start_x..row + end_x {
				let tx_index = tx_row + uv.x as usize;

				self.draw_pixel_at(index, sprite.texture.sample_pixel(tx_index, uv));
				uv.x = wrap_u(uv.x + step.x, wrap_u_start, wrap_u_end);
			}
			uv.y = wrap_v(uv.y + step.y, wrap_v_start, wrap_v_end);
		}
	}
}

impl<TX: AnyTexture + Sampler> AddAssign<I16Vec2> for SpriteTiled<'_, TX> {
	fn add_assign(&mut self, rhs: I16Vec2) { self.rect += rhs; }
}

impl<TX: AnyTexture + Sampler> SubAssign<I16Vec2> for SpriteTiled<'_, TX> {
	fn sub_assign(&mut self, rhs: I16Vec2) { self.rect -= rhs; }
}

impl_add_sub!(SpriteTiled<'a, TX: AnyTexture + Sampler> for I16Vec2);

impl<TX: AnyTexture + Sampler + Copy> DrawRenderable for SpriteTiled<'_, TX> {}
