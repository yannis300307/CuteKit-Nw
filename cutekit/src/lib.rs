#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(f16)]

pub use nadk::{
	configure_app,
	display::{COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE},
	keyboard::Key,
};

pub type Color = nadk::display::Color565;
pub type Vec2<T> = nalgebra::Vector2<T>;
pub type Vec3<T> = nalgebra::Vector3<T>;
pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type ScreenPos = Vec2<i16>;

pub mod core;
pub use crate::core::*;

pub mod gui;
pub mod render2d;
pub mod renderer;
pub mod renderer2d;

#[cfg(not(feature = "epsilon"))]
#[cfg(not(feature = "upsilon"))]
compile_error!(
	r#"No valid firmware selected!
	Add 'epsilon' or 'upsilon' to cutekit's features in your Cargo.toml"#
);

#[cfg(all(feature = "epsilon", feature = "upsilon"))]
compile_error!(
	r#"Multiple firmwares selected!
	Make sure you only have the 'epsilon' or 'upsilon' feature enabled in your Cargo.toml"#
);

pub fn init() -> Cutekit {
	if cfg!(feature = "handle_nadk") {
		nadk::init_heap();
		nadk::utils::wait_ok_released();
	}

	Cutekit::init()
}
