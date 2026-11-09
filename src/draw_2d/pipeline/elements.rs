use core::{any, fmt::Debug};

use enum_map::Enum;

use crate::{
	ColorGrayscale,
	ColorRGB,
	ColorRGBA,
	Texture,
	TextureMut,
	color::AnyColor,
	draw_2d::DrawNinePartsRectangle,
	drawing::Renderable,
	mods::draw_2d::drawing::{
		circle::DrawCircle,
		rectangle::DrawRectangle,
		rectangle_outline::DrawRectangleOutline,
		rounded_rectangle::DrawRoundedRectangle,
		sprite::DrawSprite,
		text::DrawText,
	},
	textures::AnyTextureMut,
};

/// Enum key for texture atlases used by `DrawNinePartsRectangle`.
#[derive(Clone, Copy, Debug, PartialEq, Enum)]
pub enum NineParts {
	TopLeft,
	Top,
	TopRight,
	Left,
	Middle,
	Right,
	BottomLeft,
	Bottom,
	BottomRight,
}

/// Enum used by Element2D to store in-place drawable elements
/// for the specified generic color.
///
/// Each field stores a struct with the same name preceded by "Draw"
/// (eg. `Rectangle` stores a `DrawRectangle` object).
#[derive(Clone, Copy, Debug)]
pub enum Element2DColorPrimitive<'a, T: AnyColor> {
	Rectangle(DrawRectangle<T>),
	RectangleOutline(DrawRectangleOutline<T>),
	RoundedRectangle(DrawRoundedRectangle<T>),
	Circle(DrawCircle<T>),
	NinePartsRectangle(DrawNinePartsRectangle<'a, T>),
	Sprite(DrawSprite<'a, Texture<'a, T>>),
}

/// Enum used by Pipeline 2D to store drawables in-place.
///
/// - `RGB`, `RGBA` and `Grayscale` fields store drawables using their respective color type.
/// - `Text` stores `DrawText` elements.
/// - `Renderable` can store any dynamic reference to a `Renderable` trait object.
pub enum Element2D<'a, TXF: AnyTextureMut = TextureMut<'a, ColorRGB>> {
	RGB(Element2DColorPrimitive<'a, ColorRGB>),
	RGBA(Element2DColorPrimitive<'a, ColorRGBA>),
	Grayscale(Element2DColorPrimitive<'a, ColorGrayscale>),
	Text(DrawText<'a, TXF>),
	Renderable(&'a mut dyn Renderable<'a, TXF>),
}

impl<'a, TXF: AnyTextureMut> Debug for Element2D<'a, TXF> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Renderable(_) => write!(
				f,
				"Element2D {{ dyn Renderable <{:?}> }}",
				any::type_name::<TXF>()
			),
			a => write!(f, "Element2D {{ {:?} }}", a),
		}
	}
}
