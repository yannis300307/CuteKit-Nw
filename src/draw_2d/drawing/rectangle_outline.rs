use core::ops::{AddAssign, SubAssign};

use crate::{
	Rect,
	Vec2,
	color::{AnyColor, Blend},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::drawing::{impl_add_sub, rectangle::DrawRectangle},
	textures::AnyTextureMut,
};

/// Draw operation to render a flat colored rectangle outline.
///
/// The outline will be drawn inwards, unless `thickness` is negative,
/// in which case the outline will be drawn outwards.
///
/// The `rect` field can be unordered.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawRectangleOutline<T: AnyColor> {
	pub rect:      Rect,
	pub thickness: i16,
	pub color:     T,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawRectangleOutline<T>> for DST {
	fn draw(&mut self, rectangle: &DrawRectangleOutline<T>) {
		let [start, end] = rectangle.rect.ordered().bounds();
		let pt = rectangle.thickness.max(0);
		let nt = rectangle.thickness.min(0);

		self.draw(&DrawRectangle {
			rect:  Rect::from_scalars(start.x + nt, start.y + nt, end.x - nt, start.y + pt),
			color: rectangle.color,
		});
		self.draw(&DrawRectangle {
			rect:  Rect::from_scalars(start.x + nt, start.y + pt, start.x + pt, end.y - pt),
			color: rectangle.color,
		});
		self.draw(&DrawRectangle {
			rect:  Rect::from_scalars(end.x - pt, start.y + pt, end.x - nt, end.y - pt),
			color: rectangle.color,
		});
		self.draw(&DrawRectangle {
			rect:  Rect::from_scalars(start.x + nt, end.y - pt, end.x - nt, end.y - nt),
			color: rectangle.color,
		});
	}
}

impl<T: AnyColor> AddAssign<Vec2<i16>> for DrawRectangleOutline<T> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.rect += rhs; }
}

impl<T: AnyColor> SubAssign<Vec2<i16>> for DrawRectangleOutline<T> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.rect -= rhs; }
}

impl_add_sub!(DrawRectangleOutline<T: AnyColor> for Vec2<i16>);

impl<T: AnyColor> DrawRenderable for DrawRectangleOutline<T> {}
