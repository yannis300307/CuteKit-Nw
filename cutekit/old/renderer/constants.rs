pub mod rendering {
	#[cfg(feature = "epsilon")]
	pub const MAX_TRIANGLES: usize = 800;
	#[cfg(feature = "upsilon")]
	// Sorry Upsilon users
	pub const MAX_TRIANGLES: usize = 600;
}
