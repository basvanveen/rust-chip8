use crate::memory::Memory;
use std::{thread, time};

const PC_INCREMENT:u16 = 2; 
pub struct Bus<'a> {
    pub vram: &'a [[u8;64];32],
}

pub struct Cpu {
    // memory
    vram: [[u8;64];32],
    // Registers
    vx: [u8; 16],
    i: u16, // index registter
    pc: u16, // program counter
    sp: usize, // stack pointer
    stack: Vec<u16>, //stack
    instruction_counter: u16,
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
             vram: [[0;64];32],
             vx: [0; 16],
             i: 0,
             pc: 0x200,
             sp: 0,
             stack: Vec::<u16>::new(),
             instruction_counter: 0,
        }
    }

    pub fn empty(){}

    pub fn run_instruction(&mut self, memory: &mut Memory) -> Bus {
        let second = time::Duration::from_millis(1000);
        //let now = time::Instant::now();

        // Keep in mind RAM is sized in u8 chunks but PC will read u16 chunks so we end up with (2 bytes == 1 WORD)
        // Little Endian so High Byte
        // HIGH Byte ==  ending (left part of u16)
        // LOW Byte == beginning (right part of u16)
        let high_byte = memory.read_byte(self.pc) as u16;
        let low_byte = memory.read_byte(self.pc+1) as u16;
        // OR left and right but shift to trick back into u8
        // We shift since we only use the first part of u16 in low_byte
        let instruction: u16 = (high_byte << 8) | low_byte;

        let n     = (instruction & 0x000F) as u8;
        let nn    = (instruction & 0x00FF) as u8;
        let nnn   = instruction & 0x0FFF;
        let x    = ((instruction & 0x0F00) >> 8) as u8;
        let y     = ((instruction & 0x00F0)>> 4) as u8;

        if instruction == 0x1228 {
            for index in 0..self.vram.len(){
                println!("{:#2} {:?}", index+1, self.vram[index])
            }
            //thread::sleep(second);
        }
        // debugging
        println!("n {:#X},nn {:#X},nnn {:#X},x {:#X},y {:#X},instruction {:#X}, PC {:#X}",n, nn, nnn, x, y, instruction, self.pc);
        match instruction >> 12 {
           0x0 => { match nn {
            0xE0 => self.opcode_00e0(instruction), //0
            0xEE => self.opcode_00ee(instruction),
            _ => (),
           } } //end 0x5
           0x1 => self.opcode_1nnn(nnn, instruction),
           0x2 => self.opcode_2nnn(nnn, instruction),
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
           0xD => self.opcode_Dxyn(x, y, n, memory, instruction),
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
            0x29 => self.opcode_fx29(x, memory, instruction), //29
            0x33 => self.opcode_fx33(x, memory, instruction), //33
            0x55 => println!("{:#X} LD[I], Vx", instruction), //55
            0x65 => self.opcode_fx65(x, memory, instruction), //65
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0xE
           _ => {},//println!("Catch-all {:?}", instruction),
        }
        self.instruction_counter += 1;
        Bus {
            vram: &self.vram
        }
    }

    pub fn opcode_00e0(&mut self, _instruction: u16) {
        // Clear the display
        println!("{:#X} Clear Screen", _instruction);
        // IMPLEMENT CLEARSCREEN
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_00ee(&mut self, _instruction: u16) {
        // Return from a subroutine
        println!("{:#X} RET", _instruction);
        // IMPLEMENT CLEARSCREEN
        let addr = self.stack.pop().unwrap();
        self.set_pc(addr);
        println!("addr {:#X} popped from stack,PC:{:?} instruction: {:#X}", addr , self.pc, _instruction);
        //panic!();
    }

    pub fn opcode_1nnn(&mut self, nnn: u16, _instruction: u16){
        // Jump to location nnn, The interpreter sets the program counter to nnn.
        println!("{:#X} JMP addr, nnn:{:#X}", _instruction, nnn);
        self.set_pc(nnn);
    }

    pub fn opcode_2nnn(&mut self, nnn:u16, _instruction: u16){
        // Call subroutine at nnn. The interpreter increments the stack pointer,
        // then puts current PC (AFTER THIS instruction) on the top of the stack. The PC is then set to nnn.
        println!("{:#X} CALL addr", _instruction);
        self.sp += 2;
        self.stack.push(self.pc+2);
        println!("PC {:#X} pushed to stack", self.pc);
        self.pc = nnn; 
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

    pub fn opcode_Dxyn(&mut self, x: u8, y: u8, n: u8, memory: &mut Memory, _instruction: u16){
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        // V[x], V[y] contain coordinates for the display start print, n is number of bytes to read from I
        println!("{:#X} DRW Vx,Vy, nibble", _instruction);
        let vx:u8 = self.vx[x as usize];
        let vy:u8 = self.vx[y as usize];

        let mut offset:u16  = self.i;

        println!("X coord {:?} Y coord: {:?}", vx, vy);

        for sprite in 0..n{
        let value:u8 = memory.read_byte(offset + (sprite as u16));
        let row = vy + sprite; 
        for bit in (0..8){
          let shiftright = (value >> (7 - bit));
          let pixel      = shiftright & 0b_0000_0001; // AND to only get LSB as pixel
          // temp out of bound fix / not wrapping around
          if (vx+bit) < 64 {
          self.vram[row as usize][(vx+bit) as usize] = pixel; // Fill Position in Row Y in place VX+BIT
          }
        }
        }

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
        //panic!();
    }

    pub fn opcode_7xkk(&mut self, x:u8, kk:u8, _instruction:u16){
        // Set Vx = Vx + kk. Adds the value kk to the value of the register Vx, then stores the result in Vx.
        println!("{:#X} ADD Vx, byte", _instruction);
        let vx = self.get_vx(x);
        self.write_vx(x, (vx + kk));
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx33(&mut self, x: u8, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD B, Vx", _instruction);
        let vx = self.get_vx(x);
        memory.write_byte(self.i, (vx/100));
        memory.write_byte(self.i + 1, (vx%100)/100);
        memory.write_byte(self.i + 2, (vx%10));
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx65(&mut self,x:u8, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD Vx, [I]", _instruction);
        for number in 0..x{
          memory.write_byte(self.i+x as u16, self.get_vx(x));
        }
        self.i = self.i + x as u16 + 1;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx29(&mut self,x:u8, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD F, [Vx", _instruction);
        self.i = (self.get_vx(x) as u16) * 5; // sprites are stored 8bit by 5 high starting 0.
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