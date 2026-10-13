#![allow(unused)]
#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;
#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
pub fn init_heap() { unsafe { crate::HEAP.init(adresses::heap_start(), adresses::heap_size()) } }

#[macro_use]
/// A crate containing useful macros to write cleaner code.
pub mod macros;

/// Provides abstraction to the display.
pub mod display;

/// Provides abstraction to the backlight.
pub mod backlight;

/// Provides abstraction to the timers and some additional functions.
pub mod time;

/// Generates random numbers with the built-in random generator of the OS. Also provide some higher level functions.
pub mod random;

/// Provides abstraction to the keyboard. Also contains a high level input manager for game engines.
pub mod keyboard;

/// A crate to handle the program's panics.
pub mod panic_handler;

/// Provide addresses to low level stuff.
pub mod adresses;

// Provide high level access to the OS' storage. Contains bindings for Yaya.cout's storage.c lib. Also works on simulator.
pub mod storage;

/// Provide some random useful functions.
pub mod utils;
