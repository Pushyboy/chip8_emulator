extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::render;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

struct Display {
    pub canvas: render::Canvas<Window>,
}

impl Display {
    const PIXEL_SIZE: u32 = 10;

    pub fn new(sdl_context : Sdl) -> Result<Display, String> {
        let video_subsystem = sdl_context.video()?;

        let window= video_subsystem
            .window(
                "Chip8 Emulator", 
                64 * Display::PIXEL_SIZE, 
                32 * Display::PIXEL_SIZE,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
    
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
    
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
 
        Ok(Display { canvas })
    }

    pub fn draw_pixel(&mut self, point: Point, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);

        let Point(x, y) = point;

        self.canvas.fill_rect(Rect::new(
            x * Display::PIXEL_SIZE as i32,
            y * Display::PIXEL_SIZE as i32,
            Display::PIXEL_SIZE,
            Display::PIXEL_SIZE,
        ))?;

        Ok(())
    }
}