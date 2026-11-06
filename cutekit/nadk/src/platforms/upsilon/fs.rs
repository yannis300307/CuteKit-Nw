use core::{array, ffi::CStr};

use crate::upsilon::Upsilon;

extern crate alloc;
use alloc::borrow::Cow;

impl Upsilon {
	pub fn file_write(filename: &CStr, content: &[u8]) -> Result<(), ()> {
		if unsafe { extapp_fileWrite(filename.as_ptr(), content.as_ptr(), content.len(), 0) } {
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn file_exists(filename: &CStr) -> bool {
		unsafe { extapp_fileExists(filename.as_ptr(), 0) }
	}

	pub fn file_read(filename: &CStr) -> Result<Cow<'_, [u8]>, ()> {
		let mut length: usize = 0;

		let array_pointer =
			unsafe { extapp_fileRead(filename.as_ptr(), &mut length as *mut usize, 0) };
		if array_pointer.is_null() {
			return Err(());
		}

		Ok(Cow::Borrowed(unsafe {
			core::slice::from_raw_parts(array_pointer, length)
		}))
	}

	pub fn file_read_slice(
		filename: &CStr,
		start: usize,
		mut slice_length: usize,
	) -> Result<Cow<'_, [u8]>, ()> {
		let mut lenght: usize = 0;

		let array_pointer = unsafe {
			extapp_fileRead(filename.as_ptr(), &mut lenght as *mut usize, 0)
				.offset(start as isize)
		};
		if array_pointer.is_null() {
			return Err(());
		}

		if lenght < slice_length {
			slice_length = lenght;
		}

		Ok(Cow::Borrowed(unsafe {
			core::slice::from_raw_parts(array_pointer, slice_length)
		}))
	}

	pub fn file_erase(filename: &CStr) -> Result<(), ()> {
		if unsafe { extapp_fileErase(filename.as_ptr(), 0) } {
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn file_list_with_extension<const N: usize>(
		extension: &CStr,
		records: &mut [Cow<str>; N],
	) -> Result<usize, ()> {
		let mut byte_records: [*mut u8; N] = array::repeat(core::ptr::null_mut());
		unsafe {
			let final_len = extapp_fileListWithExtension(
				byte_records.as_mut_ptr(),
				records.len() as isize,
				extension.as_ptr(),
				0,
			);
			if final_len == -1 {
				return Err(());
			}

			let mut rec_iter = records.iter_mut();
			for byte_record in byte_records {
				if let Ok(s) = CStr::from_ptr(byte_record).to_str() {
					match rec_iter.next() {
						Some(record) => *record = Cow::Borrowed(s),
						None => break,
					}
				}
			}

			return Ok(final_len as usize);
		}
	}
}

unsafe extern "C" {
	fn extapp_fileWrite(
		filename: *const u8,
		content: *const u8,
		len: usize,
		storage: isize,
	) -> bool;
	fn extapp_fileExists(filename: *const u8, storage: isize) -> bool;
	fn extapp_fileRead(filename: *const u8, len: *mut usize, storage: isize) -> *const u8;
	fn extapp_fileErase(filename: *const u8, storage: isize) -> bool;
	fn extapp_fileListWithExtension(
		filename: *mut *mut u8,
		maxrecord: isize,
		extension: *const u8,
		storage: isize,
	) -> isize;
}
