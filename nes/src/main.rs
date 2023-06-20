mod nes;
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rom_path = "default";
    let mut debug = false;

    for arg in args.iter() {
        if arg.starts_with("--rom=") {
            rom_path = &arg[6..]
        }
        if arg == "-d" {
            debug = true;
        }
    }

    let mut ns = nes::Nes::new();
    if debug == true {
        ns.debug();
    }

    if rom_path == "default" {
        // c8.memory.load_program(&C8_ROM);
    } else {
        let mut f = fs::File::open(&rom_path).expect("no file found");
        let metadata = fs::metadata(&rom_path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        ns.cpu.program_counter = 0xC000;
        ns.memory.load_program(&buffer);
    }
}
