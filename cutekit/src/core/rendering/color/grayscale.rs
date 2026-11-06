use crate::{ColorGrayscale, ColorRGB, ColorRGBA};

impl ColorGrayscale {
	/// Creates a new grayscale color from a single value in the range \[0;255].
	#[inline]
	pub const fn new<T: [const] Into<u8>>(value: T) -> Self { Self { value: value.into() } }

	/// Returns the equivalent of this color in a RGB color format.
	#[inline]
	pub const fn as_rgb(&self) -> ColorRGB { ColorRGB::from_grayscale(*self) }

	/// Returns the equivalent of this color in a RGBA color format.
	#[inline]
	pub const fn as_rgba(&self) -> ColorRGBA { ColorRGBA::from_grayscale(*self) }
}
