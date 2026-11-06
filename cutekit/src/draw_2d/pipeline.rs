use crate::{
	AnyTexture,
	ColorGrayscale,
	ColorRGB,
	ColorRGBA,
	Texture,
	TextureMut,
	Vec2,
	color::Blend,
	draw_2d::{
		DrawCircle,
		DrawNinePartsRectangle,
		DrawRectangle,
		DrawRectangleOutline,
		DrawRoundedRectangle,
		DrawSprite,
		DrawText,
	},
	drawing::{Draw, Renderable},
	mods::draw_2d::pipeline::elements::{Element2D, Element2DColorPrimitive},
	textures::AnyTextureMut,
};

pub mod elements;
pub mod registry;

/// Pipeline object used to dynamically store drawable elements.
///
/// This pipeline is also able to store commonly used 2D drawables
/// without using dynamic references, helping with cache locality and performance.
/// See `draw_2d::Element2D` for the list of in-place drawables.
///
/// The constant `SIZE` generic corresponds to
/// the maximum amount of drawables the pipeline can store.
pub struct Pipeline2D<'a, const SIZE: usize, TXF: AnyTextureMut = TextureMut<'a, ColorRGB>>
where
	for<'b> ColorRGB: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
	for<'b> ColorRGBA: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
	for<'b> ColorGrayscale: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
{
	queue: heapless::Vec<Element2D<'a, TXF>, SIZE>,
}

impl<'a, const SIZE: usize, TXF: AnyTextureMut> Pipeline2D<'a, SIZE, TXF>
where
	for<'b> ColorRGB: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
	for<'b> ColorRGBA: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
	for<'b> ColorGrayscale: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
{
	/// Creates a new pipeline.
	pub fn new() -> Self {
		Self {
			queue: heapless::Vec::new(),
		}
	}
}

impl<const SIZE: usize, TXF: AnyTextureMut> Renderable<'_, TXF> for Pipeline2D<'_, SIZE, TXF>
where
	for<'b> ColorRGB: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
	for<'b> ColorRGBA: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
	for<'b> ColorGrayscale: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
{
	fn render(&mut self, tile: &mut TXF::LtAs<'_>, pos: Vec2<i16>) {
		fn draw_element<T: Blend<ColorRGB>, TXF: AnyTextureMut>(
			tile: &mut TXF::LtAs<'_>,
			pos: Vec2<i16>,
			element: &mut Element2DColorPrimitive<T>,
		) where
			for<'b> TXF::LtAs<'b>: Draw<DrawRectangle<T>>
				+ Draw<DrawRectangleOutline<T>>
				+ Draw<DrawRoundedRectangle<T>>
				+ Draw<DrawCircle<T>>,
			for<'b> T: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
		{
			match element {
				Element2DColorPrimitive::Rectangle(v) => {
					<DrawRectangle<T> as Renderable<TXF>>::render(v, tile, pos)
				}
				Element2DColorPrimitive::RectangleOutline(v) => {
					<DrawRectangleOutline<T> as Renderable<TXF>>::render(v, tile, pos)
				}
				Element2DColorPrimitive::RoundedRectangle(v) => {
					<DrawRoundedRectangle<T> as Renderable<TXF>>::render(v, tile, pos)
				}
				Element2DColorPrimitive::Circle(v) => {
					<DrawCircle<T> as Renderable<TXF>>::render(v, tile, pos)
				}
				Element2DColorPrimitive::NinePartsRectangle(v) => {
					<DrawNinePartsRectangle<T> as Renderable<TXF>>::render(v, tile, pos)
				}
				Element2DColorPrimitive::Sprite(v) => {
					<DrawSprite<Texture<T>> as Renderable<TXF>>::render(v, tile, pos)
				}
			}
		}

		for ele in self.queue.iter_mut() {
			match ele {
				Element2D::RGB(v) => draw_element::<ColorRGB, TXF>(tile, pos, v),
				Element2D::RGBA(v) => draw_element::<ColorRGBA, TXF>(tile, pos, v),
				Element2D::Grayscale(v) => draw_element::<ColorGrayscale, TXF>(tile, pos, v),
				Element2D::Text(v) => DrawText::render(v, tile, pos),
				Element2D::Renderable(v) => v.render(tile, pos),
			}
		}
	}
}
