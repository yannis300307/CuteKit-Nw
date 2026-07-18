pub mod v2;

use nalgebra::Vector2;
use crate::{
    constants::rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, nadk::display::Color565, renderer2d::{
        draw_queue::DrawQueue,
        elements::{Element, Font, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
        sprite::TransparentTexture,
    }
};


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
    None,
}

#[derive(Clone, Copy)]
pub enum AlignDirection {
    Down,
    Up,
    Right,
    Left,
    None,
}

pub enum NodeType<'a> {
    Primitive(&'a dyn Primitive<'a>),
    Container(&'a dyn ContainerNode<'a>),
}

enum ChildrenType<'a, 'b> {
    Nodes(&'b [NodeType<'a>]),
    Primitives(&'b [&'b dyn Primitive<'a>]),
    None,
}

pub trait Node<'a> {
    fn get_layout_ovewrite(&self) -> Layout;
    fn get_size(&self) -> Vector2<isize>;
}

pub trait ContainerNode<'a>: Node<'a> {
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b>;
    fn get_align_direction(&self) -> AlignDirection;
    fn get_last_element_offset(&self) -> Vector2<isize>;
    fn get_last_element_size(&self) -> Option<Vector2<isize>>;
}

pub trait Primitive<'a>: Node<'a> {
    fn get_element(
        &self,
        pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'a>;
}

pub struct Container<'a> {
    pub children: &'a [NodeType<'a>],
    pub align: AlignDirection,
    pub layout_override: Layout,
}

pub struct ColorRectanglePrimitive {
    pub size: Vector2<u16>,
    pub color: Color565,
    pub layout_override: Layout
}

impl<'a> ContainerNode<'a> for Container<'a> {
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b> {
        ChildrenType::Nodes(self.children)
    }

    fn get_align_direction(&self) -> AlignDirection {
       self.align
    }

    fn get_last_element_offset(&self) -> Vector2<isize> {
        let mut max_size = Vector2::new(0, 0);
        if self.children.is_empty() {
            return max_size;
        }
        for child in self.children.iter().take(self.children.len() - 1) {
            let size = match child {
                NodeType::Primitive(primitive) => primitive.get_size(),
                NodeType::Container(container_node) => container_node.get_size(),
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

    fn get_last_element_size(&self) -> Option<Vector2<isize>> {
        match self.get_children() {
            ChildrenType::Nodes(nodes) => {
                match nodes.last()? {
                    NodeType::Primitive(node) => Some(node.get_size()),
                    NodeType::Container(node) => Some(node.get_size()),
                }
            },
            ChildrenType::Primitives(nodes) => Some(nodes.last()?.get_size()),
            ChildrenType::None => Some(Vector2::new(0, 0)),
        }
    }
}

impl<'a> Node<'a> for Container<'a> {
    fn get_layout_ovewrite(&self) -> Layout {
        self.layout_override
    }

    fn get_size(&self) -> Vector2<isize> {
        let mut max_size = Vector2::new(0, 0);
        for child in self.children.iter() {
            let size = match child {
                NodeType::Primitive(primitive) => primitive.get_size(),
                NodeType::Container(container_node) => container_node.get_size(),
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

impl<'a> Node<'a> for ColorRectanglePrimitive {
    fn get_size(&self) -> Vector2<isize> {
        self.size.map(|x| x as isize)
    }
    
    fn get_layout_ovewrite(&self) -> Layout {
        self.layout_override
    }
}

impl<'a> Primitive<'a> for ColorRectanglePrimitive {
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
        Element::ColorRectangle {
            pos,
            size,
            color: self.color,
        }
    }
}

pub struct Menu<'a> {
    pub base_node: Container<'a>,
}

impl<'a> Menu<'a> {
    fn render_primitive<'b, const SIZE: usize>(draw_queue: &mut DrawQueue<'a, SIZE>, primitive: &dyn Primitive<'a>, offset: Vector2<isize>, force_size: (Option<isize>, Option<isize>)) -> Result<Vector2<isize>, ()> {
        let element: Element<'a> = primitive.get_element(offset, force_size.0, force_size.1);
        draw_queue.queue_element(element)?;
        let size = primitive.get_size();

        Ok(size)
    }

    fn render_container<'b, const SIZE: usize>(draw_queue: &mut DrawQueue<'a, SIZE>, container: &dyn ContainerNode<'a>, mut offset: Vector2<isize>, force_size: (Option<isize>, Option<isize>)) -> Result<Vector2<isize>, ()> {
        let child_force_size: (Option<isize>, Option<isize>) = match container.get_align_direction() {
            AlignDirection::Up | AlignDirection::Down => (force_size.0, None),
            AlignDirection::Right | AlignDirection::Left => (None, force_size.1),
            AlignDirection::None => todo!(),
        };

        let last_element_offset = container.get_last_element_offset();
        let last_element_size = container.get_last_element_size().unwrap_or(Vector2::repeat(0));

        let width = if let Some(width) = force_size.0 {
            width - last_element_size.x
        } else {
            last_element_offset.x
        };

        let height = if let Some(height) = force_size.1 {
            height - last_element_size.y
        } else {
            last_element_offset.y
        };

        offset = match container.get_align_direction() {
            AlignDirection::Down => offset,
            AlignDirection::Up => Vector2::new(offset.x, offset.y + height),
            AlignDirection::Right => Vector2::new(offset.x + width, offset.y),
            AlignDirection::Left => offset,
            AlignDirection::None => todo!(),
        };

        match container.get_children() {
            ChildrenType::Nodes(nodes) => {
                for node in nodes {
                    let size = 
                    match node {
                        NodeType::Primitive(primitive) => {
                            Self::render_primitive(draw_queue, *primitive, offset, child_force_size)?
                        },
                        NodeType::Container(container) => {
                            Self::render_container(draw_queue, *container, offset, force_size)?
                        },
                    };
                    match container.get_align_direction() {
                        AlignDirection::Down => offset.y += size.y,
                        AlignDirection::Up => offset.y -= size.y,
                        AlignDirection::Right => offset.x -= size.x,
                        AlignDirection::Left => offset.x += size.x,
                        AlignDirection::None => todo!(),
                    }
                }
            },
            ChildrenType::Primitives(nodes) => {
                for node in nodes {
                    let size = Self::render_primitive(draw_queue, *node, offset, child_force_size)?;
                    match container.get_align_direction() {
                        AlignDirection::Down => offset.y += size.y,
                        AlignDirection::Up => offset.y -= size.y,
                        AlignDirection::Right => offset.x -= size.x,
                        AlignDirection::Left => offset.x += size.x,
                        AlignDirection::None => todo!(),
                    }
                }
            },
            ChildrenType::None => {},
        }
        Ok(offset)
    }
    
    pub fn render<const SIZE: usize>(
        &self,
        draw_queue: &mut DrawQueue<'a, SIZE>,
    ) -> Result<(), ()> {
        Self::render_container(draw_queue, &self.base_node, Vector2::repeat(0), (Some(SCREEN_WIDTH as isize), Some(SCREEN_HEIGHT as isize)))?;
        Ok(())
    }
}
