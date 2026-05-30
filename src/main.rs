#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(f16)]

use nalgebra::{Vector2, Vector3};

use crate::{
    input_manager::InputManager, nadk::{
        display::{self, COLOR_RED, ScreenRect}, keyboard::wait_until_pressed, time, utils::wait_ok_released
    }, renderer::{
        Renderer,
        mesh::{Mesh, MeshTriangle, Triangle},
    }, timing::TimingManager
};

#[macro_use]
mod nadk;

mod camera;
mod constants;
mod input_manager;
mod renderer;
mod timing;

mod model;

setup_allocator!();

configure_app!(b"Numcraft\0", 9, "../target/assets/icon.nwi", 3437);

#[unsafe(no_mangle)]
fn main() {
    init_heap!();

    wait_ok_released();

    display::push_rect_uniform(ScreenRect::new(20, 20, 50, 30), COLOR_RED);

    let mut input_manager = InputManager::new();
    let mut time_manager = TimingManager::new();
    let mut renderer = Renderer::new();

    let mut mesh = Mesh::new();

    model::load_mesh(&mut mesh);

    loop {
        time_manager.update();
        input_manager.update();
        let delta = time_manager.get_delta_time();
        renderer.draw_mesh(&mesh);
        renderer.draw_game();
        renderer.camera.update(delta, &input_manager);

        if input_manager.is_just_pressed(nadk::keyboard::Key::Back) {
            break;
        }
        time::wait_milliseconds(16);
    }
}
