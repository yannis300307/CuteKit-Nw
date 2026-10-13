pub mod context;
pub mod inputs;
pub mod rendering;

pub use context::Cutekit;

pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 240;

// Minimum 2
pub const SCREEN_TILE_SUBDIVISION: usize = 16;
const _: () = assert!(SCREEN_HEIGHT % SCREEN_TILE_SUBDIVISION == 0);

pub const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH;
pub const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT / SCREEN_TILE_SUBDIVISION;

pub const MAX_PIPELINES: usize = 8;
