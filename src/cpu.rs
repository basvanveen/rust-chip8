use crate::memory::Memory;
use crate::display::Display;

const PC_INCREMENT:u16 = 2; 
pub struct Cpu {

    // Registers
    vx: [u8; 16],
    i: u16, // index registter
    pc: u16, // program counter
    sp: u8, // stack pointer
    stack: [u16; 16], //stack
    instruction_counter: u16
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
             vx: [0; 16],
             i: 0,
             pc: 0x200,
             sp: 0,
             stack: [0; 16],
             instruction_counter: 0
        }
    }

    pub fn empty(){}

    pub fn run_instruction(&mut self, memory: &mut Memory, display: &mut Display) {
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
        let nn    = (instruction & 0x00FF) as u8;
        let nnn   = instruction & 0x0FFF;
        let x    = ((instruction & 0x0F00) >> 8) as u8;
        let y     = ((instruction & 0x00F0)>> 4) as u8;
        

        //self.pc += 2;
        if self.instruction_counter == 50 {panic!();}
        // debugging
        println!("n {:#X},nn {:#X},nnn {:#X},x {:#X},y {:#X},instruction {:#X},",n, nn, nnn, x, y, instruction);
        match instruction >> 12 {
           0x0 => { match nn {
            0xE0 => self.opcode_00e0(instruction), //0
            _ => (),
           } } //end 0x5
           0x1 => self.opcode_1nnn(nnn, instruction),
           0x2 => println!("{:#X} CALL addr", instruction),
           0x3 => println!("{:#X} SE Vx, byte", instruction),
           0x4 => println!("{:#X} SNE Vx, byte", instruction),
           0x5 => { match instruction << 12 {
            0x0 => println!("{:#X} SE Vx, Vy", instruction), //0
            _ => (),
           } } //end 0x5
           0x6 => self.opcode_6xkk(x, nn, instruction),
           0x7 => self.opcode_7xkk(x, nn, instruction),
           0x8 => { match n {
            0x0 => println!("{:#X} LD Vx,Vy", instruction), //0
            0x1 => println!("{:#X} OR Vx,Vy", instruction), //1
            0x2 => println!("{:#X} AND Vx,Vy", instruction), //2
            0x3 => println!("{:#X} XOR Vx,Vy", instruction), //0
            0x4 => println!("{:#X} ADD Vx,Vy", instruction), //0
            0x5 => println!("{:#X} SUB Vx,Vy", instruction), //0
            0x6 => println!("{:#X} SHR Vx,Vy", instruction), //0
            0x7 => println!("{:#X} SUBN VxVy", instruction), //0
            0xE => println!("{:#X} SHL VxVy", instruction), //0
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0x8
           0x9 => { match n {
            0x0 => println!("{:#X} SNE Vx, Vy", instruction), //0
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0x5
           0xA => self.opcode_Annn(nnn, instruction),
           0xB => println!("{:#X} JP V0, addr", instruction),
           0xC => println!("{:#X} RND Vx, byte", instruction),
           0xD => self.opcode_Dxyn(x, y, n, memory ,display, instruction),
           0xE => { match nn {
            0x9E => println!("{:#X} SKP Vx", instruction), //9E
            0xA1 => println!("{:#X} SKNP Vx", instruction), //A1
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0xE
           0xF => { match nn {
            0x07 => println!("{:#X} LD Vx, DT", instruction), //9E
            0x0A => println!("{:#X} LD Vx, K", instruction), //0A
            0x15 => println!("{:#X} LD DT, Vx", instruction), //15
            0x18 => println!("{:#X} LD ST, Vx", instruction), //18
            0x1E => println!("{:#X} ADD I, Vx", instruction), //1E
            0x29 => println!("{:#X} LD F, Vx", instruction), //29
            0x33 => println!("{:#X} LD B, Vx", instruction), //33
            0x55 => println!("{:#X} LD[I], Vx", instruction), //55
            0x65 => println!("{:#X} LD Vx, [I]", instruction), //65
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0xE
           _ => {},//println!("Catch-all {:?}", instruction),
        }
        self.instruction_counter += 1;
    }

    pub fn opcode_00e0(&mut self, _instruction: u16) {
        // Clear the display
        println!("{:#X} Clear Screen", _instruction);
        // IMPLEMENT CLEARSCREEN
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_1nnn(&mut self, nnn: u16, _instruction: u16){
        // Jump to location nnn, The interpreter sets the program counter to nnn.
        println!("{:#X} JMP addr, nnn:{:#X}", _instruction, nnn);
        self.set_pc(nnn);
    }

    pub fn opcode_Annn(&mut self, addr: u16, _instruction: u16){
        // The value of register I i set to nnn
        println!("{:#X} LD I, addr", _instruction);
        self.i = addr;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_6xkk(&mut self, x: u8, kk: u8, _instruction: u16){
        // Set Vx = kk. The interpreter put the value kk into register Vx
        println!("{:#X} LD Vx, byte", _instruction);
        self.write_vx(x, kk);
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_Dxyn(&mut self, x: u8, y: u8, n: u8, memory: &mut Memory, display: &mut Display, _instruction: u16){
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        // V[x], V[y] contain coordinates for the display start print, n is number of bytes to read from I
        println!("{:#X} DRW Vx,Vy, nibble", _instruction);
        let vx:u8 = self.vx[x as usize];
        let vy:u8 = self.vx[y as usize];
         
        let mut offset:u16  = self.i;
        for sprite in 0..n{
        //display.set_display(vx, vy, n);
        println!("VALUE{:#X}", memory.read_byte(offset));
        offset = offset + 1;
        }
        println!("X coord {:?} Y coord: {:?}", vx, vy);
        
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_7xkk(&mut self, x:u8, kk:u8, _instruction:u16){
        // Set Vx = Vx + kk. Adds the value kk to the value of the register Vx, then stores the result in Vx.
        println!("{:#X} ADD Vx, byte", _instruction);
        let vx = self.get_vx(x);
        self.write_vx(x, (vx + kk));
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }
    pub fn write_vx(&mut self,x: u8, value: u8){
        self.vx[x as usize] = value;
    }

    pub fn get_vx(&mut self, x: u8) -> u8 {
        self.vx[x as usize]
    }

    pub fn set_pc(&mut self, pc: u16){
        self.pc = pc;
    }

    pub fn get_pc(&mut self) -> u16{
        self.pc
    }
    
}