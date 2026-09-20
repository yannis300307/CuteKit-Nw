use nalgebra::Vector2;

use crate::{
    gui::{Layout, Node, Primitive, margin::Margin, traits::{ContainerNode, InteractiveNode, NodeKind}}, nadk::display::Color565, renderer2d::elements::Element,
};

pub struct ColorRectanglePrimitive {
    pub size: Vector2<u16>,
    pub color: Color565,
    pub layout_override: Layout,
    pub margin: Margin,
    pub outline_thickness: Option<u16>,
}

impl<'a> Node<'a> for ColorRectanglePrimitive {
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

    fn as_primitive<'child>(&'child self) -> Option<&'child (dyn Primitive<'a> + 'child)> {
        Some(self)
    }

    fn node_id(&self) -> u32 { 2 }
    fn get_raw_pointer(&self) -> *const () { self as *const Self as *const ()}
    fn get_raw_pointer_mut(&mut self) -> *mut () { self as *mut Self as *mut ()}
}

impl<'a> Primitive<'a> for ColorRectanglePrimitive {
    fn get_element<'render>(
        &self,
        pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'render> {
        let mut size = self.size;
        if let Layout::Default | Layout::Transparent = self.layout_override {
            if let Some(width) = width {
                size.x = width as u16;
            }
            if let Some(height) = height {
                size.y = height as u16;
            }
        }
        let pos = pos; // TODO: update with layout ovewrite
        if let Some(thickness) = self.outline_thickness {
            Element::ColorRectangleOutline {
                pos,
                size,
                color: self.color,
                thickness,
            }
        } else {
            Element::ColorRectangle {
                pos,
                size,
                color: self.color,
            }
        }
    }
}

unsafe impl<'a> NodeKind<'a> for ColorRectanglePrimitive {
    const ID: u32 = 2;
}
