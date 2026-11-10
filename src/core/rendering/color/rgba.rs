use crate::{ColorGrayscale, ColorRGB, ColorRGBA};

impl ColorRGBA {
	/// Creates a new RGBA color from 8-bits red, green, blue and alpha channels.
	///
	/// Each channel must be in the range \[0;255].
	#[inline]
	pub const fn new<T: [const] Into<u16>>(r: T, g: T, b: T, a: T) -> Self {
		Self {
			rgb: ColorRGB::new(r, g, b),
			a:   a.into() as u8,
		}
	}

	/// Creates a new RGBA color from red, green, blue and alpha channels.
	///
	/// Each channel must exactly match the value they will be stored in using RGBA 24,
	/// meaning the given values must be within the following ranges:
	/// - red: \[0;31]
	/// - green: \[0;63]
	/// - blue: \[0;31]
	/// - alpha: \[0;255]
	#[inline]
	pub const fn new_5658(r: u16, g: u16, b: u16, a: u16) -> Self {
		Self {
			rgb: ColorRGB::new_565(r, g, b),
			a:   a as u8,
		}
	}

	/// Creates a new RGBA color from a packed 8-bits representation.
	///
	/// The packed representation is usully represented
	/// using hexadecimal format (`0xRRGGBBAA`).
	///
	/// Each channel must be in the range \[0;255].
	#[inline]
	pub const fn unpack(rgba32: u32) -> Self {
		Self {
			rgb: ColorRGB::unpack(rgba32 >> 8),
			a:   (rgba32 & 0xFF) as u8,
		}
	}

	/// Creates a new RGBA color from 8-bits red, green, blue and alphs channels
	/// stored in a slice.
	///
	/// Each channel must be in the range \[0;255].
	#[inline]
	pub const fn from_slice<T: [const] Into<u16> + Copy>(rgba: &[T; 4]) -> Self {
		let [r, g, b, a] = rgba;
		Self::new(*r, *g, *b, *a)
	}

	/// Creates a new opaque RGBA color from an RGB color.
	#[inline]
	pub const fn from_rgb(color: ColorRGB) -> Self {
		Self {
			rgb: color,
			a:   0xFF,
		}
	}

	/// Creates a new opaque RGBA color from a grayscale color.
	#[inline]
	pub const fn from_grayscale(color: ColorGrayscale) -> Self {
		Self::from_rgb(ColorRGB::from_grayscale(color))
	}

	/// Returns an RGB color corresponding to this color as if it were opaque.
	#[inline]
	pub const fn as_rgb(&self) -> ColorRGB { self.rgb }

	/// Returns an array containg the red, green and blue values of this color.
	///
	/// The values are converted to regular 8-bit color values,
	/// meaning the returned values will be in the \[0;255] range.
	#[inline]
	pub const fn rgb(&self) -> [u16; 3] { return self.as_rgb().rgb(); }

	/// Returns an array containg the red, green, blue and alpha values of this color.
	///
	/// The values are converted to regular 8-bit color values,
	/// meaning the returned values will be in the \[0;255] range.
	#[inline]
	pub const fn rgba(&self) -> [u16; 4] {
		let [r, g, b] = self.rgb();
		return [r, g, b, self.a as u16];
	}

	/// Returns an array containg the red, green and blue values of this color.
	///
	/// The values are kept as they are in the RGB 16 color,
	/// meaning the returned values will be within the following ranges:
	/// - red: \[0;31]
	/// - green: \[0;63]
	/// - blue: \[0;31]
	#[inline]
	pub const fn rgb_565(&self) -> [u16; 3] { return self.as_rgb().rgb_565(); }

	/// Returns an array containg the red, green, blue and alpha values of this color.
	///
	/// The values are kept as they are in the RGBA 24 color,
	/// meaning the returned values will be within the following ranges:
	/// - red: \[0;31]
	/// - green: \[0;63]
	/// - blue: \[0;31]
	/// - alpha: \[0;255]
	#[inline]
	pub const fn rgba_5658(&self) -> [u16; 4] {
		let [r, g, b] = self.rgb_565();
		return [r, g, b, self.a as u16];
	}
}

impl From<ColorRGB> for ColorRGBA {
	#[inline]
	fn from(color: ColorRGB) -> Self { Self::from_rgb(color) }
}

impl From<ColorGrayscale> for ColorRGBA {
	#[inline]
	fn from(color: ColorGrayscale) -> Self { Self::from_grayscale(color) }
}
