#![cfg_attr(target_os = "none", no_std)]
#![feature(const_trait_impl)]
#![feature(const_index, const_cmp, const_convert)]
#![feature(min_specialization)]
#![feature(fn_traits)]

//! Rendering toolkit for limited hardware.
//! Made for Numworks calculators.

pub use nadk::{Key, configure_app, get_random_u32, utils::rng::*};

pub type Vec2<T = f32> = nalgebra::Vector2<T>;
pub type Vec3<T = f32> = nalgebra::Vector3<T>;

cfg_select! {
	feature = "expose" => {
		pub mod core;
		pub mod mods;

		pub use core::*;

		pub use mods::*;
	}
	_ => {
		pub(crate) mod core;
		pub(crate) mod mods;

		pub use core::public::*;
		pub(crate) use core::*;

		pub use mods::public::*;
	}
}

pub mod prelude {
	#![allow(unused_imports)]

	pub use enum_map::{EnumMap, enum_map};

	#[cfg(feature = "draw_2d")]
	pub use crate::draw_2d::*;
	#[cfg(feature = "draw_3d")]
	pub use crate::draw_3d::*;
	#[cfg(feature = "kibble")]
	pub use crate::kibble::*;
	#[cfg(feature = "ui")]
	pub use crate::ui::*;
	pub use crate::*;
}
