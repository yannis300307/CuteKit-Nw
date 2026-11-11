use core::{any, fmt::Debug};

use bytemuck::NoUninit;
use glam::I16Vec2;

use crate::{
	Rect,
	textures::{AnyColor, AnyTexture, AnyTextureMut, Texture, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	/// Creates a new texture from a given slice.
	///
	/// The slice's size must exactly match the total texture size.
	#[inline]
	pub const fn from_data(size: I16Vec2, data: &'a mut [T]) -> Self {
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
	pub fn from_bytes(size: I16Vec2, data: &'a mut [u8]) -> Self
	where
		T: NoUninit,
	{
		assert!(size.x >= 0 && size.y >= 0);
		assert!(data.len() == size.x as usize * size.y as usize * size_of::<T>());
		Self {
			data: bytemuck::cast_slice_mut(data),
			stride: size.x,
			size,
			clip: Rect::new(I16Vec2::ZERO, size),
		}
	}

	/// Returns an immutable version of this texture.
	#[inline]
	pub const fn immutable(&'a self) -> Texture<'a, T> {
		Texture {
			data:   &self.data,
			stride: self.size.x,
			size:   self.size,
			clip:   self.clip,
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
		assert!(self.clip.is_ordered().all());
		let rect = (rect + self.clip.start) & self.clip;

		let start = self.index_of(rect.start);
		let end = self.index_of(rect.end - 1) + 1;
		let rsize = rect.size();
		Texture {
			data:   &self.data[start..end],
			stride: self.stride,
			size:   rsize,
			clip:   Rect::new(I16Vec2::ZERO, rsize),
		}
	}

	/// Returns a new mutable texture corresponding
	/// to a given subarea of this texture.
	///
	/// The subarea will be clamped inside of this texture's clip area.
	///
	/// This texture's clip and the given rect must be ordered,
	/// or the function will panic.
	pub fn view_mut(&'a mut self, rect: Rect) -> TextureMut<'a, T> {
		assert!(rect.is_ordered().all());
		assert!(self.clip.is_ordered().all());
		let rect = (rect + self.clip.start) & self.clip;

		let start = self.index_of(rect.start);
		let end = self.index_of(rect.end - 1) + 1;
		let rsize = rect.size();
		TextureMut {
			data:   &mut self.data[start..end],
			stride: self.stride,
			size:   rsize,
			clip:   Rect::new(I16Vec2::ZERO, rsize),
		}
	}

	/// Resets this texture's clip area to take the entirety of the texture area.
	#[inline]
	pub const fn reset_clip(&mut self) { self.clip = Rect::new(I16Vec2::ZERO, self.size) }

	/// Returns this texture's pixel buffer.
	#[inline]
	pub const fn get_data(&self) -> &[T] { self.data }

	/// Returns this texture's pixel buffer as mutable.
	#[inline]
	pub const fn get_data_mut(&mut self) -> &mut [T] { self.data }
}

impl<'a, T: AnyColor> AnyTexture for TextureMut<'a, T> {
	type Pixel = T;

	#[inline]
	fn size(&self) -> I16Vec2 { self.size }

	#[inline]
	fn stride(&self) -> i16 { self.stride }

	#[inline]
	fn get_clip(&self) -> Rect { self.clip }

	#[inline]
	fn clip(&mut self, clip: Rect) { self.clip = clip & Rect::new(I16Vec2::ZERO, self.size) }

	#[inline]
	fn index_of(&self, p: I16Vec2) -> usize {
		p.y as usize * self.stride as usize + p.x as usize
	}

	#[inline]
	fn get_pixel_at(&self, index: usize) -> T { self.data[index] }
}

impl<T: AnyColor> AnyTextureMut for TextureMut<'_, T> {
	type LtAs<'b> = TextureMut<'b, T>;

	#[inline]
	fn set_pixel_at(&mut self, index: usize, val: T) { self.data[index] = val }
}

impl<T: AnyColor> Debug for TextureMut<'_, T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"TextureMut {{ {:?}: {}x{}; stride: {}, clip: {:?} }}",
			any::type_name::<T>(),
			self.size.x,
			self.size.y,
			self.stride,
			self.clip
		)
	}
}
