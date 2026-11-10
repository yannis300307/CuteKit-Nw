use enum_map::Enum;

use crate::mods::draw_2d::font::{Charset, charsets::base::Numbers};

/// Charset enum used to handle basic numeric representation.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Enum, Default)]
pub enum Numeric {
	#[default]
	Err,
	Number(Numbers),
	Minus,
	Dot,
}

impl Charset for Numeric {
	fn from_char(c: char) -> Option<Self> {
		match Numbers::from_char(c) {
			Some(n) => Some(Self::Number(n)),
			None => match c {
				'-' => Some(Self::Minus),
				'.' => Some(Self::Dot),
				_ => None,
			},
		}
	}
}
