use core::{
	fmt::Debug,
	mem::swap,
	ops::{Add, BitAnd, BitAndAssign, Sub},
};

use crate::{Vec2, utils::math::Integer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct AnyRect<T: Integer, const ORD: bool>(pub Vec2<T>, pub Vec2<T>);

pub type Rect<T> = AnyRect<T, false>;
pub type OrdRect<T> = AnyRect<T, true>;

impl<const ORD: bool, T: Integer> AnyRect<T, ORD> {
	#[inline]
	pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
		if ORD {
			assert!(start.x <= end.x && start.y <= end.y);
		}
		Self(start, end)
	}

	#[inline]
	pub fn from_coords(x1: T, y1: T, x2: T, y2: T) -> Self {
		Self::new(Vec2::new(x1, y1), Vec2::new(x2, y2))
	}

	#[inline]
	pub fn from_size(pos: Vec2<T>, size: Vec2<T>) -> Self { Self::new(pos, pos + size) }

	#[inline]
	pub fn at_origin(size: Vec2<T>) -> Self
	where
		i8: Into<T>,
	{
		Self::new(Vec2::new(0.into(), 0.into()), size)
	}

	#[inline]
	pub fn ordered(mut self) -> AnyRect<T, true> {
		if !ORD {
			if self.0.x > self.1.x {
				swap(&mut self.0.x, &mut self.1.x);
			}
			if self.0.y > self.1.y {
				swap(&mut self.0.y, &mut self.1.y);
			}
		}
		AnyRect(self.0, self.1)
	}

	#[inline]
	pub fn unordered(self) -> AnyRect<T, false> { AnyRect(self.0, self.1) }

	#[inline]
	pub fn size(&self) -> Vec2<T> { self.1 - self.0 }

	#[inline]
	pub fn lsize(&self) -> Vec2<isize>
	where
		isize: From<T>,
	{
		(self.1 - self.0).map(|x| isize::from(x))
	}

	#[inline]
	pub fn bounds(&self) -> (Vec2<T>, Vec2<T>) { (self.0, self.1) }

	#[inline]
	pub fn contains(&self, p: Vec2<T>) -> bool {
		p.x >= self.0.x && p.x < self.1.x && p.y >= self.0.y && p.y < self.1.y
	}

	#[inline]
	pub fn encapsulates<const ORD2: bool>(&self, rect: AnyRect<T, ORD2>) -> bool {
		let a = rect.ordered();
		let b = self.ordered();
		a.0.x >= b.0.x && a.1.x <= b.1.x && a.0.y >= b.0.y && a.1.y <= b.1.y
	}

	pub fn intersection<const ORD2: bool>(self, bounds: AnyRect<T, ORD2>) -> Self {
		let bounds = bounds.ordered();
		if ORD {
			Self(
				Vec2::new(self.0.x.max(bounds.0.x), self.0.y.max(bounds.0.y)),
				Vec2::new(self.1.x.min(bounds.1.x), self.1.y.min(bounds.1.y)),
			)
		} else {
			Self(
				Vec2::new(
					self.0.x.clamp(bounds.0.x, bounds.1.x),
					self.0.y.clamp(bounds.0.y, bounds.1.y),
				),
				Vec2::new(
					self.1.x.clamp(bounds.0.x, bounds.1.x),
					self.1.y.clamp(bounds.0.y, bounds.1.y),
				),
			)
		}
	}
}

impl<const ORD: bool, T: Integer> Add<Vec2<T>> for AnyRect<T, ORD> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Vec2<T>) -> Self { Self(self.0 + rhs, self.1 + rhs) }
}

impl<const ORD: bool, T: Integer> Sub<Vec2<T>> for AnyRect<T, ORD> {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Vec2<T>) -> Self { Self(self.0 - rhs, self.1 - rhs) }
}

impl<const ORD: bool, const ORD2: bool, T: Integer> BitAnd<AnyRect<T, ORD2>>
	for AnyRect<T, ORD>
{
	type Output = Self;

	#[inline]
	fn bitand(self, rhs: AnyRect<T, ORD2>) -> Self { self.intersection(rhs) }
}

impl<const ORD: bool, const ORD2: bool, T: Integer> BitAndAssign<AnyRect<T, ORD2>>
	for AnyRect<T, ORD>
{
	#[inline]
	fn bitand_assign(&mut self, rhs: AnyRect<T, ORD2>) { *self = self.intersection(rhs); }
}

impl<const ORD: bool, T: Integer> From<(Vec2<T>, Vec2<T>)> for AnyRect<T, ORD> {
	#[inline]
	fn from((start, end): (Vec2<T>, Vec2<T>)) -> Self { Self::new(start, end) }
}

impl<const ORD: bool, T: Integer> From<(T, T, T, T)> for AnyRect<T, ORD> {
	#[inline]
	fn from((x1, y1, x2, y2): (T, T, T, T)) -> Self { Self::from_coords(x1, y1, x2, y2) }
}

impl<T: Integer> From<OrdRect<T>> for Rect<T> {
	#[inline]
	fn from(value: OrdRect<T>) -> Self { value.unordered() }
}

impl<T: Integer> From<Rect<T>> for OrdRect<T> {
	#[inline]
	fn from(value: Rect<T>) -> Self { value.ordered() }
}

impl<const ORD: bool, T: Integer> Debug for AnyRect<T, ORD> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"Rect<{}>([{}, {}] -> [{}, {}])",
			ORD, self.0.x, self.0.y, self.1.x, self.1.y
		)
	}
}
