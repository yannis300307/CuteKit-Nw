use bytemuck::AnyBitPattern;

use crate::texture::MutableTexture;

impl<const SIZE: usize, T: AnyBitPattern> MutableTexture<SIZE, T> {
	pub fn draw_horizontal_line<const BC: bool>(
		&mut self,
		mut start_x: i16,
		mut end_x: i16,
		y: i16,
		val: T,
	) {
		if BC {
			if y < self.clip.0.y || y >= self.clip.1.y {
				return;
			}
			start_x = start_x.max(self.clip.0.x);
			end_x = end_x.min(self.clip.1.x);
			if start_x > end_x {
				return;
			}
		}

		let row = (y * self.size.x) as usize;
		let from = row + start_x as usize;
		let to = row + end_x as usize;
		self.data[from..to].fill(val);
	}
}
