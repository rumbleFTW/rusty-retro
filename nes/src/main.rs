mod nes;


fn main() {
    let mut ns = nes::Nes::new();
    ns.cpu.y = 0x04;
    ns.cpu.x = 0x02;
    ns.memory.primary_memory[0xFFFC] = 0x6C;
    ns.memory.primary_memory[0xFFFD] = 0x20;
    ns.memory.primary_memory[0xFFFE] = 0x10;
    ns.memory.primary_memory[0x1020] = 0xFC;
    ns.memory.primary_memory[0x1021] = 0xBA;
    ns.memory.primary_memory[0xBAFC] = 0x69;
    ns.emulate_cycle();
    ns.debug();
}
