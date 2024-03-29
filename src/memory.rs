pub struct Memory {
    mem: [u8; 4096],
}

impl Memory {

    pub fn new() -> Memory {
        Memory{
            mem: [0; 4096],
        }   
    }
    
    pub fn write_byte(&mut self, address: u16, value: u8){
        self.mem[address as usize] = value;
    }
    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn sprites(&mut self){
        // 16 sprites of height(elements of u8) 5 (width 8 so u8)
        let sprites: [[u8; 5]; 16] = 
        [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        let mut i = 0; // count for each element inside sprite of sprites for memory
        for sprite in sprites.iter() {
           for &element in sprite {
                self.write_byte(i, element);
                i += 1;
           }
        }
    }
    pub fn full_memory(&mut self) -> [u8; 4096] { 
        self.mem
    }
}