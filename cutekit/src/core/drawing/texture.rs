use bytemuck::AnyBitPattern;

use crate::{
	CutekitMatrix,
	Vec2,
	color::Blendable,
	texture::{MutableTexture, Texture},
};

impl<const SIZE: usize, T: AnyBitPattern> MutableTexture<SIZE, T> {
	pub fn draw_texture<U: AnyBitPattern>(&mut self, pos: Vec2<i16>, texture: &Texture<U>)
	where
		T: Blendable<U>,
	{
		let size = texture.clip.size();
		let incr = size.map(|x| x.signum());

		let start = pos.max_with(self.clip.0);
		let end = (pos + size.abs()).min_with(self.clip.1);
		let uv = Vec2::new(
			if self.clip.0.x < self.clip.1.x {
				texture.clip.0.x - pos.x.min(self.clip.0.x)
			} else {
				texture.clip.1.x + pos.x.min(self.clip.0.x)
			},
			if self.clip.0.y < self.clip.1.y {
				texture.clip.0.y - pos.y.min(self.clip.0.y)
			} else {
				texture.clip.1.y + pos.y.min(self.clip.0.y)
			},
		);

		let width = self.size.x as usize;

		let mut u_row = uv.y as isize * texture.size.x as isize;
		let u_row_incr = texture.size.x as isize * incr.y as isize;

		for row in (start.y as usize * width..end.y as usize * width).step_by(width) {
			let mut up = u_row + uv.x as isize;

			for p in row + start.x as usize..row + end.x as usize {
				self.data[p].blend(texture.data[up as usize]);
				up += incr.x as isize;
			}
			u_row += u_row_incr;
		}
	}
}
