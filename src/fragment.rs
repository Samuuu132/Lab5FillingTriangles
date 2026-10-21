use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct Fragment {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Fragment {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Fragment { x, y, color }
    }
}
