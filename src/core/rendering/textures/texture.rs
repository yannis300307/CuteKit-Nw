use core::{any, fmt::Debug};

use crate::{
	Rect,
	Vec2,
	textures::{AnyColor, AnyTexture, Texture},
};

impl<'a, T: AnyColor> Texture<'a, T> {
	/// Creates a new texture from a given slice.
	///
	/// This method is a variant of `from data` with const compatibility,
	/// as nalgebra vectors cannot be indexed in const.
	///
	/// The slice's size must exactly match the total texture size.
	#[inline]
	pub const fn from_data_const(width: i16, height: i16, data: &'a [T]) -> Self {
		let size = Vec2::new(width, height);
		Self {
			data,
			stride: width,
			size,
			clip: Rect::from_scalars(0, 0, width, height),
		}
	}

	/// Creates a new texture from a given slice.
	///
	/// The slice's size must exactly match the total texture size.
	#[inline]
	pub fn from_data(size: Vec2<i16>, data: &'a [T]) -> Self {
		assert!(size.x >= 0 && size.y >= 0);
		assert!(data.len() == size.x as usize * size.y as usize);
		Self {
			data,
			stride: size.x,
			size,
			clip: Rect::at_origin(size),
		}
	}

	/// Creates a new texture from a given byte slice.
	///
	/// The slice's size must exactly match the total texture size in bytes.
	#[inline]
	pub fn from_bytes(size: Vec2<i16>, data: &'a [u8]) -> Self {
		assert!(size.x >= 0 && size.y >= 0);
		assert!(data.len() == size.x as usize * size.y as usize * size_of::<T>());
		Self {
			data: bytemuck::cast_slice(data),
			stride: size.x,
			size,
			clip: Rect::at_origin(size),
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
		assert!(rect.is_ordered());
		let rect = rect & Rect::at_origin(self.size);
		let ord_rect = rect.ordered();

		let start = self.index_of(ord_rect.start);
		let end = self.index_of(ord_rect.end.add_scalar(-1)) + 1;
		Texture {
			data:   &self.data[start..end],
			stride: self.stride,
			size:   rect.size(),
			clip:   (self.clip & rect) - rect.start,
		}
	}

	/// Resets this texture's clip area to take the entirety of the texture area.
	#[inline]
	pub fn reset_clip(&mut self) { self.clip = Rect::at_origin(self.size) }

	/// Returns this texture's pixel buffer.
	#[inline]
	pub fn get_data(&self) -> &[T] { self.data }
}

impl<'a, T: AnyColor> AnyTexture for Texture<'a, T> {
	//type LtAs<'b> = Texture<'b, T>;
	type Pixel = T;

	#[inline]
	fn size(&self) -> Vec2<i16> { self.size }

	#[inline]
	fn stride(&self) -> i16 { self.stride }

	#[inline]
	fn get_clip(&self) -> Rect { self.clip }

	#[inline]
	fn clip(&mut self, rect: Rect) { self.clip = rect & Rect::at_origin(self.size) }

	#[inline]
	fn index_of(&self, p: Vec2<i16>) -> usize {
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
