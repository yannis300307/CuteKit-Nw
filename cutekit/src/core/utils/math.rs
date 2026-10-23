use core::{
	fmt::{Debug, Display},
	iter::Step,
};

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
		+ Step
		+ Display,
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
impl Integer for isize {}
impl Integer for usize {}
