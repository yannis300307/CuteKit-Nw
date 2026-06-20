use nalgebra::Vector2;

use crate::{
    nadk::display::Color565,
    renderer2d::{
        elements::{CustomPlugin, Element, Font, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
        renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH},
        sprite::TransparentTexture,
    },
};

pub struct DrawQueue<'a, const SIZE: usize> {
    queue: heapless::Vec<Element<'a>, SIZE>,
}

impl<'a, const SIZE: usize> DrawQueue<'a, SIZE> {
    pub fn new() -> Self {
        Self {
            queue: heapless::Vec::new(),
        }
    }

    pub fn queue_element(&mut self, element: Element<'a>) -> Result<(), ()> {
        if self.queue.push(element).is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

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
        parts: &'a NinePartsTexture,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        scaling_mode: ScaleMode,
    ) -> Result<(), ()> {
        self.queue_element(Element::NinePartsRectangle {
            parts,
            pos,
            size,
            scaling_mode,
        })
    }

    pub fn add_text(
        &mut self,
        pos: Vector2<isize>,
        text: &'a str,
        font: &'a Font,
        font_color: Color565,
        background_color: Option<Color565>,
    ) -> Result<(), ()> {
        self.queue_element(Element::Text {
            pos,
            text,
            font,
            font_color,
            background_color,
        })
    }

    pub fn add_transparent_sprite(
        &mut self,
        pos: Vector2<isize>,
        texture: &'a TransparentTexture,
    ) -> Result<(), ()> {
        self.queue_element(Element::TransparentSprite { pos, texture })
    }

    pub fn add_transparent_scaled_sprite(
        &mut self,
        pos: Vector2<isize>,
        size: Vector2<u16>,
        texture: &'a TransparentTexture,
        scale_mode: ScaleMode,
    ) -> Result<(), ()> {
        self.queue_element(Element::TransparentScaledSprite {
            pos,
            size,
            texture,
            scale_mode,
        })
    }

    pub fn add_textured_triangle(
        &mut self,
        p1: Vector2<i16>,
        p2: Vector2<i16>,
        p3: Vector2<i16>,
        t1: Vector2<f32>,
        t2: Vector2<f32>,
        t3: Vector2<f32>,
        texture: &'a TransparentTexture,
    ) -> Result<(), ()> {
        self.queue_element(Element::TexturedTriangle {
            p1,
            p2,
            p3,
            t1,
            t2,
            t3,
            texture,
        })
    }

    pub fn add_plugin(&mut self, object: &'a mut dyn CustomPlugin) -> Result<(), ()> {
        self.queue_element(Element::CustomPlugin { object })
    }

    pub fn get_iterator(&mut self) -> core::slice::IterMut<'_, Element<'a>> {
        self.queue.iter_mut()
    }
}
