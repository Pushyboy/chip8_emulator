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