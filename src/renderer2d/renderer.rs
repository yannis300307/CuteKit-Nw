use heapless::Vec;
use libm::roundf;
use nalgebra::Vector2;

use crate::{
    constants::rendering::*,
    nadk::display::{COLOR_BLACK, Color565, ScreenRect, push_rect},
    renderer2d::nine_parts_rectangle::NinePartsTexture,
};

pub const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH.div_ceil(SCREEN_TILE_SUBDIVISION);
pub const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT.div_ceil(SCREEN_TILE_SUBDIVISION);

/// A texture struct that store a size and a reference to the actual pixels.
#[derive(Clone)]
pub struct Texture {
    width: u16,
    height: u16,
    data: &'static [Color565],
}

/// A scaling mode enum used in elements that requiere a scalling strategy.
/// `Stretch` will distort the texture to match the size of the area
/// that needs to be drawn while `tile` will repeat the texture without scaling it.
#[derive(Clone, Copy)]
pub enum ScaleMode {
    Stretch,
    Tile,
}

/// The font is used for the text rendering. The texture is the tilemap of the characters.
/// The grid_size must be in characters (not pixels).
/// The chars string stores the available characters in this font.
#[derive(Clone)]
pub struct Font {
    texture: Texture,
    grid_size: Vector2<u16>,
    chars: &'static str,
}

#[derive(Clone)]
pub enum Element<'a> {
    /// A flat colored rectangle. Very fast to draw.
    ColorRectangle {
        pos: Vector2<isize>,
        size: Vector2<u16>,
        color: Color565,
    },
    /// A textured non-scaled rectangle. Also known as a sprite.
    /// Use ScaledSprite to change the scaling of the texture.
    /// Fast to draw.
    Sprite {
        pos: Vector2<isize>,
        texture: Texture,
    },
    /// A textured scaled rectangle. Also known as a sprite.
    /// Use ScaledSprite to change the scaling of the texture.
    /// Slower than Sprite.
    ScaledSprite {
        pos: Vector2<isize>,
        size: Vector2<u16>,
        texture: Texture,
        scale_mode: ScaleMode,
    },
    /// A textured rectangle drawn using 9 parts of a textures.
    /// One for each corners, sides and one for the center part of the rectangle.
    /// The goal of this element is to have the same flexibility has a regular
    /// rect but with the advantages of a scaled sprite.
    /// Given the nine parts texture and the other arguments, the renderer will
    /// automatically adapt the image by properly scalling each part.
    /// This element is quite slow to draw.
    NinePartsRectangle {
        texture: NinePartsTexture,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        scaling_mode: ScaleMode,
    },
    /// A simple flat color circle. Very fast to draw.
    Circle {
        center: Vector2<isize>,
        radius: f32,
        color: Color565,
    },
    /// A flat color rounded corner rectangle.
    /// Quite fast to draw.
    RoundedRectangle {
        pos: Vector2<isize>,
        size: Vector2<u16>,
        corner_radius: f32,
        color: Color565,
    },
    /// A simple colored text label. The text is drawn using the given font object.
    /// Setting the background color to None will make it transparent.
    /// Quite slow to draw
    Text {
        pos: Vector2<isize>,
        text: &'a str,
        font: Font,
        font_color: Color565,
        background_color: Option<Color565>,
    },
}

pub struct Renderer2d<'a, const SIZE: usize> {
    draw_queue: heapless::Vec<Element<'a>, SIZE>,
    pub(crate) tile_frame_buffer: [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    clear_color: Color565,
}

impl<'a, const SIZE: usize> Renderer2d<'a, SIZE> {
    pub fn new(clear_color: Color565) -> Self {
        return Self {
            draw_queue: heapless::Vec::new(),
            tile_frame_buffer: [COLOR_BLACK; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
            clear_color,
        };
    }

    pub fn queue_element(&mut self, element: Element<'a>) -> Result<(), ()> {
        if self.draw_queue.push(element).is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn clear_frame_frame_buffer(&mut self) {
        for i in 0..SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT {
            self.tile_frame_buffer[i] = self.clear_color;
        }
    }

    #[inline(always)]
    pub(crate) fn draw_pixel(&mut self, x: usize, y: usize, color: Color565) {
        self.tile_frame_buffer[x + y * SCREEN_TILE_WIDTH] = color;
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
            end.x = SCREEN_TILE_WIDTH as isize - 1;
        }
        if end.y >= SCREEN_TILE_HEIGHT as isize {
            end.y = SCREEN_TILE_HEIGHT as isize - 1;
        }

        for y in pos.y..=end.y {
            for x in pos.x..=end.x {
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
    pub fn draw_circle(&mut self, r: f32, center: Vector2<isize>, color: Color565) {
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

    pub fn draw_rounded_rectangle(
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

    fn draw_shapes(&mut self, buffer_offset: Vector2<isize>) {
        for index in 0..self.draw_queue.len() {
            let element = &self.draw_queue[index];
            match element {
                Element::ColorRectangle { pos, size, color } => {
                    self.draw_rectangle(pos - buffer_offset, size.map(|x| x as isize), *color)
                }
                Element::Sprite { pos, texture } => todo!(),
                Element::ScaledSprite {
                    pos,
                    size,
                    texture,
                    scale_mode,
                } => todo!(),
                Element::NinePartsRectangle {
                    texture,
                    pos,
                    size,
                    scaling_mode,
                } => self.draw_nine_parts_rectangle(texture.clone(), *pos - buffer_offset, *size, *scaling_mode),
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
                } => todo!(),
            }
        }
    }

    pub fn draw(&mut self) {
        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_frame_frame_buffer();

                self.draw_shapes(Vector2::new(
                    (x * SCREEN_TILE_WIDTH) as isize,
                    (y * SCREEN_TILE_HEIGHT) as isize,
                ));

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
        self.draw_queue.clear();
    }
}

impl<'a, const SIZE: usize> Renderer2d<'a, SIZE> {
    pub fn add_rectangle(
        &mut self,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        color: Color565,
    ) -> Result<(), ()> {
        self.queue_element(Element::ColorRectangle { pos, size, color })
    }

    pub fn add_circle(
        &mut self,
        center: Vector2<isize>,
        radius: f32,
        color: Color565,
    ) -> Result<(), ()> {
        self.queue_element(Element::Circle {
            center,
            radius,
            color,
        })
    }

    pub fn add_rounded_rectangle(
        &mut self,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        corner_radius: f32,
        color: Color565,
    ) -> Result<(), ()> {
        self.queue_element(Element::RoundedRectangle {
            pos,
            size,
            corner_radius,
            color,
        })
    }

    pub fn add_nine_parts_rectangle(
        &mut self,
        texture: NinePartsTexture,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        scaling_mode: ScaleMode,
    ) -> Result<(), ()> {
        self.queue_element(Element::NinePartsRectangle {
            texture,
            pos,
            size,
            scaling_mode,
        })
    }
}
