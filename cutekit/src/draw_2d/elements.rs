use core::fmt::Debug;

use enum_map::Enum;

use crate::{
	Color,
	OrdRect,
	Rect,
	TextureAtlas,
	Vec2,
	mods::draw_2d::text::AnyFont,
	textures::{AnyColor, Texture},
};

#[derive(Clone, Copy, Debug, Enum)]
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

#[derive(Clone, Copy, Debug)]
pub enum Element2D<'a, T: AnyColor = Color> {
	Rectangle {
		rect:  Rect<i16>,
		value: T,
	},
	RectangleOutline {
		rect:      Rect<i16>,
		thickness: i16,
		value:     T,
	},
	RoundedRectangle {
		rect:          Rect<i16>,
		corner_radius: f32,
		value:         T,
	},
	NinePartsRectangle {
		texture: &'a TextureAtlas<'a, NineParts, T>,
	},
	Circle {
		center: Vec2<i16>,
		radius: f32,
		value:  T,
	},
	Sprite {
		pos:     Vec2<i16>,
		texture: &'a Texture<'a, T>,
	},
	ScaledSprite {
		rect:    OrdRect<i16>,
		texture: &'a Texture<'a, T>,
	},
	TiledSprite {
		tile_rect: OrdRect<i16>,
		rect:      OrdRect<i16>,
		texture:   &'a Texture<'a, T>,
	},
	Text {
		pos:  Vec2<i16>,
		text: &'a str,
		font: &'a dyn AnyFont<Color>,
	},
}
