struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
}
