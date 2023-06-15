use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

struct Keyboard {
    event_pump: sdl2::EventPump,
    is_pressed: bool,
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new(sdl_context: &mut Sdl) -> Result<Keyboard, String> {
        let mut event_pump = sdl_context.event_pump()?;
        let is_pressed = false;
        let key_pressed = None;

        Ok(Keyboard { 
            event_pump,
            is_pressed,
            key_pressed
        })
    }



    // Store the key pressed as an Option<u8> and the state when pressed.
    // In the main loop, if it is awaiting a key stroke and a key is pressed 
    pub fn read_user_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break,
                Event::KeyDown { scancode: Some(scancode), ..} => {
                    match scancode {
                        Scancode::Num1 => self.key_pressed = Some(0x1),     
                        Scancode::Num2 => self.key_pressed = Some(0x2), 
                        Scancode::Num3 => self.key_pressed = Some(0x3),      
                        Scancode::Num4 => self.key_pressed = Some(0xC),       // C
                        Scancode::Q => self.key_pressed = Some(0x4),          // 4
                        Scancode::W => self.key_pressed = Some(0x5),          // 5
                        Scancode::E => self.key_pressed = Some(0x6),          // 6
                        Scancode::R => self.key_pressed = Some(0xD),          // D
                        Scancode::A => self.key_pressed = Some(0x7),          // 7
                        Scancode::S => self.key_pressed = Some(0x8),          // 8
                        Scancode::D => self.key_pressed = Some(0x9),          // 9
                        Scancode::F => self.key_pressed = Some(0xE),          // E
                        Scancode::Z => self.key_pressed = Some(0xA),          // A
                        Scancode::X => self.key_pressed = Some(0x0),          // 0
                        Scancode::C => self.key_pressed = Some(0xB),          // B
                        Scancode::V => self.key_pressed = Some(0xF),          // F
                        _ => {},
                    }

                    self.is_pressed = true;
                },
                _ => {},
            }
        }
    }
}