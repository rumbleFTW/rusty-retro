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

    pub fn execute(&mut self, instruction: u8) {
        
        match instruction {

// >> JMP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
            
            0x4C => {
                /*  Sets the program counter to the address specified by the operand.
                    Opcode: JMP
                    Address mode: Absolute
                    Alias: JMP_ABS
                    Bytes: 3
                    Cycles: 3
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle

                self.cpu.program_counter = (hi as u16) << 8 | lo as u16;

            },
            0x6C => {
                /*  Sets the program counter to the address specified by the operand.
                    Opcode: JMP
                    Address mode: Indirect
                    Alias: JMP_IND
                    Bytes: 3
                    Cycles: 5
                */
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
                                                                // -1 because the pc is incremented outside the switch block
            },

// << JMP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> JSR starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
            
            0x20 => {
                /*  The JSR instruction pushes the address (minus one) of the return point on to 
                    the stack and then sets the program counter to the target memory address.
                    Opcode: JSR
                    Address mode: Absolute
                    Alias: JSR_ABS
                    Bytes: 3
                    Cycles: 6
                */
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

// << JSR ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> LDA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xA9 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Immediate
                    Alias: LDA_IM
                    Bytes: 2
                    Cycles: 2
                */
                self.cpu.program_counter += 1;
                self.cpu.accumulator = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xA5 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Zero page
                    Alias: LDA_ZP
                    Bytes: 2
                    Cycles: 3
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle
                self.cpu.accumulator = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xB5 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Zero page, X
                    Alias: LDA_ZPX
                    Bytes: 2
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);                // two cpu cycle
                self.cpu.accumulator = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xAD => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Absolute
                    Alias: LDA_ABS
                    Bytes: 3
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
                self.cpu.accumulator = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xBD => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Absolute, X
                    Alias: LDA_ABX
                    Bytes: 3
                    Cycles: 4 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.x as u16);
                self.cpu.accumulator = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xB9 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Absolute, Y
                    Alias: LDA_ABY
                    Bytes: 3
                    Cycles: 4 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16);
                self.cpu.accumulator = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xA1 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: (Indirect, X)
                    Alias: LDA_INX
                    Bytes: 2
                    Cycles: 6
                */
                self.cpu.program_counter += 1;
                let indirect_address = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);             // one cpu cycle

                let lo: u8 = self.memory.primary_memory[indirect_address as usize];
                let hi: u8 = self.memory.primary_memory[indirect_address as usize + 1];
                                                                // two cpu cycle

                let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
                self.cpu.accumulator = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xB1 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: (Indirect), Y
                    Alias: LDA_INY
                    Bytes: 2
                    Cycles: 5 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let indirect_address = self.memory.primary_memory[self.cpu.program_counter as usize];                                      // one cpu cycle

                let lo: u8 = self.memory.primary_memory[indirect_address as usize];
                let hi: u8 = self.memory.primary_memory[indirect_address as usize + 1];
                                                                // two cpu cycle

                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16) ;
                self.cpu.accumulator = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },

// << LDA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> LDX starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
            
            0xA2 => {
                /*  Loads a byte of memory into the X register setting the zero and negative flags 
                    as appropriate.
                    Opcode: LDX
                    Address mode: Immediate
                    Alias: LDX_IM
                    Bytes: 2
                    Cycles: 2
                */
                self.cpu.program_counter += 1;
                self.cpu.x = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle

                if self.cpu.x == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.x & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xA6 => {
                /*  Loads a byte of memory into the X register setting the zero and negative flags 
                    as appropriate.
                    Opcode: LDX
                    Address mode: Zero page
                    Alias: LDX_ZP
                    Bytes: 2
                    Cycles: 3
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle
                self.cpu.x = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.x == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.x & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xB6 => {
                /*  Loads a byte of memory into the X register setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDX
                    Address mode: Zero page, Y
                    Alias: LDX_ZPY
                    Bytes: 2
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.y);                // two cpu cycle
                self.cpu.x = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.x == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.x & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xAE => {
                /*  Loads a byte of memory into the X register setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDX
                    Address mode: Absolute
                    Alias: LDX_ABS
                    Bytes: 3
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
                self.cpu.x = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.x == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.x & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xBE => {
                /*  Loads a byte of memory into the X register setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDX
                    Address mode: Absolute, Y
                    Alias: LDX_ABY
                    Bytes: 3
                    Cycles: 4 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16);
                self.cpu.x = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.x == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.x & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },

// << LDX ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> LDY starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
            
            0xA0 => {
                /*  Loads a byte of memory into the Y register setting the zero and negative flags 
                    as appropriate.
                    Opcode: LDY
                    Address mode: Immediate
                    Alias: LDY_IM
                    Bytes: 2
                    Cycles: 2
                */
                self.cpu.program_counter += 1;
                self.cpu.y = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle

                if self.cpu.y == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.y & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xA4 => {
                /*  Loads a byte of memory into the Y register setting the zero and negative flags 
                    as appropriate.
                    Opcode: LDY
                    Address mode: Zero page
                    Alias: LDY_ZP
                    Bytes: 2
                    Cycles: 3
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle
                self.cpu.y = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.y == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.y & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xB4 => {
                /*  Loads a byte of memory into the Y register setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDY
                    Address mode: Zero page, X
                    Alias: LDY_ZPX
                    Bytes: 2
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);                // two cpu cycle
                self.cpu.y = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.y == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.y & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xAC => {
                /*  Loads a byte of memory into the Y register setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDY
                    Address mode: Absolute
                    Alias: LDY_ABS
                    Bytes: 3
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
                self.cpu.y = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.y == 0 {                            // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.y & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0xBC => {
                /*  Loads a byte of memory into the Y register setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDY
                    Address mode: Absolute, X
                    Alias: LDY_ABX
                    Bytes: 3
                    Cycles: 4 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.x as u16);
                self.cpu.y = self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.y == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.y & 0b1000_0000 == 0b1000_0000 {    // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },

// << LDY ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> LSR starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x4A => {
                /*  Each of the bits in A or M is shift one place to the right. The bit that was 
                    in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
                    Opcode: LSR
                    Address mode: Accumulator
                    Alias: LSR_ACC
                    Bytes: 1
                    Cycles: 2
                */
                if self.cpu.accumulator & 0x01 == 0x01 {        // Setting the carry flag
                    self.cpu.status |= 0b0000_0001;
                }

                self.cpu.accumulator = self.cpu.accumulator >> 1;

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x46 => {
                /*  Each of the bits in A or M is shift one place to the right. The bit that was 
                    in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
                    Opcode: LSR
                    Address mode: Zero page
                    Alias: LSR_ZP
                    Bytes: 2
                    Cycles: 5
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];

                if self.memory.primary_memory[zero_page_address as usize] & 0x01 == 0x01 {        // Setting the carry flag
                    self.cpu.status |= 0b0000_0001;
                }

                self.memory.primary_memory[zero_page_address as usize] = self.memory.primary_memory[zero_page_address as usize] >> 1;

                if self.memory.primary_memory[zero_page_address as usize] == 0 {                                               // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.memory.primary_memory[zero_page_address as usize] & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x56 => {
                /*  Each of the bits in A or M is shift one place to the right. The bit that was 
                    in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
                    Opcode: LSR
                    Address mode: Zero page, X
                    Alias: LSR_ZPX
                    Bytes: 2
                    Cycles: 6
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);

                if self.memory.primary_memory[zero_page_address as usize] & 0x01 == 0x01 {        // Setting the carry flag
                    self.cpu.status |= 0b0000_0001;
                }

                self.memory.primary_memory[zero_page_address as usize] = self.memory.primary_memory[zero_page_address as usize] >> 1;

                if self.memory.primary_memory[zero_page_address as usize] == 0 {                                               // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.memory.primary_memory[zero_page_address as usize] & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x4E => {
                /*  Each of the bits in A or M is shift one place to the right. The bit that was 
                    in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
                    Opcode: LSR
                    Address mode: Absolute
                    Alias: LSR_ABS
                    Bytes: 3
                    Cycles: 6
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.x as u16);

                if self.memory.primary_memory[absolute_address as usize] & 0x01 == 0x01 {                                              // Setting the carry flag
                    self.cpu.status |= 0b0000_0001;
                }

                self.memory.primary_memory[absolute_address as usize] = self.memory.primary_memory[absolute_address as usize] >> 1;

                if self.memory.primary_memory[absolute_address as usize] == 0 {                                               // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.memory.primary_memory[absolute_address as usize] & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x5E => {
                /*  Each of the bits in A or M is shift one place to the right. The bit that was 
                    in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
                    Opcode: LSR
                    Address mode: Absolute, X
                    Alias: LSR_ABX
                    Bytes: 3
                    Cycles: 7
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.x as u16).wrapping_add(self.cpu.x as u16);

                if self.memory.primary_memory[absolute_address as usize] & 0x01 == 0x01 {                                              // Setting the carry flag
                    self.cpu.status |= 0b0000_0001;
                }

                self.memory.primary_memory[absolute_address as usize] = self.memory.primary_memory[absolute_address as usize] >> 1;

                if self.memory.primary_memory[absolute_address as usize] == 0 {                                               // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.memory.primary_memory[absolute_address as usize] & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },

// << LSR ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> NOP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0xEA => {
                /*  The NOP instruction causes no changes to the processor other than the normal 
                    incrementing of the program counter to the next instruction.
                    Opcode: NOP
                    Address mode: Implied
                    Alias: NOP_IMP
                    Bytes: 1
                    Cycles: 2
                */
            },

// << NOP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> ORA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x09 => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: Immediate
                    Alias: ORA_IMM
                    Bytes: 2
                    Cycles: 2
                */
                self.cpu.program_counter += 1;
                self.cpu.accumulator |= self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x05 => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: Zero page
                    Alias: ORA_ZP
                    Bytes: 2
                    Cycles: 3
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];                                         // one cpu cycle
                self.cpu.accumulator |= self.memory.primary_memory[zero_page_address as usize];                                         // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x15 => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: Zero page, X
                    Alias: ORA_ZPX
                    Bytes: 2
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);                // two cpu cycle
                self.cpu.accumulator |= self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x0D => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: Absolute
                    Alias: ORA_ABS
                    Bytes: 3
                    Cycles: 4
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
                self.cpu.accumulator |= self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x1D => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: Absolute, X
                    Alias: ORA_ABX
                    Bytes: 3
                    Cycles: 4 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.x as u16);
                self.cpu.accumulator |= self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x19 => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: Absolute, Y
                    Alias: ORA_ABY
                    Bytes: 3
                    Cycles: 4 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let lo: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                self.cpu.program_counter += 1;
                let hi: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                                                                // one cpu cycle
                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16);
                self.cpu.accumulator |= self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x01 => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: (Indirect, X)
                    Alias: ORA_INX
                    Bytes: 2
                    Cycles: 6
                */
                self.cpu.program_counter += 1;
                let indirect_address = self.memory.primary_memory[self.cpu.program_counter as usize].wrapping_add(self.cpu.x);             // one cpu cycle

                let lo: u8 = self.memory.primary_memory[indirect_address as usize];
                let hi: u8 = self.memory.primary_memory[indirect_address as usize + 1];
                                                                // two cpu cycle

                let absolute_address: u16 = (hi as u16) << 8 | lo as u16;
                self.cpu.accumulator |= self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },
            0x11 => {
                /*  An inclusive OR is performed, bit by bit, on the accumulator contents using 
                    the contents of a byte of memory.
                    Opcode: ORA
                    Address mode: (Indirect), Y
                    Alias: ORA_INY
                    Bytes: 2
                    Cycles: 5 (+1 if page crossed)
                */
                self.cpu.program_counter += 1;
                let indirect_address = self.memory.primary_memory[self.cpu.program_counter as usize];                                      // one cpu cycle

                let lo: u8 = self.memory.primary_memory[indirect_address as usize];
                let hi: u8 = self.memory.primary_memory[indirect_address as usize + 1];
                                                                // two cpu cycle

                let absolute_address: u16 = ((hi as u16) << 8 | lo as u16).wrapping_add(self.cpu.y as u16) ;
                self.cpu.accumulator |= self.memory.primary_memory[absolute_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },

// << ORA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> PHA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x48 => {
                /*  Pushes a copy of the accumulator on to the stack.
                    Opcode: PHA
                    Address mode: Implied
                    Alias: PHA_IMP
                    Bytes: 1
                    Cycles: 3
                */
                self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize] = self.cpu.accumulator;                                   // two cpu cycle
                self.cpu.stack_pointer -= 1;
            },

// << PHA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> PHP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x08 => {
                /*  Pushes a copy of the status flags on to the stack.
                    Opcode: PHP
                    Address mode: Implied
                    Alias: PHP_IMP
                    Bytes: 1
                    Cycles: 3
                */
                self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize] = self.cpu.status;                                        // two cpu cycle
                self.cpu.stack_pointer -= 1;
            },

// << PHP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> PLA starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x68 => {
                /*  Pulls an 8 bit value from the stack and into the accumulator. The zero and 
                    negative flags are set as appropriate.
                    Opcode: PLA
                    Address mode: Implied
                    Alias: PLA_IMP
                    Bytes: 1
                    Cycles: 4
                */
                self.cpu.stack_pointer += 1;
                self.cpu.accumulator = self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize];                                      // two cpu cycle

                if self.cpu.accumulator == 0 {                  // Setting the zero flag
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 0b1000_0000 {
                                                                // Setting the negative flag
                    self.cpu.status |= 0b1000_0000;
                }
            },

// << PLA ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> PLP starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

            0x28 => {
                /*  Pulls an 8 bit value from the stack and into the processor flags. The flags
                    will take on new states as determined by the value pulled.
                    Opcode: PLP
                    Address mode: Implied
                    Alias: PLP_IMP
                    Bytes: 1
                    Cycles: 4
                */
                self.cpu.stack_pointer += 1;
                self.cpu.status = self.memory.primary_memory[0x100 + self.cpu.stack_pointer as usize];                                         // two cpu cycle
            },

// << PLP ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// >> INS starts >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// << INS ends <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

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