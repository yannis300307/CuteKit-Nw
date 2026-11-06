#![cfg_attr(target_os = "none", no_std)]
#![no_main]

use core::array;

use cutekit as kit;
use kit::prelude::*;

configure_app!(b"CuteKitTest\0", "../target/assets/icon.nwi");

const TEXTURE: Texture<ColorRGBA> = include_texture!("../assets/.kibble/9parts.kib");
const FONT: Texture<ColorGrayscale> = include_texture!("../assets/.kibble/font.kib");

#[unsafe(no_mangle)]
fn main() {
	kit::init();

	let mut font_textures = array::repeat::<Texture<ColorGrayscale>, 95>(FONT);
	FONT.subdivide(Vec2::new(95, 1), &mut font_textures);

	let font = ColorFont::<Ascii>::from_fmt(
		-1,
		16,
		ColorRGBA::unpack(0xFF00FFFF),
		ColorRGBA::unpack(0xFF000000),
		" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
		font_textures[0],
		&font_textures,
		EnumMap::from_fn(|_| Vec2::zeros()),
	);

	let mut texture = TEXTURE.view(TEXTURE.get_clip().ordered());
	texture.clip(Rect::from_scalars(15, 15, 45, 45));

	let mut atlas_textures =
		array::repeat::<Texture<ColorRGBA>, 9>(Texture::from_data_const(0, 0, &[]));
	texture.subdivide(Vec2::new(3, 3), &mut atlas_textures);
	let atlas = TextureAtlas::from_array(atlas_textures);
	// atlas
	// 	.get_mut(NineParts::Right)
	// 	.clip(Rect::from_scalars(5, 5, 15, 15));

	let mut l = Vec2::new(50, 50);
	let mut sl = Vec2::new(50, 50);
	let mut r = Vec2::new(20, 20);
	let mut sr = Vec2::new(50, 50);

	loop {
		kit::update();

		let shift = kit::is_key_down(Key::Exp) as usize;
		let keysets = [
			[Key::Up, Key::Down, Key::Left, Key::Right],
			[Key::Toolbox, Key::Comma, Key::Imaginary, Key::Power],
		];
		let values = [[&mut r, &mut l], [&mut sr, &mut sl]];
		for (i, keys) in keysets.iter().enumerate() {
			if kit::is_key_down(keys[0]) {
				values[shift][i].y -= 1;
			}
			if kit::is_key_down(keys[1]) {
				values[shift][i].y += 1;
			}
			if kit::is_key_down(keys[2]) {
				values[shift][i].x -= 1;
			}
			if kit::is_key_down(keys[3]) {
				values[shift][i].x += 1;
			}
		}

		let mut render = Pipeline2D::<5>::new();

		render.register(DrawRectangleOutline {
			rect:      Rect::from_size(sl, l),
			thickness: -3,
			color:     RGB_WHITE,
		});

		render.register(DrawNinePartsRectangle {
			rect:      Rect::from_size(sl, l),
			tile_size: r,
			atlas:     &atlas,
		});

		let fps = heapless::format!(8; "{:.2}", kit::fps()).unwrap();
		kit::render(
			RGB_PURPLE,
			&mut [
				&mut render,
				&mut DrawText {
					pos:  Vec2::new(50, 160),
					text: fps.as_str(),
					font: &font,
				},
			],
		);
		drop(render);
	}
}
