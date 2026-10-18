#![cfg_attr(target_os = "none", no_std)]
#![feature(const_trait_impl)]
#![feature(f16)]
#![feature(step_trait)]
#![feature(generic_const_exprs)]

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

pub mod core;
pub use crate::core::*;

pub mod render2d;

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
		#[cfg(target_os = "none")]
		nadk::init_heap();
		nadk::utils::wait_ok_released();
	}

	Cutekit::init()
}
