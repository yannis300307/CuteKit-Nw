use enum_map::Enum;

use crate::mods::draw_2d::text::{
	Charset,
	charsets::base::{CaseLetters, Letters, Numbers},
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Enum, Default)]
pub enum Ascii {
	#[default]
	Err,
	NonPrintable,
	Upper(Letters),
	Lower(Letters),
	Number(Numbers),
	Space,
	ExclamationMark,
	DoubleQuote,
	Hashtag,
	Dollar,
	Percent,
	Ampersand,
	SingleQuote,
	OpeningParenthesis,
	ClosingParenthesis,
	Asterisk,
	Plus,
	Comma,
	Dash,
	Period,
	Slash,
	Colon,
	Semicolon,
	LowerThan,
	Equals,
	HigherThan,
	QuestionMark,
	AtSymbol,
	OpeningSquareBracket,
	Backslash,
	ClosingSquareBracket,
	Caret,
	Underscore,
	Backtick,
	OpeningCurlyBracket,
	Pipe,
	ClosingCurlyBracket,
	Approximation,
}

impl Charset for Ascii {
	fn from_char(c: char) -> Option<Self> {
		if !c.is_ascii() {
			return None;
		}
		Some(match CaseLetters::from_char(c) {
			Some(CaseLetters::Lower(v)) => Self::Lower(v),
			Some(CaseLetters::Upper(v)) => Self::Upper(v),
			_ => match Numbers::from_char(c) {
				Some(v) => Self::Number(v),
				_ => match c {
					' ' => Self::Space,
					'!' => Self::ExclamationMark,
					'"' => Self::DoubleQuote,
					'#' => Self::Hashtag,
					'$' => Self::Dollar,
					'%' => Self::Percent,
					'&' => Self::Ampersand,
					'\'' => Self::SingleQuote,
					'(' => Self::OpeningParenthesis,
					')' => Self::ClosingParenthesis,
					'*' => Self::Asterisk,
					'+' => Self::Plus,
					',' => Self::Comma,
					'-' => Self::Dash,
					'.' => Self::Period,
					'/' => Self::Slash,
					':' => Self::Colon,
					';' => Self::Semicolon,
					'<' => Self::LowerThan,
					'=' => Self::Equals,
					'>' => Self::HigherThan,
					'?' => Self::QuestionMark,
					'@' => Self::AtSymbol,
					'[' => Self::OpeningSquareBracket,
					'\\' => Self::Backslash,
					']' => Self::ClosingSquareBracket,
					'^' => Self::Caret,
					'_' => Self::Underscore,
					'`' => Self::Backtick,
					'{' => Self::OpeningCurlyBracket,
					'|' => Self::Pipe,
					'}' => Self::ClosingCurlyBracket,
					'~' => Self::Approximation,
					_ => Self::NonPrintable,
				},
			},
		})
	}
}
