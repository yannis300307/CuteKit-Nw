#![cfg_attr(target_os = "none", no_std)]
#![feature(const_trait_impl)]
#![feature(step_trait)]

pub use nadk::{
	configure_app as setup,
	display::{COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE},
	keyboard::Key,
};

pub type Color = nadk::display::Color565;
pub type Vec2<T> = nalgebra::Vector2<T>;
pub type Vec3<T> = nalgebra::Vector3<T>;
pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;

cfg_select! {
	feature = "expose" => {
		pub mod core;
		pub mod mods;

		pub use core::{public::*, *};

		#[cfg(feature = "draw_2d")]
		pub use mods::draw_2d::public as draw_2d;
		#[cfg(feature = "draw_3d")]
		pub use mods::draw_3d::public as draw_3d;
		#[cfg(feature = "ui")]
		pub use mods::ui::public as ui;
	}
	_ => {
		pub(crate) mod core;
		pub(crate) mod mods;

		pub use core::public::*;
		pub(crate) use core::*;

		#[cfg(feature = "draw_2d")]
		pub use mods::draw_2d::public as draw_2d;
		#[cfg(feature = "draw_3d")]
		pub use mods::draw_3d::public as draw_3d;
		#[cfg(feature = "ui")]
		pub use mods::ui::public as ui;
	}
}

#[cfg(not(any(feature = "epsilon", feature = "upsilon")))]
compile_error!(
	r#"No valid firmware selected!
	Add 'epsilon' or 'upsilon' to cutekit's features in your Cargo.toml"#
);

#[cfg(all(feature = "epsilon", feature = "upsilon"))]
compile_error!(
	r#"Multiple firmwares selected!
	Make sure you only have the 'epsilon' or 'upsilon' feature enabled in your Cargo.toml"#
);
