pub struct Cpu {
    /* The virtual cpu required for the chip8 system 
    */
    //  Currently running instruction
    pub current_instruction: u16,

    // Timers
    pub sound_timer: u8,
    pub delay_timer:u8,
    
    // Necessary registers for the operations
    pub registers: [u8;16],
    
    // Misc vars
    pub i: u16,
    pub program_counter: u16,

    // Stack pointer to point to the top of the stack
    pub stack_pointer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {    
                        current_instruction: 0x0000,
                        i: 0x0,
                        program_counter: 0x200,
                        stack_pointer : 255,
                        sound_timer : 0,
                        delay_timer : 0,
                        registers: [0x0000; 16],
                   }
    }

    pub fn update_timers (&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}