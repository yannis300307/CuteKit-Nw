pub mod prelude {
	pub use super::{
		SCREEN_HEIGHT,
		SCREEN_WIDTH,
		color::{ColorGrayscale, ColorRGB, ColorRGBA, palette::*},
		drawing::Renderable,
		rect::Rect,
		textures::{AnyTexture, SamplerTexture, Texture, TextureAtlas, TextureMut},
		vectors::vec2::*,
	};
}

pub mod public {
	pub use super::{
		app::*,
		app_state::*,
		prelude::*,
		vectors::{vec3::*, vec4::*},
	};
}
pub use public::*;

pub mod app;
pub mod app_state;
pub mod inputs;
pub mod vectors;
pub mod rendering {
	pub mod color;
	pub mod drawing;
	pub mod rect;
	pub mod textures;
}
pub use rendering::*;

pub const SCREEN_WIDTH: i16 = nadk::SCREEN_WIDTH as i16;
pub const SCREEN_HEIGHT: i16 = nadk::SCREEN_HEIGHT as i16;

pub const SCREEN_SUBDIVISIONS: i16 = 16;

pub const SCREEN_TILE_WIDTH: i16 = SCREEN_WIDTH;
pub const SCREEN_TILE_HEIGHT: i16 = SCREEN_HEIGHT / SCREEN_SUBDIVISIONS;

const _: () = assert!(SCREEN_SUBDIVISIONS >= 2, "Expected at least 2 subdivisions");
const _: () = assert!(
	SCREEN_HEIGHT % SCREEN_SUBDIVISIONS == 0,
	"The screen cannot be evenly distributed among the current amount of subdivisions"
);
