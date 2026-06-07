#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(f16)]

use nalgebra::Vector3;

use crate::{
    ingame_ui::draw_ui,
    input_manager::InputManager,
    nadk::{
        display::{self, COLOR_RED, ScreenRect},
        time,
        utils::wait_ok_released,
    },
    renderer::{Renderer, mesh::FlatMesh},
    timing::TimingManager,
};

use include_bytes_aligned::include_bytes_aligned;

#[macro_use]
mod nadk;

mod camera;
mod constants;
mod input_manager;
mod renderer;
mod timing;

mod draw_tools;
mod ingame_ui;

setup_allocator!();

configure_app!(b"Numcraft\0", 9, "../target/assets/icon.nwi", 3437);

// Hey you reading the commit history of the repo! If you're wondering why these files are not included in the repo,
// it's because the model used to develop the 3D engine was not open source. So We can't redistribute it under the GPL 3 license.
// However, you can still replace the model with your own converted model. Have a good day!
static BODY_VERTICIES: &'static [u8] =
    include_bytes_aligned!(4, "../assets/model/body_verticies.bin");
static BODY_FACES: &'static [u8] = include_bytes_aligned!(4, "../assets/model/body_faces.bin");
static SKIRT_VERTICIES: &'static [u8] =
    include_bytes_aligned!(4, "../assets/model/skirt_verticies.bin");
static SKIRT_FACES: &'static [u8] = include_bytes_aligned!(4, "../assets/model/skirt_faces.bin");
static ZIP_VERTICIES: &'static [u8] =
    include_bytes_aligned!(4, "../assets/model/zip_verticies.bin");
static ZIP_FACES: &'static [u8] = include_bytes_aligned!(4, "../assets/model/zip_faces.bin");
static HAIR_VERTICIES: &'static [u8] =
    include_bytes_aligned!(4, "../assets/model/hair_verticies.bin");
static HAIR_FACES: &'static [u8] = include_bytes_aligned!(4, "../assets/model/hair_faces.bin");
static FACE_VERTICIES: &'static [u8] =
    include_bytes_aligned!(4, "../assets/model/face_verticies.bin");
static FACE_FACES: &'static [u8] = include_bytes_aligned!(4, "../assets/model/face_faces.bin");

#[unsafe(no_mangle)]
fn main() {
    init_heap!();

    wait_ok_released();

    display::push_rect_uniform(ScreenRect::new(20, 20, 50, 30), COLOR_RED);

    let mut input_manager = InputManager::new();
    let mut time_manager = TimingManager::new();
    let mut renderer = Renderer::new();

    renderer.camera.update_pos(Vector3::new(0.0, 1.0, -2.0));

    let body = FlatMesh {
        triangles: bytemuck::cast_slice(&BODY_FACES),
        vertices: bytemuck::cast_slice(&BODY_VERTICIES),
    };
    let skirt = FlatMesh {
        triangles: bytemuck::cast_slice(&SKIRT_FACES),
        vertices: bytemuck::cast_slice(&SKIRT_VERTICIES),
    };
    let zip = FlatMesh {
        triangles: bytemuck::cast_slice(&ZIP_FACES),
        vertices: bytemuck::cast_slice(&ZIP_VERTICIES),
    };
    let hair = FlatMesh {
        triangles: bytemuck::cast_slice(&HAIR_FACES),
        vertices: bytemuck::cast_slice(&HAIR_VERTICIES),
    };
    let face = FlatMesh {
        triangles: bytemuck::cast_slice(&FACE_FACES),
        vertices: bytemuck::cast_slice(&FACE_VERTICIES),
    };

    loop {
        time_manager.update();
        input_manager.update();
        let delta = time_manager.get_delta_time();
        renderer.draw_flat_mesh(&face);
        renderer.draw_flat_mesh(&hair);
        renderer.draw_flat_mesh(&body);
        renderer.draw_flat_mesh(&zip);
        renderer.draw_flat_mesh(&skirt);
        renderer.draw_game(Some(&draw_ui));
        renderer.camera.update(delta, &input_manager);

        if input_manager.is_just_pressed(nadk::keyboard::Key::Back) {
            break;
        }
        time::wait_milliseconds(16);
    }
}
