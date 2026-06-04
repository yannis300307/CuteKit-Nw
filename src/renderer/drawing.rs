use crate::{
    constants::rendering::*,
    nadk::display::{Color565, ScreenRect, push_rect, wait_for_vblank},
    renderer::{Renderer, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH},
};

pub struct DrawInfo {
    pub buffer_width: isize,
    pub buffer_height: isize,
    pub offset_x: isize,
    pub offset_y: isize,
}

impl Renderer {
    pub fn clear_intermediate_buffers(&mut self)
    {
        self.transformed_vertex_buffer.clear();
        self.projected_buffer.clear();
    }

    pub fn draw_game(
        &mut self,
        custom_layer_function: Option<&dyn Fn(&DrawInfo, &mut [Color565])>,
    ) {
        self.mat_view = self.get_mat_view();

        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_screen(Color565::new(0b01110, 0b110110, 0b11111));
                for i in 0..SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT {
                    self.tile_depth_buffer[i] = f16::MAX;
                }
                self.draw_triangles(x, y);

                let drawing_info = DrawInfo {
                    buffer_width: SCREEN_TILE_WIDTH as isize,
                    buffer_height: SCREEN_TILE_HEIGHT as isize,
                    offset_x: (x * SCREEN_TILE_WIDTH) as isize,
                    offset_y: (y * SCREEN_TILE_HEIGHT) as isize,
                };

                if let Some(func) = custom_layer_function {
                    func(&drawing_info, &mut self.tile_frame_buffer);
                }

                push_rect(
                    ScreenRect {
                        x: (SCREEN_TILE_WIDTH * x) as u16,
                        y: (SCREEN_TILE_HEIGHT * y) as u16,
                        width: SCREEN_TILE_WIDTH as u16,
                        height: SCREEN_TILE_HEIGHT as u16,
                    },
                    &self.tile_frame_buffer,
                );
            }
        }
        if self.enable_vsync {
            wait_for_vblank();
        }
        self.triangles_to_render.clear();
        self.transformed_vertex_buffer.clear();
        self.projected_buffer.clear();
    }
}
