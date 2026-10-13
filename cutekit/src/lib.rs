#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(f16)]

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

#[macro_use]
mod nadk;

pub use crate::nadk::keyboard::Key;

pub mod context;
pub use crate::context::Cutekit;
pub use crate::nadk::display::{
	COLOR_BLACK,
	COLOR_BLUE,
	COLOR_GREEN,
	COLOR_RED,
	COLOR_WHITE,
	Color565,
};

pub mod constants;
pub mod gui;
pub mod inputs;
pub mod renderer;
pub mod renderer2d;

cfg_select! {
	feature = "handle_nadk" => {
		setup_allocator!();
		configure_app!(b"CuteKitTest\0", 12, "../target/assets/icon.nwi", 3437);
	}
	_ => {}
}

pub fn init() -> Cutekit {
	if cfg!(feature = "handle_nadk") {
		init_heap!();
		nadk::utils::wait_ok_released();
	}

	Cutekit::init()
}
