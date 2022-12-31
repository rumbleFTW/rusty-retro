use rand::Rng;

mod cpu;
mod memory;
mod display;
mod keyboard;

pub struct Chip8 {
    pub cpu: cpu::Cpu,
    pub memory: memory::Memory,
    pub display: display::Display,
    pub keyboard: keyboard::Keyboard
}

impl Chip8 {

    pub fn new()-> Chip8 {
        return Chip8 {
            cpu: cpu::Cpu::new(),
            memory: memory::Memory::new(),
            display: display::Display::new(),
            keyboard: keyboard::Keyboard::new()
        }
    }

    pub fn execute (&mut self, instruction:u16) {
        /*  executes an instruction on the hardware
            arg: the hex instruction
        */ 
        self.cpu.program_counter += 2;
        let x: usize = ((instruction & 0x0F00) >> 4*2) as usize;
        let  y: usize = ((instruction & 0x00F0) >> 4) as usize;
        match instruction & 0xF000 {
            0x0000 => {
                /*  Case 0NNN
                */
                match instruction {
                    0x00E0 => {
                        /*  Clear the display.
                        */
                        self.display.clear();
                    },
                    0x00EE => {
                        /*  Return from a subroutine.
                            The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                        */
                        self.cpu.program_counter = self.memory.stack[self.cpu.stack_pointer as usize];
                        self.cpu.stack_pointer = self.cpu.stack_pointer.wrapping_sub(1);
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
                self.cpu.program_counter = instruction & 0x0FFF;
            },
            0x2000 => {
                /*  Call subroutine at nnn.
                    The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                */
                self.cpu.stack_pointer = self.cpu.stack_pointer.wrapping_add(1);
                self.memory.stack[self.cpu.stack_pointer as usize] = self.cpu.program_counter;
                self.cpu.program_counter = instruction & 0x0FFF;
            },
            0x3000 => {
                /*  Skip next instruction if Vx = kk.
                    The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                */
                if self.cpu.registers[x] == (instruction & 0x00FF) as u8 {
                    if self.cpu.registers[x] == (instruction & 0x00FF) as u8 {
                        self.cpu.program_counter += 2;
                    }
                }
            },
            0x4000 => {
                /*  Skip next instruction if Vx != kk.
                    The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                */
                if self.cpu.registers[x] != (instruction & 0x00FF) as u8 {
                    self.cpu.program_counter += 2;
                }
            },
            0x5000 => {
                /*  Skip next instruction if Vx = Vy.
                    The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                */
                if self.cpu.registers[x] == self.cpu.registers[y] {
                    self.cpu.program_counter += 2;
                }
            },
            0x6000 => {
                /*  Set Vx = kk.
                    The interpreter puts the value kk into register Vx.
                */
                self.cpu.registers[x] = (instruction & 0x00FF) as u8;
            },
            0x7000 => {
                /*  Set Vx = Vx + kk.
                    Adds the value kk to the value of register Vx, then stores the result in Vx.
                */
                self.cpu.registers[x] = self.cpu.registers[x].wrapping_add((instruction & 0x00FF) as u8);
            },
            0x8000 => {
                /*  Case 8xyN
                */
                match instruction & 0x000F {
                    0x0000 => {
                        /*  Set Vx = Vy.
                            Stores the value of register Vy in register Vx.
                        */
                        self.cpu.registers[x] = self.cpu.registers[y];
                    },
                    0x0001 => {
                        /*  Set Vx = Vx OR Vy.
                            Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        */
                        self.cpu.registers[x] |= self.cpu.registers[y];
                    },
                    0x0002 => {
                        /*  Set Vx = Vx AND Vy.
                            Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        */
                        self.cpu.registers[x] &= self.cpu.registers[y];
                    },
                    0x0003 => {
                        /*  Set Vx = Vx XOR Vy.
                            Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0. 
                        */
                        self.cpu.registers[x] ^= self.cpu.registers[y];
                    },
                    0x0004 => {
                        /*  Set Vx = Vx + Vy, set VF = carry.
                            The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                        */
                        let sum = self.cpu.registers[x] as u16 + self.cpu.registers[y] as u16;
                        if sum > 0xFF { 
                            self.cpu.registers[0xF] =  1;
                        } else {
                            self.cpu.registers[0xF] =  0;
                        }
                        self.cpu.registers[x] = sum as u8;
                    },
                    0x0005 => {
                        /*  Set Vx = Vx - Vy, set VF = NOT borrow.
                            If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                        */
                        let diff = self.cpu.registers[x] as i16 - self.cpu.registers[y] as i16;
                        if diff > 0x0 { 
                            self.cpu.registers[0xF] =  1;
                        } else {
                            self.cpu.registers[0xF] =  0;
                        }
                        self.cpu.registers[x] = diff as u8;
                    },
                    0x0006 => {
                        /*  Set Vx = Vx SHR 1.
                            If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        */
                        self.cpu.registers[0xF] = self.cpu.registers[x] & 0x1;
                        self.cpu.registers[x] >>=  1;
                    },
                    0x0007 => {
                        /*  Set Vx = Vy - Vx, set VF = NOT borrow.
                            If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx..
                        */

                        let diff = self.cpu.registers[y] as i16 - self.cpu.registers[x] as i16;
                        if diff > 0x0 { 
                            self.cpu.registers[0xF] =  1;
                        } else {
                            self.cpu.registers[0xF] =  0;
                        }
                        self.cpu.registers[x] = diff as u8;
                    },
                    0x000E => {
                        /*  Set Vx = Vx SHL 1.
                            If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                        */
                        self.cpu.registers[0xF] = self.cpu.registers[x] & 0x80;
                        self.cpu.registers[x] <<= 1;
                    },
                    _ => panic!("Unrecognized instruction {:#X}", instruction),
                }
            },
            0x9000 => {
                /*  Skip next instruction if Vx != Vy.
                    The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                */
                if self.cpu.registers[x] != self.cpu.registers[y] {
                    self.cpu.program_counter += 2;
                }
            },
            0xA000 => {
                /*  Set i = nnn.
                    The value of register i is set to nnn.
                */
                self.cpu.i = instruction & 0x0FFF;
            },
            0xB000 => {
                /*  Jump to location nnn + V0.
                    The program counter is set to nnn plus the value of V0.
                */
                self.cpu.program_counter = self.cpu.registers[0] as u16 + (instruction & 0x0FFF);
            },
            0xC000 => {
                /*  Set Vx = random byte AND kk.
                    The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
                */
                let mut rng = rand::thread_rng();
                self.cpu.registers[x] = (rng.gen_range(0..256) as u16 & (instruction & 0x00FF)) as u8;
            },
            0xD000 => {
                /*  Display n-byte sprite starting at memory location i at  
                    (Vx, Vy), set VF = collision.
                    The interpreter reads n bytes from memory, starting at the address stored in i. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If self.causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                    */
                    self.display.render(&mut self.cpu.registers, x, y, (instruction & 0x000F) as u8, &self.memory.primary_memory, self.cpu.i);
            },
            0xE000 => {
                /*  Case ExNN
                */
                match instruction & 0x00FF {
                    0x009E => {
                        /*  Skip next instruction if key with the value of Vx 
                            is pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                        */
                        if self.keyboard.is_key_pressed(self.cpu.registers[x]) {
                            self.cpu.program_counter += 2;
                        }
                    },
                    0x00A1 => {
                        /*  Skip next instruction if key with the value of Vx 
                            is not pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                        */
                        if !self.keyboard.is_key_pressed(self.cpu.registers[x]) {
                            self.cpu.program_counter += 2;
                        }
                    },
                    _ => panic!("Unrecognized instruction {:#X}", instruction),
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
                        self.cpu.registers[x] = self.cpu.delay_timer;
                    },
                    0x000A => {
                        /*  Wait for a key press, store the value of the key 
                            in Vx. All execution stops until a key is pressed, then the value of that key is stored in Vx.
                        */
                        if let Some(val) = self.keyboard.pressed_key {
                            self.cpu.registers[x] = val;
                        }
                    },
                    0x0015 => {
                        /*  Set delay timer = Vx.
                            DT is set equal to the value of Vx.
                        */
                        self.cpu.delay_timer = self.cpu.registers[x] as u8;
                    },
                    0x0018 => {
                        /*  Set sound timer = Vx.
                            ST is set equal to the value of Vx.
                        */
                        self.cpu.sound_timer = self.cpu.registers[x] as u8;
                    },
                    0x001E => {
                        /*  Set i = i + Vx.
                            The values of i and Vx are added, and the results are stored in i.
                        */
                        self.cpu.i += self.cpu.registers[x] as u16;
                    },
                    0x0029 => {
                        /*  Set i = location of sprite for digit Vx.
                            The value of i is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                        */
                        self.cpu.i = self.cpu.registers[x] as u16 * 5;
                    },
                    0x0033 => {
                        /*  Store BCD representation of Vx in memory
                            locations i, i+1, and i+2. The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in i, the tens digit at location i+1, and the ones digit at location i+2.
                        */
                        self.memory.primary_memory[(self.cpu.i+2) as usize] = self.cpu.registers[x] % 10;
                        self.memory.primary_memory[(self.cpu.i+1) as usize] = (self.cpu.registers[x] % 100) / 10;
                        self.memory.primary_memory[(self.cpu.i) as usize] = self.cpu.registers[x] / 100;
                    },
                    0x0055 => {
                        /*  Store registers V0 through Vx in memory starting 
                            at location i. The interpreter copies the values of registers V0 through Vx into memory, starting at the address in i.
                        */
                        for i in 0..x+1 {
                            self.memory.primary_memory[self.cpu.i as usize + i] = self.cpu.registers[i];
                        }
                        self.cpu.i += (x + 1) as u16;
                    },
                    0x0065 => {
                        /*  Read registers V0 through Vx from memory starting 
                            at location i. The interpreter reads values from memory starting at location i into registers V0 through Vx.
                        */
                        for i in 0..x+1 {
                            self.cpu.registers[i] = self.memory.primary_memory[self.cpu.i as usize + i];
                        }
                        self.cpu.i += x as u16 + 1;
                    },
                    _ => panic!("Unrecognized instruction {:#X}", instruction),
                }
            },
            _ => panic!("Unrecognized instruction {:#X}", instruction),
        }

    }

    pub fn emulate_cycle(&mut self) {
        let opcode = (self.memory.primary_memory[self.cpu.program_counter as usize] as u16) << 8 | self.memory.primary_memory[self.cpu.program_counter as usize + 1] as u16;
        self.execute(opcode);
        self.cpu.update_timers();
        // std::thread::sleep(time::Duration::from_millis(1));
    }

    pub fn debug(&self) {
        println!("Program counter: {}, I: {:#x}", self.cpu.program_counter, self.cpu.i);
        println!("Registers:");
        for i in 0..16 {
            print!("{:#x} ", self.cpu.registers[i]);
        }
        print!("\n");
        println!("\nMemory:");
        for i in 0..4096 {
            print!("{:#x} ", self.memory.primary_memory[i]);
        }
        print!("\n");
    } 
}