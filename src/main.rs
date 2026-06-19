#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(f16)]

use nalgebra::{Vector2, Vector3};

use crate::{
    ingame_ui::draw_ui, input_manager::InputManager, nadk::{
        display::{self, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE, Color565, ScreenRect},
        time::{self, wait_milliseconds},
        utils::wait_ok_released,
    }, renderer::{Renderer, mesh::{FlatMesh, TexturedMesh}}, renderer2d::{draw_queue::DrawQueue, elements::{Font, ScaleMode, Texture}, nine_parts_rectangle::NinePartsTexture, renderer::Renderer2d, sprite::TransparentTexture}, timing::TimingManager
};

use include_bytes_aligned::include_bytes_aligned;

#[macro_use]
mod nadk;

mod camera;
mod constants;
mod input_manager;
mod renderer;
mod timing;

mod ingame_ui;
mod renderer2d;

setup_allocator!();

configure_app!(b"Numcraft\0", 9, "../target/assets/icon.nwi", 3437);

// Hey you reading the commit history of the repo! If you're wondering why these files are not included in the repo,
// it's because the model used to develop the 3D engine was not open source. So We can't redistribute it under the GPL 3 license.
// However, you can still replace the model with your own converted model. Have a good day!
static BODY_VERTICIES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_body_verticies.bin"
);
static BODY_FACES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_body_faces.bin"
);
static SKIRT_VERTICIES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_skirt_verticies.bin"
);
static SKIRT_FACES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_skirt_faces.bin"
);
static ZIP_VERTICIES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_zip_verticies.bin"
);
static ZIP_FACES: &'static [u8] =
    include_bytes_aligned!(4, "../assets/model/Anime_character_optimised_zip_faces.bin");
static HAIR_VERTICIES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_hair_verticies.bin"
);
static HAIR_FACES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_hair_faces.bin"
);
static FACE_VERTICIES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_face_verticies.bin"
);
static FACE_FACES: &'static [u8] = include_bytes_aligned!(
    4,
    "../assets/model/Anime_character_optimised_face_faces.bin"
);

static TEXTURE: &[u8] = include_bytes!("../target/assets/texture.bin");

#[unsafe(no_mangle)]
fn main() {
    init_heap!();

    wait_ok_released();

    let mut input_manager = InputManager::new();
    let mut time_manager = TimingManager::new();

    /*let texture = Texture { width: 512, height: 512, data: bytemuck::cast_slice(TEXTURE) };
    let mut renderer = Renderer::new();
    renderer.load_texture(&texture);

    renderer.camera.update_pos(Vector3::new(0.0, 1.0, -2.0));

    let body = TexturedMesh {
        triangles: bytemuck::cast_slice(&BODY_FACES),
        vertices: bytemuck::cast_slice(&BODY_VERTICIES),
    };
    let skirt = TexturedMesh {
        triangles: bytemuck::cast_slice(&SKIRT_FACES),
        vertices: bytemuck::cast_slice(&SKIRT_VERTICIES),
    };
    let zip = TexturedMesh {
        triangles: bytemuck::cast_slice(&ZIP_FACES),
        vertices: bytemuck::cast_slice(&ZIP_VERTICIES),
    };
    let hair = TexturedMesh {
        triangles: bytemuck::cast_slice(&HAIR_FACES),
        vertices: bytemuck::cast_slice(&HAIR_VERTICIES),
    };
    let face = TexturedMesh {
        triangles: bytemuck::cast_slice(&FACE_FACES),
        vertices: bytemuck::cast_slice(&FACE_VERTICIES),
    };



    loop {
        time_manager.update();
        input_manager.update();
        let delta = time_manager.get_delta_time();
        renderer.draw_textured_mesh(&face);
        renderer.draw_textured_mesh(&hair);
        renderer.draw_textured_mesh(&body);
        renderer.draw_textured_mesh(&zip);
        renderer.draw_textured_mesh(&skirt);
        renderer.draw_game(Some(&draw_ui));
        renderer.camera.update(delta, &input_manager);

        if input_manager.is_just_pressed(nadk::keyboard::Key::Back) {
            break;
        }
        time::wait_milliseconds(16);
    }*/

    let mut renderer2d = Renderer2d::new(COLOR_BLACK);

    let mut a = 0;

    let font = Font { data: include_bytes!("../target/assets/font.bin"), font_image_width: 1235, char_width: 13, char_height: 16, chars: " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~" };
    
    let nine_parts_texture = TransparentTexture {
        data: bytemuck::cast_slice(include_bytes!("../target/assets/9parts.bin")),
        width: 60,
        height: 60,
    };
    let parts: NinePartsTexture = NinePartsTexture {
        texture: &nine_parts_texture,
        left_border_size: 20,
        right_border_size: 20,
        top_border_size: 20,
        bottom_border_size: 20,
    };


    loop {
        time_manager.update();
        input_manager.update();
        if input_manager.is_just_pressed(nadk::keyboard::Key::Back) {
            break;
        }

        let frame_time = heapless::format!(30; "time: {}", time_manager.get_frame_time()).unwrap();

        let mut draw_queue: DrawQueue<'_, 100> = DrawQueue::new();

        draw_queue.add_rectangle(Vector2::new(20 + a, 200 - a),Vector2::new(100, 50), COLOR_RED).unwrap();
        draw_queue.add_circle(Vector2::new(100 + a, 100 - a), 70.0, COLOR_BLUE).unwrap();
        draw_queue.add_circle(Vector2::new(100 + a, 100 - a), 20.0, COLOR_RED).unwrap();
        draw_queue.add_rounded_rectangle(Vector2::new(a / 2, 120), Vector2::new(100, 30 + a as u16), 15.0, COLOR_GREEN).unwrap();

        draw_queue.add_rounded_rectangle(Vector2::new(a / 2, 120), Vector2::new(100, 30 + a  as u16), 10.0, COLOR_GREEN).unwrap();
        draw_queue.add_rounded_rectangle(Vector2::new(a / 2, 120 - a / 2), Vector2::new(100, 60 + a as u16), 15.0, COLOR_GREEN).unwrap();
        draw_queue.add_rounded_rectangle(Vector2::new(a / 2, 120), Vector2::new(100 + a as u16, 100 + a as u16), 30.0, COLOR_RED).unwrap();
        draw_queue.add_rounded_rectangle(Vector2::new(a / 2, 120), Vector2::new(100, 30 + a as u16 / 2), 15.0, COLOR_BLUE).unwrap();

        draw_queue.add_nine_parts_rectangle(&parts, Vector2::new(50 + a, 50 + a * 2), Vector2::new(100 + a as u16, 100 + a as u16), ScaleMode::Tile).unwrap();
        draw_queue.add_nine_parts_rectangle(&parts, Vector2::new(100 + a, 50 - a), Vector2::new(30 + a as u16, 100 + a as u16), ScaleMode::Stretch).unwrap();

        draw_queue.add_nine_parts_rectangle(&parts, Vector2::new(100 + a, 50 - a), Vector2::new(30 + a as u16, 150 + a as u16), ScaleMode::Stretch).unwrap();

        draw_queue.add_nine_parts_rectangle(&parts, Vector2::new(100 + a, 50 - a), Vector2::new(100 + a as u16, 100 + a as u16), ScaleMode::Stretch).unwrap();

        draw_queue.add_nine_parts_rectangle(&parts, Vector2::new(120 + a /2, 50 - a), Vector2::new(30 + a as u16, 100 + a as u16 / 2), ScaleMode::Stretch).unwrap();

        draw_queue.add_text(Vector2::new(a / 2, a), frame_time.as_str(), &font, COLOR_WHITE, None).unwrap();
        draw_queue.add_text(Vector2::new( 60 + a, 50 + a), "Hello !", &font, COLOR_BLACK, Some(COLOR_WHITE)).unwrap();
        draw_queue.add_text(Vector2::new( 60 + a, 50 + a), "Hello !", &font, COLOR_BLACK, Some(COLOR_WHITE)).unwrap();

        draw_queue.add_text(Vector2::new( 60 + a, 70 + a), "Hello !", &font, COLOR_RED, None).unwrap();

        draw_queue.add_text(Vector2::new( 60 + a, 90 + a), "Hello !", &font, COLOR_BLACK, Some(COLOR_WHITE)).unwrap();

        draw_queue.add_text(Vector2::new( 60 + a, 110 + a), "Hello !", &font, COLOR_BLACK, None).unwrap();

        draw_queue.add_text(Vector2::new( 60 + a, 130 + a), "Hello !", &font, COLOR_BLACK, Some(COLOR_WHITE)).unwrap();

        draw_queue.add_text(Vector2::new( 60 + a, 150 + a), "Hello !", &font, COLOR_BLACK, Some(COLOR_GREEN)).unwrap();

        draw_queue.add_transparent_sprite(Vector2::new(10 + a / 2, 10 + a / 3), &nine_parts_texture).unwrap();

        draw_queue.add_textured_triangle(Vector2::new(5 + a as i16, 10 + a as i16 / 2), Vector2::new(200 - a as i16, 160 - a as i16 / 2), Vector2::new(180 - a as i16 / 2, 200 + a as i16 / 2), Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), Vector2::new(1.0, 0.0), &nine_parts_texture).unwrap();

        renderer2d.draw(&draw_queue.get_iterator());
        a += 1;
        a %= 200;

    }
}
