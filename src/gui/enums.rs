use nalgebra::Vector2;

use crate::gui::{Primitive, traits::ContainerNode};

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
    /// Detach the element from the nodes flow and stick it to an anchor with the given offset. The anchor is placed on the parent node.
    Relative(Anchor, Vector2<isize>),
    /// Keeps the element in flow but removes its bounding box. Elements can flow over it. It can be used to stack elements in a div.
    Transparent,
    Default,
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

pub enum ChildrenType<'a, 'b> {
    Nodes(&'b [NodeType<'a>]),
    Primitives(&'b [&'b dyn Primitive<'a>]),
    None,
}
