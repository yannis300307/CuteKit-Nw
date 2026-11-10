use core::ffi::CStr;
use std::{
	borrow::Cow,
	fs,
	io::{self, Read, Seek},
};

use crate::epsilon::Epsilon;

impl Epsilon {
	pub fn file_write(filename: &CStr, content: &[u8]) -> Result<(), ()> {
		let filename_str = filename.to_str().map_err(|_| ())?;

		fs::create_dir_all("simulator/storage").map_err(|_| ())?;
		fs::write(format!("simulator/storage/{}", filename_str), content).map_err(|_| ())
	}

	pub fn file_exists(filename: &CStr) -> bool {
		filename.to_str().is_ok_and(|filename_str| {
			fs::exists(format!("simulator/storage/{}", filename_str)).is_ok_and(|x| x)
		})
	}

	pub fn file_read(filename: &CStr) -> Result<Cow<'_, [u8]>, ()> {
		let filename_str = filename.to_str().map_err(|_| ())?;

		Ok(Cow::Owned(
			fs::read(format!("simulator/storage/{}", filename_str)).map_err(|_| ())?,
		))
	}

	pub fn file_read_slice(
		filename: &CStr,
		start: usize,
		slice_length: usize,
	) -> Result<Cow<'_, [u8]>, ()> {
		let filename_str = filename.to_str().map_err(|_| ())?;

		let mut f = fs::File::open(filename_str).map_err(|_| ())?;

		f.seek(io::SeekFrom::Start(start as u64)).map_err(|_| ())?;
		let mut buf = vec![0; slice_length];
		f.read_exact(&mut buf).map_err(|_| ())?;

		Ok(Cow::Owned(buf))
	}

	pub fn file_erase(filename: &CStr) -> Result<(), ()> {
		let filename_str = filename.to_str().map_err(|_| ())?;
		fs::remove_file(format!("simulator/storage/{}", filename_str)).map_err(|_| ())
	}

	pub fn file_list_with_extension<const N: usize>(
		extension: &CStr,
		records: &mut [Cow<str>; N],
	) -> Result<usize, ()> {
		let extension_str = extension.to_str().map_err(|_| ())?;

		let mut count = 0;
		for entry in fs::read_dir("simulator/storage").unwrap() {
			if let Ok(entry) = entry
				&& let Some(name) = entry.file_name().to_str()
				&& name.ends_with(extension_str)
			{
				records[count] = Cow::Owned(name.to_owned());
				count += 1;
			}
		}
		return Ok(count);
	}
}
