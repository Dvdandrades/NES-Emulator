# NES Emulator

A Rust-based emulator project focused on implementing a MOS 6502-style CPU core and using it to run a small memory-mapped Snake game through SDL2.

This repository is an educational emulator in progress. It currently executes a built-in 6502 program and renders a 32x32 pixel display, but it is not yet a complete NES console emulator with cartridge loading, PPU/APU emulation, mapper support, or `.nes` ROM execution.

## Features

- 6502-style CPU registers, status flags, stack, memory access, and reset vector handling.
- Opcode metadata table with instruction length, cycle count, mnemonic, and addressing mode.
- Implemented instruction groups including:
  - Loads and stores: `LDA`, `LDX`, `LDY`, `STA`, `STX`, `STY`
  - Arithmetic and logic: `ADC`, `SBC`, `AND`, `EOR`, `ORA`, `BIT`
  - Shifts and rotates: `ASL`, `LSR`, `ROL`, `ROR`
  - Increments, decrements, comparisons, branches, jumps, subroutines, and stack operations
  - Processor status flag updates for zero, negative, carry, overflow, interrupt, decimal, and break behavior
- Built-in Snake demo program loaded at `0x0600`.
- SDL2 window output using a 32x32 memory-mapped screen region.
- Keyboard input mapped into emulated memory for game control.
- Unit tests for core CPU behavior.

## Requirements

- Rust toolchain, preferably installed with [rustup](https://rustup.rs/)
- SDL2 development libraries

## Getting Started

Clone the repository and run the project:

```bash
git clone https://github.com/Dvdandrades/NES-Emulator
cd nes-emulator
cargo run
```

The application opens an SDL2 window titled `Snake game`.

## Controls

| Key | Action |
| --- | --- |
| `W` | Move up |
| `A` | Move left |
| `S` | Move down |
| `D` | Move right |
| `Esc` | Quit |

Closing the window also exits the emulator.

## Running Tests

```bash
cargo test
```

Current tests cover basic CPU execution behavior, including immediate loads, transfers, increment wrapping, multi-instruction execution, and zero-page memory reads.

## Project Structure

```text
.
├── Cargo.toml
├── README.md
└── src
    ├── cpu.rs       # CPU state, memory trait, instruction execution, and tests
    ├── main.rs      # SDL2 setup, Snake demo program, input, and rendering loop
    └── opcodes.rs   # Opcode metadata and addressing mode lookup table
```

## Architecture

### CPU Core

The CPU is represented by the `CPU` struct in `src/cpu.rs`. It contains the accumulator, index registers, status flags, program counter, stack pointer, and a flat 64 KB memory array.

Programs are loaded at `0x0600`, and the reset vector at `0xFFFC` is set to that address. This matches the needs of the bundled Snake demo, whose screen memory occupies `0x0200..0x0600`.

### Memory

Memory access is abstracted through the `Mem` trait:

- `mem_read`
- `mem_write`
- `mem_read_u16`
- `mem_write_u16`

The emulator uses little-endian reads and writes for 16-bit values, matching 6502 behavior.

### Opcode Dispatch

Opcode metadata lives in `src/opcodes.rs`. At runtime, the CPU fetches the next opcode, looks up its metadata in `OPCODES_MAP`, executes the matching instruction implementation, and advances the program counter unless the instruction changed control flow.

### Rendering

The demo treats memory from `0x0200` through `0x05FF` as a 32x32 one-byte-per-pixel framebuffer. `src/main.rs` converts these color indexes into RGB pixels and uploads them to an SDL2 texture.

### Input

Keyboard events write ASCII-like direction values into memory address `0x00FF`. The Snake program reads that address to determine movement direction.

## Current Limitations

- No `.nes` ROM loader.
- No iNES header parsing.
- No PPU, APU, controller, cartridge, or mapper emulation.
- No cycle-accurate timing.
- Opcode cycle counts are recorded but not used for hardware-accurate scheduling.
- The included SDL2 demo is tied to the built-in Snake program.

## Development

Useful commands:

```bash
cargo fmt
cargo clippy
cargo test
cargo run
```

When adding CPU instructions, update both the opcode table in `src/opcodes.rs` and the dispatch logic in `src/cpu.rs`. Add focused tests for the instruction, addressing mode, flag behavior, and wrapping behavior where relevant.
