use enum_map::Enum;

use crate::mods::draw_2d::text::Charset;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Enum, Default)]
pub enum Letters {
	#[default]
	Err,
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Enum, Default)]
pub enum CaseLetters {
	#[default]
	Err,
	Lower(Letters),
	Upper(Letters),
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Enum, Default)]
pub enum Numbers {
	#[default]
	Err,
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
}

impl Charset for Letters {
	fn from_char(c: char) -> Option<Self> {
		match c {
			'a'..='z' => Some(c as usize - 'a' as usize),
			'A'..='Z' => Some(c as usize - 'A' as usize),
			_ => None,
		}
		.and_then(|x| Some(Self::from_usize(x + Self::into_usize(Self::A))))
	}
}

impl Charset for CaseLetters {
	fn from_char(c: char) -> Option<Self> {
		match c {
			'a'..='z' => Some(Self::Lower(Letters::from_usize(
				c as usize - 'a' as usize + Letters::into_usize(Letters::A),
			))),
			'A'..='Z' => Some(Self::Upper(Letters::from_usize(
				c as usize - 'A' as usize + Letters::into_usize(Letters::A),
			))),
			_ => None,
		}
	}
}

impl Charset for Numbers {
	fn from_char(c: char) -> Option<Self> {
		match c.to_digit(10) {
			Some(n) => Some(Self::from_usize(n as usize + Self::into_usize(Self::Zero))),
			None => None,
		}
	}
}
