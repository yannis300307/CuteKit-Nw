#!/usr/bin/env python3

VERSION = 1

def rgb16(r: int, g: int, b: int, _a: int):
	return r >> (8-5) << (6+5) | g >> (8-6) << 5 | b >> (8-5)

def rgba16(r: int, g: int, b: int, a: int):
	return rgb16(r, g, b, a), a

def l8(r: int, g: int, b: int, _a: int):
	return sum((r, g, b)) // 3

COLORS = {
	"RGB":			("H", rgb16),
	"RGBA":			("HB", rgba16),
	"Grayscale":	("B", l8)
}

DEFAULT_DIR = ".kibble"

##################
# IMPLEMENTATION #
##################

def eprint(*args, **kwargs): print("\x1b[1;31m[ERROR]\x1b[22m", *args, "\x1b[0m", file=sys.stderr, **kwargs)

import argparse
import struct
import sys
from collections.abc import Callable
from pathlib import Path
from typing import Any

try: import png
except ImportError:
	eprint("You need pypng to run this script.\n  Use `pip3 install pypng`")
	sys.exit(1)

class ColorType:
	signature = 1
	def __init__(self, repr: str, process: Callable[[int, int, int, int], Any], signature: int = 0):
		if signature > 0:
			self.signature = signature
		else:
			self.signature = ColorType.signature
			ColorType.signature += 1
		self.repr = repr
		self.depth = struct.calcsize(repr)
		self.process = process

COLORS = {k: ColorType(*v) for k, v in COLORS.items()}

def convert(source: Path, dest: Path, color: ColorType):
	reader = png.Reader(filename=source)

	width, height, rows, image_infos = reader.asRGBA8()
	if (image_infos["bitdepth"] > 8):
		eprint(f"{source}: Color depth is too high")
		return

	header_fmt = "3s B HHB 6x c"
	header = bytearray(struct.calcsize(header_fmt))
	struct.pack_into(
		header_fmt, header, 0,
		"kib".encode("ascii"),
		VERSION,
		width, height, color.signature,
		"\n".encode("ascii")
	)

	data = bytearray(width * height * color.depth)

	i = 0
	for row in rows:
		for x in range(0, len(row), 4):
			r, g, b, a = row[x:][:4]
			pixel = color.process(r, g, b, a)

			if hasattr(pixel, "__iter__"):
				struct.pack_into(color.repr, data, i, *pixel)
			else:
				struct.pack_into(color.repr, data, i, pixel)
			i += color.depth

	with dest.open("wb") as out:
		out.write(header)
		out.write(data)

def parse_config(config: dict[str, Any], name: str, source: Path, dest: Path, subdest: str):

	new_dest = config.get("kib_directory")
	if new_dest is not None:
		if not isinstance(new_dest, str):
			eprint(f"{name}: Expected string for `kib_directory`, got `{type(new_dest).__name__}`")
			return
		if new_dest.startswith("@"):
			dest = origin / Path(new_dest[1:])
		else:
			dest /= new_dest
	else:
		dest /= subdest

	for key, value in config.items():

		if key in COLORS:
			if not isinstance(value, list):
				eprint(f"{name}: Expected string array for `{key}`, got `{type(value).__name__}`")
				continue

			for i, file_glob in enumerate(value):
				if not isinstance(file_glob, str):
					eprint(f"{name}: Expected string values in `{key}`, found `{type(file_glob).__name__}` at index {i}")
					continue

				file_sources = list(source.glob(file_glob))

				if len(file_sources) == 0:
					eprint(f"{name}: no matching file found for {file_glob}")
					continue

				for file in file_sources:
					rel_source = file.relative_to(origin)

					if file.suffix != ".png":
						eprint(f"{name}: {rel_source} is not a PNG file")
						continue

					file_dest = (dest / rel_source).with_suffix(".kib")
					rel_dest = file_dest.relative_to(origin)

					if file_dest.exists() and file_dest.stat().st_mtime > file.stat().st_mtime:
						if args.verbose: print(f"{name}: {rel_dest} unchanged for {key}")
						continue

					file_dest.parent.mkdir(parents=True, exist_ok=True)
					if args.verbose: print(f"{name}: Converting {rel_source} to {rel_dest} for {key}")
					convert(file, file_dest, COLORS[key])

		else:
			new_name = key if name == "[]" else f"{name}.{key}"
			new_path = source / key
			if new_path.is_dir():
				parse_config(value, new_name, new_path, dest, key)
			else:
				eprint(new_name, ": Key is not a known type nor a valid directory")

def main() -> int:
	parser = argparse.ArgumentParser(description="Convert PNG files to Kibble")
	parser.add_argument("config", help="path to a json or toml config file")
	parser.add_argument("-v", "--verbose", action="store_true")

	global args
	args = parser.parse_args()

	config_path = Path(args.config).resolve()
	if config_path.is_dir():
		config_path /= "kibble"
	if config_path.suffix == "":
		for ext in [".toml", ".json"]:
			ext_path = config_path.with_suffix(ext)
			if ext_path.exists():
				config_path = ext_path
				break

	if not config_path.exists():
		eprint(f"No toml or json config file found for ~/{config_path.relative_to(Path.home())}")
		return 1

	with config_path.open("rb") as config_file:

		match config_path.suffix:
			case ".toml":
				import tomllib
				config = tomllib.load(config_file)
			case ".json":
				import json
				config = json.load(config_file)
			case _:
				eprint("Unsupported extension:", config_path)
				return 1

		global origin
		origin = config_path.parent
		path = origin
		parse_config(config, "[]", path, path, DEFAULT_DIR)
	return 0

if __name__ == "__main__":
	sys.exit(main())
