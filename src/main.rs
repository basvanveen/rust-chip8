use crate::chip8::Chip8;

mod memory;
mod chip8;
mod cpu;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels;
use std::time::Duration;

use sdl2::rect::Rect;


const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 320;



fn main() {
  let sdl_context = sdl2::init().unwrap();
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

  let mut rng = rand::thread_rng();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;

  // CHIP 8 stuff
  let mut chip8 = Chip8::new();
  chip8.load_rom("src/rom/ibmlogo.ch8");
  'running: loop {
      for event in event_pump.poll_iter() {
          match event {
              // Close Window
              Event::Quit {..} => {
                  break 'running
              },
              // Else continue
              _ => {}
          }
      }

      i = (i + 1) % 255;
      canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

      // Run Instructions on Chip8 VM
      let output = chip8.run_instruction();

      canvas.set_draw_color(Color::RGB(255, 255, 255));

      let vram = output.vram;
      for (y, row) in vram.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            let x = (x as u32) * 10;
            let y = (y as u32) * 10;

            if pixel == 1 {canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));}else{
              canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            }

            let _ = canvas
                .fill_rect(Rect::new(x as i32, y as i32, 10, 10));
        }
    }
      canvas.present();
      // limit FPS if needed
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }

}
