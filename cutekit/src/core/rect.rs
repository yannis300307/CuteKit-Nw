use core::{
	mem::swap,
	ops::{Add, BitAnd, BitAndAssign, Sub},
};

use crate::{Vec2, math::Integer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct Rect<T: Integer = i16>(pub Vec2<T>, pub Vec2<T>);

impl<T: Integer> Rect<T> {
	pub const fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
		Self(Vec2::new(x1, y1), Vec2::new(x2, y2))
	}

	pub fn from_size(pos: Vec2<T>, size: Vec2<T>) -> Self { Self(pos, pos + size).reorder() }

	pub fn size(&self) -> Vec2<T> { self.1 - self.0 }

	pub const fn bounds(&self) -> (Vec2<T>, Vec2<T>) { (self.0, self.1) }

	pub fn intersect(self, bounds: Self) -> Self {
		Self::new(
			self.0.x.max(bounds.0.x),
			self.0.y.max(bounds.0.y),
			self.1.x.min(bounds.1.x),
			self.1.y.min(bounds.1.y),
		)
	}

	pub fn reorder(self) -> Self {
		let mut cpy = self;
		if self.0.x > self.1.x {
			swap(&mut cpy.0.x, &mut cpy.1.x);
		}
		if self.0.y > self.1.y {
			swap(&mut cpy.0.y, &mut cpy.1.y);
		}
		cpy
	}
}

impl<T: Integer> Add<Vec2<T>> for Rect<T> {
	type Output = Self;

	fn add(self, rhs: Vec2<T>) -> Self { Self(self.0 + rhs, self.1 + rhs) }
}

impl<T: Integer> Sub<Vec2<T>> for Rect<T> {
	type Output = Self;

	fn sub(self, rhs: Vec2<T>) -> Self { Self(self.0 - rhs, self.1 - rhs) }
}

impl<T: Integer> BitAnd for Rect<T> {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self { self.intersect(rhs) }
}

impl<T: Integer> BitAndAssign for Rect<T> {
	fn bitand_assign(&mut self, rhs: Self) { *self = self.intersect(rhs); }
}
