use core::iter::StepBy;

use libm::floorf;

use crate::{
	CutekitMatrix,
	Vec2,
	color::Blendable,
	rect::AnyRect,
	textures::{AnyColor, Texture, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	pub fn draw_sprite<U: AnyColor>(&mut self, pos: Vec2<i16>, texture: &Texture<U>)
	where
		T: Blendable<U>,
	{
		let pos = pos.cast::<isize>();
		let size = texture.clip.lsize();

		let start = pos.max_with(self.clip.0.cast::<isize>());
		let end = (pos + size.abs()).min_with(self.clip.1.cast::<isize>());

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let incr = size.map(|x| x.signum());
		let uv = Vec2::new(0, 1).map(|i: usize| {
			if texture.clip.0[i] < texture.clip.1[i] {
				texture.clip.0[i] as isize + (start[i] - pos[i])
			} else {
				texture.clip.0[i] as isize - 1 - (start[i] - pos[i])
			}
		});

		let width = self.stride as usize;

		let mut v_row = uv.y * texture.stride as isize;
		let v_row_incr = texture.stride as isize * incr.y;

		for row in (start.y as usize * width..end.y as usize * width).step_by(width) {
			let mut up = v_row + uv.x;

			for p in row + start.x as usize..row + end.x as usize {
				self.draw_pixel_at(p, texture.get_pixel_at(up as usize));
				up += incr.x;
			}
			v_row += v_row_incr;
		}
	}

	pub fn draw_sprite_scaled<U: AnyColor, const ORD: bool>(
		&mut self,
		rect: AnyRect<i16, ORD>,
		texture: &Texture<U>,
	) where
		T: Blendable<U>,
	{
		let rect = AnyRect::<isize, ORD>(rect.0.cast::<isize>(), rect.1.cast::<isize>());

		let start = rect.0.max_with(self.clip.0.cast::<isize>());
		let end = rect.1.min_with(self.clip.1.cast::<isize>());

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let incr = texture
			.clip
			.size()
			.cast::<f32>()
			.component_div(&rect.size().cast::<f32>());
		let uv = Vec2::new(0, 1).map(|i: usize| {
			let x = texture.clip.0[i] as f32 + (start[i] - rect.0[i]) as f32 * incr[i];
			if texture.clip.0[i] < texture.clip.1[i] {
				x
			} else {
				x - 1.0
			}
		});

		let width = self.stride as usize;
		let tx_w = texture.stride as f32;

		let mut v = uv.y;

		for row in (start.y as usize * width..end.y as usize * width).step_by(width) {
			let mut up = floorf(v) * tx_w + uv.x;

			for p in row + start.x as usize..row + end.x as usize {
				self.draw_pixel_at(p, texture.get_pixel_at(up as usize));
				up += incr.x;
			}
			v += incr.y;
		}
	}

	pub fn draw_sprite_tiled<U: AnyColor, const ORD: bool>(
		&mut self,
		rect: AnyRect<i16, ORD>,
		texture: &Texture<U>,
	) where
		T: Blendable<U>,
	{
		let rect = AnyRect::<isize, ORD>(rect.0.cast::<isize>(), rect.1.cast::<isize>());

		let start = rect.0.max_with(self.clip.0.cast::<isize>());
		let end = rect.1.min_with(self.clip.1.cast::<isize>());

		if start.x >= end.x || start.y >= end.y {
			return;
		}

		let size = texture.clip.lsize();
		let incr = size.map(|x| x.signum());
		let uv = Vec2::new(0, 1).map(|i: usize| {
			if size[i] > 0 {
				texture.clip.0[i] as isize + (start[i] - rect.0[i]) % size[i]
			} else {
				texture.clip.0[i] as isize - 1 - (start[i] - rect.0[i]) % size[i]
			}
		});

		let lc = |a: &mut isize, b: isize, _: isize, s: isize| {
			if *a < b {
				*a -= s;
			}
		};
		let hc = |a: &mut isize, _: isize, c: isize, s: isize| {
			if *a >= c {
				*a -= s;
			}
		};

		let wrap_x = if size.x > 0 { hc } else { lc };
		let wrap_y = if size.x > 0 { hc } else { lc };

		let width = self.stride as usize;
		let tx_w = texture.stride as isize;

		let mut v_row = uv.y * tx_w;
		let v_row_incr = tx_w * incr.y;

		let v_start = texture.clip.0.y as isize * tx_w;
		let v_end = texture.clip.1.y as isize * tx_w;
		let v_size = size.y * tx_w;

		for row in (start.y as usize * width..end.y as usize * width).step_by(width) {
			wrap_y(&mut v_row, v_start, v_end, v_size);
			let mut up = v_row + uv.x;

			let u_start = v_row as isize + texture.clip.0.x as isize;
			let u_end = v_row as isize + texture.clip.1.x as isize;
			for p in row + start.x as usize..row + end.x as usize {
				wrap_x(&mut up, u_start, u_end, size.x);
				self.draw_pixel_at(p, texture.get_pixel_at(up as usize));
				up += incr.x;
			}
			v_row += v_row_incr;
		}
	}
}
