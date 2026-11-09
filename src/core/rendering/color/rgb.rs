use nadk::Color565;

use crate::{ColorGrayscale, ColorRGB};

impl ColorRGB {
	const B_MASK: u16 = 0b00000000_00011111;
	const G_MASK: u16 = 0b00000111_11100000;
	const R_MASK: u16 = 0b11111000_00000000;

	/// Creates a new RGB color from 8-bits red, green and blue channels.
	///
	/// Each channel must be in the range \[0;255]
	#[inline]
	pub const fn new<T: [const] Into<u16>>(r: T, g: T, b: T) -> Self {
		let r = r.into() << 8 & Self::R_MASK;
		let g = g.into() << 3 & Self::G_MASK;
		let b = b.into() >> 3 & Self::B_MASK;
		Self { rgb: r | g | b }
	}

	/// Creates a new RGB color from red, green and blue channels.
	///
	/// Each channel must exactly match the value they will be stored in using RGB 16,
	/// meaning the given values must be within the following ranges:
	/// - red: \[0;31]
	/// - green: \[0;63]
	/// - blue: \[0;31]
	#[inline]
	pub const fn new_565(r: u16, g: u16, b: u16) -> Self {
		Self {
			rgb: r << (6 + 5) | g << 5 | b,
		}
	}

	/// Creates a new RGB color from a packed 8-bits representation.
	///
	/// The packed representation is usully represented
	/// using hexadecimal format (`0xRRGGBB`).
	///
	/// Each channel must be in the range \[0;255].
	#[inline]
	pub const fn unpack(rgb24: u32) -> Self {
		let r = (rgb24 >> 8) as u16 & Self::R_MASK;
		let g = (rgb24 >> 5) as u16 & Self::G_MASK;
		let b = (rgb24 >> 3) as u16 & Self::B_MASK;
		Self { rgb: r | g | b }
	}

	/// Creates a new RGB color from 8-bits red, green and blue channels
	/// stored in a slice.
	///
	/// Each channel must be in the range \[0;255].
	#[inline]
	pub const fn from_slice<T: [const] Into<u16> + Copy>(rgb: &[T; 3]) -> Self {
		let [r, g, b] = rgb;
		Self::new(*r, *g, *b)
	}

	/// Creates a new RGB color from a grayscale color.
	#[inline]
	pub const fn from_grayscale(color: ColorGrayscale) -> Self {
		Self::new(color.value as u16, color.value as u16, color.value as u16)
	}

	/// Returns an array containg the red, green and blue values of this color.
	///
	/// The values are converted to regular 8-bit color values,
	/// meaning the returned values will be in the \[0;255] range.
	#[inline]
	pub const fn rgb(&self) -> [u16; 3] {
		let r = (self.rgb & Self::R_MASK) >> 8;
		let g = (self.rgb & Self::G_MASK) >> 3;
		let b = (self.rgb & Self::B_MASK) << 3;
		return [r, g, b];
	}

	/// Returns an array containg the red, green and blue values of this color.
	///
	/// The values are kept as they are in the RGB 16 color,
	/// meaning the returned values will be within the following ranges:
	/// - red: \[0;31]
	/// - green: \[0;63]
	/// - blue: \[0;31]
	#[inline]
	pub const fn rgb_565(&self) -> [u16; 3] {
		let r = (self.rgb & Self::R_MASK) >> (6 + 5);
		let g = (self.rgb & Self::G_MASK) >> 5;
		let b = self.rgb & Self::B_MASK;
		return [r, g, b];
	}

	/// Converts this color into the color type used by nadk.
	///
	/// This only performs a type cast, the data is kept unchanged.
	#[inline]
	pub fn to_nadk(self) -> Color565 { bytemuck::cast(self) }

	/// Converts a color slice into a slice of the color type used by nadk.
	///
	/// This only performs a type cast, the data is kept unchanged.
	#[inline]
	pub fn slice_to_nadk(slice: &[Self]) -> &[Color565] { bytemuck::cast_slice(slice) }
}

impl From<ColorGrayscale> for ColorRGB {
	#[inline]
	fn from(color: ColorGrayscale) -> Self { Self::from_grayscale(color) }
}
