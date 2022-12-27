pub struct Emulator {
    /* The virtual hardware required for the chip8 system 
    */
    
    // The 64x32 Display
    display: [[u8;32];64],

    // Max speed
    speed: u16,

    // Timers
    sound_timer: u8,
    delay_timer:u8,
    
    // Main memory of size 4K
    memory: [u8;4096],
    
    // Stack and stack pointer. Can take max 16 depths
    stack_pointer: u8,
    stack: [u16;64],
    
    // Necessary registers for the operations
    registers: [u8;16],
    
    // Misc vars
    i: u16,
    program_counter: u16,

    // A byte array of 0s, 1 indicates the button is pressed
    keyboard: [u8;16],
}

impl Emulator {
    pub fn new() -> Emulator {
        return Emulator {   i: 0x0,
                            program_counter: 0,
                            stack_pointer : 0,
                            sound_timer : 0,
                            delay_timer : 0,
                            memory: [0x000; 4096],
                            stack: [0x00; 64],
                            registers: [0x0000; 16],
                            keyboard: [0x0000; 16],
                            speed: 60,
                            display: [[0;32];64],
                        }
    }
    pub fn init(&mut self) {
        self.i = 0x0;
        self.program_counter = 0x200;
        self.stack_pointer = 0;
        self.sound_timer = 0;
        self.delay_timer = 0;
        self.speed = 60;
        self.display = [[0;32];64];
    }
    pub fn load_sprites(&mut self) {
        /*  Load default sprites from 0-F into the main memory  
        */
        let default_sprites = [
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
        for i in 0..80 {
            self.memory[i] = default_sprites[i];
        }
    }

    pub fn load_program (&mut self, program: &[u8]) {
        /*  Load the program as u8 array into the main memory
        */
        for i in 0..program.len() {
            self.memory[0x200+i] = program[i];
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

    pub fn execute (&mut self, instruction:u16) {
        /*  executes an instruction on the hardware
            arg: the hex instruction
        */ 
        eprintln!("Executing instruction: {:#x}", instruction);
        self.program_counter += 2;
        let x: usize = ((instruction & 0x0F00) >> 4*2) as usize;
        let  y: usize = ((instruction & 0x00F0) >> 4) as usize;
        match instruction & 0xF000 {
            0x0000 => {
                /*  Case 0NNN
                */
                match instruction {
                    0x00E0 => {
                        // print!("{esc}c", esc = 27 as char);
                        println!("Clear...");
                        // // // // //
                        //   clear  // 
                        // // // // //
                    },
                    0x00EE => {
                        /*  Return from a subroutine.
                            The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                        */
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.stack_pointer -= 1;
                    },
                    _ => {
                        /*  Jump to a machine code routine at nnn.
                            This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
                        */
                    },
                }
            },
            0x1000 => {
                /*  Jump to location nnn.
                    The interpreter sets the program counter to nnn.
                */
                self.program_counter = instruction & 0x0FFF;
            },
            0x2000 => {
                /*  Call subroutine at nnn.
                    The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                */
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = instruction & 0x0FFF;
            },
            0x3000 => {
                /*  Skip next instruction if Vx = kk.
                    The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                */
                if self.registers[x] == (instruction & 0x00FF) as u8 {
                    self.program_counter += 2;
                }
            },
            0x4000 => {
                /*  Skip next instruction if Vx != kk.
                    The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                */
                if self.registers[x] != (instruction & 0x00FF) as u8 {
                    self.program_counter += 2;
                }
            },
            0x5000 => {
                /*  Skip next instruction if Vx = Vy.
                    The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                */
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            },
            0x6000 => {
                /*  Set Vx = kk.
                    The interpreter puts the value kk into register Vx.
                */
                self.registers[x] = (instruction & 0x00FF) as u8;
            },
            0x7000 => {
                /*  Set Vx = Vx + kk.
                    Adds the value kk to the value of register Vx, then stores the result in Vx.
                */
                self.registers[x] += (instruction & 0x00FF) as u8;
            },
            0x8000 => {
                /*  Case 8xyN
                */
                match instruction & 0x000F {
                    0x0000 => {
                        /*  Set Vx = Vy.
                            Stores the value of register Vy in register Vx.
                        */
                        self.registers[x] = self.registers[y];
                    },
                    0x0001 => {
                        /*  Set Vx = Vx OR Vy.
                            Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        */
                        self.registers[x] |= self.registers[y];
                    },
                    0x0002 => {
                        /*  Set Vx = Vx AND Vy.
                            Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        */
                        self.registers[x] &= self.registers[y];
                    },
                    0x0003 => {
                        /*  Set Vx = Vx XOR Vy.
                            Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0. 
                        */
                        self.registers[x] ^= self.registers[y];
                    },
                    0x0004 => {
                        /*  Set Vx = Vx + Vy, set VF = carry.
                            The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                        */
                        self.registers[0xF] =  if (self.registers[x]+self.registers[y]) as u16 > 0xFF { 1 } else { 0 };
                        self.registers[x] = self.registers[x] + self.registers[y];
                    },
                    0x0005 => {
                        /*  Set Vx = Vx - Vy, set VF = NOT borrow.
                            If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                        */
                        self.registers[0xF] =  if self.registers[x] > self.registers[y] { 1 } else { 0 };
                        self.registers[x] = self.registers[x] - self.registers[y];
                    },
                    0x0006 => {
                        /*  Set Vx = Vx SHR 1.
                            If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        */
                        self.registers[0xF] = self.registers[x] & 0x1;
                        self.registers[x] >>=  1;
                    },
                    0x0007 => {
                        /*  Set Vx = Vy - Vx, set VF = NOT borrow.
                            If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx..
                        */
                        self.registers[0xF] =  if self.registers[y] > self.registers[x] { 1 } else { 0 };
                        self.registers[x] = self.registers[y] - self.registers[x];
                    },
                    0x000E => {
                        /*  Set Vx = Vx SHL 1.
                            If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                        */
                        self.registers[0xF] = self.registers[x] & 0x80;
                        self.registers[x] <<= 1;
                    },
                    _ => unreachable!()
                }
            },
            0x9000 => {
                /*  Skip next instruction if Vx != Vy.
                    The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                */
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            },
            0xA000 => {
                /*  Set i = nnn.
                    The value of register i is set to nnn.
                */
                self.i = instruction & 0x0FFF;
            },
            0xB000 => {
                /*  Jump to location nnn + V0.
                    The program counter is set to nnn plus the value of V0.
                */
                self.program_counter = self.registers[0] as u16 + (instruction & 0x0FFF);
            },
            0xC000 => {
                /*  Set Vx = random byte AND kk.
                    The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
                */
                // self.registers[x] = (rand::thread_rng().gen_range(0..256)) & (instruction & 0x00FF);
                // // // // //
                //  Random //
                // // //  //
            },
            0xD000 => {
                /*  Display n-byte sprite starting at memory location i at  
                    (Vx, Vy), set VF = collision.
                    The interpreter reads n bytes from memory, starting at the address stored in i. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If self.causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                    */
                    // // // // /
                    //   GUI  // 
                    // // // // /
            },
            0xE000 => {
                /*  Case ExNN
                */
                match instruction & 0x00FF {
                    0x009E => {
                        /*  Skip next instruction if key with the value of Vx 
                            is pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                        */
                        // // // // // // // // // // // 
                        // Key implementation          //
                        // // // // // // // // // // // 
                    },
                    0x00A1 => {
                        /*  Skip next instruction if key with the value of Vx 
                            is not pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                        */
                        // // // // // // // // // // // 
                        // Key implementation          //
                        // // // // // // // // // // // 
                    },
                    _ => unreachable!()
                }
            },
            0xF000 => {
                /*  Case FxNN
                */
                match instruction & 0x00FF {
                    0x0007 => {
                        /*  Set Vx = delay timer value.
                            The value of DT is placed into Vx.
                        */
                        self.registers[x] = self.delay_timer;
                    },
                    0x000A => {
                        /*  Wait for a key press, store the value of the key 
                            in Vx. All execution stops until a key is pressed, then the value of that key is stored in Vx.
                        */
                        // // // // // // // // // // // 
                        //     Key implementation      //
                        // // // // // // // // // // // 
                    },
                    0x0015 => {
                        /*  Set delay timer = Vx.
                            DT is set equal to the value of Vx.
                        */
                        self.delay_timer = self.registers[x] as u8;
                    },
                    0x0018 => {
                        /*  Set sound timer = Vx.
                            ST is set equal to the value of Vx.
                        */
                        self.sound_timer = self.registers[x] as u8;
                    },
                    0x001E => {
                        /*  Set i = i + Vx.
                            The values of i and Vx are added, and the results are stored in i.
                        */
                        self.i += self.registers[x] as u16;
                    },
                    0x0029 => {
                        /*  Set i = location of sprite for digit Vx.
                            The value of i is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                        */
                        // // // // // // // // // // // // 
                        //      Font implementation       //
                        // // // // // // // // // // // // 
                    },
                    0x0033 => {
                        /*  Store BCD representation of Vx in memory
                            locations i, i+1, and i+2. The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in i, the tens digit at location i+1, and the ones digit at location i+2.
                        */
                        // // // // // // // // // // // // 
                        //       BCD implementation       //
                        // // // // // // // // // // // // 
                    },
                    0x0055 => {
                        /*  Store registers V0 through Vx in memory starting 
                            at location i. The interpreter copies the values of registers V0 through Vx into memory, starting at the address in i.
                        */
                        for i in 0..x+1 {
                            self.memory[i] = self.registers[self.i as usize + i];
                        }
                    },
                    0x0065 => {
                        /*  Read registers V0 through Vx from memory starting 
                            at location i. The interpreter reads values from memory starting at location i into registers V0 through Vx.
                        */
                        for i in 0..x+1 {
                            self.registers[i] = self.memory[self.i as usize + i];
                        }
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    
    }    

    pub fn emulate_cycle (&mut self) {
        for _i in 0..self.speed {
            let opcode = (self.memory[self.program_counter as usize] as u16) << 8 | self.memory[self.program_counter as usize + 1] as u16;
            self.execute(opcode);
            self.update_timers();
        }
    }

    pub fn debug (&self) {
        println!("Memory:");
        for i in 1..self.memory.len()+1 {
            print!("{:#2x} ", self.memory[i-1]);
            if i % 16 == 0 {
                print!("\n");
            }
        }
    }
}