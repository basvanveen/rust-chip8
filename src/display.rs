pub struct Display {
    display: [[u8;8];4]
}

impl Display{
    pub fn new() -> Display {
        Display {
            display: [[0;8];4]
        }
    }

    pub fn set_display(&mut self, x: u8, y: u8, n:u8){
        // Jump to location nnn, The interpreter sets the program counter to nnn.
        println!("x:{:?} y:{:?} n:{:?}",x ,y, n);
    }
}