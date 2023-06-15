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
    pub display: [[bool;64];32],
    pub ram: [u8; 4096],
    pub stack: Stack,
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keyboard: [bool; 16],
    pub state: State
}

impl Chip_8 {
    pub fn build() -> Chip_8 {
        let mut cpu = Chip_8 {
            display: [[false; 64]; 32],
            ram: [0; 4096],
            stack: Stack::new(),
            v: [0; 16],
            i: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: [false; 16],
            state: State::ACTIVE,
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
            self.ram[i] = FONT[i];
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
            self.ram[0x200 + i] = buffer[i];
        }

        Ok(())
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn fetch_decode_execute(&mut self) {
        let ram = &self.ram;
        let loc = self.pc as usize;

        let op_code = ((ram[loc] as u16) << 8) | (ram[loc + 1] as u16);
        self.execute(op_code); 
    }

    // Executes opcode
    pub fn execute(&mut self, op_code: u16) {

        if let State::INACTIVE = self.state {
            return;
        }

        let v = &mut self.v;
        let pc = &mut self.pc;

        let prefix = ((op_code & 0xF000) >> 12) as u8;
        let x = ((op_code & 0x0F00) >> 8) as usize;
        let y = ((op_code & 0x00F0) >> 4) as usize;
        let k = (op_code & 0x000F) as u8;
        let kk = (op_code & 0x00FF) as u8;
        let nnn = op_code & 0x0FFF;

        match (prefix, x, y, k) {
            (0x0, 0x0, 0xE, 0x0) => self.clear_display(),
            (0x0, 0x0, 0xE, 0xE) => self.return_from_subroutine(),
            (0x1,   _,   _,   _) => self.jump_to_nnn(nnn),
            (0x2,   _,   _,   _) => self.call_nnn(nnn),
            (0x3,   _,   _,   _) => if v[x] == kk { *pc += 2 },
            (0x4,   _,   _,   _) => if v[x] != kk { *pc += 2 },
            (0x5,   _,   _,   _) => if v[x] == v[y] { *pc += 2 }, 
            (0x6,   _,   _,   _) => v[x] = kk,
            (0x7,   _,   _,   _) => self.add_kk_to_vx(x, kk),
            (0x8,   _,   _, 0x0) => v[x] = v[y],
            (0x8,   _,   _, 0x1) => v[x] |= v[y],
            (0x8,   _,   _, 0x2) => v[x] &= v[y],
            (0x8,   _,   _, 0x3) => v[x] ^= v[y],
            (0x8,   _,   _, 0x4) => self.add_vy_to_vx(x, y),
            (0x8,   _,   _, 0x5) => self.sub_vy_from_vx(x, y),
            (0x8,   _,   _, 0x6) => self.shift_vx_right(x),
            (0x8,   _,   _, 0x7) => self.sub_vx_from_vy(x, y),
            (0x8,   _,   _, 0xE) => self.shift_vx_left(x),
            (0x9,   _,   _, 0x0) => if v[x] != v[y] { *pc += 2 },
            (0xA,   _,   _,   _) => self.i = nnn,
            (0xB,   _,   _,   _) => self.pc = (v[0] as u16) + nnn,
            (0xC,   _,   _,   _) => self.gen_random(x, kk),
            (0xD,   _,   _,   _) => self.draw(x, y, k as usize),
            (0xE,   _, 0x9, 0xE) => self.skip_if_key(x),
            (0xE,   _, 0xA, 0x1) => self.skip_if_nkey(x),
            (0xF,   _, 0x0, 0x7) => v[x] = self.delay_timer,
            (0xF,   _, 0x0, 0xA) => self.wait_for_key(),
            (0xF,   _, 0x1, 0x5) => self.delay_timer = v[x],
            (0xF,   _, 0x1, 0x8) => self.sound_timer = v[x],
            (0xF,   _, 0x1, 0xE) => self.i += v[x] as u16,              // Might Overflow?????
            (0xF,   _, 0x2, 0x9) => self.get_font_key(x),
            (0xF,   _, 0x3, 0x3) => self.store_BCD(x),
            (0xF,   _, 0x5, 0x5) => self.v_dump(x),
            (0xF,   _, 0x6, 0x5) => self.v_load(x),
            _                    => panic!("Error! Bad Opcode {}", op_code),    // Find a way to make it hexadecimal

        }

        // Increment pc if the code isnt 00EE 1NNN 2NNN
        if op_code != 0x00EE || prefix != 1 || prefix != 2 {
            self.pc += 2;
        }

    }
    
    // Read in the sprite from the I register and 
    // draw it to the screen
    fn draw(&mut self, x: usize, y: usize, height: usize) {
        let start = self.i as usize;
        
        // Starting position(should wrap)
        let start_column = (self.v[x] % 64) as usize;
        let start_row = (self.v[y] % 32) as usize;

        // Set VF to 0
        self.v[0x0F] = 0;

        // Read in each row as a byte
        for i in 0..height {
            let row = self.ram[start + i];
            
            // Read in each bit in the byte
            for j in 0..8 {

                if (start_row + height) > 32 {
                    continue;
                } else if (start_column + j) > 64 {
                    break;
                }

                let curr_bit = (row >> (7-j)) & 0b1;
                let curr_pixel = self.display[start_row + height][start_column + j];

                match curr_bit {
                    0b0 => {

                    },
                    0b1 => (),
                    _ => ()
                }
            }
            
        }
    }

    // Clear the display
    fn clear_display(&mut self) {
        self.display = [[false; 64]; 32];
    }

    // Return from a subroutine
    fn return_from_subroutine(&mut self) {
        self.pc = self.stack.pop().expect("Empty stack.");
    }

    // Jump to location nnn
    fn jump_to_nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // Call subroutine nnn
    fn call_nnn(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    // Stores the sum of Vx and kk into Vx
    fn add_kk_to_vx(&mut self, x: usize, kk: u8) {
        let v = &mut self.v;

        match v[x].checked_add(kk) {
            Some(sum) => {
                v[x] = sum;
            }
            None => {
                let sum = (v[x] as u16) + (kk as u16);
                v[x] = (sum & 0x00FF) as u8;
            }
        } 
    }

    // Adds the value in Vy to Vx. Carries if necessary
    fn add_vy_to_vx(&mut self, x: usize, y: usize) {
        let v = &mut self.v;

        match v[x].checked_add(v[y]) {
            Some(sum) => {
                v[x] = sum;
                v[0x0F] = 0;
            }
            None => {
                let sum = (v[x] as u16) + (v[y] as u16);
                v[x] = (sum & 0x00FF) as u8;
                v[0x0F] = 1;
            }
        } 
    }

    // Subtracts the value in Vy from Vx. Borrows if necessary
    fn sub_vy_from_vx(&mut self, x: usize, y: usize) {
        let v = &mut self.v;

        match v[x].checked_sub(v[y]) {
            Some(diff) => {
                v[x] = diff;
                v[0x0F] = 1;
            }
            None => {
                let diff = v[x].wrapping_sub(v[y]);
                v[x] = diff;
                v[0x0F] = 0;
            }
        } 
    }

    // Bitshifts the value in Vx to the right and 
    // stores the lsb in Vf
    fn shift_vx_right(&mut self, x: usize) {
        let v = &mut self.v;

        match (v[x] & 0x01) {
            0x01 => v[0x0F] = 1,
            0x00 => v[0x0F] = 0,
            _ => (),
        }
        v[x] >>= 1;
    }

    // Sets Vx = Vy - Vx and borrows if necessary
    fn sub_vx_from_vy(&mut self, x: usize, y: usize) {
        let v = &mut self.v;

        match v[y].checked_sub(v[x]) {
            Some(diff) => {
                v[x] = diff;
                v[0x0F] = 1;
            }
            None => {
                let diff = v[y].wrapping_sub(v[x]);
                v[x] = diff;
                v[0x0F] = 0;
            }
        } 
    }

    // Bitshifts the value in Vx to the left and 
    // stores the msb in Vf
    fn shift_vx_left(&mut self, x: usize) {
        let v = &mut self.v;

        match (v[x] & 0x80) >> 7 {
            0x01 => v[0x0F] = 1,
            0x00 => v[0x0F] = 0,
            _ => (),
        }
        v[x] <<= 1;
    }

    // Generate a random number and logical AND 
    // it with kk
    fn gen_random(&mut self, x: usize, kk: u8) {
        let r = rand::thread_rng().gen_range(0..=255);
        self.v[x] = r & kk;
    }

    // TODO Investiagte if evaluating the value counts as a immutable reference

    fn skip_if_key(&mut self, x: usize) {
        let key = self.v[x];
        if self.keyboard[key as usize] {
            self.pc += 2;
        }
    }

    fn skip_if_nkey(&mut self, x: usize) {
        let key = self.v[x];

        if !self.keyboard[key as usize] {
            self.pc += 2;
        }
    }

    // Sets i to the address of a font character
    fn get_font_key(&mut self, x: usize) {
        self.i = 0x0050 + 5 * x as u16;
    }

    // Stop performing instructions until a key
    // is pressed so it can be stored in vx
    fn wait_for_key(&mut self) {
        self.state = State::INACTIVE;

        // When you press a key, you can check if the state
        // is inactive and set vx to if it it is
    }

    // Stores the BCD of the value in Vx in RAM
    fn store_BCD(&mut self, x: usize) {
        let i = self.i as usize;
        let num = self.v[x];

        let num_hun = num % 10;
        let num_ten = num / 10 % 10;
        let num_one = num / 100 % 10;

        self.ram[i] = num_hun;
        self.ram[i + 1] = num_ten;
        self.ram[i + 2] = num_one;
    }

    // Dumps V0..Vx in RAM starting at i
    fn v_dump(&mut self, x: usize) {
        let i = self.i as usize;

        for loc in 0..=x {
            self.ram[i + loc] = self.v[loc];
        }
    }

    // Fills V0..Vx from RAM starting at i
    fn v_load(&mut self, x: usize) {
        let i = self.i as usize;

        for loc in 0..=x {
            self.v[loc] = self.ram[i + loc];
        }
    }


}


fn main() {
    println!("Hello, world!");
}
