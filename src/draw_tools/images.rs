use core::mem::transmute;

use nalgebra::Vector2;

use crate::{
    draw_tools::shapes::{get_pixel, put_pixel},
    nadk::display::{COLOR_BLACK, Color565}, renderer::drawing::DrawInfo,
};

pub struct NinePartsTexture {
    pub data: &'static [u8],
    pub texture_width: usize,
    pub texture_height: usize,
    pub left_border_size: usize,
    pub right_border_size: usize,
    pub top_border_size: usize,
    pub bottom_border_size: usize,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct TransparentRGB565 {
    pub rgb: Color565,
    pub alpha: u8,
}

#[inline]
fn get_pixel_transparent(
    buffer: &[TransparentRGB565],
    buffer_width: isize,
    buffer_height: isize,
    x: isize,
    y: isize,
) -> TransparentRGB565 {
    if x < 0 || x >= buffer_width {
        return TransparentRGB565 {
            rgb: COLOR_BLACK,
            alpha: 0,
        };
    }
    if y < 0 || y >= buffer_height {
        return TransparentRGB565 {
            rgb: COLOR_BLACK,
            alpha: 0,
        };
    }
    buffer[(x + y * buffer_width) as usize]
}

#[inline]
fn add_alpha_color(a: Color565, b: TransparentRGB565) -> Color565 {
    let rgb = b.rgb;
    let a_comp = a.get_components();
    let b_comp = rgb.get_components();
    Color565::new(
        (255 - b.alpha as u16) * a_comp.0 + b_comp.0 * b.alpha as u16,
        (255 - b.alpha as u16) * a_comp.1 + b_comp.1 * b.alpha as u16,
        (255 - b.alpha as u16) * a_comp.2 + b_comp.2 * b.alpha as u16,
    )
}

pub fn draw_region(
    parts: &NinePartsTexture,
    out_buffer_start: Vector2<isize>,
    texture_buffer_start: Vector2<isize>,
    width: isize,
    height: isize,
    buffer: &mut [Color565],
    buffer_width: isize,
    buffer_height: isize,
) {
    let color_vec: &'static [TransparentRGB565] = unsafe { transmute(parts.data) }; // Bro, this is fine
    for y in 0..height {
        for x in 0..width {
            let color = get_pixel_transparent(
                color_vec,
                parts.texture_width as isize,
                parts.texture_height as isize,
                texture_buffer_start.x + x,
                texture_buffer_start.y + y,
            );

            if color.alpha == 0 {
                continue;
            } else if color.alpha == 255 {
                put_pixel(
                    buffer,
                    buffer_width,
                    buffer_height,
                    out_buffer_start.x + x,
                    out_buffer_start.y + y,
                    color.rgb,
                );
            } else {
                let old_color = get_pixel(buffer, buffer_width, buffer_height, x, y);
                put_pixel(
                    buffer,
                    buffer_width,
                    buffer_height,
                    out_buffer_start.x + x,
                    out_buffer_start.y + y,
                    add_alpha_color(old_color, color),
                );
            }
        }
    }
}

pub fn draw_region_strech(
    parts: &NinePartsTexture,
    out_buffer_start: Vector2<isize>,
    texture_buffer_start: Vector2<isize>,
    tex_width: isize,
    tex_height: isize,
    out_width: isize,
    out_height: isize,
    buffer: &mut [Color565],
    buffer_width: isize,
    buffer_height: isize,
) {
    let color_vec: &'static [TransparentRGB565] = unsafe { transmute(parts.data) }; // Bro, this is fine
    for y in 0..out_height {
        let pix_y = texture_buffer_start.y + y * tex_height / out_height;
        for x in 0..out_width {
            let color = get_pixel_transparent(
                color_vec,
                parts.texture_width as isize,
                parts.texture_height as isize,
                texture_buffer_start.x + x * tex_width / out_width,
                pix_y,
            );

            if color.alpha == 0 {
                continue;
            } else if color.alpha == 255 {
                put_pixel(
                    buffer,
                    buffer_width,
                    buffer_height,
                    out_buffer_start.x + x,
                    out_buffer_start.y + y,
                    color.rgb,
                );
            } else {
                let old_color = get_pixel(buffer, buffer_width, buffer_height, x, y);
                put_pixel(
                    buffer,
                    buffer_width,
                    buffer_height,
                    out_buffer_start.x + x,
                    out_buffer_start.y + y,
                    add_alpha_color(old_color, color),
                );
            }
        }
    }
}

pub fn draw_region_tile(
    parts: &NinePartsTexture,
    out_buffer_start: Vector2<isize>,
    texture_buffer_start: Vector2<isize>,
    tex_width: isize,
    tex_height: isize,
    out_width: isize,
    out_height: isize,
    buffer: &mut [Color565],
    buffer_width: isize,
    buffer_height: isize,
) {
    let color_vec: &'static [TransparentRGB565] = unsafe { transmute(parts.data) }; // Bro, this is fine
    let mut tex_x = 0;
    let mut tex_y = 0;
    for y in 0..out_height {
        let pix_y = texture_buffer_start.y + tex_y;
        for x in 0..out_width {
            let color = get_pixel_transparent(
                color_vec,
                parts.texture_width as isize,
                parts.texture_height as isize,
                texture_buffer_start.x + tex_x,
                pix_y,
            );

            if color.alpha == 0 {
                continue;
            } else if color.alpha == 255 {
                put_pixel(
                    buffer,
                    buffer_width,
                    buffer_height,
                    out_buffer_start.x + x,
                    out_buffer_start.y + y,
                    color.rgb,
                );
            } else {
                let old_color = get_pixel(buffer, buffer_width, buffer_height, x, y);
                put_pixel(
                    buffer,
                    buffer_width,
                    buffer_height,
                    out_buffer_start.x + x,
                    out_buffer_start.y + y,
                    add_alpha_color(old_color, color),
                );
            }
            tex_x += 1;
            if tex_x >= tex_width {
                tex_x = 0;
            }
        }
        tex_y += 1;
        if tex_y >= tex_width {
            tex_y = 0;
        }
    }
}

pub enum NinePartsScaleMode {
    Stretch,
    Tile,
}

pub fn nine_parts_rectangle(
    parts: &NinePartsTexture,
    center: Vector2<isize>,
    width: isize,
    height: isize,
    scaling_mode: NinePartsScaleMode,
    buffer: &mut [Color565],
    draw_info: &DrawInfo,
) {
    let half_width = width / 2;
    let half_height = height / 2;

    let center = center - Vector2::new(draw_info.offset_x, draw_info.offset_y);

    if !(center.x + half_width >= 0
        && center.x - half_width < draw_info.buffer_width
        && center.y + half_height >= 0
        && center.y - half_height < draw_info.buffer_height)
    {
        return;
    }

    let top_left_corner = center + Vector2::new(-half_width, -half_height);
    let bottom_left_corner =
        center + Vector2::new(-half_width, half_height - parts.bottom_border_size as isize);
    let bottom_right_corner = center
        + Vector2::new(
            half_width - parts.right_border_size as isize,
            half_height - parts.bottom_border_size as isize,
        );
    let top_right_corner =
        center + Vector2::new(half_width - parts.right_border_size as isize, -half_height);

    let top_corner =
        center + Vector2::new(-half_width + parts.left_border_size as isize, -half_height);
    let bottom_corner = center
        + Vector2::new(
            -half_width + parts.left_border_size as isize,
            half_height - parts.bottom_border_size as isize,
        );
    let left_corner =
        center + Vector2::new(-half_width, -half_height + parts.top_border_size as isize);
    let right_corner = center
        + Vector2::new(
            half_width - parts.right_border_size as isize,
            -half_height + parts.top_border_size as isize,
        );

    let center_corner = center
        + Vector2::new(
            -half_width + parts.left_border_size as isize,
            -half_height + parts.top_border_size as isize,
        );

    draw_region(
        &parts,
        top_left_corner,
        Vector2::new(0, 0),
        parts.left_border_size as isize,
        parts.top_border_size as isize,
        buffer,
        draw_info.buffer_width,
        draw_info.buffer_height,
    );

    draw_region(
        &parts,
        bottom_left_corner,
        Vector2::new(
            0,
            (parts.texture_height - parts.bottom_border_size) as isize,
        ),
        parts.left_border_size as isize,
        parts.bottom_border_size as isize,
        buffer,
        draw_info.buffer_width,
        draw_info.buffer_height,
    );

    draw_region(
        &parts,
        bottom_right_corner,
        Vector2::new(
            (parts.texture_width - parts.right_border_size) as isize,
            (parts.texture_height - parts.bottom_border_size) as isize,
        ),
        parts.right_border_size as isize,
        parts.bottom_border_size as isize,
        buffer,
        draw_info.buffer_width,
        draw_info.buffer_height,
    );

    draw_region(
        &parts,
        top_right_corner,
        Vector2::new((parts.texture_width - parts.right_border_size) as isize, 0),
        parts.right_border_size as isize,
        parts.top_border_size as isize,
        buffer,
        draw_info.buffer_width,
        draw_info.buffer_height,
    );

    match scaling_mode {
        NinePartsScaleMode::Stretch => {
            draw_region_strech(
                &parts,
                top_corner,
                Vector2::new((parts.left_border_size) as isize, 0),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                width - (parts.left_border_size + parts.right_border_size) as isize,
                parts.top_border_size as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_strech(
                &parts,
                bottom_corner,
                Vector2::new(
                    (parts.left_border_size) as isize,
                    (parts.texture_height - parts.bottom_border_size) as isize,
                ),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                width - (parts.left_border_size + parts.right_border_size) as isize,
                parts.bottom_border_size as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_strech(
                &parts,
                left_corner,
                Vector2::new(0, parts.top_border_size as isize),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                parts.left_border_size as isize,
                height - (parts.top_border_size + parts.bottom_border_size) as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_strech(
                &parts,
                right_corner,
                Vector2::new(
                    (parts.texture_width - parts.right_border_size) as isize,
                    parts.top_border_size as isize,
                ),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                parts.right_border_size as isize,
                height - (parts.top_border_size + parts.bottom_border_size) as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_strech(
                &parts,
                center_corner,
                Vector2::new(
                    parts.left_border_size as isize,
                    parts.top_border_size as isize,
                ),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                width - (parts.left_border_size + parts.right_border_size) as isize,
                height - (parts.top_border_size + parts.bottom_border_size) as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );
        }
        NinePartsScaleMode::Tile => {
            draw_region_tile(
                &parts,
                top_corner,
                Vector2::new((parts.left_border_size) as isize, 0),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                width - (parts.left_border_size + parts.right_border_size) as isize,
                parts.top_border_size as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_tile(
                &parts,
                bottom_corner,
                Vector2::new(
                    (parts.left_border_size) as isize,
                    (parts.texture_height - parts.bottom_border_size) as isize,
                ),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                width - (parts.left_border_size + parts.right_border_size) as isize,
                parts.bottom_border_size as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_tile(
                &parts,
                left_corner,
                Vector2::new(0, parts.top_border_size as isize),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                parts.left_border_size as isize,
                height - (parts.top_border_size + parts.bottom_border_size) as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_tile(
                &parts,
                right_corner,
                Vector2::new(
                    (parts.texture_width - parts.right_border_size) as isize,
                    parts.top_border_size as isize,
                ),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                parts.right_border_size as isize,
                height - (parts.top_border_size + parts.bottom_border_size) as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );

            draw_region_tile(
                &parts,
                center_corner,
                Vector2::new(
                    parts.left_border_size as isize,
                    parts.top_border_size as isize,
                ),
                parts.right_border_size as isize,
                parts.top_border_size as isize,
                width - (parts.left_border_size + parts.right_border_size) as isize,
                height - (parts.top_border_size + parts.bottom_border_size) as isize,
                buffer,
                draw_info.buffer_width,
                draw_info.buffer_height,
            );
        }
    }
}
