use sdl2::{video::Window};
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::pixels;
use sdl2::rect::Rect;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 320;

pub struct Display {
    canvas: Canvas<Window>
}

impl Display{
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let mut i = 0;
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Rust based chip8 emulator @basvanveen", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
      
        let mut canvas = window.into_canvas().build().unwrap();
      
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
        Display {canvas: canvas }
    }

    pub fn draw(&mut self, vram: &[[u8;64];32]){
        let mut i = 125;
        i = (i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

  
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
  
       //let vram = vram;
        for (y, row) in vram.iter().enumerate() {
          for (x, &pixel) in row.iter().enumerate() {
              let x = (x as u32) * 10;
              let y = (y as u32) * 10;
  
              if pixel == 1 {self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));}else{
                self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
              }
  
              let _ = self.canvas
                  .fill_rect(Rect::new(x as i32, y as i32, 10, 10));
          }
      }
        self.canvas.present();
    }
}