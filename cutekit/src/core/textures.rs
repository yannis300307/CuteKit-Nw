use core::fmt::Debug;

use bytemuck::AnyBitPattern;
use enum_map::{Enum, EnumMap};

use crate::{Color, ColorAlpha, Vec2, rect::Rect};

pub trait AnyColor: AnyBitPattern + Debug {}
impl<T: AnyBitPattern + Debug> AnyColor for T {}

#[readonly::make]
#[derive(Clone, Copy)]
pub struct Texture<'a, T: AnyColor = Color> {
	pub(crate) data:   &'a [T],
	pub(crate) stride: i16,
	pub size:          Vec2<i16>,
	pub clip:          Rect<i16>,
}

#[readonly::make]
#[derive()]
pub struct TextureMut<'a, T: AnyColor = Color> {
	pub(crate) data:   &'a mut [T],
	#[readonly]
	pub(crate) stride: i16,
	#[readonly]
	pub size:          Vec2<i16>,
	#[readonly]
	pub clip:          Rect<i16>,
}

pub type TextureAlpha<'a> = Texture<'a, ColorAlpha>;
pub type TextureMutAlpha<'a> = TextureMut<'a, ColorAlpha>;

#[derive(Debug)]
pub struct TextureAtlas<'a, K: Enum, T: AnyColor = Color>(EnumMap<K, Texture<'a, T>>);

pub mod atlas;
pub mod mut_texture;
pub mod texture;
