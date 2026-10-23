use enum_map::Enum;

use crate::mods::draw_2d::text::{
	Charset,
	charsets::base::{CaseLetters, Letters, Numbers},
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Enum, Default)]
pub enum BasicText {
	#[default]
	Err,
	Upper(Letters),
	Lower(Letters),
	Number(Numbers),
	Space,
	Comma,
	Period,
	Dash,
	ExclamationMark,
	QuestionMark,
	Colon,
	Semicolon,
	DoubleQuote,
}

impl Charset for BasicText {
	fn from_char(c: char) -> Option<Self> {
		match CaseLetters::from_char(c) {
			Some(CaseLetters::Lower(v)) => Some(Self::Lower(v)),
			Some(CaseLetters::Upper(v)) => Some(Self::Upper(v)),
			_ => match Numbers::from_char(c) {
				Some(v) => Some(Self::Number(v)),
				_ => match c {
					' ' => Some(Self::Space),
					',' => Some(Self::Comma),
					'.' => Some(Self::Period),
					'-' => Some(Self::Dash),
					'!' => Some(Self::ExclamationMark),
					'?' => Some(Self::QuestionMark),
					':' => Some(Self::Colon),
					';' => Some(Self::Semicolon),
					'"' => Some(Self::DoubleQuote),
					_ => None,
				},
			},
		}
	}
}
