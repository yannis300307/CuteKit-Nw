use nalgebra::Vector2;

use crate::gui::{AlignDirection, ChildrenType, Layout, Node, margin::Margin, traits::{ContainerNode, InteractiveNode, NodeKind, Primitive}};

pub struct Container<'a> {
    pub children: &'a mut [&'a mut dyn Node<'a>],
    pub align: AlignDirection,
    pub layout_override: Layout,
    pub expand: bool,
    pub margin: Margin,
}

impl<'a> ContainerNode<'a> for Container<'a> {
    fn get_children(&self) -> &[&'a mut dyn Node<'a>] {
        self.children
    }

    fn get_children_mut(&mut self) -> &mut [&'a mut dyn Node<'a>] {
        &mut *self.children
    }

    fn get_align_direction(&self) -> AlignDirection {
        self.align
    }

    fn get_expand(&self) -> bool {
        self.expand
    }
}

impl<'a> Node<'a> for Container<'a> {
    fn get_layout_ovewrite(&self) -> Layout {
        self.layout_override
    }

    fn get_size(&self, mut force_size: (Option<isize>, Option<isize>)) -> Vector2<isize> {
        if let Layout::Relative(..) = self.get_layout_ovewrite() {
            // Ignore the force_size as the element is detached from the flow
            force_size = (None, None);
        }

        let child_force_size = match self.get_align_direction() {
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
                    size = Vector2::repeat(0)
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
                    size = Vector2::repeat(0)
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
                    if size.x > total_size.x {
                        total_size.x = size.x;
                    }
                }
                AlignDirection::Right | AlignDirection::Left => {
                    total_size.x += size.x;
                    // Because the elements are aligned, the size of the container is the size of the largest element
                    if size.y > total_size.y {
                        total_size.y = size.y;
                    }
                }
            }
        }
        // Add the remaining margin to the size of the container
        match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => total_size.y += last_margin,
            AlignDirection::Right | AlignDirection::Left => total_size.x += last_margin,
        }

        // The container contains an expanded child so its size in its flow direction is the maximum_size
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
            Vector2::new(
                force_size.0.unwrap_or(total_size.x),
                force_size.1.unwrap_or(total_size.y),
            )
        }
    }

    fn get_margin(&self) -> Margin {
        self.margin
    }

    fn as_container<'child>(&'child self) -> Option<&'child (dyn ContainerNode<'a> + 'child)> {
        Some(self)
    }

    fn as_container_mut<'child>(&'child mut self) -> Option<&'child mut (dyn ContainerNode<'a> + 'child)> {
        Some(self)
    }

    fn node_id(&self) -> u32 { 3 }
    fn get_raw_pointer(&self) -> *const () { self as *const Self as *const ()}
    fn get_raw_pointer_mut(&mut self) -> *mut () { self as *mut Self as *mut ()}
}

unsafe impl<'a> NodeKind<'a> for Container<'a> {
    const ID: u32 = 3;
}
