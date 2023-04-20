mod nes;
// use std::fs;
// use std::io::Read;

fn main() {
    let mut ns = nes::Nes::new();
    ns.debug();
    ns.emulate_cycle();
    println!("Hello world!");
}
