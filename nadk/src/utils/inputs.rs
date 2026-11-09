use crate::{Key, scan_keyboard};

impl Key {
	/// Match a character with the given key.
	/// If alpha is active and shift is not, you will get lowercase letter.
	/// With alpha active and shift inactive, you will get uppercase letters.
	/// With only shift active, you will get special characters.
	/// If none of alpha and shift are active, you will get digits.
	/// You should expect the same behavior as the Epsilon's Python app.
	pub fn get_matching_char(&self, shift_active: bool, alpha_active: bool) -> Option<char> {
		use Key::*;
		match match (alpha_active, shift_active) {
			(false, false) => match *self {
				One => '1',
				Two => '2',
				Three => '3',
				Four => '4',
				Five => '5',
				Six => '6',
				Seven => '7',
				Eight => '8',
				Nine => '9',
				Zero => '0',
				Comma => ',',
				LeftParenthesis => '(',
				RightParenthesis => ')',
				Multiplication => '*',
				Division => '/',
				Plus => '+',
				Minus => '-',
				Dot => '.',
				_ => '\0',
			},
			(false, true) => match *self {
				Exp => '[',
				Ln => ']',
				Log => '{',
				Imaginary => '}',
				Comma => '_',
				Pi => '=',
				Sqrt => '<',
				Power => '>',
				_ => '\0',
			},
			(true, false) => match *self {
				Exp => 'a',
				Ln => 'b',
				Log => 'c',
				Imaginary => 'd',
				Comma => 'e',
				Power => 'f',
				Sine => 'g',
				Cosine => 'h',
				Tangent => 'i',
				Pi => 'j',
				Sqrt => 'k',
				Square => 'l',
				Seven => 'm',
				Eight => 'n',
				Nine => 'o',
				LeftParenthesis => 'p',
				RightParenthesis => 'q',
				Four => 'r',
				Five => 's',
				Six => 't',
				Multiplication => 'u',
				Division => 'v',
				One => 'w',
				Two => 'x',
				Three => 'y',
				Plus => 'z',
				Minus => ' ',
				Zero => '?',
				Dot => '!',
				_ => '\0',
			},
			(true, true) => match *self {
				Exp => 'A',
				Ln => 'B',
				Log => 'C',
				Imaginary => 'D',
				Comma => 'E',
				Power => 'F',
				Sine => 'G',
				Cosine => 'H',
				Tangent => 'I',
				Pi => 'J',
				Sqrt => 'K',
				Square => 'L',
				Seven => 'M',
				Eight => 'N',
				Nine => 'O',
				LeftParenthesis => 'P',
				RightParenthesis => 'Q',
				Four => 'R',
				Five => 'S',
				Six => 'T',
				Multiplication => 'U',
				Division => 'V',
				One => 'W',
				Two => 'X',
				Three => 'Y',
				Plus => 'Z',
				Minus => ' ',
				Zero => '?',
				Dot => '!',
				_ => '\0',
			},
		} {
			'\0' => None,
			c => Some(c),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct KeyboardState {
	last:    u64,
	current: u64,
}

impl KeyboardState {
	/// Creates a new, empty keyboard state
	pub const fn new() -> Self {
		Self {
			last:    0,
			current: 0,
		}
	}

	/// Updates the keyboard state.
	pub fn update(&mut self) {
		self.last = self.current;
		self.current = scan_keyboard();
	}

	/// Returns true if the key is currently pressed.
	pub fn key_down(&self, key: Key) -> bool { self.current & (1 << key as u8) != 0 }

	/// Returns true if the key has just been pressed.
	pub fn key_just_pressed(&self, key: Key) -> bool {
		self.current & (1 << key as u8) != 0 && self.last & (1 << key as u8) == 0
	}

	/// Returns true if the key has just been released.
	pub fn key_just_released(&self, key: Key) -> bool {
		self.current & (1 << key as u8) == 0 && self.last & (1 << key as u8) != 0
	}
}
