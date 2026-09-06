use nalgebra::Vector2;

use crate::{gui::{enums::{AlignDirection, ChildrenType, Layout}, margin::Margin}, nadk, renderer2d::elements::Element};

pub trait Node<'a> {
    fn get_layout_ovewrite(&self) -> Layout;
    fn get_size(&self, force_size: (Option<isize>, Option<isize>)) -> Vector2<isize>;
    fn get_margin(&self) -> Margin;
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

pub trait InteractiveNode<'a>: Node<'a> {
    /// Called on the selected node each time the user presses a key.
    /// The return value is the signal to be passed to the event handler.
    /// Setting it to None will not trigger an event.
    fn handle_key(&mut self, key: nadk::keyboard::Key) -> Option<usize>;

    /// Called on the selected node each time the user presses an arrow key.
    /// Returning true will prevent the default behavior.
    /// False will let the layout system process the default behavior.
    /// The default behavior is to select the next node in the direction of the pressed arrow.
    fn handle_navigation(&mut self) -> bool {false} 
}