use crate::{Color, ColorAlpha, ColorGrayscale, textures::AnyColor};

pub trait KibColor: AnyColor {
	const SIGNATURE: u8;
}

impl KibColor for Color {
	const SIGNATURE: u8 = 1;
}
impl KibColor for ColorAlpha {
	const SIGNATURE: u8 = 2;
}
impl KibColor for ColorGrayscale {
	const SIGNATURE: u8 = 3;
}
