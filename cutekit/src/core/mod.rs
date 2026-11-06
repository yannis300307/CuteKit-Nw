pub mod public {
	pub use super::{
		app::*,
		app_state::*,
		color::{palette::*, ColorGrayscale, ColorRGB, ColorRGBA},
		rect::Rect,
		textures::{AnyTexture, SamplerTexture, Texture, TextureAtlas, TextureMut},
		SCREEN_HEIGHT,
		SCREEN_WIDTH,
	};
}
pub use public::*;

pub mod app;
pub mod app_state;
pub mod inputs;
pub mod rendering {
	pub mod color;
	pub mod drawing;
	pub mod rect;
	pub mod textures;
}
pub use rendering::*;

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
