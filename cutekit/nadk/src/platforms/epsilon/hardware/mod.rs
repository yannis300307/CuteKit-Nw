use crate::{CalculatorModel, epsilon::Epsilon, heap};

pub mod fs;

impl Epsilon {
	pub fn init() {
		unsafe {
			heap::get().init(core::ptr::addr_of_mut!(_heap_start) as usize, 100_000);
		}
	}

	pub fn get_model() -> CalculatorModel {
		match unsafe { extapp_calculatorModel() } {
			1 => CalculatorModel::EpsilonN011X,
			2 => CalculatorModel::EpsilonN0120,
			_ => CalculatorModel::Unknown,
		}
	}
}

#[macro_export]
macro_rules! configure_app {
	($app_name: expr, $icon_path: expr) => {
		#[used]
		#[unsafe(link_section = ".rodata.eadk_app_name")]
		pub static EADK_APP_NAME: [u8; $app_name.len()] = *$app_name;

		#[used]
		#[unsafe(link_section = ".rodata.eadk_api_level")]
		pub static EADK_APP_API_LEVEL: u32 = 0;

		#[used]
		#[unsafe(link_section = ".rodata.eadk_app_icon")]
		pub static EADK_APP_ICON: [u8; include_bytes!($icon_path).len()] =
			*include_bytes!($icon_path);
	};
}

unsafe extern "C" {
	pub static mut _heap_start: u8;
	pub static mut _heap_end: u8;

	fn extapp_calculatorModel() -> u8;
}
