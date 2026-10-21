mod fragment;
mod framebuffer;
mod line;
mod triangle;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use triangle::{rasterize, Triangle, TriangleMode};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn test_triangles() -> Vec<Triangle> {
    vec![
        Triangle::new(
            Vector2::new(100.0, 450.0),
            Vector2::new(400.0, 100.0),
            Vector2::new(700.0, 450.0),
            Color::new(0, 200, 100, 255),
        ),
    ]
}

fn draw_mode(framebuffer: &mut Framebuffer, triangles: &[Triangle], mode: TriangleMode) {
    framebuffer.clear();
    let fragments = rasterize(triangles, mode);
    framebuffer.draw_fragments(&fragments);
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH as u32, HEIGHT as u32, Color::BLACK);
    let triangles = test_triangles();
    let mut mode = TriangleMode::Wireframe;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Lab 5 - Rellenar triangulos")
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            mode = TriangleMode::Wireframe;
        }

        draw_mode(&mut framebuffer, &triangles, mode);

        let texture = rl
            .load_texture_from_image(&thread, &framebuffer.color_buffer)
            .expect("No se pudo crear la textura");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
