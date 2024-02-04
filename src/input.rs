use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
  event_pump: EventPump
}

impl Input{

    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let mut event_pump = sdl_context.event_pump().unwrap();
        Input { event_pump: event_pump }
    }

    pub fn poll(&mut self) -> Result<usize, ()>  {
        let mut button: usize = 0;
        for event in self.event_pump.poll_iter() {
            match event {
                // Close Window
                Event::Quit {..} => {
                    panic!();
                },
                Event::KeyDown { keycode:Some(Keycode::Num1), .. } => {
                  println!("1"); button = 0x1;
                },
                Event::KeyDown { keycode:Some(Keycode::Num2), .. } => {
                  println!("2"); button = 0x2;
                },
                Event::KeyDown { keycode:Some(Keycode::Num3), .. } => {
                  println!("3"); button = 0x3;
                },
                Event::KeyDown { keycode:Some(Keycode::Num4), .. } => {
                  println!("4"); button = 0xC;
                },
                Event::KeyDown { keycode:Some(Keycode::Q), .. } => {
                  println!("Q"); button = 0x4;
                },
                Event::KeyDown { keycode:Some(Keycode::W), .. } => {
                  println!("W"); button = 0x5;
                },
                Event::KeyDown { keycode:Some(Keycode::E), .. } => {
                  println!("E"); button = 0x6;
                },
                Event::KeyDown { keycode:Some(Keycode::R), .. } => {
                  println!("R"); button = 0xD;
                },
                Event::KeyDown { keycode:Some(Keycode::A), .. } => {
                  println!("A"); button = 0x7;
                },
                Event::KeyDown { keycode:Some(Keycode::S), .. } => {
                  println!("S"); button = 0x8;
                },
                Event::KeyDown { keycode:Some(Keycode::D), .. } => {
                  println!("D"); button = 0x9;
                },
                Event::KeyDown { keycode:Some(Keycode::F), .. } => {
                  println!("F"); button = 0xE;
                },
                Event::KeyDown { keycode:Some(Keycode::Z), .. } => {
                  println!("Z"); button = 0xA;
                },
                Event::KeyDown { keycode:Some(Keycode::X), .. } => {
                  println!("X"); button = 0x0;
                },
                Event::KeyDown { keycode:Some(Keycode::C), .. } => {
                  println!("C"); button = 0xB;
                },
                Event::KeyDown { keycode:Some(Keycode::V), .. } => {
                  println!("V"); button = 0xF;
                },
               // Else continue
                _ => {}
            }
        }
        Ok(button)
    }
}