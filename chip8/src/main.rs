use std::fs;
use std::io::Read;
use std::env;
mod emulator;

fn run_emu (rom: Vec<u8>) {
    let mut emu = emulator::Emulator::new();
    emu.init();
    emu.load_sprites();
    emu.load_program(&rom);
    emu.emulate_cycle();
    // emu.debug();
}

fn main () {
    let args: Vec<String> = env::args().collect();
    let rom_path = "./roms/".to_owned()+&args[1];
    println!("{}", rom_path);
    let mut f = fs::File::open(&rom_path).expect("no file found");
    let metadata = fs::metadata(&rom_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    run_emu(buffer);
}