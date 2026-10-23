use core::{any, fmt::Debug};

use bytemuck::NoUninit;

use crate::{
	Rect,
	Vec2,
	rect::{AnyRect, OrdRect},
	textures::{AnyColor, Texture, TextureMut},
};

impl<'a, T: AnyColor> TextureMut<'a, T> {
	#[inline]
	pub fn from_data(size: Vec2<i16>, data: &'a mut [T]) -> Self {
		assert!(size.x > 0 && size.y > 0);
		assert!(data.len() == size.x as usize * size.y as usize);
		Self {
			data,
			stride: size.x,
			size,
			clip: Rect::at_origin(size),
		}
	}

	#[inline]
	pub fn from_bytes(size: Vec2<i16>, data: &'a mut [u8]) -> Self
	where
		T: NoUninit,
	{
		assert!(size.x > 0 && size.y > 0);
		assert!(data.len() == size.x as usize * size.y as usize * size_of::<T>());
		Self {
			data: bytemuck::cast_slice_mut(data),
			stride: size.x,
			size,
			clip: Rect::at_origin(size),
		}
	}

	#[inline]
	pub fn as_texture(&'a self) -> Texture<'a, T> {
		Texture {
			data:   &self.data,
			stride: self.size.x,
			size:   self.size,
			clip:   self.clip,
		}
	}

	pub fn view(&'a self, rect: OrdRect<i16>) -> Texture<'a, T> {
		let rect = rect & OrdRect::at_origin(self.size);
		let ord_rect = rect.ordered();

		let start = self.index_of(ord_rect.0);
		let end = self.index_of(rect.1.add_scalar(-1)) + 1;
		Texture {
			data:   &self.data[start..end],
			stride: self.stride,
			size:   rect.size(),
			clip:   (self.clip & rect) - rect.0,
		}
	}

	pub fn view_mut(&'a mut self, rect: OrdRect<i16>) -> TextureMut<'a, T> {
		let rect = rect & OrdRect::at_origin(self.size);
		let ord_rect = rect.ordered();

		let start = self.index_of(ord_rect.0);
		let end = self.index_of(rect.1.add_scalar(-1)) + 1;
		TextureMut {
			data:   &mut self.data[start..end],
			stride: self.stride,
			size:   rect.size(),
			clip:   (self.clip & rect) - rect.0,
		}
	}

	#[inline]
	pub fn reset_clipping(&mut self) { self.clip = Rect::at_origin(self.size) }

	#[inline]
	pub fn clip<const ORD: bool>(&mut self, clip: AnyRect<i16, ORD>) {
		self.clip = (clip & OrdRect::at_origin(self.size)).unordered()
	}

	#[inline]
	pub fn index_of(&self, p: Vec2<i16>) -> usize {
		p.y as usize * self.stride as usize + p.x as usize
	}

	#[inline]
	pub fn get_pixel_at(&self, index: usize) -> T { self.data[index] }
}

impl<'a, T: AnyColor> Debug for TextureMut<'a, T> {
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
