use crate::chip8::Chip8;

mod memory;
mod chip8;
mod cpu;
mod display;

fn main() {
    
    let mut chip8 = Chip8::new();
    chip8.load_rom("src/rom/ibmlogo.ch8");
    chip8.print_memory();
    // Start running instructions
    loop{
      chip8.run_instruction();
    }
}
