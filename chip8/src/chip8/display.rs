const WIDTH: usize = 64;
const HEIGHT: usize = 32;
pub struct Display {
    pub buffer: [u8; WIDTH * HEIGHT]
}

impl Display {
    pub fn new()-> Display {
        return Display {
            buffer: [0x0; WIDTH * HEIGHT]
        }
    }
    
    pub fn clear(&mut self) {
        /*  Clears the whole screen
        */
        for i in 0..WIDTH * HEIGHT {
            self.buffer[i] = 0x0;
        }
    }

    pub fn render(&mut self, registers: &mut [u8; 16], x: usize, y: usize, n: u8, primary_memory: &[u8; 4096], i: u16) {
        let coord_x = registers[x] as usize  % 64;
        let coord_y = registers[y] as usize % 32;
        registers[0xF] = 0x0;
        for row in 0..n as usize{
            let sprite = primary_memory[(i + row as u16) as usize];
            for col in 0..8 {
                if (coord_x + col < 64) && (coord_y + row < 32) {
                    let byte = (sprite & (0b1000_0000 >> col)) >> 7 - col;
                    let index = (coord_x + col) + ((coord_y + row) * 64);
                    let pixel = self.buffer[index];
                    if byte == 1 && pixel == 1 {
                        self.buffer[index] = 0;
                        registers[0xF] = 0x1;
                    }
                    if byte == 1 && pixel == 0 {
                        self.buffer[index] = 1;
                    }
                }
            }
        }
    }
}