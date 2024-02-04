use crate::chip8::Chip8;

mod memory;
mod chip8;
mod cpu;
mod input;
mod display;

use display::Display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn main() {
  let sdl_context = sdl2::init().unwrap();
  let mut display = Display::new(&sdl_context);

  let mut rng = rand::thread_rng();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;

  // CHIP 8 stuff
  let mut chip8 = Chip8::new();
  chip8.load_rom("src/rom/PONG");
  'running: loop {
      for event in event_pump.poll_iter() {
          match event {
              // Close Window
              Event::Quit {..} => {
                  break 'running
              },
              Event::KeyDown { keycode:Some(Keycode::Num1), .. } => {
                println!("1"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::Num2), .. } => {
                println!("2"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::Num3), .. } => {
                println!("3"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::Num4), .. } => {
                println!("4"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::Q), .. } => {
                println!("Q"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::W), .. } => {
                println!("W"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::E), .. } => {
                println!("E"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::R), .. } => {
                println!("R"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::A), .. } => {
                println!("A"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::S), .. } => {
                println!("S"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::D), .. } => {
                println!("D"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::F), .. } => {
                println!("F"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::Z), .. } => {
                println!("Z"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::X), .. } => {
                println!("X"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::C), .. } => {
                println!("C"); break 'running;
              },
              Event::KeyDown { keycode:Some(Keycode::V), .. } => {
                println!("V"); break 'running;
              },
             // Else continue
              _ => {}
          }
      }
        // Run Instructions on Chip8 VM
        let output = chip8.run_instruction();
        // Draw Display
        display.draw(output.vram);
        // limit FPS if needed
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
  }

}
