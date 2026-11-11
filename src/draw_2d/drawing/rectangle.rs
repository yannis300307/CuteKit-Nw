use core::ops::{AddAssign, SubAssign};

use glam::I16Vec2;

use crate::{
	Rect,
	color::{AnyColor, Blend},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::drawing::impl_add_sub,
	textures::AnyTextureMut,
};

/// Draw operation to render a flat colored rectangle.
///
/// The `rect` field can be unordered.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawRectangle<T: AnyColor> {
	pub rect:  Rect,
	pub color: T,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawRectangle<T>> for DST {
	fn draw(&mut self, rectangle: &DrawRectangle<T>) {
		let [start, end] = (rectangle.rect.ordered() & self.get_clip()).bounds();
		if start.x > end.x || start.y > end.y {
			return;
		}

		let width = self.stride() as usize;
		let row = start.y as usize * width;
		let mut from = row + start.x as usize;
		let mut to = row + end.x as usize;

		for _ in start.y..end.y {
			for p in from..to {
				self.draw_pixel_at(p, rectangle.color);
			}
			from += width;
			to += width;
		}
	}
}

impl<T: AnyColor> AddAssign<I16Vec2> for DrawRectangle<T> {
	fn add_assign(&mut self, rhs: I16Vec2) { self.rect += rhs; }
}

impl<T: AnyColor> SubAssign<I16Vec2> for DrawRectangle<T> {
	fn sub_assign(&mut self, rhs: I16Vec2) { self.rect -= rhs; }
}

impl_add_sub!(DrawRectangle<T: AnyColor> for I16Vec2);

impl<T: AnyColor> DrawRenderable for DrawRectangle<T> {}
