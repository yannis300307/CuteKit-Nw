#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(f16)]

use nalgebra::{Vector2, Vector3};

use crate::{
    gui::{Anchor, Button, ColorRectanglePrimitive, Container, Layout, Margin, Menu, Node, NodeType, Primitive, TextPrimitive, TransparentSpritePrimitive}, ingame_ui::draw_ui, input_manager::InputManager, nadk::{
        display::{self, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE, Color565, ScreenRect},
        time::{self, wait_milliseconds},
        utils::wait_ok_released,
    }, renderer::{Renderer, mesh::{FlatMesh, TexturedMesh}}, renderer2d::{draw_queue::DrawQueue, elements::{Element, Font, ScaleMode, Texture}, nine_parts_rectangle::NinePartsTexture, renderer::Renderer2d, sprite::TransparentTexture}, timing::TimingManager
};

use include_bytes_aligned::include_bytes_aligned;

#[macro_use]
mod nadk;

mod constants;
mod input_manager;
mod renderer;
mod timing;
mod gui;

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

static TEXTURE: &[u8] = include_bytes_aligned!(4, "../target/assets/texture.bin");

#[unsafe(no_mangle)]
fn main() {
    init_heap!();

    wait_ok_released();

    let mut input_manager = InputManager::new();
    let mut time_manager = TimingManager::new();

    let texture = Texture { width: 512, height: 512, data: bytemuck::cast_slice(TEXTURE) };
    /*let mut renderer = Renderer::new();
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
    };*/

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
        let delta = time_manager.get_delta_time();
        //renderer.camera.update(delta, &input_manager);

        /*renderer.draw_textured_mesh(&face);
        renderer.draw_textured_mesh(&hair);
        renderer.draw_textured_mesh(&body);
        renderer.draw_textured_mesh(&zip);
        renderer.draw_textured_mesh(&skirt);*/

        let frame_time = heapless::format!(30; "time: {}", time_manager.get_frame_time()).unwrap();


        let primitive1 = ColorRectanglePrimitive {
            pos: Vector2::new(0, 0),
            size: Vector2::new(100, 20),
            color: COLOR_RED
        };

        let primitive2 = TextPrimitive {
            pos: Vector2::new(0, 0),
            text: "Hello !",
            font: &font,
            font_color: COLOR_BLACK,
            background_color: None,
        };

        let button_children: [&dyn Primitive; _]  = [&primitive1, &primitive2];

        let button1 = Button {
            children: &button_children,
            pos: Vector2::new(0, 0),
            layout: Layout::Expand(true, false, Margin { top: 5, bottom: 5, right: 5, left: 5 }),
        };

        let top_lvl_container: [NodeType; _] = [
            NodeType::Normal(&button1),
            NodeType::Normal(&button1),
        ];

        let menu = Menu {
            base_node: Container {
                children: &top_lvl_container,
                layout: Layout::Expand(true, false, Margin::new(5, 5, 5, 5)),
                pos: Vector2::new(0, 0),
                size: Some(Vector2::new(320, 240))
            },
        };

        let mut draw_queue: DrawQueue<'_, 100> = DrawQueue::new();

        menu.render(&mut draw_queue).unwrap();

        renderer2d.draw(&mut draw_queue);

    }
}
