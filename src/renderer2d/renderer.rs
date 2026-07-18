use libm::roundf;
use nalgebra::Vector2;

use crate::{
    constants::rendering::*,
    nadk::display::{COLOR_BLACK, COLOR_BLUE, Color565, ScreenRect, push_rect},
    renderer2d::{
        draw_queue::DrawQueue,
        elements::{CustomPlugin, Element, Font},
        sprite::TransparentTexture,
        textured_triangle::TexTriangle2D,
    },
};

pub const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH.div_ceil(SCREEN_TILE_SUBDIVISION);
pub const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT.div_ceil(SCREEN_TILE_SUBDIVISION);

#[inline]
fn add_alpha_color(a: Color565, b: Color565, b_alpha: u16) -> Color565 {
    let a_comp = a.get_components();
    let b_comp = b.get_components();
    Color565::new(
        ((255 - b_alpha) * a_comp.0 + b_comp.0 * b_alpha) / 255,
        ((255 - b_alpha) * a_comp.1 + b_comp.1 * b_alpha) / 255,
        ((255 - b_alpha) * a_comp.2 + b_comp.2 * b_alpha) / 255,
    )
}

pub struct Renderer2d {
    pub(super) tile_frame_buffer: [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    clear_color: Color565,
}

impl<'a> Renderer2d {
    pub fn new(clear_color: Color565) -> Self {
        return Self {
            tile_frame_buffer: [COLOR_BLACK; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
            clear_color,
        };
    }

    fn clear_frame_frame_buffer(&mut self) {
        for i in 0..SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT {
            self.tile_frame_buffer[i] = self.clear_color;
        }
    }

    #[inline(always)]
    pub(super) fn draw_pixel(&mut self, x: usize, y: usize, color: Color565) {
        self.tile_frame_buffer[x + y * SCREEN_TILE_WIDTH] = color;
    }

    #[inline]
    pub(super) fn get_pixel(&self, x: usize, y: usize) -> Color565 {
        self.tile_frame_buffer[x + y * SCREEN_TILE_WIDTH]
    }

    fn draw_text(
        &mut self,
        pos: Vector2<isize>,
        text: &'a str,
        font: &Font,
        font_color: Color565,
        background_color: Option<Color565>,
    ) {
        let char_size = Vector2::new(font.char_width as isize, font.char_height as isize);

        if pos.x > SCREEN_TILE_WIDTH as isize
            || pos.y > SCREEN_TILE_HEIGHT as isize
            || pos.x + text.len() as isize * char_size.x < 0
            || pos.y + char_size.y < 0
        {
            return;
        }

        let font_width = font.font_image_width as isize;

        let mut start_char_index: usize = 0;
        if pos.x < 0 {
            start_char_index = (-pos.x / char_size.x) as usize;
        }

        if start_char_index > text.len() {
            return;
        }

        let max_char_count = (SCREEN_TILE_WIDTH) / (char_size.x) as usize + 2; // TODO: could be replaced by a + 1 by fixing the end character dropping bug

        let mut max_char_y = char_size.y;
        if pos.y + char_size.y > SCREEN_TILE_HEIGHT as isize {
            max_char_y = SCREEN_TILE_HEIGHT as isize - pos.y;
        }

        let mut start_char_y = 0;
        if pos.y < 0 {
            start_char_y = -pos.y;
        }

        for index in start_char_index..text.len().min(start_char_index + max_char_count) {
            let char: isize = font
                .chars
                .find(|c| c as u8 == text.as_bytes()[index])
                .unwrap() as isize;

            let letter_offset = char_size.x * char;

            // Destination x relative to the start of the text
            let out_letter_offset = index as isize * char_size.x;

            let mut max_char_width = char_size.x;

            // Right of the letter
            let letter_x_stop = pos.x + out_letter_offset + char_size.x;
            // Adjust the max width to avoid overflow
            if letter_x_stop > SCREEN_TILE_WIDTH as isize {
                max_char_width = char_size.x - (letter_x_stop - SCREEN_TILE_WIDTH as isize);
            }

            let mut letter_x_start = 0;
            if pos.x + out_letter_offset < 0 {
                letter_x_start = -(pos.x + out_letter_offset);
            }

            if let Some(background_color) = background_color {
                for y in start_char_y..max_char_y {
                    let mut out_x = out_letter_offset + letter_x_start;
                    for x in letter_x_start..max_char_width {
                        let pixel =
                            font.data[((letter_offset + x) + font_width * y) as usize] as u16;
                        let color = add_alpha_color(background_color, font_color, pixel);
                        self.draw_pixel((pos.x + out_x) as usize, (pos.y + y) as usize, color);
                        out_x += 1;
                    }
                }
            } else {
                for y in start_char_y..max_char_y {
                    let mut out_x = out_letter_offset + letter_x_start;
                    for x in letter_x_start..max_char_width {
                        let pixel =
                            font.data[((letter_offset + x) + font_width * y) as usize] as u16;
                        let background_color =
                            self.get_pixel((pos.x + out_x) as usize, (pos.y + y) as usize);
                        let color = add_alpha_color(background_color, font_color, pixel);
                        self.draw_pixel((pos.x + out_x) as usize, (pos.y + y) as usize, color);
                        out_x += 1;
                    }
                }
            }
        }
    }

    fn draw_rectangle(&mut self, mut pos: Vector2<isize>, size: Vector2<isize>, color: Color565) {
        let mut end = pos + size;
        if end.x < 0
            || end.y < 0
            || pos.x > SCREEN_TILE_WIDTH as isize
            || pos.y > SCREEN_TILE_HEIGHT as isize
        {
            return;
        }

        if pos.x < 0 {
            pos.x = 0;
        }
        if pos.y < 0 {
            pos.y = 0;
        }
        if end.x >= SCREEN_TILE_WIDTH as isize {
            end.x = SCREEN_TILE_WIDTH as isize;
        }
        if end.y >= SCREEN_TILE_HEIGHT as isize {
            end.y = SCREEN_TILE_HEIGHT as isize;
        }

        for y in pos.y..end.y {
            for x in pos.x..end.x {
                self.draw_pixel(x as usize, y as usize, color);
            }
        }
    }

    #[inline]
    fn draw_horizontal_line(
        &mut self,
        y: isize,
        x_start: isize,
        mut x_stop: isize,
        color: Color565,
    ) {
        let mut x = x_start;
        if y < 0
            || y >= SCREEN_TILE_HEIGHT as isize
            || x_start > SCREEN_TILE_WIDTH as isize
            || x_stop < 0
        {
            return;
        }
        if x < 0 {
            x = 0;
        }
        if x_stop >= SCREEN_TILE_WIDTH as isize {
            x_stop = SCREEN_TILE_WIDTH as isize - 1;
        }
        while x <= x_stop {
            self.draw_pixel(x as usize, y as usize, color);
            x += 1;
        }
    }

    /// Based on the Midpoint algorithm on Wikipedia: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    pub(super) fn draw_circle(&mut self, r: f32, center: Vector2<isize>, color: Color565) {
        let mut t1 = r / 16.0;
        let mut x = r;
        let mut y = 0.0;
        while x >= y {
            self.draw_horizontal_line(
                center.y + y as isize,
                center.x - x as isize,
                center.x + x as isize,
                color,
            );
            self.draw_horizontal_line(
                center.y - y as isize,
                center.x - x as isize,
                center.x + x as isize,
                color,
            );
            self.draw_horizontal_line(
                center.y + x as isize,
                center.x - y as isize,
                center.x + y as isize,
                color,
            );
            self.draw_horizontal_line(
                center.y - x as isize,
                center.x - y as isize,
                center.x + y as isize,
                color,
            );
            y += 1.0;
            t1 += y;
            let t2 = t1 - x;
            if t2 >= 0.0 {
                t1 = t2;
                x -= 1.0;
            }
        }
    }

    pub(super) fn draw_rounded_rectangle(
        &mut self,
        r: f32,
        pos: Vector2<isize>,
        size: Vector2<isize>,
        color: Color565,
    ) {
        let mut t1 = r / 16.0;
        let mut x = r;
        let mut y = 0.0;
        let r_isize = r as isize;
        while x >= y {
            let rounded_x = roundf(x) as isize;
            let rounded_y = roundf(y) as isize;
            // Fill the rounded parts
            self.draw_horizontal_line(
                pos.y - rounded_y + r_isize,
                pos.x + r_isize - rounded_x,
                pos.x + size.x + rounded_x - r_isize,
                color,
            );
            self.draw_horizontal_line(
                pos.y + size.y + rounded_y - r_isize - 1,
                pos.x + r_isize - rounded_x,
                pos.x + size.x + rounded_x - r_isize,
                color,
            );
            self.draw_horizontal_line(
                pos.y - rounded_x + r_isize,
                pos.x + r_isize - rounded_y,
                pos.x + size.x + rounded_y - r_isize,
                color,
            );
            self.draw_horizontal_line(
                pos.y + size.y + rounded_x - r_isize - 1,
                pos.x + r_isize - rounded_y,
                pos.x + size.x + rounded_y - r_isize,
                color,
            );
            y += 1.0;
            t1 += y;
            let t2 = t1 - x;
            if t2 >= 0.0 {
                t1 = t2;
                x -= 1.0;
            }
        }
        self.draw_rectangle(
            Vector2::new(pos.x, pos.y + r_isize),
            Vector2::new(size.x, size.y - r_isize * 2),
            color,
        );
    }

    // Adapted from http://members.chello.at/~easyfilter/bresenham.html
    fn draw_line(
        &mut self,
        mut start: Vector2<isize>,
        stop: Vector2<isize>,
        mut width: f32,
        color: Color565,
    ) {
        let d = (stop - start).abs();
        let s = Vector2::new(
            if start.x < stop.x { 1 } else { -1 },
            if start.y < stop.y { 1 } else { -1 },
        );
        let mut err = d.x - d.y;
        let mut x2;
        let mut y2;
        let mut e2;
        let ed = if d.x + d.y == 0 {
            1.0
        } else {
            d.map(|x| x as f32).norm()
        };
        width = (width + 1.0) / 2.0;
        loop {
            let opacity = (255.0 * (width - (err - d.x + d.y).abs() as f32 / ed)).clamp(0.0, 255.0);
            let bg_color = self.get_pixel(start.x as usize, start.y as usize);
            self.draw_pixel(
                start.x as usize,
                start.y as usize,
                add_alpha_color(bg_color, color, opacity as u16),
            );
            e2 = err;
            x2 = start.x;
            if 2 * e2 >= -d.x {
                e2 += d.y;
                y2 = start.y;
                while (e2 as f32) < ed * width && (stop.y != y2 || d.x > d.y) {
                    y2 += s.y;
                    let opacity = (255.0 * (width - e2.abs() as f32 / ed)).clamp(0.0, 255.0);
                    let bg_color = self.get_pixel(start.x as usize, y2 as usize);
                    self.draw_pixel(
                        start.x as usize,
                        y2 as usize,
                        add_alpha_color(bg_color, color, opacity as u16),
                    );
                    e2 += d.x;
                }
                if start.x == stop.x {
                    break;
                };
                e2 = err;
                err -= d.y;
                start.x += s.x;
            }
            if 2 * e2 <= d.y {
                e2 = d.x - e2;
                while (e2 as f32) < ed * width && (stop.x != x2 || d.x < d.y) {
                    x2 += s.x;
                    let opacity = (255.0 * (width - e2.abs() as f32 / ed)).clamp(0.0, 255.0);
                    let bg_color = self.get_pixel(x2 as usize, start.y as usize);
                    self.draw_pixel(
                        x2 as usize,
                        start.y as usize,
                        add_alpha_color(bg_color, color, opacity as u16),
                    );
                    e2 += d.y;
                }
                if start.y == stop.y {
                    break;
                }
                err += d.x;
                start.y += s.y;
            }
        }
    }

    fn draw_shapes(
        &mut self,
        buffer_offset: Vector2<isize>,
        draw_queue: &mut core::slice::IterMut<'_, Element<'_>>,
    ) {
        for element in draw_queue.by_ref() {
            match element {
                Element::ColorRectangle { pos, size, color } => {
                    self.draw_rectangle(*pos - buffer_offset, size.map(|x| x as isize), *color);
                }
                Element::TransparentSprite { pos, texture } => self.draw_region(
                    texture,
                    *pos - buffer_offset,
                    Vector2::repeat(0),
                    texture.width as isize,
                    texture.height as isize,
                ),
                Element::TransparentScaledSprite {
                    pos,
                    size,
                    texture,
                    scale_mode,
                } => match scale_mode {
                    super::elements::ScaleMode::Stretch => self.draw_region_strech(
                        texture,
                        *pos - buffer_offset,
                        Vector2::repeat(0),
                        texture.width as isize,
                        texture.height as isize,
                        size.x as isize,
                        size.y as isize,
                    ),
                    super::elements::ScaleMode::Tile => self.draw_region_tile(
                        texture,
                        *pos - buffer_offset,
                        Vector2::repeat(0),
                        texture.width as isize,
                        texture.height as isize,
                        size.x as isize,
                        size.y as isize,
                    ),
                },
                Element::NinePartsRectangle {
                    parts,
                    pos,
                    size,
                    scaling_mode,
                } => self.draw_nine_parts_rectangle(
                    parts,
                    *pos - buffer_offset,
                    *size,
                    *scaling_mode,
                ),
                Element::Circle {
                    center,
                    radius,
                    color,
                } => self.draw_circle(*radius as f32, *center - buffer_offset, *color),
                Element::RoundedRectangle {
                    pos,
                    size,
                    corner_radius,
                    color,
                } => self.draw_rounded_rectangle(
                    *corner_radius,
                    *pos - buffer_offset,
                    size.map(|x| x as isize),
                    *color,
                ),
                Element::Text {
                    pos,
                    text,
                    font,
                    font_color,
                    background_color,
                } => self.draw_text(
                    *pos - buffer_offset,
                    text,
                    *font,
                    *font_color,
                    *background_color,
                ),
                Element::TexturedTriangle {
                    p1,
                    p2,
                    p3,
                    t1,
                    t2,
                    t3,
                    texture,
                } => {
                    let buffer_offset = buffer_offset.map(|x| x as i16);
                    self.draw_textured_triangle(
                        *p1 - buffer_offset,
                        *p2 - buffer_offset,
                        *p3 - buffer_offset,
                        *t1,
                        *t2,
                        *t3,
                        texture,
                    )
                }
                Element::CustomPlugin { object } => {
                    object.draw(&mut self.tile_frame_buffer, buffer_offset);
                }
            }
        }
    }

    pub fn draw_textured_triangle(
        &mut self,
        p1: Vector2<i16>,
        p2: Vector2<i16>,
        p3: Vector2<i16>,
        t1: Vector2<f32>,
        t2: Vector2<f32>,
        t3: Vector2<f32>,
        texture: &'a TransparentTexture,
    ) {
        let tri = TexTriangle2D {
            p1,
            p2,
            p3,
            t1,
            t2,
            t3,
        };
        self.clip_and_draw_2d_triangle(tri, texture);
    }

    pub fn draw<const SIZE: usize>(&mut self, draw_queue: &mut DrawQueue<SIZE>) {
        for element in draw_queue.get_iterator() {
            if let Element::CustomPlugin { object } = element {
                object.pre_frame();
            }
        }

        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_frame_frame_buffer();

                self.draw_shapes(
                    Vector2::new(
                        (x * SCREEN_TILE_WIDTH) as isize,
                        (y * SCREEN_TILE_HEIGHT) as isize,
                    ),
                    &mut draw_queue.get_iterator(),
                );

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

        for element in draw_queue.get_iterator() {
            if let Element::CustomPlugin { object } = element {
                object.post_frame();
            }
        }
    }
}
