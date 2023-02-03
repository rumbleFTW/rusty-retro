mod nes;


fn main() {
    let mut ns = nes::Nes::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the path to the rom to run!");
    }
    let rom_path = &args[1];
    let mut c8 = chip8::Chip8::new();
    let mut f = fs::File::open(&rom_path).expect("no file found");
    let metadata = fs::metadata(&rom_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
}
