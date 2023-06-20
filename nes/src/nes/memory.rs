pub struct Memory {
    // Primary memory of size 64K
    pub primary_memory: [u8; 64 * 1024],
}

impl Memory {
    pub fn load_program(&mut self, program: &[u8]) {
        for i in 16..program.len() {
            println!("{:#X}", 0x8000 + i);
            self.primary_memory[0x8000 + i] = program[i];
        }
    }
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            primary_memory: [0x0; 64 * 1024],
        };
    }
}
