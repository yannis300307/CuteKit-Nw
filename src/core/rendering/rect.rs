use core::{
	fmt::Debug,
	mem::swap,
	ops::{Add, AddAssign, BitAnd, BitAndAssign, Sub, SubAssign},
};

use glam::I16Vec2;
use glam_traits::{BVec, GVec, GVec2, GVec3, GVec4, IntVec};

/// Representation of a rectangular area on a texture or on the screen.
///
/// start usually represents the top-left corner of the rectangle,
/// while end usually represents its bottom-right one.
///
/// start and end can swapped around, the resulting rect then becomes "unordered".
/// Unordered rects are valid in some contexts, and can lead to special behaviors.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect<V: GVec = I16Vec2> {
	pub start: V,
	pub end:   V,
}

impl<V: GVec2> Rect<V> {
	/// Creates a new rect from two decomposed points.
	///
	/// The points match an ordered rect's top-left and bottom-right corner.
	/// This method behaves similarly to `Rect::new`.
	#[inline]
	pub fn pack2(x1: V::Scalar, y1: V::Scalar, x2: V::Scalar, y2: V::Scalar) -> Self {
		Self::new(V::new(x1, y1), V::new(x2, y2))
	}
}

impl<V: GVec3> Rect<V> {
	/// Creates a new rect from two decomposed points.
	///
	/// The points match an ordered rect's top-left and bottom-right corner.
	/// This method behaves similarly to `Rect::new`.
	#[inline]
	pub fn pack3(
		x1: V::Scalar,
		y1: V::Scalar,
		z1: V::Scalar,
		x2: V::Scalar,
		y2: V::Scalar,
		z2: V::Scalar,
	) -> Self {
		Self::new(V::new(x1, y1, z1), V::new(x2, y2, z2))
	}
}

impl<V: GVec4> Rect<V> {
	/// Creates a new rect from two decomposed points.
	///
	/// The points match an ordered rect's top-left and bottom-right corner.
	/// This method behaves similarly to `Rect::new`.
	#[inline]
	pub fn pack4(
		x1: V::Scalar,
		y1: V::Scalar,
		z1: V::Scalar,
		w1: V::Scalar,
		x2: V::Scalar,
		y2: V::Scalar,
		z2: V::Scalar,
		w2: V::Scalar,
	) -> Self {
		Self::new(V::new(x1, y1, z1, w1), V::new(x2, y2, z2, w2))
	}
}

impl<V: GVec> Rect<V> {
	/// Creates a new Rect from two points.
	///
	/// The points match an ordered rect's top-left and bottom-right corner.
	#[inline]
	pub const fn new(start: V, end: V) -> Self { Self { start, end } }

	/// Creates a new rect from two slices.
	///
	/// The slices match an ordered rect's top-left and bottom-right corner.
	/// This method behaves similarly to `Rect::new`.
	#[inline]
	pub fn from_slices(start: &[V::Scalar], end: &[V::Scalar]) -> Self {
		Self::new(V::from_slice(start), V::from_slice(end))
	}

	/// Creates a new rect from the position of its top-left corner and its size.
	#[inline]
	pub fn from_size(pos: V, size: V) -> Self { Self::new(pos, pos + size) }

	/// Returns true if this rect's end point is greater or equal than its start point on both axis.
	#[inline]
	pub fn is_ordered(&self) -> V::BVec { self.start.cmple(self.end) }

	/// Returns this rect with its values rearranged to guarantee that it is ordered.
	#[inline]
	pub fn ordered(self) -> Rect<V> { self.ordered_on(self) }

	/// Returns this rect with its values rearranged according to another rect's ordering.
	///
	/// If the second rect is unordered on one axis, this rect's points will swap on that axis.
	#[inline]
	pub fn ordered_on(mut self, rect: Rect<V>) -> Rect<V> {
		for i in 0..V::DIM {
			if rect.start[i] > rect.end[i] {
				swap(&mut self.start[i], &mut self.end[i]);
			}
		}
		Rect::new(self.start, self.end)
	}

	/// Returns the size of this rect.
	///
	/// The size will be negative on one axis if the rect is unordered on that axis.
	///
	/// This is equivalent to `rect.end - rect.start`.
	#[inline]
	pub fn size(&self) -> V { self.end - self.start }

	/// Returns the size of this rect as an i32.
	///
	/// The size will be negative on one axis if the rect is unordered on that axis.
	///
	/// This method avoids the case where the returned size overflows
	/// due to the start and end point being too far apart.
	///
	/// This is equivalent to `rect.end - rect.start`.
	#[inline]
	pub fn lsize(&self) -> V::ISizeVec {
		self.end
			.as_isizevec()
			.wrapping_sub(self.start.as_isizevec())
	}

	/// Returns this rect's start and end point packed in an array.
	#[inline]
	pub fn bounds(&self) -> [V; 2] { [self.start, self.end] }

	/// Returns true if the given point is inside the bounds of this rect.
	///
	/// This method will return false if the rect is unordered.
	#[inline]
	pub fn contains(&self, p: V) -> bool {
		p.cmpge(self.start).all() && p.cmplt(self.end).all()
	}

	/// Returns true if the given rect is fully inside the bounds of this rect.
	///
	/// This method will return false if the one of the rects is unordered.
	#[inline]
	pub fn encapsulates(&self, rect: Rect<V>) -> bool {
		rect.start.cmpge(self.start).all() && rect.end.cmple(self.end).all()
	}

	/// Returns the area shared by this rect and the given one.
	///
	/// This method is like the `&` operator, with the exception
	/// that the second rect is not reordered.
	///
	/// If the second rect is unordered on an axis,
	/// the returned rect will have a size of zero on that axis.
	pub fn intersection(self, bounds: Rect<V>) -> Self {
		let ord = self.ordered();
		Self::new(ord.start.max(bounds.start), ord.end.min(bounds.end))
	}
}

impl<V: GVec> Add<V> for Rect<V> {
	type Output = Self;

	/// Returns this rect offset by the right-hand-side vector
	fn add(mut self, rhs: V) -> Self {
		self += rhs;
		self
	}
}

impl<V: GVec> AddAssign<V> for Rect<V> {
	/// Offsets this rect by the right-hand-side vector
	fn add_assign(&mut self, rhs: V) {
		self.start += rhs;
		self.end += rhs
	}
}

impl<V: GVec> Sub<V> for Rect<V> {
	type Output = Self;

	/// Returns this rect offset by the right-hand-side vector
	fn sub(mut self, rhs: V) -> Self {
		self -= rhs;
		self
	}
}

impl<V: GVec> SubAssign<V> for Rect<V> {
	/// Offsets this rect by the right-hand-side vector
	fn sub_assign(&mut self, rhs: V) {
		self.start -= rhs;
		self.end -= rhs;
	}
}

impl<V: GVec> BitAnd<Rect<V>> for Rect<V> {
	type Output = Self;

	/// Returns the area shared by both rects.
	fn bitand(self, rhs: Rect<V>) -> Self { self.intersection(rhs.ordered()) }
}

impl<V: GVec> BitAndAssign<Rect<V>> for Rect<V> {
	/// Clamps this rect to be fully inside the right-hand-side rect.
	fn bitand_assign(&mut self, rhs: Rect<V>) { *self = self.intersection(rhs.ordered()); }
}

impl<V: GVec> Debug for Rect<V> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Rect({} -> {})", self.start, self.end,)
	}
}
