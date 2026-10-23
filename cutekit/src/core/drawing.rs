use core::fmt::Debug;

use crate::{
	Color,
	Vec2,
	textures::{AnyColor, TextureMut},
};

pub mod circle;
pub mod line;
pub mod pixel;
pub mod rectangle;
pub mod sprite;

pub trait DrawableMut<T: AnyColor = Color>: Debug {
	fn draw_mut(&mut self, tile: &mut TextureMut<T>, pos: Vec2<i16>);
}

pub trait Drawable<T: AnyColor = Color>: Debug {
	fn draw(&self, tile: &mut TextureMut<T>, pos: Vec2<i16>);
}

impl<D: Drawable<T>, T: AnyColor> DrawableMut<T> for D {
	fn draw_mut(&mut self, tile: &mut TextureMut<T>, pos: Vec2<i16>) { self.draw(tile, pos); }
}

pub trait TextureSampler<T: AnyColor, U: AnyColor>: Fn(T, (f32, f32)) -> U {}
impl<T: AnyColor, U: AnyColor, F: Fn(T, (f32, f32)) -> U> TextureSampler<T, U> for F {}
