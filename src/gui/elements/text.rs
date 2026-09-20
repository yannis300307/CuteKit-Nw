use nalgebra::Vector2;

use crate::{
    gui::{Layout, Node, Primitive, margin::Margin, traits::{ContainerNode, InteractiveNode, NodeKind}}, nadk::display::Color565, renderer2d::elements::{Element, Font},
};

pub struct TextPrimitive<'a> {
    pub text: &'a str,
    pub font: &'a Font,
    pub font_color: Color565,
    pub background_color: Option<Color565>,
    pub layout_override: Layout,
    pub margin: Margin,
}

impl<'a> Node<'a> for TextPrimitive<'a> {
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        if let Layout::Relative(..) = self.layout_override {
            Vector2::new(
                (self.text.len() * self.font.char_width as usize) as isize,
                self.font.char_height as isize,
            )
        } else {
            Vector2::new(
                force_size
                    .0
                    .unwrap_or((self.text.len() * self.font.char_width as usize) as isize),
                force_size.1.unwrap_or(self.font.char_height as isize),
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

    fn node_id(&self) -> u32 { 6 }
    fn get_raw_pointer(&self) -> *const () { self as *const Self as *const ()}
    fn get_raw_pointer_mut(&mut self) -> *mut () { self as *mut Self as *mut ()}
}

impl<'a> Primitive<'a> for TextPrimitive<'a> {
    fn get_element<'render>(
        &self,
        mut pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'render> where 'a: 'render {
        if let Layout::Default | Layout::Transparent = self.layout_override {
            if let Some(width) = width {
                let actual_lenght = (self.text.len() * self.font.char_width as usize) as isize;
                pos.x += (width - actual_lenght) / 2;
            }
            if let Some(height) = height {
                pos.y += (height - self.font.char_height as isize) / 2;
            }
        }
        Element::Text {
            pos,
            text: self.text,
            font: self.font,
            font_color: self.font_color,
            background_color: self.background_color,
        }
    }
}

unsafe impl<'a> NodeKind<'a> for TextPrimitive<'a> {
    const ID: u32 = 6;
}
