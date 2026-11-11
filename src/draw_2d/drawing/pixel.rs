use core::ops::{AddAssign, SubAssign};

use glam::I16Vec2;

use crate::{
	color::{AnyColor, Blend},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::drawing::impl_add_sub,
	textures::AnyTextureMut,
};

/// Draw operation to render a pixel.
///
/// The draw operation will be bound-checked.
///
/// This draw operation is highly inefficient,
/// as such, it is not recommended to use it.
/// Prefer using the texture's `draw_pixel_at` method,
/// or making your own draw operation to make batching more efficient.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawPixel<T: AnyColor> {
	pub pos:   I16Vec2,
	pub color: T,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawPixel<T>> for DST {
	#[inline]
	fn draw(&mut self, pixel: &DrawPixel<T>) {
		if !self.get_clip().contains(pixel.pos) {
			return;
		}
		let index = self.index_of(pixel.pos);
		self.draw_pixel_at(index, pixel.color);
	}
}

impl<T: AnyColor> AddAssign<I16Vec2> for DrawPixel<T> {
	fn add_assign(&mut self, rhs: I16Vec2) { self.pos += rhs; }
}

impl<T: AnyColor> SubAssign<I16Vec2> for DrawPixel<T> {
	fn sub_assign(&mut self, rhs: I16Vec2) { self.pos -= rhs; }
}

impl_add_sub!(DrawPixel<T: AnyColor> for I16Vec2);

impl<T: AnyColor> DrawRenderable for DrawPixel<T> {}
