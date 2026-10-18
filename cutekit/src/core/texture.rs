use core::array;

use bytemuck::AnyBitPattern;

use crate::{Color, ColorAlpha, Rect, Vec2};

pub trait AnyTexture {
	fn clip(&mut self, clip: Rect);
	fn reset_clipping(&mut self);
}

#[derive(Clone, Debug)]
pub struct Texture<T: AnyBitPattern = Color> {
	pub data: &'static [T],
	pub size: Vec2<i16>,
	pub clip: Rect<i16>,
}

#[derive(Clone, Debug)]
pub struct MutableTexture<const SIZE: usize, T: AnyBitPattern = Color> {
	pub data: [T; SIZE],
	pub size: Vec2<i16>,
	pub clip: Rect<i16>,
}

pub type TextureAlpha = Texture<ColorAlpha>;
pub type MutableTextureAlpha<const SIZE: usize> = MutableTexture<SIZE, ColorAlpha>;

impl<T: AnyBitPattern> Texture<T> {
	pub fn from_data(size: Vec2<i16>, data: &'static [T]) -> Self {
		assert!(size.x > 0 && size.y > 0);
		assert!(data.len() == size.x as usize * size.y as usize);
		Self {
			data,
			size,
			clip: Rect(Vec2::new(0, 0), size),
		}
	}

	pub fn from_bytes(size: Vec2<i16>, data: &'static [u8]) -> Self {
		assert!(size.x > 0 && size.y > 0);
		assert!(data.len() == size.x as usize * size.y as usize * size_of::<T>());
		Self {
			data: bytemuck::cast_slice(data),
			size,
			clip: Rect(Vec2::new(0, 0), size),
		}
	}
}

impl<T: AnyBitPattern> AnyTexture for Texture<T> {
	fn reset_clipping(&mut self) { self.clip = Rect(Vec2::new(0, 0), self.size) }

	fn clip(&mut self, clip: Rect) { self.clip = clip.intersect(Rect(Vec2::new(0, 0), self.size)) }
}

impl<const SIZE: usize, T: AnyBitPattern> MutableTexture<SIZE, T> {
	pub fn new(size: Vec2<i16>, fill: T) -> Self {
		assert!(size.x > 0 && size.y > 0);
		Self {
			data: array::repeat::<T, SIZE>(fill),
			size,
			clip: Rect(Vec2::new(0, 0), size),
		}
	}
}

impl<const SIZE: usize, T: AnyBitPattern> AnyTexture for MutableTexture<SIZE, T> {
	fn reset_clipping(&mut self) { self.clip = Rect(Vec2::new(0, 0), self.size) }

	fn clip(&mut self, clip: Rect) { self.clip = clip.intersect(Rect(Vec2::new(0, 0), self.size)) }
}
