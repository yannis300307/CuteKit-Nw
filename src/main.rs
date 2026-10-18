#![cfg_attr(target_os = "none", no_std)]
#![no_main]

use cutekit::{
	COLOR_BLACK,
	COLOR_BLUE,
	COLOR_WHITE,
	Color,
	Key,
	Vec2,
	configure_app,
	drawing::Pipeline,
	render2d::{Element2D, Pipeline2D},
	texture::Texture,
};
use include_bytes_aligned::include_bytes_aligned;
mod cutekit_main;

configure_app!(b"CuteKitTest\0", 12, "../target/assets/icon.nwi", 3437);

static TEXTURE: &[u8] = include_bytes_aligned!(4, "../target/assets/model.bin");

#[unsafe(no_mangle)]
fn main() {
	let mut ck = cutekit::init();

	let mut render = Pipeline2D::<30>::new();

	let texture = Texture::from_bytes(Vec2::new(512, 512), TEXTURE);

	let mut pos = Vec2::new(40, 40);
	let mut size = Vec2::new(40, 80);

	loop {
		ck.update();

		if ck.inputs.key_down(Key::Up) {
			pos.y -= 1;
		}
		if ck.inputs.key_down(Key::Down) {
			pos.y += 1;
		}
		if ck.inputs.key_down(Key::Left) {
			pos.x -= 1;
		}
		if ck.inputs.key_down(Key::Right) {
			pos.x += 1;
		}

		render.clear();

		render
			.register(Element2D::Sprite {
				pos,
				texture: &texture,
			})
			.unwrap();

		ck.render(COLOR_BLACK, [&mut render]);
	}
}
