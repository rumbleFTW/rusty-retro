pub struct Cpu {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub x: u8,
    pub y: u8,
    pub accumulator:u8,
    
    /*  Status flag register
        7  bit  0
        ---- ----
        NVss DIZC
        |||| ||||
        |||| |||+- Carry
        |||| ||+-- Zero
        |||| |+--- Interrupt Disable
        |||| +---- Decimal
        ||++------ No CPU effect, see: the B flag
        |+-------- Overflow
        +--------- Negative
    */
    pub status: u8
}

impl Cpu {

    pub fn new()-> Cpu {
        return Cpu  {
                        program_counter: 0xFFFC,
                        stack_pointer: 0xFF,
                        x: 0x00,
                        y: 0x00,
                        accumulator: 0x00,
                        status: 0x00
                    }
    }

}