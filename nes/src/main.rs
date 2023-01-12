mod nes;


fn main() {
    let mut ns = nes::Nes::new();
    ns.memory.primary_memory[0xFFFC] = 0xA9;
    ns.memory.primary_memory[0xFFFD] = 0x42;
    ns.emulate_cycle();
    ns.debug();
}
