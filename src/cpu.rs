use crate::memory::Memory;
pub struct Cpu {

    // Registers
    vx: [u8; 16],
    i: u16, // index registter
    pc: u16, // program counter
    sp: u8, // stack pointer
    stack: [u16; 16] //stack
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
        // Keep in mind RAM is sized in u8 chunks but PC will read u16 chunks so we end up with (2 bytes == 1 WORD)
        // Little Endian so High Byte
        // HIGH Byte ==  ending (left part of u16)
        // LOW Byte == beginning (right part of u16)
        let high_byte = memory.read_byte(self.pc) as u16;
        let low_byte = memory.read_byte(self.pc+1) as u16;
        // OR left and right but shift to trick back into u8
        // We shift since we only use the first part of u16 in low_byte
        let instruction: u16 = (high_byte << 8) | low_byte;
        let shifted: u16 = high_byte << 8;
        //println!("Shifted: {:#02x}", shifted);
        //println!("We are reading {:#02x} with LOW BYTE: {:#02x} and HIGH BYTE {:#02x}", instruction, low_byte, high_byte);
        //println!("We are reading {:#02X} with HIGH BYTE: {:#02X} and LOW BYTE {:#02X}", instruction, high_byte, low_byte);
        let n     = (instruction & 0x000F) as u8;
        let nn: u16    = 0xDEAD;
        let nnn   = instruction & 0x0FFF;
        let x    = ((instruction & 0x0F00) >> 8) as u8;
        let y     = ((instruction & 0x00F0)) >> 4 as u8;

        self.pc += 2;
        if self.pc == 1500 {panic!();}
        // debugging
        println!("n {:#X},nn {:#X},nnn {:#X},x {:#X},y {:#X},instruction {:#X},",n, nn, nnn, x, y, instruction);
        match instruction >> 12 {
           0x1 => println!("{:#X} JMP addr", instruction),
           0x2 => println!("{:#X} CALL addr", instruction),
           0x3 => println!("{:#X} SE Vx, byte", instruction),
           0x4 => println!("{:#X} SNE Vx, byte", instruction),
           0x5 => println!("{:#X} SE Vx, Vy", instruction), //0
           0x6 => println!("{:#X} LD Vx, byte", instruction),
           0x7 => println!("{:#X} ADD Vx, byte", instruction),
           0x8 => println!("{:#X} LD Vx,Vy", instruction), //0


           _ => {},//println!("Catch-all {:?}", instruction),
        }

    }
}