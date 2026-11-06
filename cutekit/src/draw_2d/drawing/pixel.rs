use core::ops::{AddAssign, SubAssign};

use crate::{
	Vec2,
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
	pub pos:   Vec2<i16>,
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

impl<T: AnyColor> AddAssign<Vec2<i16>> for DrawPixel<T> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.pos += rhs; }
}

impl<T: AnyColor> SubAssign<Vec2<i16>> for DrawPixel<T> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.pos -= rhs; }
}

impl_add_sub!(DrawPixel<T: AnyColor> for Vec2<i16>);

impl<T: AnyColor> DrawRenderable for DrawPixel<T> {}
