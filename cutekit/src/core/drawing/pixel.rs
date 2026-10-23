use crate::{
	Vec2,
	color::Blend,
	textures::{AnyColor, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	#[inline]
	pub fn draw_pixel<U: Blend<T>>(&mut self, pos: Vec2<i16>, val: U) {
		if !self.clip.contains(pos) {
			return;
		}
		let index = (pos.y * self.stride + pos.x) as usize;
		self.data[index] = U::blend(self.data[index], val);
	}

	#[inline]
	pub fn draw_pixel_at<U: Blend<T>>(&mut self, index: usize, val: U) {
		self.data[index] = U::blend(self.data[index], val);
	}
}
