struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    display: [[bool; 64]; 32],
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 4096];
        memory[0x050..0x050 + FONTSET.len()].copy_from_slice(&FONTSET);
        Chip8 {
            memory,
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            display: [[false; 64]; 32],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> std::io::Result<()> {
        let bytes = std::fs::read(path)?;

        let start = 0x200;
        let end = start + bytes.len();

        self.memory[start..end].copy_from_slice(&bytes);

        Ok(())
    }

    pub fn cycle(&mut self) {
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize] as u16);
        self.pc += 2;

        match opcode {
            0x00E0 => {
                self.display = [[false; 64]; 32];
            }
            _ => println!("{:#06X}", opcode),
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    let _ = chip8.load_rom("roms/Pong.ch8");

    loop {
        chip8.cycle();
    }
}
