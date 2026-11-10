use enum_map::{Enum, EnumMap};

use crate::{
	textures::{AnyColor, Texture, TextureAtlas},
	Rect,
	Vec2,
};

impl<'a, K: Enum, T: AnyColor> TextureAtlas<'a, K, T> {
	/// Creates a new texture atlas from the given array.
	///
	/// The array should be ordered like the the atlas enum's variants.
	pub fn from_array(textures: K::Array<Texture<'a, T>>) -> Self {
		Self(EnumMap::from_array(textures))
	}

	/// Creates a new texture atlas from the given enum map.
	pub fn from_map(textures: EnumMap<K, Texture<'a, T>>) -> Self { Self(textures) }

	/// Gets the texture that matches the given variant.
	pub fn get(&self, key: K) -> &Texture<'a, T> { &self.0[key] }

	/// Gets the texture that matches the given variant as mutable.
	pub fn get_mut(&mut self, key: K) -> &mut Texture<'a, T> { &mut self.0[key] }

	/// Sets the texture that matches the given variant to the given texture.
	pub fn set(&mut self, key: K, texture: Texture<'a, T>) { self.0[key] = texture; }

	/// Gets the atlas's internal enum map.
	pub fn enum_map(&mut self) -> &mut EnumMap<K, Texture<'a, T>> { &mut self.0 }

	/// Gets the atlas's internal textures as a slice.
	pub fn as_slice(&mut self) -> &[Texture<'a, T>] { self.0.as_slice() }

	/// Gets the atlas's internal textures as a mutable slice.
	pub fn as_mut_slice(&mut self) -> &mut [Texture<'a, T>] { self.0.as_mut_slice() }
}

impl<'a, T: AnyColor> Texture<'a, T> {
	/// Fills the given slice with rectangular regions of this texture's clip.
	///
	/// The slice's size must exactly match the total amount of subdivisions.
	/// This texture's clip size must be divisible by the amount of subdivisions.
	pub fn subdivide<const N: usize>(
		&'a self,
		subdivisions: Vec2<i16>,
		array: &mut [Texture<'a, T>; N],
	) {
		assert!(subdivisions.x > 0 && subdivisions.y > 0);
		assert!(subdivisions.product() as usize == N);

		let size = self.clip.size();

		assert!(size.x % subdivisions.x == 0 && size.y % subdivisions.y == 0);

		let subtexture_size = size.component_div(&subdivisions);
		let step = subtexture_size.component_mul(&size.map(|x| x.signum()));

		let mut i = 0;
		let mut pos = self.clip.start;
		for _ in 0..subdivisions.y {
			pos.x = self.clip.start.x;
			for _ in 0..subdivisions.x {
				array[i] = self.view(Rect::from_size(pos, subtexture_size));
				pos.x += step.x;
				i += 1;
			}
			pos.y += step.y;
		}
	}
}
