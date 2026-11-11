use core::{any, fmt::Debug};

use glam::I16Vec2;

use crate::{
	Rect,
	textures::{AnyColor, AnyTexture, Texture},
};

impl<'a, T: AnyColor> Texture<'a, T> {
	/// Creates a new texture from a given slice.
	///
	/// The slice's size must exactly match the total texture size.
	#[inline]
	pub const fn from_data(size: I16Vec2, data: &'a [T]) -> Self {
		assert!(size.x >= 0 && size.y >= 0);
		assert!(data.len() == size.x as usize * size.y as usize);
		Self {
			data,
			stride: size.x,
			size,
			clip: Rect::new(I16Vec2::ZERO, size),
		}
	}

	/// Creates a new texture from a given byte slice.
	///
	/// The slice's size must exactly match the total texture size in bytes.
	#[inline]
	pub fn from_bytes(size: I16Vec2, data: &'a [u8]) -> Self {
		assert!(size.x >= 0 && size.y >= 0);
		assert!(data.len() == size.x as usize * size.y as usize * size_of::<T>());
		Self {
			data: bytemuck::cast_slice(data),
			stride: size.x,
			size,
			clip: Rect::new(I16Vec2::ZERO, size),
		}
	}

	/// Returns a new immutable texture corresponding
	/// to a given subarea of this texture.
	///
	/// The subarea will be clamped inside of this texture's clip area.
	///
	/// This texture's clip and the given rect must be ordered,
	/// or the function will panic.
	pub fn view(&'a self, rect: Rect) -> Texture<'a, T> {
		assert!(rect.is_ordered().all());
		let rect = rect & Rect::new(I16Vec2::ZERO, self.size);
		let ord_rect = rect.ordered();

		let start = self.index_of(ord_rect.start);
		let end = self.index_of(ord_rect.end - 1) + 1;
		Texture {
			data:   &self.data[start..end],
			stride: self.stride,
			size:   rect.size(),
			clip:   (self.clip & rect) - rect.start,
		}
	}

	/// Resets this texture's clip area to take the entirety of the texture area.
	#[inline]
	pub const fn reset_clip(&mut self) { self.clip = Rect::new(I16Vec2::ZERO, self.size) }

	/// Returns this texture's pixel buffer.
	#[inline]
	pub const fn get_data(&self) -> &[T] { self.data }
}

impl<'a, T: AnyColor> AnyTexture for Texture<'a, T> {
	//type LtAs<'b> = Texture<'b, T>;
	type Pixel = T;

	#[inline]
	fn size(&self) -> I16Vec2 { self.size }

	#[inline]
	fn stride(&self) -> i16 { self.stride }

	#[inline]
	fn get_clip(&self) -> Rect { self.clip }

	#[inline]
	fn clip(&mut self, rect: Rect) { self.clip = rect & Rect::new(I16Vec2::ZERO, self.size) }

	#[inline]
	fn index_of(&self, p: I16Vec2) -> usize {
		p.y as usize * self.stride as usize + p.x as usize
	}

	#[inline]
	fn get_pixel_at(&self, index: usize) -> T { self.data[index] }
}

impl<'a, T: AnyColor> Debug for Texture<'a, T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"Texture {{ {:?}: {}x{}; stride: {}, clip: {:?} }}",
			any::type_name::<T>(),
			self.size.x,
			self.size.y,
			self.stride,
			self.clip
		)
	}
}
