use nalgebra::Vector2;

use crate::{
    constants::rendering::*, nadk::display::{Color565, ScreenRect, push_rect}, renderer::Renderer, renderer2d::{elements::CustomPlugin, renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH}},
};

impl<'a> Renderer<'a> {
    pub fn clear_intermediate_buffers(&mut self) {
        self.transformed_vertex_buffer.clear();
        self.projected_buffer.clear();
    }
}

impl<'a> CustomPlugin for Renderer<'a> {
    fn draw(
        &mut self,
        buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
        offset: Vector2<isize>,
    ) {
        for i in 0..SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT {
            self.tile_depth_buffer[i] = f16::MAX;
        }

        if !self.flat_triangles_to_render.is_empty() {
            self.draw_flat_triangles(offset, buffer);
        }
        if !self.tex_triangles_to_render.is_empty() {
            self.draw_tex_triangles(
                offset,
                self.texture.expect(
                    "Trying to use textured triangles without a texture loaded in the 3D renderer.",
                ),
                buffer,
            );
        }
    }

    fn pre_frame(&mut self) {
        self.mat_view = self.get_mat_view();
    }

    fn post_frame(&mut self) {
        self.tex_triangles_to_render.clear();
        self.flat_triangles_to_render.clear();
        self.transformed_vertex_buffer.clear();
        self.projected_buffer.clear();
    }
}
