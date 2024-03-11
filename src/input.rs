use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

use std::time::Duration;
use std::thread::sleep;


pub struct Input {
  event_pump: EventPump
}

impl Input{

    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let mut event_pump = sdl_context.event_pump().unwrap();
        Input { event_pump: event_pump }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()>  {
        let mut button:[bool; 16]  = [false; 16];

        for event in self.event_pump.poll_iter() {
          match event {
              // Close Window
              Event::Quit {..} => {
                  panic!();
              },
              Event::MouseButtonDown {..} => {
                //sleep(Duration::from_millis(1000));
              },
               // Else continue
               _ => { }
              }
        }

        // https://rust-sdl2.github.io/rust-sdl2/sdl2/keyboard/struct.KeyboardState.html
        // https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/keyboard-state.rs
        let keys:Vec<Keycode> = self.event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

        for key in keys {
          println!("{:?}",key);
            match key {
              Keycode::Num1 => { button[0x1] = true },
              Keycode::Num2 => { button[0x2] = true },
              Keycode::Num3 => { button[0x3] = true },
              Keycode::Num4 => { button[0xC] = true },
              Keycode::Q => { button[0x4] = true },
              Keycode::W => { button[0x5] = true },
              Keycode::E => { button[0x6] = true },
              Keycode::R => { button[0xD] = true },
              Keycode::A => { button[0x7] = true },
              Keycode::S => { button[0x8] = true },
              Keycode::D => { button[0x9] = true },
              Keycode::F => { button[0xE] = true },
              Keycode::Z => { button[0xA] = true },
              Keycode::X => { button[0x0] = true },
              Keycode::C => { button[0xB] = true },
              Keycode::V => { button[0xF] = true },
              _ => {button[0x0] = false }
            }
        }
        Ok(button)
    }

    pub fn pressed_keycode_set(e: &sdl2::EventPump) -> HashSet<Keycode> {
      e.keyboard_state().pressed_scancodes()
          .filter_map(Keycode::from_scancode)
          .collect()
    }
    
}