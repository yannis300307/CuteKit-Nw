mod camera;
mod clipping;
pub mod drawing;
mod engine_3d;
mod flat;
mod matrix_utils;
pub mod mesh;
mod textured;

calc_use!(alloc::vec::Vec);

use nalgebra::{Matrix4, Perspective3, Vector2, Vector3};

use crate::constants::rendering::*;
use crate::renderer::camera::Camera;
use crate::renderer::mesh::{FlatCompactTriangle2D, TexCompactTriangle2D};
use crate::renderer2d::elements::Texture;
use crate::renderer2d::renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH};

// Screen size related constants

const SCREEN_WIDTHF: f32 = SCREEN_WIDTH as f32;
const SCREEN_HEIGHTF: f32 = SCREEN_HEIGHT as f32;
const HALF_SCREEN_WIDTHF: f32 = SCREEN_WIDTHF / 2.0;
const HALF_SCREEN_HEIGHTF: f32 = SCREEN_HEIGHTF / 2.0;
const HALF_SCREEN: Vector2<f32> = Vector2::new(HALF_SCREEN_WIDTHF, HALF_SCREEN_HEIGHTF);

// Projection parameters
const ASPECT_RATIO: f32 = SCREEN_WIDTHF / SCREEN_HEIGHTF;

const ZNEAR: f32 = 0.1;
const ZFAR: f32 = 1000.0;

pub struct Renderer<'a>
{
	pub camera:                Camera,
	tex_triangles_to_render:   Vec<TexCompactTriangle2D>,
	flat_triangles_to_render:  Vec<FlatCompactTriangle2D>,
	tile_depth_buffer:         [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
	projection_matrix:         Perspective3<f32>,
	pub enable_vsync:          bool,
	mat_view:                  Matrix4<f32>,
	projected_buffer:          Vec<Vector2<i16>>,
	transformed_vertex_buffer: Vec<Vector3<f32>>,
	texture:                   Option<&'a Texture>,
}

impl<'a> Renderer<'a>
{
	pub fn new() -> Self
	{
		let renderer: Renderer = Renderer {
			camera:                    Camera::new(),
			projection_matrix:         Perspective3::new(ASPECT_RATIO, FOV, ZNEAR, ZFAR),
			tex_triangles_to_render:   Vec::with_capacity(MAX_TRIANGLES),
			flat_triangles_to_render:  Vec::with_capacity(1),
			tile_depth_buffer:         [0.0; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
			enable_vsync:              true,
			mat_view:                  Matrix4::zeros(),
			projected_buffer:          Vec::new(),
			transformed_vertex_buffer: Vec::new(),
			texture:                   None,
		};

		renderer
	}

	pub fn load_texture(&mut self, texture: &'a Texture) { self.texture = Some(texture); }
}
