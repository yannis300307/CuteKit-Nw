#![cfg_attr(target_os = "none", no_std)]
#![no_main]

use core::array;

use cutekit::{
	COLOR_BLACK,
	COLOR_BLUE,
	COLOR_WHITE,
	Color,
	ColorAlpha,
	ColorGrayscale,
	Key,
	OrdRect,
	Rect,
	Texture,
	TextureAlpha,
	Vec2,
	draw_2d::{Ascii, ColorFont, Element2D, Pipeline, Pipeline2D},
	setup,
};
use include_bytes_aligned::include_bytes_aligned;
mod cutekit_main;

setup!(b"CuteKitTest\0", 12, "../target/assets/icon.nwi", 3437);

static TEXTURE: &[u8] = include_bytes_aligned!(4, "../target/assets/9parts.bin");
static FONT: &[u8] = include_bytes_aligned!(4, "../target/assets/font.bin");

#[unsafe(no_mangle)]
fn main() {
	let mut ck = cutekit::init();

	let font_texture = Texture::<ColorGrayscale>::from_bytes(Vec2::new(1235, 16), FONT);
	let mut font_textures = array::repeat::<Texture<ColorGrayscale>, 95>(font_texture);
	font_texture.subdivide(Vec2::new(13, 16), &mut font_textures);

	let font = ColorFont::<Ascii>::from_fmt_monospace(
		-1,
		16,
		ColorAlpha::from_rgba888(0xFF, 0x00, 0xFF, 0xFF),
		ColorAlpha::from_rgba888(0xFF, 0x00, 0x00, 0x00),
		" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
		(font_textures[0], 0, 0),
		font_textures,
	);

	let mut texture = TextureAlpha::from_bytes(Vec2::new(60, 60), TEXTURE);

	texture.clip(OrdRect::from_coords(10, 10, 50, 50));

	let mut v = Vec2::new(0, 0);

	loop {
		ck.update();

		if ck.inputs.key_down(Key::Up) {
			v.y -= 1;
		}
		if ck.inputs.key_down(Key::Down) {
			v.y += 1;
		}
		if ck.inputs.key_down(Key::Left) {
			v.x -= 1;
		}
		if ck.inputs.key_down(Key::Right) {
			v.x += 1;
		}

		let mut view = texture.view(OrdRect::from_coords(10, 0, 60, 60));
		view.clip(Rect::from_coords(50, 0, 0, 50));

		let mut render = Pipeline2D::<5>::new();
		render.register(Element2D::RectangleOutline {
			rect:      Rect::from_coords(50, 50, 200, 150),
			thickness: -3,
			value:     COLOR_WHITE,
		});
		render.register(Element2D::TiledSprite {
			//pos:     v,
			tile_rect: OrdRect::from_size(v, Vec2::new(50, 50)),
			rect:      OrdRect::from_coords(50, 50, 200, 150),
			texture:   &view,
		});
		render.register(Element2D::<Color>::Text {
			pos:  Vec2::new(50, 160),
			text: "Hello\nWorld",
			font: &font,
		});

		ck.render(COLOR_BLACK, &mut [&mut render]);
	}
}
