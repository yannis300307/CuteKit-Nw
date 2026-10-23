use crate::{
	Vec2f,
	color::Blend,
	textures::{AnyColor, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	pub fn draw_circle<U: Blend<T>>(&mut self, center: Vec2f, r: f32, val: U) {
		let mut t1 = r / 16.0;
		let mut x = r;
		let mut y = 0.0;

		while x >= y {
			self.draw_horizontal_line(
				(center.x - x) as i16,
				(center.x + x) as i16,
				(center.y + y) as i16,
				val,
			);
			self.draw_horizontal_line(
				(center.x - x) as i16,
				(center.x + x) as i16,
				(center.y - y) as i16,
				val,
			);
			self.draw_horizontal_line(
				(center.x - y) as i16,
				(center.x + y) as i16,
				(center.y + x) as i16,
				val,
			);
			self.draw_horizontal_line(
				(center.x - y) as i16,
				(center.x + y) as i16,
				(center.y - x) as i16,
				val,
			);
			y += 1.0;
			t1 += y;
			let t2 = t1 - x;
			if t2 >= 0.0 {
				t1 = t2;
				x -= 1.0;
			}
		}
	}
}
