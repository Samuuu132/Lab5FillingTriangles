use raylib::prelude::*;
use crate::fragment::Fragment;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn draw_fragments(&mut self, fragments: &[Fragment]) {
        for fragment in fragments {
            if fragment.x >= 0 && fragment.y >= 0 {
                self.current_color = fragment.color;
                self.set_pixel(fragment.x as u32, fragment.y as u32);
            }
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn render_to_file_bmp(&self, file_path: &str) {
        use std::fs::File;
        use std::io::Write;

        let width = self.width as i32;
        let height = self.height as i32;

        let row_size = (width * 3 + 3) & !3;
        let padding = row_size - width * 3;
        let pixel_data_size = row_size * height;
        let file_size = 54 + pixel_data_size;

        let mut buffer: Vec<u8> = Vec::with_capacity(file_size as usize);

        buffer.extend_from_slice(b"BM");
        buffer.extend_from_slice(&(file_size as u32).to_le_bytes());
        buffer.extend_from_slice(&0u32.to_le_bytes());
        buffer.extend_from_slice(&54u32.to_le_bytes());

        buffer.extend_from_slice(&40u32.to_le_bytes());
        buffer.extend_from_slice(&width.to_le_bytes());
        buffer.extend_from_slice(&height.to_le_bytes());
        buffer.extend_from_slice(&1u16.to_le_bytes());
        buffer.extend_from_slice(&24u16.to_le_bytes());
        buffer.extend_from_slice(&0u32.to_le_bytes());
        buffer.extend_from_slice(&(pixel_data_size as u32).to_le_bytes());
        buffer.extend_from_slice(&2835u32.to_le_bytes());
        buffer.extend_from_slice(&2835u32.to_le_bytes());
        buffer.extend_from_slice(&0u32.to_le_bytes());
        buffer.extend_from_slice(&0u32.to_le_bytes());

        for y in (0..height).rev() {
            for x in 0..width {
                let color = self.color_buffer.get_color(x, y);
                buffer.push(color.b);
                buffer.push(color.g);
                buffer.push(color.r);
            }
            for _ in 0..padding {
                buffer.push(0);
            }
        }

        let mut file = File::create(file_path).expect("No se pudo crear el archivo BMP");
        file.write_all(&buffer).expect("No se pudo escribir el archivo BMP");
    }
}