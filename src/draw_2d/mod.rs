pub mod drawing;
pub mod font;
pub mod pipeline;

pub mod prelude {
	pub mod charsets {
		pub use super::super::font::charsets::{
			ascii::Ascii,
			basic_text::BasicText,
			numeric::Numeric,
		};
	}
	pub use super::{
		drawing::*,
		font::ColorFont,
		pipeline::{
			Pipeline2D,
			elements::{Element2D, NineParts},
			registry::Pipeline,
		},
	};
}

pub mod public {
	pub use super::prelude::{charsets::*, *};
}
