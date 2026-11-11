pub mod color;
pub mod texture;

pub const KIB_VERSION: u8 = 1;

pub mod prelude {
	pub use super::texture::include_texture;
}
pub mod public {
	pub use super::{KIB_VERSION, prelude::*};
}
