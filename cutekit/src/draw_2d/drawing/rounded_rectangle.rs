use core::ops::{AddAssign, SubAssign};

use crate::{
	Rect,
	Vec2,
	color::{AnyColor, Blend},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::drawing::{
		impl_add_sub,
		line::DrawHorizontalLine,
		rectangle::DrawRectangle,
	},
	textures::AnyTextureMut,
};

/// Draw operation to render a flat colored rectangle with rounded corners.
///
/// The `rect` field can be unordered.
///
/// The radius will be adjusted to fit within the rect's size.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawRoundedRectangle<T: AnyColor> {
	rect:   Rect,
	radius: i16,
	color:  T,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawRoundedRectangle<T>> for DST {
	fn draw(&mut self, rectangle: &DrawRoundedRectangle<T>) {
		let rect = rectangle.rect.ordered();
		let radius = rectangle.radius.min(rect.size().min() / 2);
		let [start, end] = rect.bounds();

		let mut t1 = radius as f32 / 16.0;
		let mut x = radius;
		let mut y = 0;

		let r = radius as i16;
		while x >= y {
			self.draw(&DrawHorizontalLine {
				start_x: start.x - x + r,
				end_x:   end.x + x - r,
				y:       start.y - y + r,
				color:   rectangle.color,
			});
			self.draw(&DrawHorizontalLine {
				start_x: start.x - y + r,
				end_x:   end.x + y - r,
				y:       start.y - x + r,
				color:   rectangle.color,
			});
			self.draw(&DrawHorizontalLine {
				start_x: start.x - x + r,
				end_x:   end.x + x - r,
				y:       end.y + y - r - 1,
				color:   rectangle.color,
			});
			self.draw(&DrawHorizontalLine {
				start_x: start.x - y + r,
				end_x:   end.x + y - r,
				y:       end.y + x - r - 1,
				color:   rectangle.color,
			});
			y += 1;
			t1 += y as f32;
			let t2 = t1 - x as f32;
			if t2 >= 0.0 {
				t1 = t2;
				x -= 1;
			}
		}
		self.draw(&DrawRectangle {
			rect:  Rect::from_scalars(start.x, start.y + r, end.x, end.y - r),
			color: rectangle.color,
		});
	}
}

impl<T: AnyColor> AddAssign<Vec2<i16>> for DrawRoundedRectangle<T> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.rect += rhs; }
}

impl<T: AnyColor> SubAssign<Vec2<i16>> for DrawRoundedRectangle<T> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.rect -= rhs; }
}

impl_add_sub!(DrawRoundedRectangle<T: AnyColor> for Vec2<i16>);

impl<T: AnyColor> DrawRenderable for DrawRoundedRectangle<T> {}
