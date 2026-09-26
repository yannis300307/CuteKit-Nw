use nalgebra::Vector2;

use crate::{gui::{NavigationDirection, enums::{AlignDirection, ChildrenType, Layout}, margin::Margin}, nadk, renderer2d::elements::Element};

pub unsafe trait NodeKind<'a>: Node<'a> {
    const ID: u32;
}

pub trait Node<'a> {
    fn get_layout_ovewrite(&self) -> Layout;
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize>;
    fn get_margin(&self) -> Margin;

    fn as_primitive<'child>(&'child self) -> Option<&'child (dyn Primitive<'a> + 'child)> { None }
    fn as_container<'child>(&'child self) -> Option<&'child (dyn ContainerNode<'a> + 'child)> { None }
    fn as_interactive<'child>(&'child self) -> Option<&'child (dyn InteractiveNode<'a> + 'child)> { None }

    fn as_container_mut<'child>(&'child mut self) -> Option<&'child mut (dyn ContainerNode<'a> + 'child)> { None }
    fn as_interactive_mut<'child>(&'child mut self) -> Option<&'child mut (dyn InteractiveNode<'a> + 'child)> { None }

    // The functions bellow are required to downcast the objets to their actual type
    fn node_id(&self) -> u32;
    fn get_raw_pointer(&self) -> *const ();
    fn get_raw_pointer_mut(&mut self) -> *mut ();
}

// I'm sorry ...
pub fn node_downcast_ref<'a, T>(node: &'a dyn Node<'a>) -> Option<&'a T> where T: NodeKind<'a> + Sized {
    if node.node_id() == T::ID {
        Some(unsafe { &*(node.get_raw_pointer() as *const T)})
    }
    else {
        None
    }
}

pub fn node_downcast_ref_mut<'a, 'obj, T>(node: &'obj mut (dyn Node<'a> + 'obj)) -> Option<&'obj mut T> where T: NodeKind<'a> + Sized {
    if node.node_id() == T::ID {
        Some(unsafe { &mut *(node.get_raw_pointer_mut() as *mut T)})
    }
    else {
        None
    }
}

pub trait ContainerNode<'a>: Node<'a> {
    fn get_children<'b>(&'b self) -> &'b [&'a mut dyn Node<'a>];
    fn get_children_mut(&mut self) -> &mut [&'a mut (dyn Node<'a> + 'a)];
    fn get_align_direction(&self) -> AlignDirection;
    fn get_expand(&self) -> bool;
    fn get_selected_node_path(&self) -> Option<usize>;
    fn set_selected_node_path(&mut self, index: Option<usize>);
    fn get_expand_remaining_space(
        &self,
        max_size: Vector2<isize>,
        force_size: (Option<isize>, Option<isize>),
    ) -> Vector2<isize> {
        let mut non_expand_size = Vector2::repeat(0);
        let mut expandable_count: isize = 0;


        let mut last_margin = 0;

        for element in self.get_children().iter() {
            non_expand_size += 
                if let Some(node) = element.as_primitive() {
                    // Ignore anchored and transparent layout
                    if let Layout::Default = node.get_layout_ovewrite() {
                        let mut element_size = match self.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {
                                node.get_size((force_size.0, None))
                            }
                            AlignDirection::Left | AlignDirection::Right => {
                                node.get_size((None, force_size.1))
                            }
                        };
                        let margin = node.get_margin();
                        match self.get_align_direction() {
                            AlignDirection::Down | AlignDirection::Up => {element_size.y += if margin.top > last_margin { margin.top } else { last_margin }; last_margin = margin.bottom; },
                            AlignDirection::Right | AlignDirection::Left => {element_size.x += if margin.left > last_margin { margin.left } else { last_margin }; last_margin = margin.right; },
                        }

                        element_size
                    } else {
                        Vector2::repeat(0)
                    }
                }
                else if let Some(node) = element.as_container() {
                    if let Layout::Default = node.get_layout_ovewrite() {
                        if node.get_expand() {
                            expandable_count += 1;
                            let mut element_size = Vector2::repeat(0);
                            let margin = node.get_margin();
                            match self.get_align_direction() {
                                AlignDirection::Down | AlignDirection::Up => {element_size.y += if margin.top > last_margin { margin.top } else { last_margin }; last_margin = margin.bottom; },
                                AlignDirection::Right | AlignDirection::Left => {element_size.x += if margin.left > last_margin { margin.left } else { last_margin }; last_margin = margin.right; },
                            }

                            element_size
                        } else {
                            let mut element_size = node.get_size((None, None));
                            let margin = node.get_margin();
                            match self.get_align_direction() {
                                AlignDirection::Down | AlignDirection::Up => {element_size.y += if margin.top > last_margin { margin.top } else { last_margin }; last_margin = margin.bottom; },
                                AlignDirection::Right | AlignDirection::Left => {element_size.x += if margin.left > last_margin { margin.left } else { last_margin }; last_margin = margin.right; },
                            }

                            element_size
                        }
                    } else {
                        Vector2::repeat(0)
                    }
                } else {
                    Vector2::zeros()
                }
            }

        match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => { non_expand_size.y += last_margin },
            AlignDirection::Right | AlignDirection::Left => { non_expand_size.x += last_margin },
        }

        // TODO: A margin seems to be 2 times counted (not sure)

        if expandable_count > 0 {
            (max_size - non_expand_size) / expandable_count
        } else {
            Vector2::repeat(0)
        }
    }

    fn get_content_size(&self, mut force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        if let Layout::Relative(..) = self.get_layout_ovewrite() {
            // Ignore the force_size as the element is detached from the flow
            force_size = (None, None);
        }

        let child_force_size = 
        match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => (force_size.0, None),
            AlignDirection::Left | AlignDirection::Right => (None, force_size.1),
        };

        let mut total_size = Vector2::new(0, 0);
        let mut max_direction = false;
        let mut last_margin = 0;
        for child in self.get_children().iter() {
            let mut size = Vector2::zeros();
            if let Some(primitive) = child.as_primitive() {
                if let Layout::Default = primitive.get_layout_ovewrite() {
                    size = primitive.get_size(child_force_size);
                } else {
                    size = Vector2::repeat(0);
                }
                let margin = primitive.get_margin();
                match self.get_align_direction() {
                    AlignDirection::Down | AlignDirection::Up => {size.y += if margin.top > last_margin { margin.top } else { last_margin }; last_margin = margin.bottom; },
                    AlignDirection::Right | AlignDirection::Left => {size.x += if margin.left > last_margin { margin.left } else { last_margin }; last_margin = margin.right; },
                }
            }
            if let Some(container_node) = child.as_container() {
                if let Layout::Default = container_node.get_layout_ovewrite() {
                    if container_node.get_expand() {
                        max_direction = true;
                    }
                    size = container_node.get_size(child_force_size);
                } else {
                    size = Vector2::repeat(0);
                }
                let margin = container_node.get_margin();
                match self.get_align_direction() {
                    AlignDirection::Down | AlignDirection::Up => {size.y += if margin.top > last_margin { margin.top } else { last_margin }; last_margin = margin.bottom; },
                    AlignDirection::Right | AlignDirection::Left => {size.x += if margin.left > last_margin { margin.left } else { last_margin }; last_margin = margin.right; },
                }
            }
            match self.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => {
                    total_size.y += size.y;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if force_size.0.is_none(){
                        let margin = child.get_margin();
                        // In case of force_size being None for the other axis, we add the margins as the container is in fit mode
                        if size.x + margin.left + margin.right > total_size.x {
                            total_size.x = size.x + margin.left + margin.right;
                        }
                    } else {
                        if size.x > total_size.x {
                            total_size.x = size.x;
                        }
                    }
                }
                AlignDirection::Right | AlignDirection::Left => {
                    total_size.x += size.x;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if force_size.1.is_none() {
                        let margin = child.get_margin();
                        // In case of force_size being None for the other axis, we add the margins as the container is in fit mode
                        if size.y + margin.top + margin.bottom > total_size.y {
                            total_size.y = size.y + margin.top + margin.bottom;
                        }
                    } else {
                        if size.y > total_size.y {
                            total_size.y = size.y;
                        }
                    }
                }
            }
        }
        // Add the remaining margin to the size of the container
        match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => total_size.y += last_margin,
            AlignDirection::Right | AlignDirection::Left => total_size.x += last_margin,
        }

        // The container contains an expanded child so its size in its flow direction is the maximum size
        if max_direction {
            match self.get_align_direction() {
                AlignDirection::Down | AlignDirection::Up => {
                    Vector2::new(total_size.x, force_size.1.unwrap_or(total_size.y))
                }
                AlignDirection::Right | AlignDirection::Left => {
                    Vector2::new(force_size.0.unwrap_or(total_size.x), total_size.y)
                }
            }
        } else {
            total_size
        }
    }
}

pub trait Primitive<'a>: Node<'a> {
    fn get_element<'render>(
        &self,
        pos: Vector2<isize>,
        width: Option<isize>,
        height: Option<isize>,
    ) -> Element<'render> where 'a: 'render;
}

pub trait InteractiveNode<'a>: Node<'a> {
    /// Called on the selected node each time the user presses down a key.
    /// The return value is the signal to be passed to the event handler.
    /// Setting it to None will not trigger an event.
    fn handle_key_down(&mut self, key: nadk::keyboard::Key) -> Option<usize>;

    /// Called on the selected node each time the user releases a key.
    /// The return value is the signal to be passed to the event handler.
    /// Setting it to None will not trigger an event.
    fn handle_key_up(&mut self, key: nadk::keyboard::Key) -> Option<usize>;

    /// Called on the selected node each time the user presses an arrow key.
    /// Returning true will prevent the default behavior.
    /// False will let the layout system process the default behavior.
    /// The default behavior is to select the next node in the direction of the pressed arrow.
    fn handle_navigation(&mut self, direction: NavigationDirection) -> bool {false}

    // Return the id set by in the node tree.
    fn get_id(&self) -> usize;

    /// Get the internal marker for selection
    fn get_is_selected(&self) -> bool;

    /// Set the internal marker for selection
    fn set_is_selected(&mut self, state: bool);
}