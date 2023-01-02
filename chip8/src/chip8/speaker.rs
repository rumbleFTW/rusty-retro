pub struct Speaker {
    is_playing: bool
}

impl Speaker {
    pub fn new()-> Speaker {
        return Speaker {
            is_playing: false
        }
    }
    pub fn play_sound(&mut self) {
        self.is_playing = true;
        eprint!("\x07");
        self.is_playing = false;
    }
}