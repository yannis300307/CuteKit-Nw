use nalgebra::Vector2;

use crate::{
    gui::{Layout, Node, Primitive},
    renderer2d::{
        elements::{Element, ScaleMode},
        sprite::TransparentTexture,
    },
};

pub struct TransparentScaledSpritePrimitive<'a> {
    pub size: Vector2<u16>,
    pub texture: &'a TransparentTexture,
    pub scale_mode: ScaleMode,
    pub layout_override: Layout,
}

impl<'a> Node<'a> for TransparentScaledSpritePrimitive<'a> {
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        if let Layout::Relative(..) = self.layout_override {
            self.size.map(|x| x as isize)
        } else {
            Vector2::new(
                force_size.0.unwrap_or(self.size.x as isize),
                force_size.1.unwrap_or(self.size.y as isize),
            )
        }
    }

    fn get_layout_ovewrite(&self) -> Layout {
        self.layout_override
    }
}

impl<'a> Primitive<'a> for TransparentScaledSpritePrimitive<'a> {
    fn get_element(
        &self,
        pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Layout::None | Layout::Transparent = self.layout_override {
            if let Some(width) = width {
                size.x = width as u16;
            }
            if let Some(height) = height {
                size.y = height as u16;
            }
        }
        Element::TransparentScaledSprite {
            pos,
            size,
            texture: self.texture,
            scale_mode: self.scale_mode,
        }
    }
}
