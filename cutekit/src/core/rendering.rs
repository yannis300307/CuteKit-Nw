use crate::{Color, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, ScreenPos};

pub struct RenderQueue<T, const SIZE: usize> {
	queue: heapless::Vec<T, SIZE>,
}

pub struct TileBuffer<T = Color> {
	pub buffer: [T; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
	pub pos:    ScreenPos,
}

impl<T> TileBuffer<T>
where
	T: Copy,
{
	const HEIGHT: u16 = SCREEN_TILE_HEIGHT as u16;
	const WIDTH: u16 = SCREEN_TILE_WIDTH as u16;

	pub fn new(pos: ScreenPos, fill: T) -> Self {
		Self {
			pos,
			buffer: [fill; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
		}
	}
}

pub trait PipelineTrait {
	fn draw(&mut self, tile: &mut TileBuffer);
}

pub(crate) type PipelineRef = &'static mut dyn PipelineTrait;
