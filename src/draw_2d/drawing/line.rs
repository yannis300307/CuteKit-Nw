use core::ops::{AddAssign, SubAssign};

use glam::I16Vec2;

use crate::{
	color::{AnyColor, Blend},
	drawing::{Draw, DrawRenderable},
	mods::draw_2d::drawing::impl_add_sub,
	textures::AnyTextureMut,
};

/// Draw operation to render a flat colored horizontal line.
///
/// Commonly used as helper for other draw operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawHorizontalLine<T: AnyColor> {
	pub start_x: i16,
	pub end_x:   i16,
	pub y:       i16,
	pub color:   T,
}

impl<DST: AnyTextureMut, T: Blend<DST::Pixel>> Draw<DrawHorizontalLine<T>> for DST {
	fn draw(&mut self, line: &DrawHorizontalLine<T>) {
		let clip = self.get_clip();

		if line.y < clip.start.y || line.y >= clip.end.y {
			return;
		}
		let start_x = line.start_x.max(clip.start.x);
		let end_x = line.end_x.min(clip.end.x);
		if start_x > end_x {
			return;
		}

		let row = (line.y * self.stride()) as usize;
		let from = row + start_x as usize;
		let to = row + end_x as usize;

		for p in from..to {
			self.draw_pixel_at(p, line.color);
		}
	}
}

impl<T: AnyColor> AddAssign<I16Vec2> for DrawHorizontalLine<T> {
	fn add_assign(&mut self, rhs: I16Vec2) {
		self.start_x += rhs.x;
		self.end_x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T: AnyColor> SubAssign<I16Vec2> for DrawHorizontalLine<T> {
	fn sub_assign(&mut self, rhs: I16Vec2) {
		self.start_x -= rhs.x;
		self.end_x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl_add_sub!(DrawHorizontalLine<T: AnyColor> for I16Vec2);

impl<T: AnyColor> DrawRenderable for DrawHorizontalLine<T> {}
