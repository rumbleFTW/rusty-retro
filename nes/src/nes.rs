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
            0xA9 => {
                /*  Loads a byte of memory into the accumulator setting the zero and negative   
                    flags as appropriate.
                    Opcode: LDA
                    Address mode: Immediate
                    Alias: LDA_IM
                    Bytes: 2
                    Cycles: 2
                */
                self.cpu.program_counter += 1;                  // one cpu cycle
                self.cpu.accumulator = self.memory.primary_memory[self.cpu.program_counter as usize];

                if self.cpu.accumulator == 0 {
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 1 {
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
                self.cpu.program_counter += 1;                  // one cpu cycle
                let zero_page_address: u8 = self.memory.primary_memory[self.cpu.program_counter as usize];
                self.cpu.accumulator = self.memory.primary_memory[zero_page_address as usize];
                                                                // one cpu cycle

                if self.cpu.accumulator == 0 {
                    self.cpu.status |= 0b0000_0010;
                }

                if self.cpu.accumulator & 0b1000_0000 == 1 {
                    self.cpu.status |= 0b1000_0000;
                }
            }
            _ => panic!("Unrecognized instruction {:#X}", instruction) 
        }
        self.cpu.program_counter += 1;                        // one cpu cycle
    }

    pub fn emulate_cycle(&mut self) {
        self.execute(self.memory.primary_memory[self.cpu.program_counter as usize]);
    }

    pub fn debug(&self) {
        println!("Accumulator: {:#X}, x: {:#X}, y: {:#X}, status: {:#b}", self.cpu.accumulator, self.cpu.x, self.cpu.y, self.cpu.status);
    }
}