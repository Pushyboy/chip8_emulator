use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct Keyboard {
    pub event_pump: sdl2::EventPump,
    pub awaiting_press: bool,
    pub key_press: Option<u8>,
}

impl Keyboard {
    pub fn new(sdl_context: &Sdl) -> Result<Keyboard, String> {
        let event_pump = sdl_context.event_pump()?;
        let awaiting_press = false;
        let key_press = None;

        Ok(Keyboard { 
            event_pump,
            awaiting_press,
            key_press
        })
    }

    // Might have to check if pressable since it uses a for loop - use a bool
    // called waiting for key press - DONE

    pub fn read_user_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => std::process::exit(0),
                Event::KeyDown { scancode: Some(scancode), ..} => {
                    if self.awaiting_press == false {
                        continue;
                    }

                    match scancode {
                        Scancode::Num1 => self.key_press = Some(0x1),     
                        Scancode::Num2 => self.key_press = Some(0x2), 
                        Scancode::Num3 => self.key_press = Some(0x3),      
                        Scancode::Num4 => self.key_press = Some(0xC),       // C
                        Scancode::Q => self.key_press = Some(0x4),          // 4
                        Scancode::W => self.key_press = Some(0x5),          // 5
                        Scancode::E => self.key_press = Some(0x6),          // 6
                        Scancode::R => self.key_press = Some(0xD),          // D
                        Scancode::A => self.key_press = Some(0x7),          // 7
                        Scancode::S => self.key_press = Some(0x8),          // 8
                        Scancode::D => self.key_press = Some(0x9),          // 9
                        Scancode::F => self.key_press = Some(0xE),          // E
                        Scancode::Z => self.key_press = Some(0xA),          // A
                        Scancode::X => self.key_press = Some(0x0),          // 0
                        Scancode::C => self.key_press = Some(0xB),          // B
                        Scancode::V => self.key_press = Some(0xF),          // F
                        _ => {},
                    }

                },
                _ => {},
            }
        }
    }


    // How does u32 to i32 conversion work

    pub fn key_pressed(&mut self, key: u8) -> bool {
        let keyboard_state = self.event_pump.keyboard_state();
        let key_as_scancode = Keyboard::to_scancode(key).unwrap(); 

        if keyboard_state.is_scancode_pressed(key_as_scancode) {
            return true;
        }

        false
    }

    pub fn to_scancode(key: u8) -> Option<Scancode> {
        match key {
            0x1 => Some(Scancode::Num1),
            0x2 => Some(Scancode::Num2),
            0x3 => Some(Scancode::Num3),
            0x4 => Some(Scancode::Q),
            0x5 => Some(Scancode::W),
            0x6 => Some(Scancode::E),
            0x7 => Some(Scancode::A),
            0x8 => Some(Scancode::S),
            0x9 => Some(Scancode::D),
            0xA => Some(Scancode::Z),
            0xB => Some(Scancode::C),
            0xC => Some(Scancode::Num4),
            0xD => Some(Scancode::R),
            0xE => Some(Scancode::F),
            0xF => Some(Scancode::V),
            _ => None,
        }
    }
}