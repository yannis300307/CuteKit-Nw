use crate::{Texture, mods::kibble::color::KibColor};

#[macro_export]
macro_rules! include_texture {
	($path: expr) => {
		$crate::Texture::_from_kib(include_bytes_aligned::include_bytes_aligned!(16, $path))
	};
}

struct KibbleMD {
	width:           i16,
	height:          i16,
	color_signature: u8,
}

#[repr(C)]
struct _KibbleHeader {
	signature:       [u8; 3],
	version:         u8,
	width:           i16,
	height:          i16,
	color_signature: u8,
	padding:         [u8; 6],
	newline:         u8,
}

impl<T: KibColor> Texture<'static, T> {
	pub const fn _from_kib(bytes: &'static [u8]) -> Self {
		assert!(align_of::<T>() <= 16, "Color alignment cannot be above 16");

		let len = bytes.len();
		let st = size_of::<T>();

		assert!(len > 4, "File is too small");
		assert!(bytes[0..3] == *b"kib", "Invalid file signature");

		let version = bytes[3];
		let header_size = match version {
			1 => 16,
			_ => panic!("Invalid version"),
		};

		assert!(len > header_size, "File is too small");
		assert!(bytes[header_size - 1] == b'\n', "Malformed header");
		assert!((len - header_size) % st == 0, "Invalid data size");

		let raw_md = &bytes[4..header_size];
		let md = match version {
			1 => KibbleMD {
				width:           i16::from_le_bytes(*raw_md[0..2].first_chunk().unwrap()),
				height:          i16::from_le_bytes(*raw_md[2..4].first_chunk().unwrap()),
				color_signature: raw_md[4],
			},
			_ => panic!(),
		};

		assert!(
			md.color_signature == T::SIGNATURE,
			"Invalid color signature"
		);
		assert!(md.width > 0 && md.height > 0, "Invalid size values");

		let data_size = (len - header_size) / st;
		let data = &bytes[header_size..];

		assert!(
			md.width as usize * md.height as usize == data_size,
			"Invalid pixel data size"
		);

		Self::new(md.width, md.height, unsafe {
			core::slice::from_raw_parts(data.as_ptr().cast::<T>(), data_size)
		})
	}
}
