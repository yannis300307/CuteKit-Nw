use core::mem::transmute;

use nalgebra::Vector2;

use crate::{draw_tools::{self, shapes::{get_pixel, put_pixel}}, nadk::display::Color565, renderer::drawing::DrawInfo};


static TEXTURE: &[u8] = include_bytes!("../target/assets/texture.bin");

pub fn draw_ui(draw_info: &DrawInfo, frame_buffer: &mut [Color565])
{
    return;
        let parts: draw_tools::images::NinePartsTexture = draw_tools::images::NinePartsTexture {
            data: include_bytes!("../target/assets/9parts.bin"),
            texture_width: 60,
            texture_height: 60,
            left_border_size: 20,
            right_border_size: 20,
            top_border_size: 20,
            bottom_border_size: 20,
            
        };

        draw_tools::images::nine_parts_rectangle(&parts,
        Vector2::new(100, 100),
        120,
        120,
        draw_tools::images::NinePartsScaleMode::Tile,
        frame_buffer,
        draw_info
    );

    draw_tools::images::nine_parts_rectangle(&parts,
        Vector2::new(230, 100),
        120,
        120,
        draw_tools::images::NinePartsScaleMode::Stretch,
        frame_buffer,
        draw_info
    );

}