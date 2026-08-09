use nalgebra::Vector2;
use crate::{
    constants::rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, nadk::display::Color565, renderer2d::{
        draw_queue::DrawQueue,
        elements::{Element, Font, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
        sprite::TransparentTexture,
    }
};

pub mod elements;


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
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize>;
}

pub trait ContainerNode<'a>: Node<'a> {
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b>;
    fn get_align_direction(&self) -> AlignDirection;
    fn get_expand(&self) -> bool;
    fn get_expand_remaining_space(&self, max_size: Vector2<isize>, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize>;
    fn get_content_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize>;
    fn get_id(&self) -> usize;
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
    pub expand: bool,
    pub id: usize,
}


impl<'a> ContainerNode<'a> for Container<'a> {
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b> {
        ChildrenType::Nodes(self.children)
    }

    fn get_align_direction(&self) -> AlignDirection {
       self.align
    }
    
    fn get_expand(&self) -> bool {
        self.expand
    }
    
    fn get_expand_remaining_space(&self, max_size: Vector2<isize>, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        let mut non_expand_size = Vector2::repeat(0);
        let mut expandable_count = 0;

        for element in self.children.iter() {
            non_expand_size += match element {
                NodeType::Primitive(node) => {
                    match self.get_align_direction() {
                        AlignDirection::Down | AlignDirection::Up =>
                            node.get_size((force_size.0, None)),
                        AlignDirection::Left | AlignDirection::Right =>
                            node.get_size((None, force_size.1)),
                        _ => todo!()
                    }
                },
                NodeType::Container(node) => {
                    if node.get_expand() {
                        expandable_count += 1;
                        Vector2::repeat(0)
                    } else {
                        node.get_size((None, None))
                    }
                },
            }
        };
        if expandable_count > 0 {
            (max_size - non_expand_size) / expandable_count
        }
        else {
            Vector2::repeat(0)
        }
    }
    
    fn get_content_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        let child_force_size = match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up =>
                (force_size.0, None),
            AlignDirection::Left | AlignDirection::Right =>
                (None, force_size.1),
            _ => todo!()
        };

        let mut total_size = Vector2::new(0, 0);
        let mut max_direction = false;
        for child in self.children.iter() {
            let size;
            match child {
                NodeType::Primitive(primitive) => {
                   size =  primitive.get_size(child_force_size);
                },
                NodeType::Container(container_node) => {
                    if container_node.get_expand() {
                        max_direction = true;
                    }
                    size = container_node.get_size(child_force_size);
                },
            };
            match self.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => {
                    total_size.y += size.y;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if size.x > total_size.x {
                        total_size.x = size.x;
                    }
                },
                AlignDirection::Right | AlignDirection::Left => {
                    total_size.x += size.x;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if size.y > total_size.y {
                        total_size.y = size.y;
                    }
                },
                _ => todo!(),
            }
        }
        // The container contains an expanded child so its size in its flow direction is the maximum_size
        if max_direction {
            match self.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => {
                    Vector2::new(total_size.x, force_size.1.unwrap_or(total_size.y))
                },
                AlignDirection::Right | AlignDirection::Left => {
                    Vector2::new(force_size.0.unwrap_or(total_size.x), total_size.y)
                },
                _ => todo!(),
            }
        } else {
            total_size
        }
    }
    
    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Node<'a> for Container<'a> {
    fn get_layout_ovewrite(&self) -> Layout {
        self.layout_override
    }

    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        let child_force_size = match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up =>
                (force_size.0, None),
            AlignDirection::Left | AlignDirection::Right =>
                (None, force_size.1),
            _ => todo!()
        };

        let mut total_size = Vector2::new(0, 0);
        let mut max_direction = false;
        for child in self.children.iter() {
            let size;
            match child {
                NodeType::Primitive(primitive) => {
                   size =  primitive.get_size(child_force_size);
                },
                NodeType::Container(container_node) => {
                    if container_node.get_expand() {
                        max_direction = true;
                    }
                    size = container_node.get_size(child_force_size);
                },
            };
            match self.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => {
                    total_size.y += size.y;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if size.x > total_size.x {
                        total_size.x = size.x;
                    }
                },
                AlignDirection::Right | AlignDirection::Left => {
                    total_size.x += size.x;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if size.y > total_size.y {
                        total_size.y = size.y;
                    }
                },
                _ => todo!(),
            }
        }
        // The container contains an expanded child so its size in its flow direction is the maximum_size
        if max_direction {
            match self.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => {
                    Vector2::new(total_size.x, force_size.1.unwrap_or(total_size.y))
                },
                AlignDirection::Right | AlignDirection::Left => {
                    Vector2::new(force_size.0.unwrap_or(total_size.x), total_size.y)
                },
                _ => todo!(),
            }
        } else {
            Vector2::new(force_size.0.unwrap_or(total_size.x), force_size.1.unwrap_or(total_size.y))
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
        let size = primitive.get_size(force_size);

        Ok(Vector2::new(force_size.0.unwrap_or(size.x), force_size.1.unwrap_or(size.y)))
    }

    fn render_container_child<'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'a, SIZE>,
        parent_container: &dyn ContainerNode<'a>,
        node: &NodeType<'a>,
        mut offset: Vector2<isize>, 
        child_force_size: (Option<isize>, Option<isize>), 
        child_force_size_expanded: (Option<isize>, Option<isize>),
        force_size: (Option<isize>, Option<isize>)
    ) -> Result<Vector2<isize>, ()> {
        let size = 
        match node {
            NodeType::Primitive(primitive) => {
                Self::render_primitive(draw_queue, *primitive, offset, child_force_size)?
            },
            NodeType::Container(container) => {
                let target_size = if container.get_expand() {
                    child_force_size_expanded
                } else {
                    match container.get_align_direction() {
                        AlignDirection::Down | AlignDirection::Up => (force_size.0, None),
                        AlignDirection::Right | AlignDirection::Left => (None, force_size.1),
                        _ => todo!(),
                    }
                };

                Self::render_container(draw_queue, *container, offset, target_size)?
            },
        };
        match parent_container.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
            AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
            _ => (),
        }

        Ok(offset)
    }

    fn render_primitive_container_child<'b, const SIZE: usize>(draw_queue: &mut DrawQueue<'a, SIZE>, container: &dyn ContainerNode<'a>, node: &&dyn Primitive<'a>, mut offset: Vector2<isize>, child_force_size: (Option<isize>, Option<isize>)) -> Result<Vector2<isize>, ()> {
        let size = Self::render_primitive(draw_queue, *node, offset, child_force_size)?;
        match container.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
            AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
            _ => (),
        }

        Ok(offset)
    }

    fn render_container<'b, const SIZE: usize>(draw_queue: &mut DrawQueue<'a, SIZE>, container: &dyn ContainerNode<'a>, mut offset: Vector2<isize>, force_size: (Option<isize>, Option<isize>)) -> Result<Vector2<isize>, ()> {
        let child_force_size: (Option<isize>, Option<isize>) = match container.get_align_direction() {
            AlignDirection::Up | AlignDirection::Down => (force_size.0, None),
            AlignDirection::Right | AlignDirection::Left => (None, force_size.1),
            AlignDirection::None => todo!(),
        };

        let default_size = container.get_content_size(force_size);
        let mut size = default_size;

        // If the parent of the container doesn't apply a size constraint, the size remains the some of children of that container
        if let Some(width) = force_size.0 {
            size.x = width;
        }
        if let Some(height) = force_size.1 {
            size.y = height;
        }

        // The size available for each expanded containers
        let expand_size = container.get_expand_remaining_space(Vector2::new(force_size.0.unwrap_or(0), force_size.1.unwrap_or(0)), force_size);

        let mut child_force_size_expanded: (Option<isize>, Option<isize>) = match container.get_align_direction() {
            AlignDirection::Up | AlignDirection::Down => (force_size.0, Some(expand_size.y)),
            AlignDirection::Right | AlignDirection::Left => (Some(expand_size.x), force_size.1),
            AlignDirection::None => todo!(),
        };


        offset = match container.get_align_direction() {
            AlignDirection::Down => offset,
            // Calculate the offset to align the elements to the bottom
            AlignDirection::Up => Vector2::new(offset.x, offset.y + size.y - default_size.y),
            AlignDirection::Right => offset,
            // Same to align to the right
            AlignDirection::Left => Vector2::new(offset.x + size.x - default_size.x, offset.y),
            AlignDirection::None => todo!(),
        };

        match container.get_children() {
            ChildrenType::Nodes(nodes) => {
                // If the direction is Left or Up, we simply reverse the iterator
                match container.get_align_direction() {
                    AlignDirection::Left | AlignDirection::Up => for node in nodes.iter().rev() {
                            offset = Self::render_container_child(draw_queue, container, node, offset, child_force_size, child_force_size_expanded, force_size)?
                        }
                    _ => for node in nodes.iter() {
                            offset = Self::render_container_child(draw_queue, container, node, offset, child_force_size, child_force_size_expanded, force_size)?
                        }
                }
            },
            ChildrenType::Primitives(nodes) => {
                match container.get_align_direction() {
                    // If the direction is Left or Up, we simply reverse the iterator
                    AlignDirection::Left | AlignDirection::Up => for node in nodes.iter().rev() {
                        offset = Self::render_primitive_container_child(draw_queue, container, node, offset, child_force_size)?;
                    }
                    _ => for node in nodes.iter() {
                        offset = Self::render_primitive_container_child(draw_queue, container, node, offset, child_force_size)?;
                    }
                }
            },
            ChildrenType::None => {},
        }
        let actual_size = Vector2::new(force_size.0.unwrap_or(offset.x), force_size.1.unwrap_or(offset.y));
        Ok(actual_size)
    }
    
    pub fn render<const SIZE: usize>(
        &self,
        draw_queue: &mut DrawQueue<'a, SIZE>,
    ) -> Result<(), ()> {
        Self::render_container(draw_queue, &self.base_node, Vector2::repeat(0), (Some(SCREEN_WIDTH as isize), Some(SCREEN_HEIGHT as isize)))?;
        Ok(())
    }
}
