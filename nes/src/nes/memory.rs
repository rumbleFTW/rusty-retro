pub struct Memory {
    // Primary memory of size 64K
    pub primary_memory: [u8; 64 * 1024],
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            primary_memory: [0x0; 64 * 1024],
        };
    }
}
