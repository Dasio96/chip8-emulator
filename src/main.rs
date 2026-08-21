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

            op if (op & 0xF000) == 0x1000 => {
                self.pc = op & 0x0FFF;
            }

            op if (op & 0xF000) == 0x6000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let nn = (op & 0x00FF) as u8;
                self.registers[x] = nn;
            }

            op if (op & 0xF000) == 0x7000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let nn = (op & 0x00FF) as u8;
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }

            op if (op & 0xF000) == 0xA000 => {
                self.i = op & 0x0FFF;
            }

            op if (op & 0xF000) == 0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = op & 0x0FFF;
            }

            0x00EE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            op if (op & 0xF000) == 0x3000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let nn = (op & 0x00FF) as u8;
                if self.registers[x] == nn {
                    self.pc += 2
                }
            }

            op if (op & 0xF000) == 0x4000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let nn = (op & 0x00FF) as u8;
                if self.registers[x] != nn {
                    self.pc += 2
                }
            }

            op if (op & 0xF00F) == 0x5000 => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }

            op if (op & 0xF00F) == 0x9000 => {
                let x = ((op & 0xF00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }

            op if (op & 0xF000) == 0x6000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let nn = (op & 0x00FF) as u8;
                self.registers[x] = nn;
            }

            op if (op & 0xF000) == 0x7000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let nn = (op & 0x00FF) as u8;
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }

            op if (op & 0xF00F) == 0x8000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                self.registers[x] = self.registers[y];
            }

            op if (op & 0xF00F) == 0x8001 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                self.registers[x] |= self.registers[y];
            }

            op if (op & 0xF00F) == 0x8002 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                self.registers[x] &= self.registers[y];
            }

            op if (op & 0xF00F) == 0x8003 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                self.registers[x] ^= self.registers[y];
            }

            op if (op & 0xF00F) == 0x8004 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                let (sum, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = sum;
                self.registers[0xF] = if overflow { 1 } else { 0 };
            }

            op if (op & 0xF00F) == 0x8005 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                let (diff, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = diff;
                self.registers[0xF] = if !overflow { 1 } else { 0 };
            }

            op if (op & 0xF00F) == 0x8006 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let lsb = self.registers[x] & 1;
                self.registers[x] >>= 1;
                self.registers[0xF] = lsb;
            }

            op if (op & 0xF00F) == 0x8007 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;

                let (diff, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = diff;
                self.registers[0xF] = if !overflow { 1 } else { 0 };
            }

            op if (op & 0xF00F) == 0x800E => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let msb = (self.registers[x] & 0x80) >> 7;
                self.registers[x] <<= 1;
                self.registers[0xF] = msb;
            }

            op if (op & 0xF00F) == 0x9000 => {
                let x = ((op & 0x0F00) >> 8) as usize;
                let y = ((op & 0x00F0) >> 4) as usize;
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }

            op if (op & 0xF000) == 0xA000 => {
                let addr = op & 0x0FFF;
                self.i = addr;
            }

            op if (op & 0xF000) == 0xB000 => {
                let addr = op & 0x0FFF;
                self.pc = addr + (self.registers[0] as u16);
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
