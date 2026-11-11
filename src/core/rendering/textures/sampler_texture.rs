use core::{any, fmt::Debug, marker::PhantomData};

use glam::I16Vec2;

use crate::{
	Rect,
	Texture,
	color::AnyColor,
	textures::{AnyTexture, SamplerFn, SamplerTexture},
};

impl<'a, S: SamplerFn<T, U>, T: AnyColor, U: AnyColor> SamplerTexture<'a, S, T, U> {
	/// Creates a new sampler texture from the given texture and a sampler function.
	pub const fn from_texture(texture: Texture<'a, T>, sampler: S) -> Self {
		Self {
			_out: PhantomData,
			texture,
			sampler,
		}
	}
}

impl<'a, T: AnyColor> Texture<'a, T> {
	/// Creates and returns a new sampler texture
	/// from this texture and the given sampler function.
	pub const fn with_sampler<S: SamplerFn<T, U>, U: AnyColor>(
		self,
		sampler: S,
	) -> SamplerTexture<'a, S, T, U> {
		SamplerTexture::from_texture(self, sampler)
	}
}

impl<'a, S: SamplerFn<T, U>, T: AnyColor, U: AnyColor> AnyTexture
	for SamplerTexture<'a, S, T, U>
{
	//type LtAs<'b> = SamplerTexture<'b, S, T, U>;
	type Pixel = T;

	#[inline]
	fn size(&self) -> I16Vec2 { self.texture.size() }

	#[inline]
	fn stride(&self) -> i16 { self.texture.stride() }

	#[inline]
	fn get_clip(&self) -> Rect { self.texture.get_clip() }

	#[inline]
	fn clip(&mut self, rect: Rect) { self.texture.clip(rect); }

	#[inline]
	fn index_of(&self, p: I16Vec2) -> usize {
		p.y as usize * self.texture.stride as usize + p.x as usize
	}

	#[inline]
	fn get_pixel_at(&self, index: usize) -> T { self.texture.data[index] }
}

impl<'a, S: SamplerFn<T, U>, T: AnyColor, U: AnyColor> Debug for SamplerTexture<'a, S, T, U> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"SamplerTexture {{ {:?} ({:?}): {}x{}; stride: {}, clip: {:?} }}",
			any::type_name::<T>(),
			any::type_name::<U>(),
			self.texture.size.x,
			self.texture.size.y,
			self.texture.stride,
			self.texture.clip
		)
	}
}
