use nalgebra::Vector2;

use crate::{
    constants::rendering::{SCREEN_HEIGHT, SCREEN_WIDTH},
    nadk::display::Color565,
    renderer2d::{
        draw_queue::DrawQueue,
        elements::{Element, Font, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
        sprite::TransparentTexture,
    },
};

macro_rules! default_primitive_node {
    ($struct_name: ty) => {
        fn get_layout(&self) -> Layout {
            Layout::None
        }

        fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b> {
            ChildrenType::None
        }
    };
}

#[derive(Clone, Copy)]
pub struct Margin {
    pub top: isize,
    pub bottom: isize,
    pub right: isize,
    pub left: isize,
}

impl Margin {
    pub fn none() -> Self {
        Margin {
            top: 0,
            bottom: 0,
            right: 0,
            left: 0,
        }
    }

    pub fn new(top: isize, bottom: isize, right: isize, left: isize) -> Self {
        Margin {
            top,
            bottom,
            right,
            left,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Anchor {
    Center,
    Top,
    Bottom,
    Right,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy)]
pub enum Layout {
    Expand(bool, bool, Margin),
    Absolute,
    Anchor(Anchor),
    Flex(Margin),
    None,
}

pub enum NodeType<'a> {
    Normal(&'a dyn Node<'a>),
    Primitive(&'a dyn Primitive<'a>),
}

enum ChildrenType<'a, 'b> {
    Nodes(&'b [NodeType<'a>]),
    Primitives(&'b [&'b dyn Primitive<'a>]),
    None,
}

pub trait Node<'a> {
    fn get_pos(&self) -> Vector2<isize>;
    fn get_layout(&self) -> Layout;
    fn get_size(&self) -> Vector2<isize>;
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b>;
}

pub trait Primitive<'a>: Node<'a> {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        force_width: Option<isize>,
        force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a>;
}

pub struct Container<'a> {
    pub children: &'a [NodeType<'a>],
    pub pos: Vector2<isize>,
    pub layout: Layout,
    pub size: Option<Vector2<isize>>,
}

pub struct Button<'a> {
    pub children: &'a [&'a dyn Primitive<'a>],
    pub pos: Vector2<isize>,
    pub layout: Layout,
}

pub struct ColorRectanglePrimitive {
    pub pos: Vector2<isize>,
    pub size: Vector2<u16>,
    pub color: Color565,
}

pub struct TransparentSpritePrimitive<'a> {
    pub pos: Vector2<isize>,
    pub texture: &'a TransparentTexture,
}

pub struct TransparentScaledSpritePrimitive<'a> {
    pub pos: Vector2<isize>,
    pub size: Vector2<u16>,
    pub texture: &'a TransparentTexture,
    pub scale_mode: ScaleMode,
}

pub struct NinePartsRectanglePrimitive<'a> {
    pub parts: &'a NinePartsTexture<'a>,
    pub pos: Vector2<isize>,
    pub size: Vector2<u16>,
    pub scaling_mode: ScaleMode,
}

pub struct CirclePrimitive {
    pub center: Vector2<isize>,
    pub radius: f32,
    pub color: Color565,
}

pub struct RoundedRectanglePrimitive {
    pub pos: Vector2<isize>,
    pub size: Vector2<u16>,
    pub corner_radius: f32,
    pub color: Color565,
}

pub struct TextPrimitive<'a> {
    pub pos: Vector2<isize>,
    pub text: &'a str,
    pub font: &'a Font,
    pub font_color: Color565,
    pub background_color: Option<Color565>,
}

pub struct TexturedTrianglePrimitive<'a> {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub t1: Vector2<f32>,
    pub t2: Vector2<f32>,
    pub t3: Vector2<f32>,
    pub texture: &'a TransparentTexture,
}

impl<'a> Node<'a> for Container<'a> {
    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    fn get_layout(&self) -> Layout {
        self.layout
    }
    fn get_size(&self) -> Vector2<isize> {
        if let Some(size) = self.size {
            size
        } else {
            let mut max_size = Vector2::new(0, 0);
            for child in self.children {
                let size = match child {
                    NodeType::Normal(node) => node.get_size(),
                    NodeType::Primitive(primitive) => primitive.get_size(),
                };
                if size.x > max_size.x {
                    max_size.x = size.x;
                }
                if size.y > max_size.y {
                    max_size.y = size.y;
                }
            }
            max_size
        }
    }
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b> {
        ChildrenType::Nodes(self.children)
    }
}
impl<'a> Node<'a> for Button<'a> {
    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    fn get_layout(&self) -> Layout {
        self.layout
    }
    fn get_size(&self) -> Vector2<isize> {
        let mut max_size = Vector2::new(0, 0);
        for child in self.children {
            let size = child.get_size();
            if size.x > max_size.x {
                max_size.x = size.x;
            }
            if size.y > max_size.y {
                max_size.y = size.y;
            }
        }
        max_size
    }
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b> {
        ChildrenType::Primitives(self.children)
    }
}

impl<'a> Node<'a> for ColorRectanglePrimitive {
    fn get_size(&self) -> Vector2<isize> {
        self.size.map(|x| x as isize)
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    default_primitive_node!(ColorRectanglePrimitive);
}

impl<'a> Primitive<'a> for ColorRectanglePrimitive {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        force_width: Option<isize>,
        force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Some(width) = force_width {
            size.x = width as u16;
        }
        if let Some(height) = force_height {
            size.y = height as u16;
        }
        Element::ColorRectangle {
            pos: if let Some(pos) = force_pos {
                pos
            } else {
                self.pos + anchor_point
            },
            size,
            color: self.color,
        }
    }
}

impl<'a> Node<'a> for TransparentSpritePrimitive<'a> {
    fn get_size(&self) -> Vector2<isize> {
        Vector2::new(self.texture.width as isize, self.texture.width as isize)
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    default_primitive_node!(TransparentSpritePrimitive);
}

impl<'a> Primitive<'a> for TransparentSpritePrimitive<'a> {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        _force_width: Option<isize>,
        _force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        Element::TransparentSprite {
            pos: if let Some(pos) = force_pos {
                pos
            } else {
                self.pos + anchor_point
            },
            texture: self.texture,
        }
    }
}

impl<'a> Node<'a> for TransparentScaledSpritePrimitive<'a> {
    fn get_size(&self) -> Vector2<isize> {
        self.size.map(|x| x as isize)
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    default_primitive_node!(TransparentScaledSpritePrimitive);
}

impl<'a> Primitive<'a> for TransparentScaledSpritePrimitive<'a> {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        force_width: Option<isize>,
        force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Some(width) = force_width {
            size.x = width as u16;
        }
        if let Some(height) = force_height {
            size.y = height as u16;
        }
        Element::TransparentScaledSprite {
            pos: if let Some(pos) = force_pos {
                pos
            } else {
                self.pos + anchor_point
            },
            size,
            texture: self.texture,
            scale_mode: self.scale_mode,
        }
    }
}

impl<'a> Node<'a> for NinePartsRectanglePrimitive<'a> {
    fn get_size(&self) -> Vector2<isize> {
        self.size.map(|x| x as isize)
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    default_primitive_node!(NinePartsRectanglePrimitive);
}

impl<'a> Primitive<'a> for NinePartsRectanglePrimitive<'a> {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        force_width: Option<isize>,
        force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Some(width) = force_width {
            size.x = width as u16;
        }
        if let Some(height) = force_height {
            size.y = height as u16;
        }
        Element::NinePartsRectangle {
            parts: self.parts,
            pos: if let Some(pos) = force_pos {
                pos
            } else {
                self.pos + anchor_point
            },
            size,
            scaling_mode: self.scaling_mode,
        }
    }
}

impl<'a> Node<'a> for CirclePrimitive {
    fn get_size(&self) -> Vector2<isize> {
        Vector2::new((self.radius * 2.0) as isize, (self.radius * 2.0) as isize)
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.center
    }
    default_primitive_node!(CirclePrimitive);
}

impl<'a> Primitive<'a> for CirclePrimitive {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        _force_width: Option<isize>,
        _force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        Element::Circle {
            center: if let Some(pos) = force_pos {
                pos
            } else {
                self.center + anchor_point
            },
            radius: self.radius,
            color: self.color,
        }
    }
}

impl<'a> Node<'a> for RoundedRectanglePrimitive {
    fn get_size(&self) -> Vector2<isize> {
        self.size.map(|x| x as isize)
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    default_primitive_node!(RoundedRectanglePrimitive);
}

impl<'a> Primitive<'a> for RoundedRectanglePrimitive {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        force_width: Option<isize>,
        force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        let mut size = self.size;
        if let Some(width) = force_width {
            size.x = width as u16;
        }
        if let Some(height) = force_height {
            size.y = height as u16;
        }
        Element::RoundedRectangle {
            pos: if let Some(pos) = force_pos {
                pos
            } else {
                self.pos + anchor_point
            },
            size,
            corner_radius: self.corner_radius,
            color: self.color,
        }
    }
}

impl<'a> Node<'a> for TextPrimitive<'a> {
    fn get_size(&self) -> Vector2<isize> {
        Vector2::new(
            (self.text.len() * self.font.char_width as usize) as isize,
            self.font.char_height as isize,
        )
    }

    fn get_pos(&self) -> Vector2<isize> {
        self.pos
    }
    default_primitive_node!(TextPrimitive);
}

impl<'a> Primitive<'a> for TextPrimitive<'a> {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        _force_width: Option<isize>,
        _force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        Element::Text {
            pos: if let Some(pos) = force_pos {
                pos
            } else {
                self.pos + anchor_point
            },
            text: self.text,
            font: self.font,
            font_color: self.font_color,
            background_color: self.background_color,
        }
    }
}

impl<'a> Node<'a> for TexturedTrianglePrimitive<'a> {
    fn get_pos(&self) -> Vector2<isize> {
        let x1 = if self.p1.x < self.p2.x && self.p1.x < self.p3.x {
            self.p1.x
        } else if self.p2.x < self.p1.x && self.p2.x < self.p3.x {
            self.p2.x
        } else {
            self.p3.x
        };

        let y1 = if self.p1.y < self.p2.y && self.p1.y < self.p3.y {
            self.p1.y
        } else if self.p2.y < self.p1.y && self.p2.y < self.p3.y {
            self.p2.y
        } else {
            self.p3.y
        };

        Vector2::new(x1 as isize, y1 as isize)
    }

    fn get_layout(&self) -> Layout {
        Layout::None
    }

    fn get_size(&self) -> Vector2<isize> {
        let x1 = if self.p1.x < self.p2.x && self.p1.x < self.p3.x {
            self.p1.x
        } else if self.p2.x < self.p1.x && self.p2.x < self.p3.x {
            self.p2.x
        } else {
            self.p3.x
        };

        let x2 = if self.p1.x > self.p2.x && self.p1.x > self.p3.x {
            self.p1.x
        } else if self.p2.x > self.p1.x && self.p2.x > self.p3.x {
            self.p2.x
        } else {
            self.p3.x
        };

        let y1 = if self.p1.y < self.p2.y && self.p1.y < self.p3.y {
            self.p1.y
        } else if self.p2.y < self.p1.y && self.p2.y < self.p3.y {
            self.p2.y
        } else {
            self.p3.y
        };

        let y2 = if self.p1.y > self.p2.y && self.p1.y > self.p3.y {
            self.p1.y
        } else if self.p2.y > self.p1.y && self.p2.y > self.p3.y {
            self.p2.y
        } else {
            self.p3.y
        };

        Vector2::new((x2 - x1) as isize, (y2 - y1) as isize)
    }

    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b> {
        ChildrenType::None
    }
}

impl<'a> Primitive<'a> for TexturedTrianglePrimitive<'a> {
    fn get_element(
        &self,
        force_pos: Option<Vector2<isize>>,
        _force_width: Option<isize>,
        _force_height: Option<isize>,
        anchor_point: Vector2<isize>,
    ) -> Element<'a> {
        let anchor = if let Some(pos) = force_pos {
            pos
        } else {
            anchor_point
        }
        .map(|x| x as i16);

        Element::TexturedTriangle {
            p1: self.p1 + anchor,
            p2: self.p2 + anchor,
            p3: self.p3 + anchor,
            t1: self.t1,
            t2: self.t2,
            t3: self.t3,
            texture: self.texture,
        }
    }
}

pub struct Menu<'a> {
    pub base_node: Container<'a>,
}

impl<'a> Menu<'a> {
    fn add_child_to_queue<'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'a, SIZE>,
        parent: &'b dyn Node<'a>,
        child: &dyn Primitive<'a>,
        start_offset: &mut Vector2<isize>,
    ) -> Result<(), ()> {
        let mut anchor_point = *start_offset;
        let mut force_pos = None;
        let mut force_width = None;
        let mut force_height = None;
        match parent.get_layout() {
            Layout::Expand(width, height, margin) => {
                let parent_size = parent.get_size();
                let child_size = child.get_size();
                if width {
                    force_width = Some(parent_size.x - margin.left - margin.right);
                    start_offset.y += child_size.y + margin.top + margin.bottom;
                }
                if height {
                    force_height = Some(parent_size.y - margin.top - margin.bottom);
                    start_offset.x += child_size.x + margin.left + margin.right;
                }
                anchor_point.x += margin.left;
                anchor_point.y += margin.top;
            }
            Layout::Absolute => { /* Well... Nothing to change. */ }
            Layout::Anchor(anchor) => {
                let parent_anchor = parent.get_pos();
                let parent_size = parent.get_size();
                let child_size = child.get_size();
                anchor_point = match anchor {
                    Anchor::Center => parent_anchor + (parent_size - child_size) / 2,
                    Anchor::Top => Vector2::new(
                        parent_anchor.x + (parent_size.x - child_size.x) / 2,
                        parent_anchor.y,
                    ),
                    Anchor::Bottom => Vector2::new(
                        parent_anchor.x + (parent_size.x - child_size.x) / 2,
                        parent_anchor.y + parent_size.y - child_size.y,
                    ),
                    Anchor::Right => Vector2::new(
                        parent_anchor.x + parent_size.x - child_size.x,
                        parent_anchor.y + (parent_size.y - child_size.y) / 2,
                    ),
                    Anchor::Left => Vector2::new(
                        parent_anchor.x,
                        parent_anchor.y + (parent_size.y - child_size.y) / 2,
                    ),
                    Anchor::TopLeft => parent_anchor,
                    Anchor::TopRight => Vector2::new(
                        parent_anchor.x + parent_size.x - child_size.x,
                        parent_anchor.y,
                    ),
                    Anchor::BottomLeft => Vector2::new(
                        parent_anchor.x,
                        parent_anchor.y + parent_size.y - child_size.y,
                    ),
                    Anchor::BottomRight => parent_anchor + parent_size - child_size,
                }
            }
            Layout::Flex(margin) => {
                anchor_point.x += margin.left;
                anchor_point.y += margin.top;
                todo!()
            }
            Layout::None => {
                return Ok(());
            }
        };
        let element = child.get_element(force_pos, force_width, force_height, anchor_point);
        draw_queue.queue_element(element)?;
        Ok(())
    }

    fn render_object<'b, const SIZE: usize>(
        &self,
        draw_queue: &mut DrawQueue<'a, SIZE>,
        object: &'b dyn Node<'a>,
        start_offset: &mut Vector2<isize>,
    ) -> Result<(), ()> {
        let children: ChildrenType<'a, 'b> = object.get_children();
        match children {
            ChildrenType::Primitives(elements) => {
                let mut sub_start_offset = *start_offset;
                for child in elements.iter() {
                    Self::add_child_to_queue(draw_queue, object, *child, &mut sub_start_offset)?;
                }
            }

            ChildrenType::Nodes(nodes) => {
                let mut sub_start_offset = *start_offset;
                for child in nodes.iter() {
                    match child {
                        NodeType::Normal(node) => self.render_object(draw_queue, *node, &mut sub_start_offset)?,
                        NodeType::Primitive(primitive) => {
                            Self::add_child_to_queue(draw_queue, object, *primitive, &mut sub_start_offset)?;
                        }
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn render<const SIZE: usize>(
        &self,
        draw_queue: &mut DrawQueue<'a, SIZE>,
    ) -> Result<(), ()> {
        let mut start_offset = Vector2::repeat(0);
        self.render_object(draw_queue, &self.base_node, &mut start_offset)
    }
}
