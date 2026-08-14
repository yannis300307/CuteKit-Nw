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
