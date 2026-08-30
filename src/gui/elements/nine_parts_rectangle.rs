use nalgebra::Vector2;

use crate::{
    gui::{Layout, Node, Primitive, margin::Margin},
    renderer2d::{
        elements::{Element, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
    },
};

pub struct NinePartsRectanglePrimitive<'a> {
    pub parts: &'a NinePartsTexture<'a>,
    pub size: Vector2<u16>,
    pub scaling_mode: ScaleMode,
    pub layout_override: Layout,
    pub margin: Margin,
}

impl<'a> Node<'a> for NinePartsRectanglePrimitive<'a> {
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

    fn get_margin(&self) -> Margin {
        self.margin
    }
}

impl<'a> Primitive<'a> for NinePartsRectanglePrimitive<'a> {
    fn get_element(
        &self,
        pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Layout::Default | Layout::Transparent = self.layout_override {
            if let Some(width) = width {
                size.x = width as u16;
            }
            if let Some(height) = height {
                size.y = height as u16;
            }
        }
        Element::NinePartsRectangle {
            parts: self.parts,
            pos,
            size,
            scaling_mode: self.scaling_mode,
        }
    }
}
