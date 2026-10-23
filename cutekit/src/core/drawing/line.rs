use crate::{
	color::Blend,
	textures::{AnyColor, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	pub fn draw_horizontal_line<U: Blend<T>>(
		&mut self,
		start_x: i16,
		end_x: i16,
		y: i16,
		val: U,
	) {
		if y < self.clip.0.y || y >= self.clip.1.y {
			return;
		}
		let start_x = start_x.max(self.clip.0.x);
		let end_x = end_x.min(self.clip.1.x);
		if start_x > end_x {
			return;
		}

		let row = (y * self.stride) as usize;
		let from = row + start_x as usize;
		let to = row + end_x as usize;

		for p in from..to {
			self.draw_pixel_at(p, val);
		}
	}
}
