use crate::memory::Memory;
use crate::cpu::Cpu;
use std::fs::File;
use std::io::Read;

pub struct Chip8 {
    memory: Memory,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
          memory: Memory::new(),
          cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str){
        let mut rom = File::open(path).unwrap();
        let mut data: Vec<u8> = Vec::new();
        rom.read_to_end(&mut data);
        //print!("{:02X?}", data);
        let offset = 0x200; //end of VM space
        for i in 0..data.len(){
            self.memory.write_byte((offset + i) as u16,data[i])
        }
        self.memory.sprites();
    }

    pub fn print_memory(&mut self) {
        for i in 0..4096 {
            print!("{:X?}", self.memory.full_memory()[i]);
        }
    }

    pub fn run_instruction(&mut self){
        self.cpu.run_instruction(&mut self.memory);
    }
}