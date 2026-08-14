use nalgebra::Vector2;

use crate::{gui::enums::{AlignDirection, ChildrenType, Layout}, renderer2d::elements::Element};

pub trait Node<'a> {
    fn get_layout_ovewrite(&self) -> Layout;
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize>;
}

pub trait ContainerNode<'a>: Node<'a> {
    fn get_children<'b>(&'b self) -> ChildrenType<'a, 'b>;
    fn get_align_direction(&self) -> AlignDirection;
    fn get_expand(&self) -> bool;
    fn get_expand_remaining_space(
        &self,
        max_size: Vector2<isize>,
        force_size: (Option<isize>, Option<isize>),
    ) -> Vector2<isize>;
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
