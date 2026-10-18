use crate::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, Vec2, texture::MutableTexture};

pub type TileBuffer = MutableTexture<{ SCREEN_TILE_WIDTH as usize * SCREEN_TILE_HEIGHT as usize }>;

pub mod circle;
pub mod line;
pub mod rectangle;
pub mod texture;

pub trait Drawable {
	fn draw(&mut self, tile: &mut TileBuffer, pos: Vec2<i16>);
}

pub trait Pipeline: Drawable {
	type Element;

	fn register(&mut self, element: Self::Element) -> Result<usize, Self::Element>;

	fn clear(&mut self);
}
