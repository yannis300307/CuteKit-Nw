use nalgebra::Vector2;

use crate::{
    gui::{Layout, Node, Primitive},
    nadk::display::Color565,
    renderer2d::elements::Element,
};

pub struct RoundedRectanglePrimitive {
    pub size: Vector2<u16>,
    pub corner_radius: f32,
    pub color: Color565,
    pub layout_override: Layout,
}

impl<'a> Node<'a> for RoundedRectanglePrimitive {
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        Vector2::new(
            force_size.0.unwrap_or(self.size.x as isize),
            force_size.1.unwrap_or(self.size.y as isize),
        )
    }

    fn get_layout_ovewrite(&self) -> Layout {
        self.layout_override
    }
}

impl<'a> Primitive<'a> for RoundedRectanglePrimitive {
    fn get_element(
        &self,
        pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Some(width) = width {
            size.x = width as u16;
        }
        if let Some(height) = height {
            size.y = height as u16;
        }
        let pos = pos; // TODO: update with layout ovewrite
        Element::RoundedRectangle {
            pos,
            size,
            corner_radius: self.corner_radius,
            color: self.color,
        }
    }
}
