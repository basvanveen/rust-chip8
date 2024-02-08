use crate::chip8::Chip8;
use crate::input::Input;

mod memory;
mod chip8;
mod cpu;
mod display;
mod input;

use display::Display;
use std::time::Duration;


fn main() {
  let sdl_context = sdl2::init().unwrap();
  let mut display = Display::new(&sdl_context);
  let mut input = Input::new(&sdl_context);

  let mut rng = rand::thread_rng();

  let mut i = 0;

  // CHIP 8 stuff
  let mut chip8 = Chip8::new();
  chip8.load_rom("src/rom/INVADERS");
  loop {
        let start = std::time::Instant::now();
        // Poll Keyboard
        let pressed:[bool; 16] = input.poll().unwrap();
        // Run Instructions on Chip8 VM
        let output = chip8.run_instruction(pressed);
        // Draw Display
        display.draw(output.vram);
        // 60Hz still need to implement DT
        ::std::thread::sleep((Duration::from_millis(1000)-start.elapsed())/600);
  }

}
