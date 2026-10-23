use crate::{
	Rect,
	color::Blend,
	rect::{AnyRect, OrdRect},
	textures::{AnyColor, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	pub fn draw_rectangle<U: Blend<T>, const ORD: bool>(
		&mut self,
		rect: AnyRect<i16, ORD>,
		val: U,
	) {
		let rect = rect.ordered() & self.clip;
		let (start, end) = rect.bounds();
		if start.x > end.x || start.y > end.y {
			return;
		}

		let width = self.stride as usize;
		let row = start.y as usize * width;
		let mut from = row + start.x as usize;
		let mut to = row + end.x as usize;

		for _ in start.y..end.y {
			for p in from..to {
				self.draw_pixel_at(p, val);
			}
			from += width;
			to += width;
		}
	}

	pub fn draw_rectangle_outline<U: Blend<T>, const ORD: bool>(
		&mut self,
		rect: AnyRect<i16, ORD>,
		thickness: i16,
		val: U,
	) {
		let (start, end) = rect.ordered().bounds();
		let pt = thickness.max(0);
		let nt = thickness.min(0);

		self.draw_rectangle(
			Rect::from_coords(start.x + nt, start.y + nt, end.x - nt, start.y + pt),
			val,
		);
		self.draw_rectangle(
			Rect::from_coords(start.x + nt, start.y + pt, start.x + pt, end.y - pt),
			val,
		);
		self.draw_rectangle(
			Rect::from_coords(end.x - pt, start.y + pt, end.x - nt, end.y - pt),
			val,
		);
		self.draw_rectangle(
			Rect::from_coords(start.x + nt, end.y - pt, end.x - nt, end.y - nt),
			val,
		);
	}

	pub fn draw_rounded_rectangle<U: Blend<T>, const ORD: bool>(
		&mut self,
		rect: AnyRect<i16, ORD>,
		radius: f32,
		val: U,
	) {
		let rect = rect.ordered();
		let radius = radius.min((rect.size().min() / 2) as f32);
		let (start, end) = rect.bounds();

		let mut t1 = radius / 16.0;
		let mut xf = radius;
		let mut yf = 0.0;

		let r = radius as i16;
		while xf >= yf {
			let x = xf as i16;
			let y = yf as i16;

			self.draw_horizontal_line(start.x - x + r, end.x + x - r, start.y - y + r, val);
			self.draw_horizontal_line(start.x - y + r, end.x + y - r, start.y - x + r, val);
			self.draw_horizontal_line(start.x - x + r, end.x + x - r, end.y + y - r - 1, val);
			self.draw_horizontal_line(start.x - y + r, end.x + y - r, end.y + x - r - 1, val);
			yf += 1.0;
			t1 += yf;
			let t2 = t1 - xf;
			if t2 >= 0.0 {
				t1 = t2;
				xf -= 1.0;
			}
		}
		self.draw_rectangle(
			OrdRect::from_coords(start.x, start.y + r, end.x, end.y - r),
			val,
		);
	}
}
