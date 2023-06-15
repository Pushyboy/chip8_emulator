use rand::Rng;
use std::fs::File;
use std::io::Read;

pub struct Stack {
    data: [u16; 16], 
    pointer: usize,
}

impl Stack {
    const STACK_SIZE: usize = 16;

    pub fn new() -> Stack {
        Stack { 
            data: [0; 16], 
            pointer: 0 
        }
    }

    pub fn push(&mut self, item: u16) {
        if self.pointer < Self::STACK_SIZE {
            self.data[self.pointer] = item;
            self.pointer += 1;
        } else {
            panic!("Stack Overflow.");
        }
    }

    // TODO - Setting it to 0 might not be necessary
    pub fn pop(&mut self) -> Option<u16> {
        if self.pointer > 0 {
            let temp = self.data[self.pointer - 1];
            self.data[self.pointer - 1] = 0;
            self.pointer -= 1;
            Some(temp)
        } else {
            None
        }
    }
}

pub enum State {
    ACTIVE,
    INACTIVE,
}

pub struct Chip_8 {
    pub Display: [[bool;64];32],
    pub RAM: [u8; 4096],
    pub Stack: Stack,
    pub Register: [u8; 16],
    pub I_Register: u16,
    pub PC: u16,
    pub Delay_Timer: u8,
    pub Sound_Timer: u8,
    pub Keyboard: [bool; 16],
    pub State: State
}

impl Chip_8 {
    pub fn build() -> Chip_8 {
        let mut cpu = Chip_8 {
            Display: [[false; 64]; 32],
            RAM: [0; 4096],
            Stack: Stack::new(),
            Register: [0; 16],
            I_Register: 0,
            PC: 0x200,
            Delay_Timer: 0,
            Sound_Timer: 0,
            Keyboard: [false; 16],
            State: State::ACTIVE,
        };

        cpu.load_font();

        cpu
    }

    fn load_font(&mut self) {
        const FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0x0050..=0x009F {
            self.RAM[i] = FONT[i];
        }
    }

    pub fn read_into_memory(&mut self, path: &str) ->  std::io::Result<()> {
        // Open ROM
        let mut file = File::open("file")?;

        // Read ROM contents into a byte array
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Copy ROM to memory
        for i in 0..buffer.len() {
            self.RAM[0x200 + i] = buffer[i];
        }

        Ok(())
    }

    pub fn decrement_timers(&mut self) {
        if self.Delay_Timer > 0 {
            self.Delay_Timer -= 1;
        }

        if self.Sound_Timer > 0 {
            self.Sound_Timer -= 1;
        }
    }

    pub fn fetch_decode_execute(&mut self) {
        let RAM = &self.RAM;
        let loc = self.PC as usize;

        let op_code = ((RAM[loc] as u16) << 8) | (RAM[loc + 1] as u16);
        self.execute(op_code); 
        
        self.PC += 2;
    }

    pub fn execute(&mut self, op_code: u16) {
        if let State::INACTIVE = self.State {
            return;
        }

        let Register = &mut self.Register;

        let x = ((op_code & 0x0F00) >> 8) as usize;
        let y = ((op_code & 0x00F0) >> 4) as usize; 
        
        let prefix = op_code & 0xF000;
        let k = op_code & 0x000F;
        let kk = op_code & 0x00FF;
        let nnn = op_code & 0x0FFF;

        match op_code & 0xF000 {
            0x0000 => {
                match op_code {
                    0x00E0 => self.clear_display(),
                    0x00EE => {
                        self.PC = self.Stack.pop()
                        .expect("Stack is empty");
                    },
                    _ => panic!("Invalid op_code {}", op_code),
                }
            },
            0x1000 => {
                self.PC = nnn;
            },
            0x2000 => {
                self.Stack.push(self.PC);
                self.PC = nnn;
            },
            0x3000 => {
                if Register[x] == (kk as u8) {
                    self.PC += 2;
                }
            },
            0x4000 => {
                if Register[x] != (kk as u8) {
                    self.PC += 2;
                }
            },
            0x5000 => {
                if Register[x] == Register[y] {
                    self.PC += 2;
                }
            },
            0x6000 => {
                Register[x] = kk as u8;
            },
            0x7000 => {
                match Register[x].checked_add(kk as u8) {
                    Some(sum) => {
                        Register[x] = sum;
                    }
                    None => {
                        let sum = (Register[x] as u16) + kk;
                        Register[x] = (sum & 0x00FF) as u8;
                    }
                } 
            },
            0x8000 => match op_code & 0x000F {
                0x0001 => Register[x] |= Register[y],
                0x0002 => Register[x] &= Register[y],
                0x0003 => Register[x] ^= Register[y],
                0x0004 => {
                    match Register[x].checked_add(Register[y]) {
                        Some(sum) => {
                            Register[x] = sum;
                            Register[0x0F] = 0;
                        }
                        None => {
                            let sum = (Register[x] as u16) + (Register[y] as u16);
                            Register[x] = (sum & 0x00FF) as u8;
                            Register[0x0F] = 1;
                        }
                    } 
                },
                0x0005 => {
                    match Register[x].checked_sub(Register[y]) {
                        Some(diff) => {
                            Register[x] = diff;
                            Register[0x0F] = 1;
                        }
                        None => {
                            let diff = Register[x].wrapping_sub(Register[y]);
                            Register[x] = diff;
                            Register[0x0F] = 0;
                        }
                    } 
                },
                0x0006 => {
                    match (Register[x] & 0x01) {
                        0x01 => Register[0x0F] = 1,
                        0x00 => Register[0x0F] = 0,
                        _ => (),
                    }
                    Register[x] >>= 1;
                },
                0x0007 => {
                    match Register[y].checked_sub(Register[x]) {
                        Some(diff) => {
                            Register[x] = diff;
                            Register[0x0F] = 1;
                        }
                        None => {
                            let diff = Register[y].wrapping_sub(Register[x]);
                            Register[x] = diff;
                            Register[0x0F] = 0;
                        }
                    } 
                },
                0x000E => {
                    match (Register[x] & 0x80) >> 7 {
                        0x01 => Register[0x0F] = 1,
                        0x00 => Register[0x0F] = 0,
                        _ => (),
                    }
                    Register[x] <<= 1;
                },
                _ => (),
            },
            0x9000 => {
                if Register[x] != Register[y] {
                    self.PC += 2;
                }
            },
            0xA000 => {
                self.I_Register = nnn;
            },
            0xB000 => {
                self.PC = (Register[0] as u16) + nnn;
            },
            0xC000 => {
                let r = rand::thread_rng().gen_range(0..=255);
                Register[x] = (r & kk) as u8;
            },
            0xD000 => {
                self.draw(x, y, k as usize);  //TODO
            },
            0xE000 => {
                match k {
                    0x000E => {
                        let key = Register[x];
                        if self.Keyboard[key as usize] {
                            self.PC += 2;
                        }
                    },
                    0x0001 => {
                        let key = Register[x];
                        if !self.Keyboard[key as usize] {
                            self.PC += 2;
                        }
                    },
                    _ => (),
                }
            },
            0xF000 => {
                match kk {
                    0x0007 => {
                        Register[x] = self.Delay_Timer;
                    },
                    0x000A => {
                        // TODO
                    },
                    0x0015 => {
                        self.Delay_Timer = Register[x];
                    },
                    0x0018 => {
                        self.Sound_Timer = Register[x];
                    },
                    0x001E => {
                        self.I_Register += Register[x] as u16;
                    },
                    0x0029 => {

                    },
                    0x0033 => {

                    },
                    0x0055 => {

                    },
                    0x0065 => {

                    },
                    _ => (),
                }
            },
            _ => ()
        }
    }
    
    // Read in the sprite from the I register and 
    // draw it to the screen
    fn draw(&mut self, x: usize, y: usize, height: usize) {
        let start = self.I_Register as usize;
        
        // Starting position(should wrap)
        let start_column = (self.Register[x] % 64) as usize;
        let start_row = (self.Register[y] % 32) as usize;

        // Set VF to 0
        self.Register[0x0F] = 0;

        // Read in each row as a byte
        for i in 0..height {
            let row = self.RAM[start + i];
            
            // Read in each bit in the byte
            for j in 0..8 {

                if (start_row + height) > 32 {
                    continue;
                } else if (start_column + j) > 64 {
                    break;
                }

                let curr_bit = (row >> (7-j)) & 0b1;
                let curr_pixel = self.Display[start_row + height][start_column + j];

                match curr_bit {
                    0b0 => {

                    },
                    0b1 => (),
                    _ => ()
                }
            }
            
        }


    }

    fn clear_display(&mut self) {
        self.Display = [[false; 64]; 32];
    }
}

fn main() {
    println!("Hello, world!");
}
