use core::ops::{AddAssign, SubAssign};

use glam::I16Vec2;

use crate::{
	Rect,
	TextureAtlas,
	color::{AnyColor, Blend},
	draw_2d::{DrawSprite, DrawSpriteTiled},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::{drawing::impl_add_sub, pipeline::elements::NineParts},
	textures::AnyTextureMut,
};

/// Draw operation to render a nine parts rectangle.
///
/// The rendered rectangle will use its edge textures once,
/// and tile the central ones to fit the given area.
///
/// The `rect` field can be unordered.\
/// The `tile_size` field represents the area taken by a single subtexture,
/// negative values will flip the texture on that axis.
///
/// Undesirable results will happen if the rect's size is smaller
/// than the size required to fit each corner of the rectangle (2 * tile_size).
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawNinePartsRectangle<'a, T: AnyColor> {
	pub rect:      Rect,
	pub tile_size: I16Vec2,
	pub atlas:     &'a TextureAtlas<'a, NineParts, T>,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawNinePartsRectangle<'_, T>> for DST {
	fn draw(&mut self, rectangle: &DrawNinePartsRectangle<T>) {
		let rect = rectangle.rect;
		let rect_size = rect.size();
		let tile_size = rectangle.tile_size;

		if rect_size.x == 0 || rect_size.y == 0 {
			return;
		}

		let size = tile_size.abs() * rect_size.signum();

		let start = Rect::new(rect.start, rect.start + size);
		let end = Rect::new(rect.end, rect.end - size);

		let tile = Rect::new(I16Vec2::ZERO, tile_size);
		let cstart = start.ordered_on(tile);
		let cend = end.ordered_on(tile);

		self.draw(&DrawSprite {
			rect:    Rect::pack2(cstart.start.x, cstart.start.y, cstart.end.x, cstart.end.y),
			texture: rectangle.atlas.get(NineParts::TopLeft),
		});
		self.draw(&DrawSpriteTiled {
			rect: Rect::pack2(start.end.x, start.start.y, end.end.x, start.end.y),
			tile,
			texture: rectangle.atlas.get(NineParts::Top),
		});
		self.draw(&DrawSprite {
			rect:    Rect::pack2(cend.end.x, cstart.start.y, cend.start.x, cstart.end.y),
			texture: rectangle.atlas.get(NineParts::TopRight),
		});
		self.draw(&DrawSpriteTiled {
			rect: Rect::pack2(start.start.x, start.end.y, start.end.x, end.end.y),
			tile,
			texture: rectangle.atlas.get(NineParts::Left),
		});
		self.draw(&DrawSpriteTiled {
			rect: Rect::pack2(start.end.x, start.end.y, end.end.x, end.end.y),
			tile,
			texture: rectangle.atlas.get(NineParts::Middle),
		});
		self.draw(&DrawSpriteTiled {
			rect: Rect::pack2(end.end.x, start.end.y, end.start.x, end.end.y),
			tile,
			texture: rectangle.atlas.get(NineParts::Right),
		});
		self.draw(&DrawSprite {
			rect:    Rect::pack2(cstart.start.x, cend.end.y, cstart.end.x, cend.start.y),
			texture: rectangle.atlas.get(NineParts::BottomLeft),
		});
		self.draw(&DrawSpriteTiled {
			rect: Rect::pack2(start.end.x, end.end.y, end.end.x, end.start.y),
			tile,
			texture: rectangle.atlas.get(NineParts::Bottom),
		});
		self.draw(&DrawSprite {
			rect:    Rect::pack2(cend.end.x, cend.end.y, cend.start.x, cend.start.y),
			texture: rectangle.atlas.get(NineParts::BottomRight),
		});
	}
}

impl<T: AnyColor> AddAssign<I16Vec2> for DrawNinePartsRectangle<'_, T> {
	fn add_assign(&mut self, rhs: I16Vec2) { self.rect += rhs; }
}

impl<T: AnyColor> SubAssign<I16Vec2> for DrawNinePartsRectangle<'_, T> {
	fn sub_assign(&mut self, rhs: I16Vec2) { self.rect -= rhs; }
}

impl_add_sub!(DrawNinePartsRectangle<'a, T: AnyColor> for I16Vec2);

impl<T: AnyColor> DrawRenderable for DrawNinePartsRectangle<'_, T> {}
