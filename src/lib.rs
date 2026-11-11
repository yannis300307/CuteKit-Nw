#![cfg_attr(target_os = "none", no_std)]
#![feature(const_trait_impl)]
#![feature(const_index, const_cmp, const_convert)]
#![feature(min_specialization)]
#![feature(fn_traits)]

//! Rendering toolkit for limited hardware.
//! Made for Numworks calculators.

pub use nadk::{Key, configure_app, get_random_u32, utils::rng::*};

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

/// Includes common types and utilities,
/// without the sensitively named functions like `update`
pub mod prelude {
	#![allow(unused_imports)]

	pub use enum_map::{EnumMap, enum_map};
	pub use nadk::{Key, configure_app, get_random_u32, utils::rng::*};

	pub use super::{core::prelude::*, mods::prelude::*};
}
