use bytemuck::Pod;

use crate::{Color, Rect, Vec2, texture::Texture};

/// A scaling mode enum used in elements that requiere a scalling strategy.
/// `Stretch` will distort the texture to match the size of the area
/// that needs to be drawn while `tile` will repeat the texture without scaling it.
#[derive(Clone, Copy, Debug)]
pub enum ScaleMode {
	Stretch,
	Tile,
}

#[derive(Clone, Debug)]
pub enum Charset {
	AlphaNumeric,
}

/// The font is used for the text rendering.
/// The grid_size must be in characters (not pixels).
/// The chars string stores the available characters in this font.
#[derive(Clone, Debug)]
pub struct Font<T = Color>
where
	T: Pod,
{
	pub texture: Texture<T>,
	pub charset: Charset,
}

#[derive(Clone, Debug)]
pub enum Element2D<'a, T = Color>
where
	T: Pod,
{
	/// A flat colored rectangle. Very fast to draw.
	Rectangle { rect: Rect<i16>, value: T },
	/// A rectangle outline. Very fast to draw. The thickness will expand to the inside.
	RectangleOutline {
		rect:      Rect<i16>,
		thickness: i16,
		value:     T,
	},
	/// A flat color rounded corner rectangle.
	/// Quite fast to draw.
	RoundedRectangle {
		rect:          Rect<i16>,
		corner_radius: f32,
		value:         T,
	},
	/// A textured rectangle drawn using 9 parts of a textures.
	/// One for each corners, sides and one for the center part of the rectangle.
	/// The goal of this element is to have the same flexibility has a regular
	/// rect but with the advantages of a scaled sprite.
	/// Given the nine parts and the other arguments, the renderer will
	/// automatically adapt the image by properly scalling each part.
	/// This element is quite slow to draw.
	NinePartsRectangle {
		texture:      &'a Texture<T>,
		rect:         Rect<i16>,
		scaling_mode: ScaleMode,
	},
	/// A simple flat color circle. Very fast to draw.
	Circle {
		center: Vec2<i16>,
		radius: f32,
		value:  T,
	},
	/// A textured non-scaled rectangle. Also known as a sprite.
	/// Use ScaledSprite to change the scaling of the texture.
	/// Quite Fast to draw.
	Sprite {
		pos:     Vec2<i16>,
		texture: &'a Texture<T>,
	},
	/// A textured scaled rectangle. Also known as a sprite.
	/// Use ScaledSprite to change the scaling of the texture.
	/// Slower than Sprite.
	ScaledSprite {
		texture:    &'a Texture<T>,
		rect:       Rect<i16>,
		scale_mode: ScaleMode,
	},
	/// A simple colored text label. The text is drawn using the given font object.
	/// Setting the background color to None will make it transparent.
	/// Quite slow to draw
	Text {
		pos:              Vec2<i16>,
		text:             &'a str,
		font:             &'a Font,
		font_value:       T,
		background_value: T,
	},
}
