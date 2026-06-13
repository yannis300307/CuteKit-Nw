use core::mem::transmute;

use nalgebra::Vector2;

use crate::{
    nadk::display::{COLOR_BLACK, COLOR_WHITE, Color565},
    renderer::drawing::DrawInfo,
    renderer2d::{
        renderer::{Renderer2d, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, ScaleMode},
        shapes::{get_pixel, put_pixel},
    },
};

#[derive(Clone)]
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

impl<'a, const SIZE: usize> Renderer2d<'_, SIZE> {
    pub fn draw_region(
        &mut self,
        parts: &NinePartsTexture,
        out_buffer_start: Vector2<isize>,
        texture_buffer_start: Vector2<isize>,
        mut width: isize,
        mut height: isize,
    ) {
        if out_buffer_start.x + width < 0
            || out_buffer_start.y + height < 0
            || out_buffer_start.x > SCREEN_TILE_WIDTH as isize
            || out_buffer_start.y > SCREEN_TILE_HEIGHT as isize
        {
            return;
        }

        let color_vec: &'static [TransparentRGB565] = unsafe { transmute(parts.data) }; // Bro, this is fine

        let mut start = Vector2::new(0, 0);

        // Don't draw pixels out of the screen please
        if out_buffer_start.x < 0 {
            start.x = -out_buffer_start.x;
        }
        if out_buffer_start.y < 0 {
            start.y = -out_buffer_start.y;
        }
        if out_buffer_start.x + width > SCREEN_TILE_WIDTH as isize {
            width = SCREEN_TILE_WIDTH as isize - out_buffer_start.x;
        }
        if out_buffer_start.y + height > SCREEN_TILE_HEIGHT as isize {
            height = SCREEN_TILE_HEIGHT as isize - out_buffer_start.y;
        }

        for y in start.y..height {
            for x in start.x..width {
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
                    self.draw_pixel(
                        (out_buffer_start.x + x) as usize,
                        (out_buffer_start.y + y) as usize,
                        color.rgb,
                    );
                } else {
                    let old_color =
                        self.tile_frame_buffer[(x + y * SCREEN_TILE_WIDTH as isize) as usize];
                    self.draw_pixel(
                        (out_buffer_start.x + x) as usize,
                        (out_buffer_start.y + y) as usize,
                        add_alpha_color(old_color, color),
                    );
                }
            }
        }
    }

    pub fn draw_region_strech(
        &mut self,
        parts: &NinePartsTexture,
        out_buffer_start: Vector2<isize>,
        texture_buffer_start: Vector2<isize>,
        tex_width: isize,
        tex_height: isize,
        out_width: isize,
        out_height: isize,
    ) {
        if out_buffer_start.x + out_width < 0
            || out_buffer_start.y + out_height < 0
            || out_buffer_start.x > SCREEN_TILE_WIDTH as isize
            || out_buffer_start.y > SCREEN_TILE_HEIGHT as isize
        {
            return;
        }

        let mut start = Vector2::new(0, 0);
        let mut max_size = Vector2::new(out_width, out_height);

        if out_buffer_start.x < 0 {
            start.x = -out_buffer_start.x;
        }
        if out_buffer_start.y < 0 {
            start.y = -out_buffer_start.y;
        }
        if out_buffer_start.x + out_width > SCREEN_TILE_WIDTH as isize {
            max_size.x = SCREEN_TILE_WIDTH as isize - out_buffer_start.x;
        }
        if out_buffer_start.y + out_height > SCREEN_TILE_HEIGHT as isize {
            max_size.y = SCREEN_TILE_HEIGHT as isize - out_buffer_start.y;
        }

        let color_vec: &'static [TransparentRGB565] = unsafe { transmute(parts.data) }; // Bro, this is fine
        for y in start.y..max_size.y {
            let pix_y = texture_buffer_start.y + y * tex_height / out_height;
            for x in start.x..max_size.x {
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
                    self.draw_pixel(
                        (out_buffer_start.x + x) as usize,
                        (out_buffer_start.y + y) as usize,
                        color.rgb,
                    );
                } else {
                    let old_color =
                        self.tile_frame_buffer[(x + y * SCREEN_TILE_WIDTH as isize) as usize];
                    self.draw_pixel(
                        (out_buffer_start.x + x) as usize,
                        (out_buffer_start.y + y) as usize,
                        add_alpha_color(old_color, color),
                    );
                }
            }
        }
    }

    pub fn draw_region_tile(
        &mut self,
        parts: &NinePartsTexture,
        out_buffer_start: Vector2<isize>,
        texture_buffer_start: Vector2<isize>,
        tex_width: isize,
        tex_height: isize,
        out_width: isize,
        out_height: isize,
    ) {
        if out_buffer_start.x + out_width < 0
            || out_buffer_start.y + out_height < 0
            || out_buffer_start.x > SCREEN_TILE_WIDTH as isize
            || out_buffer_start.y > SCREEN_TILE_HEIGHT as isize
        {
            return;
        }

        let mut start = Vector2::new(0, 0);
        let mut max_size = Vector2::new(out_width, out_height);
        
        let mut tex_start = Vector2::new(0, 0);

        if out_buffer_start.x < 0 {
            start.x = -out_buffer_start.x;
            tex_start.x += -out_buffer_start.x % tex_width;
        }
        if out_buffer_start.y < 0 {
            start.y = -out_buffer_start.y;
            tex_start.y += -out_buffer_start.y % tex_height;
        }
        if out_buffer_start.x + out_width > SCREEN_TILE_WIDTH as isize {
            max_size.x = SCREEN_TILE_WIDTH as isize - out_buffer_start.x;
        }
        if out_buffer_start.y + out_height > SCREEN_TILE_HEIGHT as isize {
            max_size.y = SCREEN_TILE_HEIGHT as isize - out_buffer_start.y;
        }

        let mut tex_x = 0;
        let mut tex_y = tex_start.y;

        let color_vec: &'static [TransparentRGB565] = unsafe { transmute(parts.data) }; // Bro, this is fine
        for y in start.y..max_size.y {
            let pix_y = texture_buffer_start.y + tex_y;
            tex_x = tex_start.x;
            for x in start.x..max_size.x {
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
                    self.draw_pixel(
                        (out_buffer_start.x + x) as usize,
                        (out_buffer_start.y + y) as usize,
                        color.rgb,
                    );
                } else {
                    let old_color =
                        self.tile_frame_buffer[(x + y * SCREEN_TILE_WIDTH as isize) as usize];
                    self.draw_pixel(
                        (out_buffer_start.x + x) as usize,
                        (out_buffer_start.y + y) as usize,
                        add_alpha_color(old_color, color),
                    );
                }
                tex_x += 1;
                if tex_x >= tex_width {
                    tex_x = 0;
                }
            }
            tex_y += 1;
            if tex_y >= tex_height {
                tex_y = 0;
            }
        }
    }

    pub fn draw_nine_parts_rectangle(
        &mut self,
        parts: NinePartsTexture,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        scaling_mode: ScaleMode,
    ) {
        let size = size.map(|x| x as isize);

        if pos.x + size.x < 0
            && pos.x > SCREEN_TILE_WIDTH as isize
            && pos.y + size.x < 0
            && pos.y < SCREEN_TILE_HEIGHT as isize
        {
            return;
        }

        let top_left_corner = pos;
        let bottom_left_corner = pos + Vector2::new(0, size.y - parts.bottom_border_size as isize);
        let bottom_right_corner = pos
            + Vector2::new(
                size.x - parts.right_border_size as isize,
                size.y - parts.bottom_border_size as isize,
            );
        let top_right_corner = pos + Vector2::new(size.x - parts.right_border_size as isize, 0);

        let top_corner = pos + Vector2::new(parts.left_border_size as isize, 0);
        let bottom_corner = pos
            + Vector2::new(
                parts.left_border_size as isize,
                size.y - parts.bottom_border_size as isize,
            );
        let left_corner = pos + Vector2::new(0, parts.top_border_size as isize);
        let right_corner = pos
            + Vector2::new(
                size.x - parts.right_border_size as isize,
                parts.top_border_size as isize,
            );

        let center_corner = pos
            + Vector2::new(
                parts.left_border_size as isize,
                parts.top_border_size as isize,
            );

        self.draw_region(
            &parts,
            top_left_corner,
            Vector2::new(0, 0),
            parts.left_border_size as isize,
            parts.top_border_size as isize,
        );

        self.draw_region(
            &parts,
            bottom_left_corner,
            Vector2::new(
                0,
                (parts.texture_height - parts.bottom_border_size) as isize,
            ),
            parts.left_border_size as isize,
            parts.bottom_border_size as isize,
        );

        self.draw_region(
            &parts,
            bottom_right_corner,
            Vector2::new(
                (parts.texture_width - parts.right_border_size) as isize,
                (parts.texture_height - parts.bottom_border_size) as isize,
            ),
            parts.right_border_size as isize,
            parts.bottom_border_size as isize,
        );

        self.draw_region(
            &parts,
            top_right_corner,
            Vector2::new((parts.texture_width - parts.right_border_size) as isize, 0),
            parts.right_border_size as isize,
            parts.top_border_size as isize,
        );

        match scaling_mode {
            ScaleMode::Stretch => {
                self.draw_region_strech(
                    &parts,
                    top_corner,
                    Vector2::new((parts.left_border_size) as isize, 0),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    size.x - (parts.left_border_size + parts.right_border_size) as isize,
                    parts.top_border_size as isize,
                );

                self.draw_region_strech(
                    &parts,
                    bottom_corner,
                    Vector2::new(
                        (parts.left_border_size) as isize,
                        (parts.texture_height - parts.bottom_border_size) as isize,
                    ),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    size.x - (parts.left_border_size + parts.right_border_size) as isize,
                    parts.bottom_border_size as isize,
                );

                self.draw_region_strech(
                    &parts,
                    left_corner,
                    Vector2::new(0, parts.top_border_size as isize),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    parts.left_border_size as isize,
                    size.y - (parts.top_border_size + parts.bottom_border_size) as isize,
                );

                self.draw_region_strech(
                    &parts,
                    right_corner,
                    Vector2::new(
                        (parts.texture_width - parts.right_border_size) as isize,
                        parts.top_border_size as isize,
                    ),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    parts.right_border_size as isize,
                    size.y - (parts.top_border_size + parts.bottom_border_size) as isize,
                );

                self.draw_region_strech(
                    &parts,
                    center_corner,
                    Vector2::new(
                        parts.left_border_size as isize,
                        parts.top_border_size as isize,
                    ),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    size.x - (parts.left_border_size + parts.right_border_size) as isize,
                    size.y - (parts.top_border_size + parts.bottom_border_size) as isize,
                );
            }
            ScaleMode::Tile => {
                self.draw_region_tile(
                    &parts,
                    top_corner,
                    Vector2::new((parts.left_border_size) as isize, 0),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    size.x - (parts.left_border_size + parts.right_border_size) as isize,
                    parts.top_border_size as isize,
                );

                self.draw_region_tile(
                    &parts,
                    bottom_corner,
                    Vector2::new(
                        (parts.left_border_size) as isize,
                        (parts.texture_height - parts.bottom_border_size) as isize,
                    ),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    size.x - (parts.left_border_size + parts.right_border_size) as isize,
                    parts.bottom_border_size as isize,
                );

                self.draw_region_tile(
                    &parts,
                    left_corner,
                    Vector2::new(0, parts.top_border_size as isize),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    parts.left_border_size as isize,
                    size.y - (parts.top_border_size + parts.bottom_border_size) as isize,
                );

                self.draw_region_tile(
                    &parts,
                    right_corner,
                    Vector2::new(
                        (parts.texture_width - parts.right_border_size) as isize,
                        parts.top_border_size as isize,
                    ),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    parts.right_border_size as isize,
                    size.y - (parts.top_border_size + parts.bottom_border_size) as isize,
                );

                self.draw_region_tile(
                    &parts,
                    center_corner,
                    Vector2::new(
                        parts.left_border_size as isize,
                        parts.top_border_size as isize,
                    ),
                    parts.right_border_size as isize,
                    parts.top_border_size as isize,
                    size.x - (parts.left_border_size + parts.right_border_size) as isize,
                    size.y - (parts.top_border_size + parts.bottom_border_size) as isize,
                );
            }
        }
    }
}
