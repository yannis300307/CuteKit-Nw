use bytemuck::{Pod, Zeroable};

use crate::nadk::display::Color565;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TransparentRGB565 {
    pub rgb: Color565,
    pub alpha: u8,
}

/// A texture struct that store a size and a reference to the actual pixels.
#[derive(Clone)]
pub struct TransparentTexture {
    pub width: u16,
    pub height: u16,
    pub data: &'static [TransparentRGB565],
}

impl TransparentTexture {
    pub fn new(data: &'static [u8], width: u16, height: u16) -> Self
    {
        Self {
            width,
            height,
            data: bytemuck::cast_slice(data)
        }
    }
}

#[inline]
pub(super) fn add_alpha_color(a: Color565, b: TransparentRGB565) -> Color565 {
    let rgb = b.rgb;
    let a_comp = a.get_components();
    let b_comp = rgb.get_components();
    Color565::new(
        ((255 - b.alpha as u16) * a_comp.0 + b_comp.0 * b.alpha as u16) / 255,
        ((255 - b.alpha as u16) * a_comp.1 + b_comp.1 * b.alpha as u16) / 255,
        ((255 - b.alpha as u16) * a_comp.2 + b_comp.2 * b.alpha as u16) / 255,
    )
}