use core::ops::{AddAssign, SubAssign};

use crate::{
	Vec2,
	drawing::{Draw, Renderable},
	mods::draw_2d::{drawing::impl_add_sub, font::AnyFont},
	textures::AnyTextureMut,
};

/// Draw operation to render text.
///
/// The font can be any type implementing the `AnyFont` trait,
/// see `draw_2d::ColoredFont` for a default implementation.
///
/// This struct can be given to `Pipeline2D`'s `register` method.
#[derive(Clone, Copy, Debug)]
pub struct DrawText<'a, TXF: AnyTextureMut> {
	pub pos:  Vec2<i16>,
	pub text: &'a str,
	pub font: &'a dyn AnyFont<TXF>,
}

impl<'a, TXF: for<'b> AnyTextureMut<LtAs<'b> = DST>, DST: AnyTextureMut> Draw<DrawText<'a, TXF>>
	for DST
{
	fn draw(&mut self, text: &DrawText<TXF>) {
		text.font.draw_string(self, text.pos, text.text);
	}
}

impl<'a, TXF: AnyTextureMut> Renderable<'_, TXF> for DrawText<'a, TXF> {
	fn render(&mut self, tile: &mut TXF::LtAs<'_>, pos: Vec2<i16>) {
		self.font.draw_string(tile, self.pos - pos, self.text);
	}
}

impl<TX: AnyTextureMut> AddAssign<Vec2<i16>> for DrawText<'_, TX> {
	fn add_assign(&mut self, rhs: Vec2<i16>) { self.pos += rhs; }
}

impl<TX: AnyTextureMut> SubAssign<Vec2<i16>> for DrawText<'_, TX> {
	fn sub_assign(&mut self, rhs: Vec2<i16>) { self.pos -= rhs; }
}

impl_add_sub!(DrawText<'a, TX: AnyTextureMut> for Vec2<i16>);
