use core::{cmp::Ordering, fmt::Debug, iter::Step};

use nalgebra::{
	ClosedAddAssign,
	ClosedDivAssign,
	ClosedMulAssign,
	ClosedSubAssign,
	Dim,
	Matrix,
	RawStorageMut,
	Scalar,
};

pub trait CutekitMatrix {
	fn min_with(self, other: Self) -> Self;
	fn max_with(self, other: Self) -> Self;
}

impl<T, R, C, S> CutekitMatrix for Matrix<T, R, C, S>
where
	T: Scalar + Copy + Ord,
	R: Dim,
	C: Dim,
	S: RawStorageMut<T, R, C>,
{
	fn min_with(self, other: Self) -> Self {
		let mut i = 0;
		self.apply_into(|x| {
			*x = (*x).min(other[i]);
			i += 1;
		})
	}

	fn max_with(self, other: Self) -> Self {
		let mut i = 0;
		self.apply_into(|x| {
			*x = (*x).max(other[i]);
			i += 1;
		})
	}
}

pub trait Integer
where
	Self: 'static
		+ Copy
		+ Debug
		+ PartialEq
		+ Eq
		+ PartialOrd
		+ Ord
		+ ClosedAddAssign
		+ ClosedSubAssign
		+ ClosedMulAssign
		+ ClosedDivAssign
		+ Step,
{
}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}

pub trait Number
where
	Self: 'static
		+ Copy
		+ Debug
		+ PartialEq
		+ PartialOrd
		+ ClosedAddAssign
		+ ClosedSubAssign
		+ ClosedMulAssign
		+ ClosedDivAssign,
{
}
impl<T: Integer> Number for T {}
impl Number for f16 {}
impl Number for f32 {}
impl Number for f64 {}

pub trait KindaOrd: PartialOrd {
	fn kcmp(&self, other: &Self) -> Ordering;
	fn kmin(self, other: Self) -> Self;
	fn kmax(self, other: Self) -> Self;
	fn kclamp(self, min: Self, max: Self) -> Self;
}

impl<T: Integer> KindaOrd for T {
	fn kcmp(&self, other: &Self) -> Ordering { self.cmp(other) }

	fn kmin(self, other: Self) -> Self { self.min(other) }

	fn kmax(self, other: Self) -> Self { self.max(other) }

	fn kclamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
}

impl KindaOrd for f16 {
	fn kcmp(&self, other: &Self) -> Ordering { self.total_cmp(other) }

	fn kmin(self, other: Self) -> Self { self.min(other) }

	fn kmax(self, other: Self) -> Self { self.max(other) }

	fn kclamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
}

impl KindaOrd for f32 {
	fn kcmp(&self, other: &Self) -> Ordering { self.total_cmp(other) }

	fn kmin(self, other: Self) -> Self { self.min(other) }

	fn kmax(self, other: Self) -> Self { self.max(other) }

	fn kclamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
}

impl KindaOrd for f64 {
	fn kcmp(&self, other: &Self) -> Ordering { self.total_cmp(other) }

	fn kmin(self, other: Self) -> Self { self.min(other) }

	fn kmax(self, other: Self) -> Self { self.max(other) }

	fn kclamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
}
