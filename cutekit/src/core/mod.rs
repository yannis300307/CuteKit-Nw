pub mod color;
pub mod context;
pub mod drawing;
pub mod inputs;
pub mod math;
pub mod rect;
pub mod texture;

pub use color::ColorAlpha;
pub use context::Cutekit;
pub use math::CutekitMatrix;
pub use rect::Rect;

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
