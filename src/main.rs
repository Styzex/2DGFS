extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

pub fn main() {
    let mut window = Window::new(
        "TwoDGame",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut square_x: isize = 0;
    let mut square_y: isize = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input
        if window.is_key_down(Key::Down) {
            square_y += 1;
        }
        if window.is_key_down(Key::Up) {
            square_y -= 1;
        }
        if window.is_key_down(Key::Left) {
            square_x -= 1;
        }
        if window.is_key_down(Key::Right) {
            square_x += 1;
        }

        // Clear buffer (black color)
        for pixel in buffer.iter_mut() {
            *pixel = 0;  // Black
        }

        // Draw square (white color)
        for y in 0..50 {
            for x in 0..50 {
                let px = (square_x + x) as usize;
                let py = (square_y + y) as usize;
                if px < WIDTH && py < HEIGHT {
                    buffer[py * WIDTH + px] = 0xFFFFFF;  // White
                }
            }
        }

        // Update window with buffer and set frame rate
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(Duration::from_millis(16));  // ~60 FPS
    }
}
