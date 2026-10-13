#[macro_export]
macro_rules! configure_app {
	($app_name: expr, $app_name_size: expr, $icon_path: expr, $icon_size: expr) => {
		#[used]
		#[cfg(target_os = "none")]
		#[unsafe(link_section = ".rodata.eadk_app_name")]
		#[cfg(feature = "epsilon")]
		pub static EADK_APP_NAME: [u8; $app_name_size] = *$app_name;

		#[used]
		#[cfg(target_os = "none")]
		#[unsafe(link_section = ".rodata.eadk_api_level")]
		#[cfg(feature = "epsilon")]
		pub static EADK_APP_API_LEVEL: u32 = 0;

		#[used]
		#[cfg(target_os = "none")]
		#[unsafe(link_section = ".rodata.eadk_app_icon")]
		#[cfg(feature = "epsilon")]
		pub static EADK_APP_ICON: [u8; $icon_size] = *include_bytes!($icon_path);
	};
}

#[macro_export]
/// Conditional import. Only import the crate if the target is the calculator.
macro_rules! calc_use {
	($crate_name: path) => {
		#[cfg(target_os = "none")]
		use $crate_name;
	};
}

#[macro_export]
/// Conditional import. Only import the crate if the target is the simulator.
macro_rules! sim_use {
	($crate_name: path) => {
		#[cfg(not(target_os = "none"))]
		use $crate_name;
	};
}
