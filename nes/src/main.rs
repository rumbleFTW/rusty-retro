mod nes;


fn main() {
    let mut ns = nes::Nes::new();
    ns.memory.primary_memory[0xFFFC] = 0xAD;
    ns.memory.primary_memory[0xFFFD] = 0x80;
    ns.memory.primary_memory[0xFFFE] = 0x44;
    ns.memory.primary_memory[0x4480] = 0x99;
    ns.emulate_cycle();
    ns.debug();
}
