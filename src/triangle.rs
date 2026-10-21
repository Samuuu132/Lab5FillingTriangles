use raylib::prelude::*;
use crate::fragment::Fragment;
use crate::line::line;

#[derive(Clone, Copy)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
    FilledScanline,
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vector2,
    pub b: Vector2,
    pub c: Vector2,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: Vector2, b: Vector2, c: Vector2, color: Color) -> Self {
        Triangle { a, b, c, color }
    }
}

fn triangle_wireframe(t: &Triangle) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    fragments.extend(line(t.a, t.b, t.color));
    fragments.extend(line(t.b, t.c, t.color));
    fragments.extend(line(t.c, t.a, t.color));
    fragments
}

pub fn rasterize(triangles: &[Triangle], mode: TriangleMode) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    for t in triangles {
        match mode {
            TriangleMode::Wireframe => fragments.extend(triangle_wireframe(t)),
            TriangleMode::FilledBarycentric => {}
            TriangleMode::FilledScanline => {}
        }
    }

    fragments
}
