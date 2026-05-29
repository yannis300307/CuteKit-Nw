mod clipping;
mod draw_2d_triangles;
mod engine_3d;
mod matrix_utils;
pub mod mesh;
mod misc;
mod textured_triangle;

calc_use!(alloc::format);
calc_use!(alloc::vec::Vec);

use nalgebra::{Matrix4, Perspective3, Vector2, Vector3, Vector4};

use crate::{
    camera::Camera,
    constants::rendering::*,
    nadk::display::{COLOR_BLACK, Color565},
    renderer::mesh::IndexedTriangle2D,
};

// Screen size related constants

const SCREEN_WIDTHF: f32 = SCREEN_WIDTH as f32;
const SCREEN_HEIGHTF: f32 = SCREEN_HEIGHT as f32;
const HALF_SCREEN_WIDTHF: f32 = SCREEN_WIDTHF / 2.0;
const HALF_SCREEN_HEIGHTF: f32 = SCREEN_HEIGHTF / 2.0;
const HALF_SCREEN: Vector2<f32> = Vector2::new(HALF_SCREEN_WIDTHF, HALF_SCREEN_HEIGHTF);

// Screen tiling constants
const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH.div_ceil(SCREEN_TILE_SUBDIVISION);
const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT.div_ceil(SCREEN_TILE_SUBDIVISION);

// Projection parameters
const ASPECT_RATIO: f32 = SCREEN_WIDTHF / SCREEN_HEIGHTF;

const ZNEAR: f32 = 0.1;
const ZFAR: f32 = 1000.0;

static FONT_DATA: &[u8] = include_bytes!("../../target/assets/font.bin");
const FONT_WIDTH: usize = 1045;
const FONT_HEIGHT: usize = 15;

const FONT_CHAR_WIDTH: usize = 11;
static FONT_ORDER: &str = "!\" $%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^+`abcdefghijklmnopqrstuvwxyz{|}~€";

static TEXTURE: &[u8] = include_bytes!("../../target/assets/texture.bin");

pub struct Renderer {
    pub camera: Camera,
    triangles_to_render: Vec<IndexedTriangle2D>,
    tile_frame_buffer: [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    tile_depth_buffer: [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    projection_matrix: Perspective3<f32>,
    pub enable_vsync: bool,
    mat_view: Matrix4<f32>,
    projected_buffer: Vec<Vector2<i16>>,
    transformed_vertex_buffer: Vec<Vector3<f32>>
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {
            camera: Camera::new(),
            projection_matrix: Perspective3::new(ASPECT_RATIO, FOV, ZNEAR, ZFAR),
            triangles_to_render: Vec::with_capacity(MAX_TRIANGLES),
            tile_frame_buffer: [COLOR_BLACK; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
            tile_depth_buffer: [0.0; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
            enable_vsync: true,
            mat_view: Matrix4::zeros(),
            projected_buffer: Vec::new(),
            transformed_vertex_buffer: Vec::new()
        };

        renderer
    }
}
