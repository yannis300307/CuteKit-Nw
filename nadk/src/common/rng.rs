use nanorand::WyRand;

pub static mut RNG: WyRand = WyRand::new_seed(0xC001C0FFEE5EED5);

pub fn get() -> &'static WyRand {
	unsafe {
		#[allow(static_mut_refs)]
		return &RNG;
	}
}

pub fn get_mut() -> &'static mut WyRand {
	unsafe {
		#[allow(static_mut_refs)]
		return &mut RNG;
	}
}
