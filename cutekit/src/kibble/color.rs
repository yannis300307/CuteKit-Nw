use crate::{ColorGrayscale, ColorRGB, ColorRGBA, color::AnyColor};

/// Trait implemented by colors that can be stored in .kib files.
pub trait KibColor: AnyColor {
	/// Unique type signature for this color in .kib files.
	const SIGNATURE: u8;
}

impl KibColor for ColorRGB {
	const SIGNATURE: u8 = 1;
}
impl KibColor for ColorRGBA {
	const SIGNATURE: u8 = 2;
}
impl KibColor for ColorGrayscale {
	const SIGNATURE: u8 = 3;
}
