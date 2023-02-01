mod cpu;
mod memory;

pub struct Nes {
   pub cpu: cpu::Cpu,
   pub memory: memory::Memory 
}

impl Nes {
    pub fn new() -> Nes {
        return Nes  {
                        cpu: cpu::Cpu::new(),
                        memory: memory::Memory::new()
                    }
    }
// >>>> Flag operations start >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    fn set_carry_flag(&mut self, arg: u8) {
        if arg & 0x01 == 0x01 {
            self.cpu.status |= 0b0000_0001;
        }
    }

    fn set_zero_flag(&mut self, arg: u8) {
        if arg == 0 {
            self.cpu.status |= 0b0000_0010;
        }
    }

    fn set_negative_flag(&mut self, arg: u8) {
        if arg & 0b1000_0000 == 0b1000_0000 {
            self.cpu.status |= 0b1000_0000;
        }
    }


// <<<< Flag operations end <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> Addressing modes start >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    fn immediate(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        return self.memory.primary_memory[self.cpu.program_counter as usize];
    }

    fn zero_page(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        self.cpu.program_counter += 1;
        let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
        return self.memory.primary_memory[zero_page_address as usize];
    }

    fn zero_page_x(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);

        return self.memory.primary_memory[zero_page_address as usize];
    }

    fn zero_page_y(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.y);                // two cpu cycle

        return self.memory.primary_memory[zero_page_address as usize];
    }

    fn absolute(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        self.cpu.program_counter += 1;
        let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        let absolute_address: u16 = (hi as u16) << 8 | lo as u16;

        return self.memory.primary_memory[absolute_address as usize];
    }

    fn absolute_x(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        self.cpu.program_counter += 1;
        let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.x as u16);

        return self.memory.primary_memory[absolute_address as usize];
    }

    fn absolute_y(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        self.cpu.program_counter += 1;
        let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16);

        return self.memory.primary_memory[absolute_address as usize];
    }

    fn indirect(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let mut lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        self.cpu.program_counter += 1;
        let mut hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                        // one cpu cycle
        let address: u16 = (hi as u16) << 8 | lo as u16;

        lo = self.memory.primary_memory[address as usize];
                                                        // one cpu cycle

        hi = self.memory.primary_memory[address as usize + 1];
                                                        // one cpu cycle

        return ((hi as u16) << 8 | lo as u16 - 1) as u8;
                                                        // -1 because the pc is incremented outside the switch block
    }

    fn indexed_indirect(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let indirect_address = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);             // one cpu cycle

        let lo: u8 = self.memory.primary_memory[indirect_address as usize];
        let hi: u8 = self.memory.primary_memory[indirect_address as usize + 1];
                                                        // two cpu cycle

        let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
        return self.memory.primary_memory[absolute_address as usize];
    }

    fn indirect_indexed(&mut self) -> u8 {
        self.cpu.program_counter += 1;
        let indirect_address = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle

        let lo: u8 = self.memory.primary_memory[indirect_address as usize];
        let hi: u8 = self.memory.primary_memory[indirect_address as usize + 1];
                                                        // two cpu cycle

        let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16) ;
        return self.memory.primary_memory[absolute_address as usize];
    }


// <<<< Addressing modes end <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    pub fn execute(&mut self, instruction: u8) {
        
        match instruction {

// >>>> AND starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x29 => {                                           // Immediate
                self.cpu.accumulator &= self.immediate();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x25 => {                                           // Zero page
                self.cpu.accumulator &= self.zero_page();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x35 => {                                           // Zero page, X
                self.cpu.accumulator &= self.zero_page_x();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x2D => {                                           // Absolute
                self.cpu.accumulator &= self.absolute();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x3D => {                                           // Absolute, X
                self.cpu.accumulator &= self.absolute_x();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x39 => {                                           // Absolute, Y
                self.cpu.accumulator &= self.absolute_y();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x21 => {                                           // (Indirect, X)
                self.cpu.accumulator &= self.indexed_indirect();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x31 => {                                           // (Indirect), Y
                self.cpu.accumulator &= self.indirect_indexed();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },

// <<<< AND ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> ASL starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
            
            0x0A => {                                           // Accumulator
                let mut contents: i8 = self.cpu.accumulator as i8;
                contents <<= 1;
                self.cpu.accumulator = contents as u8;
                self.set_carry_flag(self.cpu.accumulator);
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x06 => {                                           // Zero page
                let memory_location = self.zero_page();
                let mut contents: i8 = self.memory.primary_memory[memory_location as usize] as i8;
                contents <<= 1;
                self.memory.primary_memory[memory_location as usize] = contents as u8;
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize]);
            },
            0x16 => {                                           // Zero page, X
                let memory_location = self.zero_page_x();
                let mut contents: i8 = self.memory.primary_memory[memory_location as usize] as i8;
                contents <<= 1;
                self.memory.primary_memory[memory_location as usize] = contents as u8;
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize]);
            },
            0x0E => {                                           // Absolute
                let memory_location = self.absolute();
                let mut contents: i8 = self.memory.primary_memory[memory_location as usize] as i8;
                contents <<= 1;
                self.memory.primary_memory[memory_location as usize] = contents as u8;
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize]);
            },
            0x1E => {                                           // Absolute, X
                let memory_location = self.absolute_x();
                let mut contents: i8 = self.memory.primary_memory[memory_location as usize] as i8;
                contents <<= 1;
                self.memory.primary_memory[memory_location as usize] = contents as u8;
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize]);
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize]);
            },

// <<<< ASL ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BCC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x90 => {
                if self.cpu.status & 0b0000_0001 == 0b0000_0000 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BCC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BCS starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xB0 => {
                if self.cpu.status & 0b0000_0001 == 0b0000_0001 {
                    let offset: u8 = self.memory.primary_memory[self.cpu.program_counter as usize + 1];
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset as u16);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BCS ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BEQ starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
            
            0xF0 => {
                if self.cpu.status & 0b0000_0010 == 0b0000_0010 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BEQ ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BIT starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< BIT ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BMI starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x30 => {
                if self.cpu.status & 0b1000_0000 == 0b1000_0000 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BMI ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BNE starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xD0 => {
                if self.cpu.status & 0b0000_0010 == 0b0000_0000 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BNE ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BPL starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x10 => {
                if self.cpu.status & 0b1000_0000 == 0b0000_0000 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BPL ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BRK starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< BRK ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BVC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x50 => {
                if self.cpu.status & 0b0100_0000 == 0b0000_0000 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BVC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> BVS starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x70 => {
                if self.cpu.status & 0b0100_0000 == 0b0100_0000 {
                    let offset: u16 = self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
                    self.cpu.program_counter = self.cpu.program_counter.wrapping_add(offset);
                    self.cpu.program_counter -= 1;              // To cancel out the final incr
                }
            },

// <<<< BVS ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CLC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x18 => {
                self.cpu.status &= 0b1111_1110;
            },

// <<<< CLC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CLD starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xD8 => {
                self.cpu.status &= 0b1111_0111;
            },

// <<<< CLD ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CLI starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x58 => {
                self.cpu.status &= 0b1111_1011;
            },

// <<<< CLI ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CLV starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xB8 => {
                self.cpu.status &= 0b1011_1111;
            },

// <<<< CLV ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CMP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xC9 => {                                           // Immediate
                let result = self.cpu.accumulator - self.immediate();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xC5 => {                                           // Zero page
                let result = self.cpu.accumulator - self.zero_page();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xD5 => {                                           // Zero page, X
                let result = self.cpu.accumulator - self.zero_page_x();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xCD => {                                           // Absolute
                let result = self.cpu.accumulator - self.absolute();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xDD => {                                           // Absolute, X
                let result = self.cpu.accumulator - self.absolute_x();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xD9 => {                                           // Absolute, Y
                let result = self.cpu.accumulator - self.absolute_y();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xC1 => {                                           // (Indirect, X)
                let result = self.cpu.accumulator - self.indexed_indirect();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xD1 => {                                           // (Indirect, X)
                let result = self.cpu.accumulator - self.indirect_indexed();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            
// <<<< CMP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CPX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xE0 => {                                           // Immediate
                let result = self.cpu.x - self.immediate();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xE4 => {                                           // Zero page
                let result = self.cpu.x - self.zero_page();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xEC => {                                           // Absolute
                let result = self.cpu.x - self.absolute();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },

// <<<< CPX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> CPY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xC0 => {                                           // Immediate
                let result = self.cpu.y - self.immediate();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xC4 => {                                           // Zero page
                let result = self.cpu.y - self.zero_page();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },
            0xCC => {                                           // Absolute
                let result = self.cpu.y - self.absolute();
                self.set_carry_flag(result);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            },

// <<<< CPY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> DEC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xC6 => {                                           // Zero page
                let memory_location = self.zero_page();
                self.memory.primary_memory[memory_location as usize]  -= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0xD6 => {                                           // Zero page, X
                let memory_location = self.zero_page_x();
                self.memory.primary_memory[memory_location as usize]  -= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0xCE => {                                           // Absolute
                let memory_location = self.absolute();
                self.memory.primary_memory[memory_location as usize]  -= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0xDE => {                                           //Absolute, X
                let memory_location = self.absolute_x();
                self.memory.primary_memory[memory_location as usize]  -= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },

// <<<< DEC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> DEX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xCA => {                                           // Implied
                self.cpu.x -= 1;
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },

// <<<< DEX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> DEY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x88 => {                                           // Implied
                self.cpu.y -= 1;
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },

// <<<< DEY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> EOR starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< EOR ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> INC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xE6 => {                                           // Zero page
                let memory_location = self.zero_page();
                self.memory.primary_memory[memory_location as usize]  += 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0xF6 => {                                           // Zero page, X
                let memory_location = self.zero_page_x();
                self.memory.primary_memory[memory_location as usize]  += 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0xEE => {                                           // Absolute
                let memory_location = self.absolute();
                self.memory.primary_memory[memory_location as usize]  += 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0xFE => {                                           //Absolute, X
                let memory_location = self.absolute_x();
                self.memory.primary_memory[memory_location as usize]  += 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },

// <<<< INC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> INX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xE8 => {                                           // Implied
                self.cpu.x += 1;
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },

// <<<< INX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> INY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xC8 => {                                           // Implied
                self.cpu.y += 1;
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },

// <<<< INY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> JMP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x4C => {                                           // Absolute
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle

                self.cpu.program_counter = (hi as u16) << 8 | lo as u16;
            },
            0x6C => {                                           // Indirect
                self.cpu.program_counter += 1;
                let mut lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let mut hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let address: u16 = (hi as u16) << 8 | lo as u16;

                lo = self.memory.primary_memory[address as usize];
                                                                // one cpu cycle

                hi = self.memory.primary_memory[address as usize + 1];
                                                                // one cpu cycle

                self.cpu.program_counter = (hi as u16) << 8 | lo as u16 - 1;
                                                                // To cancel out the final incr
            }

// <<<< JMP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> JSR starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x20 => {                                           // Absolute
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle

                self.cpu.stack_pointer -= 1;

                self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize] = (self.cpu.program_counter >> 8) as u8 & 0xFF;
                self.cpu.stack_pointer -= 1;

                self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize] = self.cpu.program_counter as u8 & 0xFF;
                self.cpu.stack_pointer -= 1;

                self.cpu.program_counter = (hi as u16) << 8 | lo as u16;
            },

// <<<< JSR ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> LDA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xA9 => {                                           // Immediate
                self.cpu.accumulator = self.immediate();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xA5 => {                                           // Zero page
                self.cpu.accumulator = self.zero_page();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xB5 => {                                           // Zero page, X
                self.cpu.accumulator = self.zero_page_x();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xAD => {                                           // Absolute
                self.cpu.accumulator = self.absolute();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xBD => {                                           // Absolute, X
                self.cpu.accumulator = self.absolute_x();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xB9 => {                                           // Absolute, Y
                self.cpu.accumulator = self.absolute_y();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xA1 => {                                           // (Indirect, X)
                self.cpu.accumulator = self.indexed_indirect();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0xB1 => {                                           // (Indirect), Y
                self.cpu.accumulator = self.indirect_indexed();
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },

// <<<< LDA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> LDX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xA2 => {                                           // Immediate
                self.cpu.x = self.immediate();
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },
            0xA6 => {                                           // Zero page
                self.cpu.x = self.zero_page();
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },
            0xB6 => {                                           // Zero page, Y
                self.cpu.x = self.zero_page_y();
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },
            0xAE => {                                           // Absolute
                self.cpu.x = self.absolute();
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },
            0xBE => {                                           // Absolute, Y
                self.cpu.x = self.absolute_y();
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },

// <<<< LDX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> LDY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xA0 => {                                           // Immediate
                self.cpu.y = self.immediate();
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },
            0xA4 => {                                           // Zero page
                self.cpu.y = self.zero_page();
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },
            0xB4 => {                                           // Zero page, X
                self.cpu.y = self.zero_page_x();
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },
            0xAC => {                                           // Absolute
                self.cpu.y = self.absolute();
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },
            0xBC => {                                           // Absolute, X
                self.cpu.y = self.absolute_x();
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },

// <<<< LDY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> LSR starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x4A => {                                           // Accumulator
                self.set_carry_flag(self.cpu.accumulator);
                self.cpu.accumulator = self.cpu.accumulator >> 1;
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x46 => {                                           // Zero page
                let memory_location = self.zero_page();
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize] );
                self.memory.primary_memory[memory_location as usize]  >>= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0x56 => {                                           // Zero page, X
                let memory_location = self.zero_page_x();
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize] );
                self.memory.primary_memory[memory_location as usize]  >>= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0x4E => {                                           // Absolute
                let memory_location = self.absolute();
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize] );
                self.memory.primary_memory[memory_location as usize]  >>= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },
            0x5E => {                                           // Absolute, X
                let memory_location = self.absolute_x();
                self.set_carry_flag(self.memory.primary_memory[memory_location as usize] );
                self.memory.primary_memory[memory_location as usize]  >>= 1;
                self.set_zero_flag(self.memory.primary_memory[memory_location as usize] );
                self.set_negative_flag(self.memory.primary_memory[memory_location as usize] );
            },

// <<<< LSR ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> NOP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xEA => {
                /* No operation */                              // Implied
            },

// <<<< NOP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> ORA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x09 => {                                           // Immediate
                self.cpu.accumulator  |= self.memory.primary_memory[self.immediate() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x05 => {                                           // Zero page

                self.cpu.accumulator  |= self.memory.primary_memory[self.zero_page() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x15 => {                                           // Zero page, X

                self.cpu.accumulator  |= self.memory.primary_memory[self.zero_page_x() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x0D => {                                           // Absolute

                self.cpu.accumulator  |= self.memory.primary_memory[self.absolute() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x1D => {                                           // Absolute, X

                self.cpu.accumulator  |= self.memory.primary_memory[self.absolute_x() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x19 => {                                           // Absolute, Y

                self.cpu.accumulator  |= self.memory.primary_memory[self.absolute_y() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x01 => {                                           // (Indirect, X)

                self.cpu.accumulator  |= self.memory.primary_memory[self.indexed_indirect() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },
            0x11 => {                                           // (Indirect), Y

                self.cpu.accumulator  |= self.memory.primary_memory[self.indirect_indexed() as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },

// <<<< ORA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> PHA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x48 => {                                           // Implied
                self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize] = self.cpu.accumulator;                                    // two cpu cycle
                self.cpu.stack_pointer -= 1;
            },

// <<<< PHA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> PHP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x08 => {                                           // Implied
                self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize] = self.cpu.status;                                         // two cpu cycle
                self.cpu.stack_pointer -= 1;
            },

// <<<< PHP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> PLA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x68 => {                                           // Implied
                self.cpu.stack_pointer += 1;
                self.cpu.accumulator = self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize];
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            }

// <<<< PLA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> PLP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x28 => {                                           // Implied
                self.cpu.stack_pointer += 1;
                self.cpu.status = self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize]; 
            },

// <<<< PLP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> ROL starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< ROL ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> ROR starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< ROR ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> RTI starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< RTI ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> RTS starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< RTS ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> SBC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< SBC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> SEC starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< SEC ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> SED starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< SED ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> SEI starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< SEI ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> STA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< STA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> STX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< STX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> STY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// <<<< STY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> TAX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xAA => {                                           // Implied
                self.cpu.x = self.cpu.accumulator;
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },

// <<<< TAX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> TAY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xA8 => {                                           // Implied
                self.cpu.y = self.cpu.accumulator;
                self.set_zero_flag(self.cpu.y);
                self.set_negative_flag(self.cpu.y);
            },

// <<<< TAY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> TSX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xBA => {                                           // Implied
                self.cpu.x = self.cpu.stack_pointer;
                self.set_zero_flag(self.cpu.x);
                self.set_negative_flag(self.cpu.x);
            },

// <<<< TSX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> TXA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x8A => {                                           // Implied
                self.cpu.accumulator = self.cpu.y;
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },

// <<<< TXA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> TXS starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x9A => {                                           // Implied
                self.cpu.stack_pointer = self.cpu.x;
            },

// <<<< TXS ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >>>> TYA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x98 => {                                           // Implied
                self.cpu.accumulator = self.cpu.y;
                self.set_zero_flag(self.cpu.accumulator);
                self.set_negative_flag(self.cpu.accumulator);
            },

// <<<< TYA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

            _ => panic!("Unrecognized instruction {:#X}", instruction) 
        }
        self.cpu.program_counter += 1;                         // one cpu cycle
    }

    pub fn emulate_cycle(&mut self) {
        self.execute(self.memory.primary_memory[self.cpu.program_counter as usize]);
    }

    pub fn debug(&self) {
        println!("Accumulator: {:#X}, x: {:#X}, y: {:#X}, status: {:#b}, Program counter: {:#X}", self.cpu.accumulator, self.cpu.x, self.cpu.y, self.cpu.status, self.cpu.program_counter);
    }
}