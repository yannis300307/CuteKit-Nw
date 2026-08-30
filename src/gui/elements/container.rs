use nalgebra::Vector2;

use crate::gui::{AlignDirection, ChildrenType, Layout, Node, NodeType, margin::Margin, traits::ContainerNode};

pub struct Container<'a> {
    pub children: &'a [NodeType<'a>],
    pub align: AlignDirection,
    pub layout_override: Layout,
    pub expand: bool,
    pub margin: Margin,
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

    fn get_expand_remaining_space(
        &self,
        max_size: Vector2<isize>,
        force_size: (Option<isize>, Option<isize>),
    ) -> Vector2<isize> {
        let mut non_expand_size = Vector2::repeat(0);
        let mut expandable_count = 0;


        let mut last_margin = 0;

        for element in self.children.iter() {
            non_expand_size += match element {
                NodeType::Primitive(node) => {
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
                NodeType::Container(node) => {
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
                }
            }
        }

        match self.get_align_direction() {
            AlignDirection::Down | AlignDirection::Up => { non_expand_size.y += last_margin },
            AlignDirection::Right | AlignDirection::Left => { non_expand_size.x += last_margin },
        }

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
        for child in self.children.iter() {
            let mut size;
            match child {
                NodeType::Primitive(primitive) => {
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
                NodeType::Container(container_node) => {
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
            };
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

    fn get_id(&self) -> usize {
        self.id
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
        for child in self.children.iter() {
            let mut size;
            match child {
                NodeType::Primitive(primitive) => {
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
                NodeType::Container(container_node) => {
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
            };
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
}

