use crate::memory::Memory;
pub struct Cpu {

    // Registers
    vx: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16]
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
             vx: [0; 16],
             i: 0,
             pc: 0x200,
             sp: 0,
             stack: [0; 16]
        }
    }

    pub fn empty(){}

    pub fn run_instruction(&mut self, memory: &mut Memory) {
        println!("Running instruction :-)");
    }
}