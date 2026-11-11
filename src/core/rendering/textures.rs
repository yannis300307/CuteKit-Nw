use core::{fmt::Debug, marker::PhantomData};

use enum_map::{Enum, EnumMap};
use glam::{I16Vec2, Vec2};

use crate::{
	ColorRGB,
	Rect,
	color::{AnyColor, Blend},
};

/// Trait implemented by 2D textures.
pub trait AnyTexture: Debug {
	/// Type stored inside of the texture.
	type Pixel: AnyColor;

	/// Gets the size of this texture.
	fn size(&self) -> I16Vec2;

	/// Gets the stride of this texture,
	/// the amount of elements needed to go down one row when using indexes.
	fn stride(&self) -> i16;

	/// Gets the clip area of this texture.
	///
	/// Also known as the scissoring area,
	/// pixels outside of the clip should not be accessible by drawing procedures.
	fn get_clip(&self) -> Rect;

	/// Sets the clip area of this texture.
	///
	/// Also known as the scissoring area,
	/// pixels outside of the clip should not be accessible by drawing procedures.
	///
	/// The given rect may be unordered,
	/// in which case the texture should flip when used as source by drawing procedures.
	/// If the given rect is unordered,
	/// the texture should not be used as destination by drawing procedures.
	fn clip(&mut self, rect: Rect);

	/// Returns the one-dimensional index of the pixel
	/// at the given position in this texture.
	fn index_of(&self, p: I16Vec2) -> usize;

	/// Returns the value of the pixel at the given index in this texture.
	///
	/// This method doesn't check if this texture contains a pixel at the given index.
	fn get_pixel_at(&self, index: usize) -> Self::Pixel;
}

/// Trait implemented by mutable 2D textures.
pub trait AnyTextureMut: AnyTexture {
	/// Representation of this texture with the given lifetime.
	type LtAs<'b>: AnyTextureMut;

	/// Sets the value of the pixel at the given index in this texture.
	///
	/// This method doesn't check if this texture contains a pixel at the given index
	/// and will set the value without blending it.
	fn set_pixel_at(&mut self, index: usize, val: Self::Pixel);

	/// Sets the value of the pixel at the given index in this texture,
	/// respecting color blending.
	///
	/// This method doesn't check if this texture contains a pixel at the given index.
	fn draw_pixel_at<T: Blend<Self::Pixel>>(&mut self, index: usize, val: T) {
		self.set_pixel_at(index, T::blend(self.get_pixel_at(index), val));
	}
}

/// A basic, read-only 2D texture.
#[derive(Clone, Copy, PartialEq)]
pub struct Texture<'a, T: AnyColor = ColorRGB> {
	data:   &'a [T],
	stride: i16,
	size:   I16Vec2,
	clip:   Rect,
}

/// A basic, mutable 2D texture.
#[derive(PartialEq)]
pub struct TextureMut<'a, T: AnyColor = ColorRGB> {
	data:   &'a mut [T],
	stride: i16,
	size:   I16Vec2,
	clip:   Rect,
}

/// Shader implementation for textures.
///
/// Short for `Fn(&Texture<T1>, usize, Vec2<f32>) -> T2`.
pub trait SamplerFn<T: AnyColor, U: AnyColor>: Fn(&Texture<T>, usize, Vec2) -> U {}
impl<T: AnyColor, U: AnyColor, F: Fn(&Texture<T>, usize, Vec2) -> U> SamplerFn<T, U> for F {}

/// Texture with a custom sampler function.
///
/// Can be used to have basic shaders on some textures.
#[derive(Clone, Copy)]
pub struct SamplerTexture<'a, S: SamplerFn<T, U>, T: AnyColor = ColorRGB, U: AnyColor = T> {
	_out:        PhantomData<U>,
	pub texture: Texture<'a, T>,
	pub sampler: S,
}

/// Collection of textures based on an enum.
///
/// The enum must implement the `Enum` trait, which can be derived.
#[derive(Debug, PartialEq)]
pub struct TextureAtlas<'a, K: Enum, T: AnyColor = ColorRGB>(EnumMap<K, Texture<'a, T>>);

pub mod atlas;
pub mod mut_texture;
pub mod sampler_texture;
pub mod texture;
