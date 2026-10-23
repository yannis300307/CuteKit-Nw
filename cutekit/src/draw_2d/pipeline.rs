use core::fmt::Debug;

use crate::{
	Color,
	ColorAlpha,
	OrdRect,
	Vec2,
	color::Blend,
	drawing::{Drawable, DrawableMut},
	mods::draw_2d::elements::Element2D,
	textures::TextureMut,
};

#[derive(Debug)]
pub enum PipelineElement<'a> {
	Opaque(Element2D<'a, Color>),
	Alpha(Element2D<'a, ColorAlpha>),
	Drawable(&'a dyn Drawable<Color>),
	DrawableMut(&'a mut dyn DrawableMut<Color>),
}

#[derive(Debug)]
pub struct Pipeline2D<'a, const SIZE: usize> {
	queue: heapless::Vec<PipelineElement<'a>, SIZE>,
}

impl<'a, const SIZE: usize> Pipeline2D<'a, SIZE> {
	pub fn new() -> Self {
		Self {
			queue: heapless::Vec::new(),
		}
	}

	pub fn clear(&mut self) { self.queue.clear(); }
}

pub trait Pipeline<T: Debug> {
	fn try_register(&mut self, element: T) -> Result<usize, T>;
	fn register(&mut self, element: T) -> usize { self.try_register(element).unwrap() }
}

impl<'a, const SIZE: usize> Pipeline<Element2D<'a, Color>> for Pipeline2D<'a, SIZE> {
	fn try_register(
		&mut self,
		element: Element2D<'a, Color>,
	) -> Result<usize, Element2D<'a, Color>> {
		match self.queue.push(PipelineElement::Opaque(element)) {
			Ok(_) => Ok(self.queue.len() - 1),
			Err(_) => Err(element),
		}
	}
}

impl<'a, const SIZE: usize> Pipeline<Element2D<'a, ColorAlpha>> for Pipeline2D<'a, SIZE> {
	fn try_register(
		&mut self,
		element: Element2D<'a, ColorAlpha>,
	) -> Result<usize, Element2D<'a, ColorAlpha>> {
		match self.queue.push(PipelineElement::Alpha(element)) {
			Ok(_) => Ok(self.queue.len() - 1),
			Err(_) => Err(element),
		}
	}
}

impl<'a, const SIZE: usize> Pipeline<&'a dyn Drawable<Color>> for Pipeline2D<'a, SIZE> {
	fn try_register(
		&mut self,
		element: &'a dyn Drawable<Color>,
	) -> Result<usize, &'a dyn Drawable<Color>> {
		if self.queue.len() == self.queue.capacity() {
			return Err(element);
		}
		self.queue.push(PipelineElement::Drawable(element)).unwrap();
		Ok(self.queue.len() - 1)
	}
}

impl<'a, const SIZE: usize> Pipeline<&'a mut dyn DrawableMut<Color>> for Pipeline2D<'a, SIZE> {
	fn try_register(
		&mut self,
		element: &'a mut dyn DrawableMut<Color>,
	) -> Result<usize, &'a mut dyn DrawableMut<Color>> {
		if self.queue.len() == self.queue.capacity() {
			return Err(element);
		}
		self.queue
			.push(PipelineElement::DrawableMut(element))
			.unwrap();
		Ok(self.queue.len() - 1)
	}
}

impl<'a, const SIZE: usize> DrawableMut<Color> for Pipeline2D<'a, SIZE> {
	fn draw_mut(&mut self, tile: &mut TextureMut<Color>, pos: Vec2<i16>) {
		fn draw_element<T: Blend<Color>>(
			tile: &mut TextureMut<Color>,
			tile_pos: Vec2<i16>,
			element: &Element2D<T>,
		) {
			match element {
				Element2D::Rectangle { rect, value } => {
					tile.draw_rectangle(*rect - tile_pos, *value);
				}
				Element2D::RectangleOutline {
					rect,
					thickness,
					value,
				} => {
					tile.draw_rectangle_outline(*rect - tile_pos, *thickness, *value);
				}
				Element2D::RoundedRectangle {
					rect,
					corner_radius,
					value,
				} => {
					tile.draw_rounded_rectangle(*rect - tile_pos, *corner_radius, *value);
				}
				Element2D::NinePartsRectangle { .. } => {
					todo!();
				}
				Element2D::Circle {
					center,
					radius,
					value,
				} => {
					tile.draw_circle((*center - tile_pos).map(|x| x as f32), *radius, *value);
				}
				Element2D::Sprite { pos, texture } => {
					tile.draw_sprite(
						OrdRect::from_size(*pos - tile_pos, texture.clip.size().abs()),
						texture,
					);
				}
				Element2D::ScaledSprite { rect, texture } => {
					tile.draw_sprite(*rect + texture.clip.0 - tile_pos, texture);
				}
				Element2D::TiledSprite {
					tile_rect,
					rect,
					texture,
				} => {
					tile.draw_sprite_tiled(*tile_rect, *rect - tile_pos, texture);
				}
				Element2D::Text { pos, text, font } => {
					font.draw_string(tile, *pos - tile_pos, text);
				}
			}
		}

		for ele in self.queue.iter_mut() {
			match ele {
				PipelineElement::Opaque(v) => draw_element(tile, pos, v),
				PipelineElement::Alpha(v) => draw_element(tile, pos, v),
				PipelineElement::Drawable(v) => v.draw(tile, pos),
				PipelineElement::DrawableMut(v) => v.draw_mut(tile, pos),
			}
		}
	}
}
