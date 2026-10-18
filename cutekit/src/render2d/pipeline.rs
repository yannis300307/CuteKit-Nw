use heapless::Vec;

use crate::{
	Vec2,
	drawing::{Drawable, Pipeline, TileBuffer},
	render2d::elements::Element2D,
};

#[derive(Clone, Debug)]
pub struct Pipeline2D<'a, const SIZE: usize> {
	queue: Vec<Element2D<'a>, SIZE>,
}

impl<'a, const SIZE: usize> Pipeline2D<'a, SIZE> {
	pub fn new() -> Self { Self { queue: Vec::new() } }
}

impl<'a, const SIZE: usize> Pipeline for Pipeline2D<'a, SIZE> {
	type Element = Element2D<'a>;

	#[allow(unused_must_use)]
	fn register(&mut self, element: Self::Element) -> Result<usize, Self::Element> {
		self.queue.push(element)?;
		Ok(self.queue.len() - 1)
	}

	fn clear(&mut self) { self.queue.clear(); }
}

impl<'a, const SIZE: usize> Drawable for Pipeline2D<'a, SIZE> {
	fn draw(&mut self, tile: &mut TileBuffer, pos: Vec2<i16>) {
		for ele in self.queue.iter_mut() {
			match ele {
				Element2D::Rectangle { rect, value } => {
					tile.draw_rectangle::<true>(*rect - pos, *value);
				}
				Element2D::RectangleOutline {
					rect,
					thickness,
					value,
				} => {
					tile.draw_rectangle_outline::<true>(*rect - pos, *thickness, *value);
				}
				Element2D::RoundedRectangle {
					rect,
					corner_radius,
					value,
				} => {
					tile.draw_rounded_rectangle::<true>(*rect - pos, *corner_radius, *value);
				}
				Element2D::NinePartsRectangle {
					texture,
					rect,
					scaling_mode,
				} => {
					todo!();
				}
				Element2D::Circle {
					center,
					radius,
					value,
				} => {
					tile.draw_circle::<true>((*center - pos).map(|x| x as f32), *radius, *value);
				}
				Element2D::Sprite { pos: tx_pos, texture } => {
					tile.draw_texture(*tx_pos - pos, texture);
				}
				Element2D::ScaledSprite {
					rect,
					texture,
					scale_mode,
				} => {
					todo!();
				}
				Element2D::Text {
					pos,
					text,
					font,
					font_value,
					background_value,
				} => {
					todo!();
				}
			}
		}
	}
}
