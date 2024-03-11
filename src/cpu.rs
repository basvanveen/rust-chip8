use crate::memory::Memory;
use std::{thread, time};
use rand::Rng;

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
    dt: u8,
    st: u8,
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
             dt: 0,
             st: 0,
        }
    }

    pub fn delay_timer(&mut self){
        if self.dt > 0 {self.dt -= 1}
    }

    pub fn run_instruction(&mut self, memory: &mut Memory, pressed: [bool; 16]) -> Bus {
        //let now = time::Instant::now();
        //println!("BUTTON {:?}", pressed);
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
        let x    = ((instruction & 0x0F00) >> 8) as usize;
        let y     = ((instruction & 0x00F0)>> 4) as usize;

        if instruction == 0x1228 {
            for index in 0..self.vram.len(){
                println!("{:#2} {:?}", index+1, self.vram[index])
            }
        }

        self.delay_timer();

        // debugging
        //println!("n {:#X},nn {:#X},nnn {:#X},x {:#X},y {:#X},instruction {:#X}, PC {:#X}",n, nn, nnn, x, y, instruction, self.pc);
        match instruction >> 12 {
           0x0 => { match nn {
            0xE0 => self.opcode_00e0(instruction), //0
            0xEE => self.opcode_00ee(instruction),
            _ => (),
           } } //end 0x5
           0x1 => self.opcode_1nnn(nnn, instruction),
           0x2 => self.opcode_2nnn(nnn, instruction),
           0x3 => self.opcode_3xkk(x, nn, instruction),
           0x4 => self.opcode_4xkk(x, nn, instruction),
           0x5 => { match instruction << 12 {
            0x0 => self.opcode_5xy0(x, y, instruction), //0
            _ => (),
           } } //end 0x5
           0x6 => self.opcode_6xkk(x, nn, instruction),
           0x7 => self.opcode_7xkk(x, nn, instruction),
           0x8 => { match n {
            0x0 => self.opcode_8xy0(x, y, instruction), //0
            0x1 => self.opcode_8xy1(x, y, instruction), //1
            0x2 => self.opcode_8xy2(x, y, instruction), //2
            0x3 => self.opcode_8xy3(x, y, instruction), //0
            0x4 => self.opcode_8xy4(x, y, instruction), //0
            0x5 => self.opcode_8xy5(x, y, instruction), //0
            0x6 => self.opcode_8xy6(x, y, instruction), //0
            0x7 => self.opcode_8xy7(x, y, instruction), //0
            0xE => self.opcode_8xye(x, y, instruction), //0
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0x8
           0x9 => { match n {
            0x0 => self.opcode_9xy0(x, y, instruction), //0
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0x5
           0xA => self.opcode_Annn(nnn, instruction),
           0xB => println!("{:#X} JP V0, addr", instruction),
           0xC => self.opcode_Cxkk(x, nn, instruction),
           0xD => self.opcode_dxyn(x, y, n, memory, instruction),
           0xE => { match nn {
            0x9E => self.opcode_ex9e(x, pressed, instruction), //9E
            0xA1 => self.opcode_exa1(x, pressed, instruction), //A1
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0xE
           0xF => { match nn {
            0x07 => self.opcode_fx07(x, memory, instruction), //9E
            0x0A => self.opcode_fx0a(x, pressed,instruction), //0A
            0x15 => self.opcode_fx15(x, memory, instruction), //15
            0x18 => self.opcode_fx18(x, memory, instruction), //18
            0x1E => self.opcode_fx1e(x, memory, instruction), //1E
            0x29 => self.opcode_fx29(x, memory, instruction), //29
            0x33 => self.opcode_fx33(x, memory, instruction), //33
            0x55 => self.opcode_fx55(x, memory, instruction), //55
            0x65 => self.opcode_fx65(x, memory, instruction), //65
            _ => println!("{:#X} UNKNOWN", instruction),
           } } //end 0xE
           _ => {},
        }
        self.instruction_counter += 1;
        Bus {
            vram: &self.vram
        }
    }

    pub fn opcode_00e0(&mut self, _instruction: u16) {
        // Clear the display
        println!("{:#X} Clear Screen", _instruction);
        self.vram = [[0;64];32];
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_00ee(&mut self, _instruction: u16) {
        // Return from a subroutine
        println!("{:#X} RET", _instruction);
        let addr = self.stack.pop().unwrap();
        self.set_pc(addr);
        println!("RETURN addr {:#X} popped from stack,PC:{:?} instruction: {:#X}", addr , self.pc, _instruction);
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

    pub fn opcode_Cxkk(&mut self, x: usize, kk: u8, _instruction: u16){
        // Set Vx = random byte AND kk. The interpreter generates a random number from 0..255. which is then
        // ANDed with value kk. The results are stored in Vx.  
        println!("{:#X} RND Vx,byte", _instruction);
        let mut rng = rand::thread_rng();
        self.vx[x] = rng.gen_range(0..255) & kk;

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_ex9e(&mut self, x: usize,pressed: [bool; 16] , _instruction: u16){
        // Skip instruction if key with value of Vx is pressed.
        // KEYBOARD Implementation *WIP*
        let vx:usize = self.vx[x] as usize;
        println!("{:#X} SKP Vx", _instruction);
        if (pressed[vx]) {
            //println!("PRESSED SKP {:?}", pressed);
            self.pc += 4;
        }else{
            self.pc += 2;
        }
    }

    pub fn opcode_exa1(&mut self, x: usize,pressed: [bool; 16] , _instruction: u16){
        // Skip instruction if key with value of Vx is not pressed.
        // KEYBOARD Implementation *WIP*
        let vx:usize = self.vx[x] as usize;
        println!("{:#X} SKPN Vx", _instruction);
        if (!pressed[vx]) {
            println!("Ins NOTPRESSE SKPN{:?}", vx);
            self.pc += 4;
        }else{
            self.pc += 2;

        }
    }

    pub fn opcode_3xkk(&mut self, x: usize, kk: u8, _instruction: u16){
        // Set Vx = kk. The interpreter put the value kk into register Vx
        println!("{:#X} SE Vx, byte", _instruction);
        if self.get_vx(x) == kk{
            let increment = self.get_pc() + PC_INCREMENT;
            self.set_pc(increment);
        }
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_4xkk(&mut self, x: usize, kk: u8, _instruction: u16){
        // Skip next instruction if Vx != kk. If not equal progam counter == +2
        println!("{:#X} SNE Vx, byte", _instruction);
        if self.vx[x] != kk {
            let increment = self.get_pc() + PC_INCREMENT;
            self.set_pc(increment);
        }
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_5xy0(&mut self, x: usize, y: usize, _instruction: u16){
        // SE Vx,Vy
        println!("{:#X} SE Vx, Vy", _instruction);
        // self.vx[x] = self.vx[y];

        if self.vx[x] == self.vx[y] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    pub fn opcode_6xkk(&mut self, x: usize, kk: u8, _instruction: u16){
        // Set Vx = kk. The interpreter put the value kk into register Vx
        println!("{:#X} LD Vx, byte", _instruction);
        self.write_vx(x, kk);
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy0(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vy
        println!("{:#X} LD Vx,Vy", _instruction);
        self.vx[x] = self.vx[y];

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy1(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx OR Vy
        println!("{:#X} OR Vx,Vy", _instruction);
        self.vx[x] = (self.vx[x] | self.vx[y]) as u8;

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_9xy0(&mut self, x: usize, y: usize, _instruction: u16){
        // SNE Vx, Vy
        println!("{:#X} SNE Vx (,Vy)", _instruction);
        if self.vx[x] & 1 == 1 { self.vx[0x0f] = 1}else {self.vx[0x0f] = 0}

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy2(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx AND Vy.
        println!("{:#X} AND Vx, Vy", _instruction);
        self.vx[x] = self.vx[x] & self.vx[y];

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy3(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx XOR Vy
        println!("{:#X} XOR Vx,Vy", _instruction);
        self.vx[x] = (self.vx[x] ^ self.vx[y]) as u8;

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy4(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx + vy, set VF = carry
        println!("{:#X} ADD Vx, Vy", _instruction);
        println!("X:{:#X} Y:{:#X}", self.vx[x], self.vx[y]);
        let result = (self.vx[x] as u16 + self.vx[y] as u16);

        println!("X:{:#X} Y:{:#X}", self.vx[x], self.vx[y]);
        self.vx[x] = (result & 0xFF) as u8;
        self.vx[0x0f] = if result > 255 { 1 } else { 0 };

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy5(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx - Vy
        //let vxvy = self.vx[x] - self.vx[y];
        println!("{:#X} SUB Vx,Vy", _instruction);
        self.vx[0x0F] = if self.vx[x] > self.vx[y] { 1 } else { 0 };

        let result:i16 = self.vx[x] as i16 - self.vx[y] as i16;
        self.vx[x] = result as u8;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy6(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx SHR 1 !!!TEST!!!
        println!("{:#X} SHR Vx(Vy) !!!TEST!!!", _instruction);
        if (self.vx[x] & 0b0000_0001) == 1 {
           self.vx[(0x0F) as usize] = 1;
        }else{
           self.vx[(0x0F) as usize] = 0;
        }

        self.vx[x] = self.vx[x] / 2;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xy7(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vy - Vx
        println!("{:#X} SUBN Vx,Vy", _instruction);
        self.vx[0x0F] = if self.vx[y] > self.vx[x] { 1 } else { 0 };

        let result:i16 = self.vx[y] as i16 - self.vx[x] as i16;
        self.vx[x] = result as u8;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_8xye(&mut self, x: usize, y: usize, _instruction: u16){
        // Set Vx = Vx SHL 1
        println!("{:#X} SHL Vx, Vy", _instruction);
        self.vx[0x0F] = if (self.vx[x] & 0b1000_0000) == 1 { 1 } else { 0 };
        self.vx[x] = self.vx[x] << 1; // Shift Left

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_dxyn(&mut self, x: usize, y: usize, n: u8, memory: &mut Memory, _instruction: u16){
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        // V[x], V[y] contain coordinates for the display start print, n is number of bytes to read from I
        println!("{:#X} DRW Vx,Vy, nibble", _instruction);
        let vx:u8  = self.vx[x];
        let vy:u8 = self.vx[y];

        let mut offset:u16  = self.i;

        println!("X coord {:?} Y coord: {:?} height: {:?}", vx, vy, n);

        for sprite in 0..n{
        let value:u8 = memory.read_byte(offset + (sprite as u16));
        let row = (vy + sprite) % 32; // wrap don't allow highjer 
        for bit in (0..8){
          let shiftright = (value >> (7 - bit));
          let pixel      = shiftright & 0b_0000_0001; // AND to only get LSB as pixel
          // Fill Position in Row Y in place VX+BIT, and XOR currentpixel ^ pixel
          if(self.vram[row as usize][((vx+bit)%64) as usize] != pixel){
            self.vx[0xf]= 1;
          }else{self.vx[0xf]=0}
          self.vram[row as usize][((vx+bit)%64) as usize] ^= pixel; 
        }
        }

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
        //panic!();
    }

    pub fn opcode_7xkk(&mut self, x: usize, kk:u8, _instruction:u16){
        // Set Vx = Vx + kk. Adds the value kk to the value of the register Vx, then stores the result in Vx.
        println!("{:#X} ADD Vx, byte", _instruction);
        let vx = self.get_vx(x);
        self.write_vx(x, (vx as i16 + kk as i16) as u8);
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx07(&mut self, x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD Vx, DT", _instruction);
        self.write_vx(x,self.dt);

        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }
    
    pub fn opcode_fx0a(&mut self, x: usize, pressed: [bool; 16], _instruction: u16){
        println!("{:#X} LD Vx, K woop", _instruction);
        //self.vx[x] = pressed as u8;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx15(&mut self, x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD DT, Vx", _instruction);
        self.dt = x as u8;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx18(&mut self, x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD ST, Vx", _instruction);
        self.st = x as u8;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx55(&mut self, x: usize, memory: &mut Memory, _instruction: u16){
        // Stores V0 to VX in memory starting at address I
        println!("{:#X} LD [I], Vx", _instruction);
        for number in 0..(x+1){
            memory.write_byte(self.i+number as u16, self.vx[number])
          }
        self.i = self.i + x as u16 + 1;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx33(&mut self, x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD B, Vx", _instruction);

        println!("X: {:?}", (self.vx[x]));
        memory.write_byte(self.i, (self.vx[x]/100));
        println!("I1: {:?}", (self.vx[x]/100));
        memory.write_byte(self.i + 1, (self.vx[x]%100)/10);
        println!("I2: {:?}", (self.vx[x]%100)/10);
        memory.write_byte(self.i + 2, (self.vx[x]%10));
        println!("I3: {:?}", (self.vx[x]%10));
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx65(&mut self,x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD Vx, [I]", _instruction);
        for number in 0..(x+1){
          self.vx[number] = memory.read_byte(self.i+number as u16);
          //memory.write_byte(self.i+x as u16, self.get_vx(number));
        }
        self.i = self.i + x as u16 + 1;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx1e(&mut self,x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} ADD I, Vx", _instruction);
        let vx = self.get_vx(x);
        self.i += vx as u16;
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn opcode_fx29(&mut self,x: usize, memory: &mut Memory, _instruction: u16){
        println!("{:#X} LD F, [Vx", _instruction);
        self.i = (self.get_vx(x) as u16) * 5; // sprites are stored 8bit by 5 high starting 0.
        let increment = self.get_pc() + PC_INCREMENT;
        self.set_pc(increment);
    }

    pub fn write_vx(&mut self,x: usize, value: u8){
        self.vx[x] = value;
    }

    pub fn get_vx(&mut self, x: usize) -> u8 {
        self.vx[x]
    }

    pub fn set_pc(&mut self, pc: u16){
        self.pc = pc;
    }

    pub fn get_pc(&mut self) -> u16{
        self.pc
    }
    
}