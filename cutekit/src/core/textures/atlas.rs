use enum_map::{Enum, EnumMap};

use crate::{
	OrdRect,
	Vec2,
	textures::{AnyColor, Texture, TextureAtlas},
};

impl<'a, K: Enum, T: AnyColor> TextureAtlas<'a, K, T> {
	pub fn from_map(textures: EnumMap<K, Texture<'a, T>>) -> Self { Self(textures) }

	pub fn get(&self, key: K) -> &Texture<'a, T> { &self.0[key] }

	pub fn get_mut(&mut self, key: K) -> &mut Texture<'a, T> { &mut self.0[key] }

	pub fn set(&mut self, key: K, texture: Texture<'a, T>) { self.0[key] = texture; }

	pub fn enum_map(&mut self) -> &mut EnumMap<K, Texture<'a, T>> { &mut self.0 }
}

impl<'a, T: AnyColor> Texture<'a, T> {
	pub fn subdivide<const N: usize>(
		&'a self,
		subtexture_size: Vec2<i16>,
		array: &mut [Texture<'a, T>; N],
	) {
		assert!(subtexture_size.x > 0 && subtexture_size.y > 0);

		let size = self.clip.size();
		let divs = size.abs().component_div(&subtexture_size);

		assert!(size.x % subtexture_size.x == 0 && size.y % subtexture_size.y == 0);
		assert!(divs.product() as usize == N);

		let step = subtexture_size.component_mul(&size.map(|x| x.signum()));

		let mut i = 0;
		let mut pos = self.clip.0;
		for _ in 0..divs.y {
			pos.x = self.clip.0.x;
			for _ in 0..divs.x {
				array[i] = self.view(OrdRect::from_size(pos, subtexture_size));
				pos.x += step.x;
				i += 1;
			}
			pos.y += step.y;
		}
	}
}
