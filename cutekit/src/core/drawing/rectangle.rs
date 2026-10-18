use bytemuck::AnyBitPattern;

use crate::{Rect, texture::MutableTexture};

impl<const SIZE: usize, T: AnyBitPattern> MutableTexture<SIZE, T> {
	pub fn draw_rectangle<const BC: bool>(&mut self, mut rect: Rect, val: T) {
		if BC {
			rect &= self.clip;
			if rect.0.x > rect.1.x {
				return;
			}
		}
		let (start, end) = rect.bounds();

		let width = self.size.x as usize;
		let row = start.y as usize * width;
		let mut from = row + start.x as usize;
		let mut to = row + end.x as usize;

		for _ in start.y..end.y {
			self.data[from..to].fill(val);
			from += width;
			to += width;
		}
	}

	pub fn draw_rectangle_outline<const BC: bool>(&mut self, rect: Rect, thickness: i16, val: T) {
		let (start, end) = rect.bounds();
		let pt = thickness.max(0);
		let nt = thickness.min(0);

		self.draw_rectangle::<BC>(
			Rect::new(start.x + nt, start.y + nt, end.x - nt, start.y + pt),
			val,
		);
		self.draw_rectangle::<BC>(
			Rect::new(start.x + nt, start.y + pt, start.x + pt, end.y - pt),
			val,
		);
		self.draw_rectangle::<BC>(
			Rect::new(end.x - pt, start.y + pt, end.x - nt, end.y - pt),
			val,
		);
		self.draw_rectangle::<BC>(
			Rect::new(start.x + nt, end.y - pt, end.x - nt, end.y - nt),
			val,
		);
	}

	pub fn draw_rounded_rectangle<const BC: bool>(&mut self, rect: Rect, mut radius: f32, val: T) {
		if BC {
			radius = radius.min((rect.size().min() / 2) as f32);
		}
		let (start, end) = rect.bounds();

		let mut t1 = radius / 16.0;
		let mut xf = radius;
		let mut yf = 0.0;

		let r = radius as i16;
		while xf >= yf {
			let x = xf as i16;
			let y = yf as i16;

			self.draw_horizontal_line::<BC>(start.x - x + r, end.x + x - r, start.y - y + r, val);
			self.draw_horizontal_line::<BC>(start.x - y + r, end.x + y - r, start.y - x + r, val);
			self.draw_horizontal_line::<BC>(start.x - x + r, end.x + x - r, end.y + y - r - 1, val);
			self.draw_horizontal_line::<BC>(start.x - y + r, end.x + y - r, end.y + x - r - 1, val);
			yf += 1.0;
			t1 += yf;
			let t2 = t1 - xf;
			if t2 >= 0.0 {
				t1 = t2;
				xf -= 1.0;
			}
		}
		self.draw_rectangle::<BC>(Rect::new(start.x, start.y + r, end.x, end.y - r), val);
	}
}
