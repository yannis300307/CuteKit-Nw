use core::fmt::Debug;

use crate::{
	AnyTexture,
	ColorGrayscale,
	ColorRGB,
	ColorRGBA,
	Texture,
	color::Blend,
	draw_2d::Pipeline2D,
	drawing::Renderable,
	mods::draw_2d::{
		drawing,
		pipeline::elements::{
			Element2D,
			Element2DColorPrimitive::{
				Circle,
				NinePartsRectangle,
				Rectangle,
				RectangleOutline,
				RoundedRectangle,
				Sprite,
			},
		},
	},
	textures::AnyTextureMut,
};

/// Trait implemented by pipelines
/// to implement specialized implementation for different input types.
pub trait Pipeline<T> {
	fn try_register(&mut self, element: T) -> Result<usize, T>;
	fn register(&mut self, element: T) -> usize
	where
		T: Debug,
	{
		self.try_register(element).unwrap()
	}
}

macro_rules! wrap {
	($($t1: ident)::+ $e: ident) => {
		$($t1)::+($e)
	};
	($($t1: ident)::+ $($e: ident)+) => {
		$($t1)::+(wrap!($($e)+))
	};
}

macro_rules! register {
	[ $($p: ty => $($s: ident)::+),+ $(,)? ] => {
	$(
	   	impl<'a, const SIZE: usize, TXF: AnyTextureMut> Pipeline<$p>
			for Pipeline2D<'a, SIZE, TXF>
			where
				for<'b> ColorRGB: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
				for<'b> ColorRGBA: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
				for<'b> ColorGrayscale: Blend<<TXF::LtAs<'b> as AnyTexture>::Pixel>,
		        {
			fn try_register(
				&mut self,
				element: $p,
			) -> Result<usize, $p> {
				match self.queue.push(wrap!(Element2D::$($s)+ element)) {
					Ok(_) => Ok(self.queue.len() - 1),
					Err(wrap!(Element2D::$($s)+ element)) => Err(element),
					_ => panic!(),
				}
			}
	    }
	)+};
}

macro_rules! register_as_color {
	[ $($($cp: ident)::+ => $($cs: ident)::+),+ $(,)? ] => {
	$(
		register![
     		$($cp)::+<ColorRGB> => RGB::$($cs)::+,
     		$($cp)::+<ColorRGBA> => RGBA::$($cs)::+,
     		$($cp)::+<ColorGrayscale> => Grayscale::$($cs)::+
     	];
	)+};
}

register![
	drawing::DrawNinePartsRectangle<'a, ColorRGB> => RGB::NinePartsRectangle,
	drawing::DrawNinePartsRectangle<'a, ColorRGBA> => RGBA::NinePartsRectangle,
	drawing::DrawNinePartsRectangle<'a, ColorGrayscale> => Grayscale::NinePartsRectangle,

	drawing::DrawSprite<'a, Texture<'a, ColorRGB>> => RGB::Sprite,
	drawing::DrawSprite<'a, Texture<'a, ColorRGBA>> => RGBA::Sprite,
	drawing::DrawSprite<'a, Texture<'a, ColorGrayscale>> => Grayscale::Sprite,

	drawing::DrawText<'a, TXF> => Text,
	&'a mut dyn Renderable<'a, TXF> => Renderable,
];

register_as_color![
	drawing::DrawRectangle => Rectangle,
	drawing::DrawRectangleOutline => RectangleOutline,
	drawing::DrawRoundedRectangle => RoundedRectangle,
	drawing::DrawCircle => Circle,
];
