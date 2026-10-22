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

pub fn mode_name(mode: TriangleMode) -> &'static str {
    match mode {
        TriangleMode::Wireframe => "contorno",
        TriangleMode::FilledBarycentric => "baricentrico",
        TriangleMode::FilledScanline => "lineas_horizontales",
    }
}

fn triangle_wireframe(t: &Triangle) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    fragments.extend(line(t.a, t.b, t.color));
    fragments.extend(line(t.b, t.c, t.color));
    fragments.extend(line(t.c, t.a, t.color));
    fragments
}

fn edge(a: Vector2, b: Vector2, p: Vector2) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

fn triangle_barycentric(t: &Triangle) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let area = edge(t.a, t.b, t.c);
    if area == 0.0 {
        return fragments;
    }

    let min_x = t.a.x.min(t.b.x).min(t.c.x).floor() as i32;
    let max_x = t.a.x.max(t.b.x).max(t.c.x).ceil() as i32;
    let min_y = t.a.y.min(t.b.y).min(t.c.y).floor() as i32;
    let max_y = t.a.y.max(t.b.y).max(t.c.y).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let w0 = edge(t.b, t.c, p) / area;
            let w1 = edge(t.c, t.a, p) / area;
            let w2 = edge(t.a, t.b, p) / area;

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                fragments.push(Fragment::new(x, y, t.color));
            }
        }
    }

    fragments
}

fn triangle_scanline(t: &Triangle) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    if edge(t.a, t.b, t.c) == 0.0 {
        return fragments;
    }

    let vertices = [t.a, t.b, t.c];

    let min_y = t.a.y.min(t.b.y).min(t.c.y).floor() as i32;
    let max_y = t.a.y.max(t.b.y).max(t.c.y).ceil() as i32;

    for y in min_y..=max_y {
        let yc = y as f32 + 0.5;
        let mut left = f32::MAX;
        let mut right = f32::MIN;

        for i in 0..3 {
            let p1 = vertices[i];
            let p2 = vertices[(i + 1) % 3];

            if p1.y == p2.y {
                if yc == p1.y {
                    left = left.min(p1.x).min(p2.x);
                    right = right.max(p1.x).max(p2.x);
                }
                continue;
            }

            if yc >= p1.y.min(p2.y) && yc <= p1.y.max(p2.y) {
                let x = p1.x + (yc - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                left = left.min(x);
                right = right.max(x);
            }
        }

        if left > right {
            continue;
        }

        let x_start = (left - 0.5).ceil() as i32;
        let x_end = (right - 0.5).floor() as i32;

        for x in x_start..=x_end {
            fragments.push(Fragment::new(x, y, t.color));
        }
    }

    fragments
}

pub fn rasterize(triangles: &[Triangle], mode: TriangleMode) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    for t in triangles {
        match mode {
            TriangleMode::Wireframe => fragments.extend(triangle_wireframe(t)),
            TriangleMode::FilledBarycentric => fragments.extend(triangle_barycentric(t)),
            TriangleMode::FilledScanline => fragments.extend(triangle_scanline(t)),
        }
    }

    fragments
}
