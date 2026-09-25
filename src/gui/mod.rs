use crate::{
    constants::rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, gui::{
        elements::Container, enums::{AlignDirection, Anchor, ChildrenType, Layout}, traits::{ContainerNode, InteractiveNode, Node, Primitive, node_downcast_ref_mut},
    }, input_manager::{self, InputManager}, nadk::{display::{COLOR_WHITE, Color565}, keyboard::Key}, renderer2d::{
        draw_queue::DrawQueue,
        elements::{Element, Font, ScaleMode},
        nine_parts_rectangle::NinePartsTexture,
        sprite::TransparentTexture,
    },
};
use nalgebra::Vector2;

pub mod elements;
pub mod enums;
pub mod margin;
pub mod traits;

/// A signal emitted by an InteractiveNode. The first usize is the user-set id of the InteractiveNode
/// and the second usize is the id of the signal.
#[derive(Clone, Copy)]
pub struct UiSignal(pub usize, pub usize);

pub struct Menu<'a> {
    pub base_node: Container<'a>,
    pub default_hover_marker: bool,
    pub last_signal: Option<UiSignal>,
}

impl<'a> Menu<'a> {
    fn render_primitive<'render, 'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'render, SIZE>,
        primitive: &'render dyn Primitive<'a>,
        offset: Vector2<isize>,
        force_size: (Option<isize>, Option<isize>),
        render_select_marker: bool,
    ) -> Result<Vector2<isize>, ()> where 'a: 'render {
        let element: Element<'render> = primitive.get_element(offset, force_size.0, force_size.1);
        let size = primitive.get_size(force_size);
        draw_queue.queue_element(element)?;

        let actual_size = Vector2::new(
            force_size.0.unwrap_or(size.x),
            force_size.1.unwrap_or(size.y),
        );

        if let Some(primitive) = primitive.as_interactive() {
            if render_select_marker && primitive.get_is_selected() {
                draw_queue.add_outline_rectangle(offset, actual_size.map(|x| x as u16), COLOR_WHITE, 2)?;
            }
        }

        Ok(actual_size)
    }

    fn get_anchor_offset_pos(
        node: &dyn Node<'a>,
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

    fn render_container_child<'render, 'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'render, SIZE>,
        parent_container: &dyn ContainerNode<'a>,
        node: &'render dyn Node<'a>,
        mut offset: Vector2<isize>,
        mut child_force_size: (Option<isize>, Option<isize>),
        child_force_size_expanded: (Option<isize>, Option<isize>),
        force_size: (Option<isize>, Option<isize>),
        parent_container_pos: Vector2<isize>,
        parent_container_size: Vector2<isize>,
        last_margin: &mut isize,
        render_select_marker: bool,
    ) -> Result<Vector2<isize>, ()> {
            if let Some(primitive) = node.as_primitive() {
                let mut pos = offset;
                let margin = primitive.get_margin();
                // Replace the pos with the anchored pos
                if let Layout::Relative(anchor, anchor_offset) = primitive.get_layout_ovewrite() {
                    pos = Self::get_anchor_offset_pos(
                        primitive,
                        anchor,
                        anchor_offset,
                        parent_container_pos,
                        parent_container_size,
                    );
                } else {
                    // Offset the node with the margin. Use the margin of the last element if it's higher than the current node's
                    match primitive.get_layout_ovewrite() {
                        Layout::Default => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {
                                // We need to increment both offset and pos as render_primitive only uses pos
                                let actual_margin = if margin.top > *last_margin {
                                    margin.top
                                } else {
                                    *last_margin
                                };
                                offset.y += actual_margin;
                                pos.y += actual_margin;
                                pos.x += margin.left;
                                if let Some(width) = &mut child_force_size.0 {
                                    *width -= margin.left + margin.right;
                                }
                            }
                            AlignDirection::Right | AlignDirection::Left => {
                                let actual_margin = if margin.left > *last_margin {
                                    margin.left
                                } else {
                                    *last_margin
                                };
                                offset.x += actual_margin;
                                pos.x += actual_margin;
                                pos.y += margin.top;
                                if let Some(height) = &mut child_force_size.1 {
                                    *height -= margin.top + margin.bottom;
                                }
                            }
                        },
                        _ => (),
                    }
                }

                let size = Self::render_primitive(draw_queue, primitive, pos, child_force_size, render_select_marker)?;
                // Ignore offset when transparent or anchored
                match primitive.get_layout_ovewrite() {
                    Layout::Default => match parent_container.get_align_direction() {
                        AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
                        AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
                    },
                    _ => (),
                }

                // Layouts others than Default are ignored because they shouldn't have effect on the flow
                if let Layout::Default = primitive.get_layout_ovewrite() {
                    match primitive.get_layout_ovewrite() {
                        Layout::Default => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {
                                *last_margin = margin.bottom
                            }
                            AlignDirection::Right | AlignDirection::Left => {
                                *last_margin = margin.right
                            }
                        },
                        _ => (),
                    }
                }
            }
            if let Some(container) = node.as_container() {
                let margin = container.get_margin();
                let mut pos = offset;

                let mut target_size;
                if let Layout::Relative(..) = container.get_layout_ovewrite() {
                    target_size = (None, None);
                } else {
                    // Offset the node with the margin. Use the margin of the last element if it's higher than the current node's
                    match container.get_layout_ovewrite() {
                        Layout::Default => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {
                                // We need to increment both offset and pos as render_primitive only uses pos
                                let actual_margin = if margin.top > *last_margin {
                                    margin.top
                                } else {
                                    *last_margin
                                };
                                offset.y += actual_margin;
                                pos.y += actual_margin;
                                pos.x += margin.left;
                                if let Some(width) = &mut child_force_size.0 {
                                    *width -= margin.left + margin.right;
                                }
                            }
                            AlignDirection::Right | AlignDirection::Left => {
                                let actual_margin = if margin.left > *last_margin {
                                    margin.left
                                } else {
                                    *last_margin
                                };
                                offset.x += actual_margin;
                                pos.x += actual_margin;
                                pos.y += margin.top;
                                if let Some(height) = &mut child_force_size.1 {
                                    *height -= margin.top + margin.bottom;
                                }
                            }
                        },
                        Layout::Relative(anchor, anchor_offset) => {
                            pos = Self::get_anchor_offset_pos(
                                container,
                                anchor,
                                anchor_offset,
                                parent_container_pos,
                                parent_container_size,
                            );
                        }
                        _ => (),
                    };
                    target_size = if container.get_expand() {
                        child_force_size_expanded
                    } else {
                        let fit_size = container.get_content_size(child_force_size);
                        match container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => (Some(fit_size.x), None),
                            AlignDirection::Right | AlignDirection::Left => (None, Some(fit_size.y)),
                        }
                    };
                };

                let size = Self::render_container(draw_queue, container, pos, target_size, render_select_marker)?;
                // Ignore offset when transparent or anchored
                match container.get_layout_ovewrite() {
                    Layout::Default => match parent_container.get_align_direction() {
                        AlignDirection::Down | AlignDirection::Up => offset.y += size.y,
                        AlignDirection::Right | AlignDirection::Left => offset.x += size.x,
                    },
                    _ => (),
                }

                // Layouts others that Default are ignored because they shouldn't have effect on the flow
                if let Layout::Default = container.get_layout_ovewrite() {
                    match container.get_layout_ovewrite() {
                        Layout::Default => match parent_container.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {
                                *last_margin = margin.bottom
                            }
                            AlignDirection::Right | AlignDirection::Left => {
                                *last_margin = margin.right
                            }
                        },
                        _ => (),
                    }
            }
        };

        Ok(offset)
    }

    fn render_container<'render, 'b, const SIZE: usize>(
        draw_queue: &mut DrawQueue<'render, SIZE>,
        container: &'render dyn ContainerNode<'a>,
        mut offset: Vector2<isize>,
        mut force_size: (Option<isize>, Option<isize>),
        render_select_marker: bool,
    ) -> Result<Vector2<isize>, ()> {
        if let Layout::Relative(..) = container.get_layout_ovewrite() {
            // Ignore the force_size as the element is detached from the flow
            force_size = (None, None);
        }

        let margin = container.get_margin();

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

        // TODO: Find a cleaner way to fix the double margin issue
        // If the container is expanded, the margins are still not taken into account
        let mut child_force_size: (Option<isize>, Option<isize>) = if container.get_expand() {
            match container.get_align_direction()
            { 
                AlignDirection::Up | AlignDirection::Down => (force_size.0.map(|x| x - (margin.left + margin.right)), None),
                AlignDirection::Right | AlignDirection::Left => (None, force_size.1.map(|y| y - (margin.top + margin.bottom))),
            }
        } else {
            match container.get_align_direction()
            { 
                AlignDirection::Up | AlignDirection::Down => (force_size.0, None),
                AlignDirection::Right | AlignDirection::Left => (None, force_size.1),
            }
        };
        // TODO: Find a cleaner way to fix the double margin issue
        if container.get_expand() {
            // Remove the size of the non-flow direction margins
            match container.get_align_direction() {
                AlignDirection::Up | AlignDirection::Down => {container_size.x -= margin.left + margin.right;},
                AlignDirection::Right | AlignDirection::Left => {container_size.y -= margin.top + margin.bottom;},
            }
        }

        // The size available for each expanded containers
        let mut expand_size = container.get_expand_remaining_space(
            Vector2::new(force_size.0.unwrap_or(0), force_size.1.unwrap_or(0)),
            force_size,
        );

        let mut child_force_size_expanded: (Option<isize>, Option<isize>) =
            match container.get_align_direction() {
                AlignDirection::Up | AlignDirection::Down => (force_size.0.map(|x| x - (margin.left + margin.right)), Some(expand_size.y)),
                AlignDirection::Right | AlignDirection::Left => (Some(expand_size.x), force_size.1.map(|y| y - (margin.top + margin.bottom))),
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
        };

        // Used for anchoring. The top left corner of the container
        let origin = offset;

        let mut last_margin = 0;


        // If the direction is Left or Up, we simply reverse the iterator
        match container.get_align_direction() {
            AlignDirection::Left | AlignDirection::Up => {
                for node in container.get_children().iter().rev() {
                    offset = Self::render_container_child(
                        draw_queue,
                        container,
                        *node,
                        offset,
                        child_force_size,
                        child_force_size_expanded,
                        force_size,
                        origin,
                        container_size,
                        &mut last_margin,
                        render_select_marker
                    )?;
                    
                    /*if let Some(width) = &mut force_size.0 {    
                        *width = container_size.x - (offset.x - container_pos.x);
                    }
                    if let Some(height) = &mut force_size.1 {
                        *height = container_size.y - (offset.y - container_pos.y);
                    }*/
                }
            }
            _ => {
                for node in container.get_children().iter() {
                    offset = Self::render_container_child(
                        draw_queue,
                        container,
                        *node,
                        offset,
                        child_force_size,
                        child_force_size_expanded,
                        force_size,
                        origin,
                        container_size,
                        &mut last_margin,
                        render_select_marker
                    )?;

                    /*if let Some(width) = &mut force_size.0 {
                        *width = container_size.x - (offset.x - container_pos.x);
                    }
                    if let Some(height) = &mut force_size.1 {
                        *height = container_size.y - (offset.y - container_pos.y);
                    }*/
                }
            }
        }
    
        let mut actual_size = Vector2::new(
            // We have to substract the offset of the container itself to get its actual offset.
            force_size.0.unwrap_or(offset.x - container_pos.x),
            force_size.1.unwrap_or(offset.y - container_pos.y),
        );

        if let Some(container) = container.as_interactive() {
            if render_select_marker && container.get_is_selected() {
                draw_queue.add_outline_rectangle(container_pos, container_size.map(|x| x as u16), COLOR_WHITE, 2)?;
            }
        }

        Ok(actual_size)
    }

    pub fn render<'render, const SIZE: usize>(
        &'render self,
        draw_queue: &mut DrawQueue<'render, SIZE>,
    ) -> Result<(), ()> {
        Self::render_container(
            draw_queue,
            &self.base_node,
            Vector2::repeat(0),
            (Some(SCREEN_WIDTH as isize), Some(SCREEN_HEIGHT as isize)),
            self.default_hover_marker,
        )?;
        Ok(())
    }

    fn update_selected_node(node: &mut dyn Node<'a>, key: Key) -> Option<UiSignal> {
        if let Some(node) = node.as_interactive_mut() {
            let signal = node.handle_key_down(key);
            if let Some(signal) = signal {
                return Some(UiSignal(node.get_id(), signal));
            }
            return None;
        }
        else if let Some(container) = node.as_container_mut() {
            if let Some(index) = container.get_selected_node_path() {
                // Ok tell me how this could fail
                return Self::update_selected_node(*container.get_children_mut().get_mut(index).unwrap(), key);
            }
        }
        panic!("Selected node was pointing to a non-interactive node.");
    }

    pub fn update(&mut self, input_manager: &InputManager) {
        let key = input_manager.get_last_pressed();
        if let Some(key) = key {
            self.last_signal = Self::update_selected_node(&mut self.base_node, key);
        }
    }

    /// Pool the last event emitted by the hovered node.
    /// Returns None in case no event where emitted since the last update
    pub fn get_last_event(&self) -> Option<UiSignal> {
        self.last_signal
    }
}
 