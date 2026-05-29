pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 4; // Minimum 2

    pub const MIN_FOV: f32 = 30.;
    pub const MAX_FOV: f32 = 110.;

    pub const FOV: f32 = 45.;

    #[cfg(feature = "epsilon")]
    pub const MAX_TRIANGLES: usize = 800;
    #[cfg(feature = "upsilon")]
    pub const MAX_TRIANGLES: usize = 600; // Sorry Upsilon users

    pub const BLURING_SCREEN_SUBDIVISION: usize = 5;
    pub const BLURING_RADIUS: isize = 2;

    pub const CAMERA_ROTATION_SPEED: f32 = 3.0;
}
