use crate::{
    constants::rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, gui::{elements::Container, enums::{AlignDirection, Anchor, ChildrenType, Layout, NodeType}, traits::{ContainerNode, Node, Primitive}}, nadk::display::Color565, renderer2d::{
        draw_queue::DrawQueue,
        elements::{Element, Font, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
        sprite::TransparentTexture,
    }
};
use nalgebra::Vector2;

pub mod elements;
pub mod margin;
pub mod enums;
pub mod traits;

pub struct Menu<'a> {
    pub base_node: Container<'a>,
}

impl<'a> Menu<'a> {
    fn render_primitive<'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'a, SIZE>,
        primitive: &dyn Primitive<'a>,
        offset: Vector2<isize>,
        force_size: (Option<isize>, Option<isize>),
    ) -> Result<Vector2<isize>, ()> {
        let element: Element<'a> = primitive.get_element(offset, force_size.0, force_size.1);
        draw_queue.queue_element(element)?;
        let size = primitive.get_size(force_size);

        Ok(Vector2::new(
            force_size.0.unwrap_or(size.x),
            force_size.1.unwrap_or(size.y),
        ))
    }

    fn get_anchor_offset_pos(
        node: &dyn Node,
        anchor: Anchor,
        offset: Vector2<isize>,
        parent_container_pos: Vector2<isize>,
        parent_container_size: Vector2<isize>,
    ) -> Vector2<isize> {
        let node_size = node.get_size((None, None));
        let anchor_pos = match anchor {
            Anchor::Center => parent_container_pos + (parent_container_size - node_size) / 2,
            Anchor::Top => parent_container_pos + Vector2::new((parent_container_size.x - node_size.x) / 2, 0),
            Anchor::Bottom => parent_container_pos + Vector2::new((parent_container_size.x - node_size.x) / 2, parent_container_size.y - node_size.y),
            Anchor::Right => parent_container_pos + Vector2::new(parent_container_size.x - node_size.x, (parent_container_size.y - node_size.y) / 2),
            Anchor::Left => parent_container_pos + Vector2::new(0, (parent_container_size.y - node_size.y) / 2),
            Anchor::TopLeft => parent_container_pos,
            Anchor::TopRight => parent_container_pos + Vector2::new(parent_container_size.x - node_size.x, 0),
            Anchor::BottomLeft => parent_container_pos + Vector2::new(0, parent_container_size.y - node_size.y),
            Anchor::BottomRight => parent_container_pos + parent_container_size - node_size,
        };
        anchor_pos + offset
    }

    fn render_container_child<'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'a, SIZE>,
        parent_container: &dyn ContainerNode<'a>,
        node: &NodeType<'a>,
        mut offset: Vector2<isize>,
        child_force_size: (Option<isize>, Option<isize>),
        child_force_size_expanded: (Option<isize>, Option<isize>),
        force_size: (Option<isize>, Option<isize>),
        parent_container_pos: Vector2<isize>,
        parent_container_size: Vector2<isize>,
        last_margin: &mut isize,
    ) -> Result<Vector2<isize>, ()> {
        match node {
            NodeType::Primitive(primitive) => {
                let mut pos = offset;
                let margin = primitive.get_margin();
                // Replace the pos with the anchored pos
                if let Layout::Relative(anchor, anchor_offset) = primitive.get_layout_ovewrite() {
                    pos = Self::get_anchor_offset_pos(
                        *primitive,
                        anchor,
                        anchor_offset,
                        parent_container_pos,
                        parent_container_size,
                    );
                } else {
                    // Offset the node with the margin. Use the margin of the last element if it's higher than the current node's
                    match primitive.get_layout_ovewrite() {
                        Layout::None => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {
                                // We need to increment both offset and pos as render_primitive only uses pos
                                let actual_margin = if margin.top > *last_margin { margin.top } else { *last_margin };
                                offset.y += actual_margin;
                                pos.y += actual_margin;
                            },
                            AlignDirection::Right | AlignDirection::Left => {
                                let actual_margin = if margin.left > *last_margin { margin.left } else { *last_margin };
                                offset.y += actual_margin;
                                pos.y += actual_margin;
                            },
                            _ => (),
                        },
                        _ => (),
                    }
                }
                let size = Self::render_primitive(draw_queue, *primitive, pos, child_force_size)?;
                // Ignore offset when transparent or anchored
                match primitive.get_layout_ovewrite() {
                    Layout::None => match parent_container.get_align_direction() {
                        AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
                        AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
                        _ => (),
                    },
                    _ => (),
                }

                // Layouts others than None are ignored because they shouldn't have effect on the flow
                if let Layout::None = primitive.get_layout_ovewrite() {
                    match primitive.get_layout_ovewrite() {
                        Layout::None => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => *last_margin = margin.bottom,
                            AlignDirection::Right | AlignDirection::Left => *last_margin = margin.right,
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            NodeType::Container(container) => {
                let margin = container.get_margin();

                let target_size = 
                if let Layout::Relative(..) = container.get_layout_ovewrite() {
                    (None, None)
                } else {
                     // Offset the node with the margin. Use the margin of the last element if it's higher than the current node's
                    match container.get_layout_ovewrite() {
                        Layout::None => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => offset.y += if margin.top > *last_margin { margin.top } else { *last_margin },
                            AlignDirection::Right | AlignDirection::Left => offset.x += if margin.left > *last_margin { margin.left } else { *last_margin },
                            _ => (),
                        },
                        _ => (),
                    };

                    if container.get_expand() {
                        child_force_size_expanded
                    } else {
                        match container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => (force_size.0, None),
                            AlignDirection::Right | AlignDirection::Left => (None, force_size.1),
                            _ => todo!(),
                        }
                    }
                };

                let mut pos = offset;
                if let Layout::Relative(anchor, anchor_offset) = container.get_layout_ovewrite() {
                    pos = Self::get_anchor_offset_pos(
                        *container,
                        anchor,
                        anchor_offset,
                        parent_container_pos,
                        parent_container_size,
                    );
                }

                let size = Self::render_container(draw_queue, *container, pos, target_size)?;
                // Ignore offset when transparent or anchored
                match container.get_layout_ovewrite() {
                    Layout::None => match parent_container.get_align_direction() {
                        AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
                        AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
                        _ => (),
                    },
                    _ => (),
                }

                // Layouts others that None are ignored because they shouldn't have effect on the flow
                if let Layout::None = container.get_layout_ovewrite() {
                    match container.get_layout_ovewrite() {
                        Layout::None => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => *last_margin = margin.bottom,
                            AlignDirection::Right | AlignDirection::Left => *last_margin = margin.right,
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
        };

        Ok(offset)
    }

    fn render_primitive_container_child<'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'a, SIZE>,
        container: &dyn ContainerNode<'a>,
        node: &&dyn Primitive<'a>,
        mut offset: Vector2<isize>,
        child_force_size: (Option<isize>, Option<isize>),
        last_margin: &mut isize
    ) -> Result<Vector2<isize>, ()> {
        let margin = node.get_margin();
        // Offset the node with the margin. Use the margin of the last element if it's higher than the current node's
        match node.get_layout_ovewrite() {
            Layout::None => match container.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => offset.y += if margin.top > *last_margin { margin.top } else { *last_margin }, // Maybe the inverse for Up/Down - Right/Left ?
                AlignDirection::Right | AlignDirection::Left => offset.x += if margin.left > *last_margin { margin.left } else { *last_margin },
                    _ => (),
                },
                _ => (),
            }
        let size = Self::render_primitive(draw_queue, *node, offset, child_force_size)?;
        // Ignore offset when transparent of anchored
        match node.get_layout_ovewrite() {
            Layout::None => match container.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
                AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
                _ => (),
            },
            _ => (),
        }

        // Layouts others than None are ignored because they shouldn't have effect on the flow
        if let Layout::None = node.get_layout_ovewrite() {
            match node.get_layout_ovewrite() {
                Layout::None => match container.get_align_direction() {
                    AlignDirection::Down | AlignDirection::Up => *last_margin = margin.bottom,
                    AlignDirection::Right | AlignDirection::Left => *last_margin = margin.right,
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(offset)
    }

    fn render_container<'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'a, SIZE>,
        container: &dyn ContainerNode<'a>,
        mut offset: Vector2<isize>,
        mut force_size: (Option<isize>, Option<isize>),
    ) -> Result<Vector2<isize>, ()> {
        if let Layout::Relative(..) = container.get_layout_ovewrite() {
            // Ignore the force_size as the element is detached from the flow
            force_size = (None, None);
        }

        let child_force_size: (Option<isize>, Option<isize>) = match container.get_align_direction()
        {
            AlignDirection::Up | AlignDirection::Down => (force_size.0, None),
            AlignDirection::Right | AlignDirection::Left => (None, force_size.1),
            AlignDirection::None => todo!(),
        };

        let container_pos = offset;

        let default_size = container.get_content_size(force_size);
        let mut container_size = default_size;

        // If the parent of the container doesn't apply a size constraint, the size remains the sum of children of that container
        if let Some(width) = force_size.0 {
            container_size.x = width;
        }
        if let Some(height) = force_size.1 {
            container_size.y = height;
        }

        // The size available for each expanded containers
        let expand_size = container.get_expand_remaining_space(
            Vector2::new(force_size.0.unwrap_or(0), force_size.1.unwrap_or(0)),
            force_size,
        );

        let mut child_force_size_expanded: (Option<isize>, Option<isize>) =
            match container.get_align_direction() {
                AlignDirection::Up | AlignDirection::Down => (force_size.0, Some(expand_size.y)),
                AlignDirection::Right | AlignDirection::Left => (Some(expand_size.x), force_size.1), 
                AlignDirection::None => todo!(),
            };

        offset = match container.get_align_direction() {
            AlignDirection::Down => offset,
            // Calculate the offset to align the elements to the bottom
            AlignDirection::Up => {
                Vector2::new(offset.x, offset.y + container_size.y - default_size.y)
            }
            AlignDirection::Right => offset,
            // Same to align to the right
            AlignDirection::Left => {
                Vector2::new(offset.x + container_size.x - default_size.x, offset.y)
            }
            AlignDirection::None => todo!(),
        };

        let mut last_margin = 0;

        match container.get_children() {
            ChildrenType::Nodes(nodes) => {
                // If the direction is Left or Up, we simply reverse the iterator
                match container.get_align_direction() {
                    AlignDirection::Left | AlignDirection::Up => {
                        for node in nodes.iter().rev() {
                            offset = Self::render_container_child(
                                draw_queue,
                                container,
                                node,
                                offset,
                                child_force_size,
                                child_force_size_expanded,
                                force_size,
                                container_pos,
                                container_size,
                                &mut last_margin
                            )?
                        }
                    }
                    _ => {
                        for node in nodes.iter() {
                            offset = Self::render_container_child(
                                draw_queue,
                                container,
                                node,
                                offset,
                                child_force_size,
                                child_force_size_expanded,
                                force_size,
                                container_pos,
                                container_size,
                                &mut last_margin
                            )?
                        }
                    }
                }
            }
            ChildrenType::Primitives(nodes) => {
                match container.get_align_direction() {
                    // If the direction is Left or Up, we simply reverse the iterator
                    AlignDirection::Left | AlignDirection::Up => {
                        for node in nodes.iter().rev() {
                            offset = Self::render_primitive_container_child(
                                draw_queue,
                                container,
                                node,
                                offset,
                                child_force_size,
                                &mut last_margin
                            )?;
                        }
                    }
                    _ => {
                        for node in nodes.iter() {
                            offset = Self::render_primitive_container_child(
                                draw_queue,
                                container,
                                node,
                                offset,
                                child_force_size,
                                &mut last_margin
                            )?;
                        }
                    }
                }
            }
            ChildrenType::None => {}
        }
        let actual_size = Vector2::new(
            force_size.0.unwrap_or(offset.x),
            force_size.1.unwrap_or(offset.y),
        );
        Ok(actual_size)
    }

    pub fn render<const SIZE: usize>(
        &self,
        draw_queue: &mut DrawQueue<'a, SIZE>,
    ) -> Result<(), ()> {
        Self::render_container(
            draw_queue,
            &self.base_node,
            Vector2::repeat(0),
            (Some(SCREEN_WIDTH as isize), Some(SCREEN_HEIGHT as isize)),
        )?;
        Ok(())
    }
}
