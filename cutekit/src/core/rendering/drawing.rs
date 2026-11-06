use core::ops::Sub;

use crate::{
	color::AnyColor,
	textures::{AnyTexture, AnyTextureMut, SamplerFn, SamplerTexture},
	ColorRGB,
	Texture,
	TextureMut,
	Vec2,
};

/// Trait implemented by objects that can be rendered on a mutable texture.
pub trait Renderable<'a, TXF: AnyTextureMut = TextureMut<'a, ColorRGB>> {
	/// Draws this object on the given texture, which is offset by the given vector.
	fn render(&mut self, tile: &mut TXF::LtAs<'_>, pos: Vec2<i16>);
}

/// Trait implemented by mutable textures that can draw a specific element.
///
/// Usually used in pair with `DrawRenderable` to implement the `Renderable` trait.
pub trait Draw<T>: AnyTextureMut {
	fn draw(&mut self, element: &T);
}

/// Trait that allows objects that can be drawn with `Draw`
/// to implement the `Renderable` trait.
pub trait DrawRenderable: Copy + Sub<Vec2<i16>, Output = Self> {}

impl<'a, T: DrawRenderable, TXF: AnyTextureMut> Renderable<'a, TXF> for T
where
	for<'b> TXF::LtAs<'b>: Draw<T>,
{
	default fn render(&mut self, tile: &mut TXF::LtAs<'_>, pos: Vec2<i16>) {
		tile.draw(&(*self - pos));
	}
}

/// Trait implemented by drawable textures
pub trait Sampler {
	type SampleColor: AnyColor;

	fn sample_pixel(&self, index: usize, uv: Vec2<f32>) -> Self::SampleColor;
}

impl<T: AnyColor> Sampler for Texture<'_, T> {
	type SampleColor = T;

	#[inline]
	fn sample_pixel(&self, index: usize, _: Vec2<f32>) -> T { self.get_pixel_at(index) }
}

impl<S: SamplerFn<T, U>, T: AnyColor, U: AnyColor> Sampler for SamplerTexture<'_, S, T, U> {
	type SampleColor = U;

	#[inline]
	fn sample_pixel(&self, index: usize, uv: Vec2<f32>) -> U {
		self.sampler.call((&self.texture, index, uv))
	}
}
