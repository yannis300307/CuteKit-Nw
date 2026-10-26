use std::{fs, process::Command};

use regex::Regex;

fn compile_c_libs() {
	unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

	let program = "npx";

	let nwlink_flags = String::from_utf8(
		Command::new(program)
			.args(["--yes", "--", "nwlink@0.0.19", "eadk-cflags"])
			.output()
			.expect("Failed to get nwlink eadk-cflags")
			.stdout,
	)
	.expect("Invalid UTF-8 in nwlink flags");

	let mut build = cc::Build::new();
	build.file("cutekit/nadk/src/storage/storage.c");
	build.flag("-std=c99");
	build.flag("-Os");
	build.flag("-Wall");
	build.flag("-ggdb");
	build.warnings(false);

	for flag in nwlink_flags.split_whitespace() {
		build.flag(flag);
	}

	build.compile("storage_c");
}

fn patch_simulator() {
	let remapped = "constexpr static KeySDLKeyPair sKeyPairs[] = {\
  KeySDLKeyPair(Key::OK,        SDL_SCANCODE_RETURN),\n\
  KeySDLKeyPair(Key::Back,      SDL_SCANCODE_BACKSPACE),\n\
  KeySDLKeyPair(Key::EXE,       SDL_SCANCODE_ESCAPE),\n\
\
  KeySDLKeyPair(Key::Var,       SDL_SCANCODE_I),\n\
\
  KeySDLKeyPair(Key::Toolbox,   SDL_SCANCODE_W),\n\
  KeySDLKeyPair(Key::Imaginary, SDL_SCANCODE_A),\n\
  KeySDLKeyPair(Key::Power,     SDL_SCANCODE_D),\n\
  KeySDLKeyPair(Key::Comma,     SDL_SCANCODE_S),\n\
  KeySDLKeyPair(Key::Shift,     SDL_SCANCODE_SPACE),\n\
  KeySDLKeyPair(Key::Exp,       SDL_SCANCODE_LSHIFT),\n\
\
  KeySDLKeyPair(Key::Down,      SDL_SCANCODE_DOWN),\n\
  KeySDLKeyPair(Key::Up,        SDL_SCANCODE_UP),\n\
  KeySDLKeyPair(Key::Left,      SDL_SCANCODE_LEFT),\n\
  KeySDLKeyPair(Key::Right,     SDL_SCANCODE_RIGHT),\n\
};";

	let file_content = fs::read_to_string("simulator/ion/src/simulator/shared/keyboard.cpp")
        .expect("Cannot open keyboard.cpp file from emulator. Please check if the simulator is clonned properly.");

	if !file_content.contains(remapped) {
		let re = Regex::new(r"constexpr static KeySDLKeyPair sKeyPairs\[] ?= ?\{[\S\s]*?};")
			.unwrap();
		let result = re.replace(&file_content, remapped);

		fs::write(
			"simulator/ion/src/simulator/shared/keyboard.cpp",
			result.as_bytes(),
		)
		.unwrap();
	}
}

fn convert_icon() {
	let output = {
		if let Ok(out) = Command::new("sh")
			.arg("-c")
			.arg("npx --yes -- nwlink@0.0.19 png-nwi assets/icon.png target/assets/icon.nwi")
			.output()
		{
			out
		} else {
			panic!(
				"Your OS is not supported! If you're using Windows, please compile Numcraft in WSL."
			);
		}
	};
	assert!(
		output.status.success(),
		"{}",
		String::from_utf8_lossy(&output.stderr)
	);
}

fn main() {
	// Turn icon.png into icon.nwi
	println!("cargo:rerun-if-changed=assets/icon.png");
	convert_icon();

	// Compile storage.c
	if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
		println!("cargo:rustc-link-arg=--relocatable");
		println!("cargo:rustc-link-arg=-no-gc-sections");

		if std::env::var("CARGO_FEATURE_UPSILON").is_ok() {
			println!("cargo:rustc-link-arg=-Ltarget/upsilon_api");
			println!("cargo:rustc-link-arg=-lapi");
		} else {
			compile_c_libs();
			println!("cargo:rustc-link-arg=-lstorage_c");
		}
	} else {
		patch_simulator();
	}
}
