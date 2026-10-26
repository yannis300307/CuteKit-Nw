pub mod color;
pub mod context;
pub mod drawing;
pub mod global_state;
pub mod inputs;
pub mod rect;
pub mod textures;
pub mod utils;

pub mod public {
	pub use super::{
		SCREEN_HEIGHT,
		SCREEN_WIDTH,
		color::{ColorAlpha, ColorGrayscale},
		context::{Cutekit, init},
		global_state::GlobalState,
		rect::{OrdRect, Rect},
		textures::{Texture, TextureAlpha, TextureAtlas, TextureMut, TextureMutAlpha},
		utils::math::CutekitMatrix,
	};
}
pub use public::*;

pub const SCREEN_WIDTH: i16 = 320;
pub const SCREEN_HEIGHT: i16 = 240;

pub const SCREEN_SUBDIVISIONS: i16 = 16;

pub const SCREEN_TILE_WIDTH: i16 = SCREEN_WIDTH;
pub const SCREEN_TILE_HEIGHT: i16 = SCREEN_HEIGHT / SCREEN_SUBDIVISIONS;

const _: () = assert!(SCREEN_SUBDIVISIONS >= 2, "Expected at least 2 subdivisions");
const _: () = assert!(
	SCREEN_HEIGHT % SCREEN_SUBDIVISIONS == 0,
	"The screen cannot be evenly distributed among the current amount of subdivisions"
);
