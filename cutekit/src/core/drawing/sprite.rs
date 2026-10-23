use crate::{
	CutekitMatrix,
	OrdRect,
	Vec2,
	color::Blend,
	drawing::TextureSampler,
	textures::{AnyColor, Texture, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	pub fn draw_sprite_with_sampler<IN: AnyColor, OUT: Blend<T>, F: TextureSampler<IN, OUT>>(
		&mut self,
		rect: OrdRect<i16>,
		texture: &Texture<IN>,
		sampler: F,
	) {
		let start = rect.0.max_with(self.clip.0);
		let end = rect.1.min_with(self.clip.1);

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let size = rect.size().cast::<f32>();
		let tx_size = texture.clip.size().cast::<f32>();

		let step = tx_size.component_div(&size);

		let tx_start = Vec2::new(0, 1).map(|i: usize| {
			let tx_start = texture.clip.0[i];
			let clipped = start[i] - rect.0[i];
			let offset = f32::from(texture.clip.0[i] > texture.clip.1[i]);

			(tx_start + clipped) as f32 * step[i] - offset
		});

		let stride = self.stride as usize;
		let tx_stride = texture.stride as usize;

		let (start_y, end_y) = (start.y as usize, end.y as usize);
		let (start_x, end_x) = (start.x as usize, end.x as usize);

		let mut v = tx_start.y;
		for row in (start_y * stride..end_y * stride).step_by(stride) {
			let tx_row = v as usize * tx_stride;
			let mut u = tx_start.x;

			for index in row + start_x..row + end_x {
				let tx_index = tx_row + u as usize;
				self.draw_pixel_at(index, sampler(texture.get_pixel_at(tx_index), (u, v)));
				u += step.x;
			}
			v += step.y;
		}
	}

	#[inline]
	pub fn draw_sprite<IN: Blend<T>>(&mut self, rect: OrdRect<i16>, texture: &Texture<IN>) {
		self.draw_sprite_with_sampler(rect, texture, |col, _| col);
	}

	pub fn draw_sprite_tiled_with_sampler<
		IN: AnyColor,
		OUT: Blend<T>,
		F: TextureSampler<IN, OUT>,
	>(
		&mut self,
		tile: OrdRect<i16>,
		rect: OrdRect<i16>,
		texture: &Texture<IN>,
		sampler: F,
	) {
		let start = rect.0.max_with(self.clip.0);
		let end = rect.1.min_with(self.clip.1);

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let tx_clip = texture.clip;
		let tile_size = tile.size().cast::<f32>();
		let tx_size = tx_clip.size().cast::<f32>();

		let stride = self.stride as usize;
		let tx_stride = texture.stride as usize;

		let step = tx_size.component_div(&tile_size);

		let wrap_p = |x, start, end| if x >= end { x - (end - start) } else { x };
		let wrap_n = |x, start, end| if x < end { x - (end - start) } else { x };

		let wrap_u = if step.x > 0.0 { wrap_p } else { wrap_n };
		let wrap_v = if step.y > 0.0 { wrap_p } else { wrap_n };
		let (wrap_u_start, wrap_u_end) = (tx_clip.0.x as f32, tx_clip.1.x as f32);
		let (wrap_v_start, wrap_v_end) = (tx_clip.0.y as f32, tx_clip.1.y as f32);

		let (start_y, end_y) = (start.y as usize, end.y as usize);
		let (start_x, end_x) = (start.x as usize, end.x as usize);

		let tx_start = Vec2::new(0, 1).map(|i: usize| {
			let tx_init = tx_clip.0[i] + tile.0[i].rem_euclid(tile_size[i] as i16);
			let tx_start = tx_clip.0[i] as f32;
			let clipped = start[i] - rect.0[i];
			let offset = f32::from(tx_clip.0[i] > tx_clip.1[i]);

			(((tx_init + clipped) as f32 * step[i] - offset) - tx_start) % tx_size[i] + tx_start
		});

		let mut v = tx_start.y;
		for row in (start_y * stride..end_y * stride).step_by(stride) {
			let tx_row = v as usize * tx_stride;
			let mut u = tx_start.x;

			for index in row + start_x..row + end_x {
				let tx_index = tx_row + u as usize;

				self.draw_pixel_at(index, sampler(texture.get_pixel_at(tx_index), (u, v)));
				u = wrap_u(u + step.x, wrap_u_start, wrap_u_end);
			}
			v = wrap_v(v + step.y, wrap_v_start, wrap_v_end);
		}
	}

	pub fn draw_sprite_tiled<IN: Blend<T>>(
		&mut self,
		tile: OrdRect<i16>,
		rect: OrdRect<i16>,
		texture: &Texture<IN>,
	) {
		self.draw_sprite_tiled_with_sampler(tile, rect, texture, |col, _| col);
	}
}
