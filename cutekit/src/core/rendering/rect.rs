use core::{
	fmt::Debug,
	mem::swap,
	ops::{Add, AddAssign, BitAnd, BitAndAssign, Sub, SubAssign},
};

use crate::Vec2;

/// Representation of a rectangular area on a texture or on the screen.
///
/// start usually represents the top-left corner of the rectangle,
/// while end usually represents its bottom-right one.
///
/// start and end can swapped around, the resulting rect then becomes "unordered".
/// Unordered rects are valid in some contexts, and can lead to special behaviors.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Rect {
	pub start: Vec2<i16>,
	pub end:   Vec2<i16>,
}

impl Rect {
	/// Creates a new Rect from two points.
	///
	/// The points match an ordered rect's top-left and bottom-right corner.
	#[inline]
	pub const fn new(start: Vec2<i16>, end: Vec2<i16>) -> Self { Self { start, end } }

	/// Creates a new rect from two decomposed points.
	///
	/// The points match an ordered rect's top-left and bottom-right corner.
	/// This method behaves similarly to `Rect::new`.
	#[inline]
	pub const fn from_scalars(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
		Self {
			start: Vec2::new(x1, y1),
			end:   Vec2::new(x2, y2),
		}
	}

	/// Creates a new rect from the position of its top-left corner and its size.
	#[inline]
	pub fn from_size(pos: Vec2<i16>, size: Vec2<i16>) -> Self { Self::new(pos, pos + size) }

	/// Creates a new rect from its size, with its position at x=0 y=0.
	#[inline]
	pub fn at_origin(size: Vec2<i16>) -> Self { Self::new(Vec2::new(0, 0), size) }

	/// Returns true if this rect's end point is greater or equal than its start point on both axis.
	#[inline]
	pub fn is_ordered(&self) -> bool {
		self.start.x <= self.end.x && self.start.y <= self.end.y
	}

	/// Returns this rect with its values rearranged to guarantee that it is ordered.
	#[inline]
	pub fn ordered(self) -> Rect { self.ordered_on(self) }

	/// Returns this rect with its values rearranged according to another rect's ordering.
	///
	/// If the second rect is unordered on one axis, this rect's points will swap on that axis.
	#[inline]
	pub fn ordered_on(mut self, rect: Rect) -> Rect {
		if rect.start.x > rect.end.x {
			swap(&mut self.start.x, &mut self.end.x);
		}
		if rect.start.y > rect.end.y {
			swap(&mut self.start.y, &mut self.end.y);
		}
		Rect::new(self.start, self.end)
	}

	/// Returns the size of this rect.
	///
	/// The size will be negative on one axis if the rect is unordered on that axis.
	///
	/// This is equivalent to `rect.end - rect.start`.
	#[inline]
	pub fn size(&self) -> Vec2<i16> { self.end.zip_map(&self.start, |a, b| a.wrapping_sub(b)) }

	/// Returns the size of this rect as an i32.
	///
	/// The size will be negative on one axis if the rect is unordered on that axis.
	///
	/// This method avoids the case where the returned size overflows
	/// due to the start and end point being too far apart.
	///
	/// This is equivalent to `rect.end - rect.start`.
	#[inline]
	pub fn lsize(&self) -> Vec2<i32> { self.end.cast::<i32>() - self.start.cast::<i32>() }

	/// Returns this rect's start and end point packed in an array.
	#[inline]
	pub fn bounds(&self) -> [Vec2<i16>; 2] { [self.start, self.end] }

	/// Returns true if the given point is inside the bounds of this rect.
	///
	/// This method will return false if the rect is unordered.
	#[inline]
	pub fn contains(&self, p: Vec2<i16>) -> bool {
		p.x >= self.start.x && p.x < self.end.x && p.y >= self.start.y && p.y < self.end.y
	}

	/// Returns true if the given rect is fully inside the bounds of this rect.
	///
	/// This method will return false if the one of the rects is unordered.
	#[inline]
	pub fn encapsulates(&self, rect: Rect) -> bool {
		rect.start.x >= self.start.x
			&& rect.end.x <= self.end.x
			&& rect.start.y >= self.start.y
			&& rect.end.y <= self.end.y
	}

	/// Returns the area shared by this rect and the given one.
	///
	/// This method is like the `&` operator, with the exception
	/// that the second rect is not reordered.
	///
	/// If the second rect is unordered on an axis,
	/// the returned rect will have a size of zero on that axis.
	pub fn intersection(self, bounds: Rect) -> Self {
		let ord = self.ordered();
		Self::from_scalars(
			ord.start.x.max(bounds.start.x),
			ord.start.y.max(bounds.start.y),
			ord.end.x.min(bounds.end.x),
			ord.end.y.min(bounds.end.y),
		)
	}
}

impl Add<Vec2<i16>> for Rect {
	type Output = Self;

	/// Returns this rect offset by the right-hand-side vector
	fn add(mut self, rhs: Vec2<i16>) -> Self {
		self += rhs;
		self
	}
}

impl AddAssign<Vec2<i16>> for Rect {
	/// Offsets this rect by the right-hand-side vector
	fn add_assign(&mut self, rhs: Vec2<i16>) {
		self.start += rhs;
		self.end += rhs
	}
}

impl Sub<Vec2<i16>> for Rect {
	type Output = Self;

	/// Returns this rect offset by the right-hand-side vector
	fn sub(mut self, rhs: Vec2<i16>) -> Self {
		self -= rhs;
		self
	}
}

impl SubAssign<Vec2<i16>> for Rect {
	/// Offsets this rect by the right-hand-side vector
	fn sub_assign(&mut self, rhs: Vec2<i16>) {
		self.start -= rhs;
		self.end -= rhs;
	}
}

impl BitAnd<Rect> for Rect {
	type Output = Self;

	/// Returns the area shared by both rects.
	fn bitand(self, rhs: Rect) -> Self { self.intersection(rhs.ordered()) }
}

impl BitAndAssign<Rect> for Rect {
	/// Clamps this rect to be fully inside the right-hand-side rect.
	fn bitand_assign(&mut self, rhs: Rect) { *self = self.intersection(rhs.ordered()); }
}

impl Debug for Rect {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"Rect([{}, {}] -> [{}, {}])",
			self.start.x, self.start.y, self.end.x, self.end.y
		)
	}
}
