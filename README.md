# **Retro system emulators written in Rust**


- ### *_CHIP8_*
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/54/Space_intercept.png/220px-Space_intercept.png">

>
CHIP-8 was created by RCA engineer Joe Weisbecker in 1977 for the COSMAC VIP microcomputer. It was intended as a simpler way to make small programs and games for the computer. Instead of using machine language for the VIP’s CDP1802 processor, you could type in hexadecimal instructions (with the VIP’s hex keypad) that resembled machine code, but which were more high-level, and interpreted on the fly by a small program (the CHIP-8 emulator/interpreter).

CHIP-8 soon spread to other computers, like the Finnish Telmac 1800, the Australian DREAM 6800, ETI-660 and MicroBee, and the Canadian ACE VDU.

By 1984 the interest in CHIP-8 petered out. However, in 1990 it had a renaissance on the HP48 graphing calculators with CHIP-48 and the now-famous SUPER-CHIP extension with higher resolution.
<

## *To run the emulator:*
(*_Make sure the appropriate version of cargo and rustc is installed_*)
1) Clone the repo
```
git clone https://github.com/rumbleFTW/rusty-retro.git
```
2) Navigate to the appropriate emulator folder
```
cd chip8
```
2) Run
```
cargo run
```

## **References**
1) https://youtu.be/jWpbHC6DtnU 
2) https://bugzmanov.hithub.io/nes_ebook/
3) https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/