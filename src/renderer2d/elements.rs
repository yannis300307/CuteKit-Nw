use nalgebra::Vector2;

use crate::{nadk::display::Color565, renderer2d::{nine_parts_rectangle::NinePartsTexture, sprite::TransparentTexture}};

/// A texture struct that store a size and a reference to the actual pixels.
#[derive(Clone)]
pub struct Texture {
    pub width: u16,
    pub height: u16,
    pub(crate) data: &'static [Color565],
}

/// A scaling mode enum used in elements that requiere a scalling strategy.
/// `Stretch` will distort the texture to match the size of the area
/// that needs to be drawn while `tile` will repeat the texture without scaling it.
#[derive(Clone, Copy)]
pub enum ScaleMode {
    Stretch,
    Tile,
}

/// The font is used for the text rendering.
/// The grid_size must be in characters (not pixels).
/// The chars string stores the available characters in this font.
#[derive(Clone)]
pub struct Font {
    pub data: &'static [u8],
    pub font_image_width: u16,
    pub char_width: u16,
    pub char_height: u16,
    pub chars: &'static str,
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
    /// Quite Fast to draw.
    TransparentSprite {
        pos: Vector2<isize>,
        texture: &'a TransparentTexture,
    },
    /// A textured scaled rectangle. Also known as a sprite.
    /// Use ScaledSprite to change the scaling of the texture.
    /// Slower than Sprite.
    TransparentScaledSprite {
        pos: Vector2<isize>,
        size: Vector2<u16>,
        texture: &'a TransparentTexture,
        scale_mode: ScaleMode,
    },
    /// A textured rectangle drawn using 9 parts of a textures.
    /// One for each corners, sides and one for the center part of the rectangle.
    /// The goal of this element is to have the same flexibility has a regular
    /// rect but with the advantages of a scaled sprite.
    /// Given the nine parts and the other arguments, the renderer will
    /// automatically adapt the image by properly scalling each part.
    /// This element is quite slow to draw.
    NinePartsRectangle {
        parts: &'a NinePartsTexture<'a>,
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
        font: &'a Font,
        font_color: Color565,
        background_color: Option<Color565>,
    },
    /// A textured triangle supporting transparent textures
    /// Quite fast to draw
    TexturedTriangle {
        p1: Vector2<i16>,
        p2: Vector2<i16>,
        p3: Vector2<i16>,
        t1: Vector2<f32>,
        t2: Vector2<f32>,
        t3: Vector2<f32>,
        texture: &'a TransparentTexture
    }
}
