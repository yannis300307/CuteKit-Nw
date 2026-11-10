use core::ops::{AddAssign, SubAssign};

use crate::{
	Vec2,
	color::{AnyColor, Blend},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::drawing::{impl_add_sub, line::DrawHorizontalLine},
	textures::AnyTextureMut,
};

/// Draw operation to render a flat colored circle.
///
/// The circle will not render with a negative radius.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawCircle<T: AnyColor> {
	pub center: Vec2<i16>,
	pub radius: i16,
	pub color:  T,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawCircle<T>> for DST {
	fn draw(&mut self, circle: &DrawCircle<T>) {
		let mut t1 = circle.radius as f32 / 16.0;
		let mut x = circle.radius;
		let mut y = 0;

		while x >= y {
			self.draw(&DrawHorizontalLine {
				start_x: circle.center.x - x,
				end_x:   circle.center.x + x,
				y:       circle.center.y + y,
				color:   circle.color,
			});
			self.draw(&DrawHorizontalLine {
				start_x: circle.center.x - x,
				end_x:   circle.center.x + x,
				y:       circle.center.y - y,
				color:   circle.color,
			});
			self.draw(&DrawHorizontalLine {
				start_x: circle.center.x - y,
				end_x:   circle.center.x + y,
				y:       circle.center.y + x,
				color:   circle.color,
			});
			self.draw(&DrawHorizontalLine {
				start_x: circle.center.x - y,
				end_x:   circle.center.x + y,
				y:       circle.center.y - x,
				color:   circle.color,
			});
			y += 1;
			t1 += y as f32;
			let t2 = t1 - x as f32;
			if t2 >= 0.0 {
				t1 = t2;
				x -= 1;
			}
		}
	}
}

impl<T: AnyColor> AddAssign<Vec2<i16>> for DrawCircle<T> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.center += rhs; }
}

impl<T: AnyColor> SubAssign<Vec2<i16>> for DrawCircle<T> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.center -= rhs; }
}

impl_add_sub!(DrawCircle<T: AnyColor> for Vec2<i16>);

impl<T: AnyColor> DrawRenderable for DrawCircle<T> {}
