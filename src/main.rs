mod cases;
mod compare;
mod fragment;
mod framebuffer;
mod line;
mod triangle;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use cases::test_cases;
use compare::differences;
use triangle::{mode_name, rasterize, Triangle, TriangleMode};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn draw_mode(framebuffer: &mut Framebuffer, triangles: &[Triangle], mode: TriangleMode) {
    framebuffer.clear();
    let fragments = rasterize(triangles, mode);
    framebuffer.draw_fragments(&fragments);
}

fn save_images() {
    std::fs::create_dir_all("images").expect("No se pudo crear la carpeta images");

    let modes = [
        TriangleMode::Wireframe,
        TriangleMode::FilledBarycentric,
        TriangleMode::FilledScanline,
    ];

    for case in test_cases() {
        let mut framebuffer = Framebuffer::new(WIDTH as u32, HEIGHT as u32, Color::BLACK);

        for mode in modes {
            draw_mode(&mut framebuffer, &case.triangles, mode);
            let path = format!("images/{}_{}.png", case.name, mode_name(mode));
            framebuffer.render_to_file(&path);
        }

        let bary = rasterize(&case.triangles, TriangleMode::FilledBarycentric);
        let scan = rasterize(&case.triangles, TriangleMode::FilledScanline);
        let (only_bary, only_scan) = differences(&bary, &scan);
        println!(
            "{}: barycentric {} fragmentos, scanline {} fragmentos, solo barycentric {}, solo scanline {}",
            case.name,
            bary.len(),
            scan.len(),
            only_bary,
            only_scan
        );
    }
}

fn main() {
    if std::env::args().any(|a| a == "--save-images") {
        save_images();
        return;
    }

    let mut framebuffer = Framebuffer::new(WIDTH as u32, HEIGHT as u32, Color::BLACK);
    let cases = test_cases();
    let mut current = 0;
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
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            mode = TriangleMode::FilledBarycentric;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            mode = TriangleMode::FilledScanline;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            current = (current + 1) % cases.len();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            current = (current + cases.len() - 1) % cases.len();
        }

        draw_mode(&mut framebuffer, &cases[current].triangles, mode);

        let texture = rl
            .load_texture_from_image(&thread, &framebuffer.color_buffer)
            .expect("No se pudo crear la textura");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_text(
            &format!("{} - {}", cases[current].name, mode_name(mode)),
            10,
            10,
            20,
            Color::WHITE,
        );
    }
}
