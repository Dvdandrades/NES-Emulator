use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::cpu::AddressingMode;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8, // Clock cycles consumed by the instruction
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

// The code runs the first time the variable is accessed, not when the program starts.
lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
        // Force Break
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        // No Operation
        OpCode::new(0xea, "NOP", 1, 2, AddressingMode::NoneAddressing),

        // Arithmetic
        // Add Memory to Accumulator with Carry
        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, 5/*+1 if page crossed (256 bytes)*/, AddressingMode::Indirect_Y),

        // Substract Memory from Accumulator with borrow
        OpCode::new(0xe9, "SBC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, "SBC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, "SBC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xed, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, "SBC", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_X),
        OpCode::new(0xf9, "SBC", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_Y),
        OpCode::new(0xe1, "SBC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xf1, "SBC", 2, 5/*+1 if page crossed (256 bytes)*/, AddressingMode::Indirect_Y),

        // AND Memory with Accumulator
        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, "AND", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_X),
        OpCode::new(0x39, "AND", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_Y),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x31, "AND", 2, 5/*+1 if page crossed (256 bytes)*/, AddressingMode::Indirect_Y),

        // Exclusive-or Memory with Accumulator
        OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5d, "EOR", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_X),
        OpCode::new(0x59, "EOR", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_Y),
        OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x51, "EOR", 2, 5/*+1 if page crossed (256 bytes)*/, AddressingMode::Indirect_Y),

        // OR Memory with Accumulator
        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x0d, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1d, "ORA", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_X),
        OpCode::new(0x19, "ORA", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_Y),
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x11, "ORA", 2, 5/*+1 if page crossed (256 bytes)*/, AddressingMode::Indirect_Y),

        // Shifts
        // Shift One Bit Left
        OpCode::new(0x0a, "ASL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, "ASL", 3, 7, AddressingMode::Absolute_X),

        // Shift One Bit Right
        OpCode::new(0x4a, "LSR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x4e, "LSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x5e, "LSR", 3, 7, AddressingMode::Absolute_X),

        // Rotate One Bit Left
        OpCode::new(0x2a, "ROL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x2e, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3e, "ROL", 3, 7, AddressingMode::Absolute_X),

        // Rotate One Bit Right
        OpCode::new(0x6a, "ROR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x6e, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7e, "ROR", 3, 7, AddressingMode::Absolute_X),

        // Increment by One
        OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, "INC", 3, 7, AddressingMode::Absolute_X),

        // Increment Index X by One
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),
        // Increment Index Y by One
        OpCode::new(0xc8, "INY", 1, 2, AddressingMode::NoneAddressing),

        // Decrement by One
        OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, "DEC", 3, 7, AddressingMode::Absolute_X),

        // Decrement Index X by One
        OpCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing),
        // Decrement Index Y by One
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),

        // Compare Memory and Accumulator
        OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_X),
        OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed (256 bytes)*/, AddressingMode::Absolute_Y),
        OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed (256 bytes)*/, AddressingMode::Indirect_Y),

        // Compare Memory and Index Y
        OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute),

        // Compare Memory and Index X
        OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute),

        // Branching
        // Jump to New Location
        OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::NoneAddressing), //AddressingMode that acts as Immidiate
        OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::NoneAddressing), //AddressingMode:Indirect with 6502 bug

        // Jump to New Location Saving Return Address
        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::NoneAddressing),
        // Return from Subroutine
        OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

        // Return from Interrupt
        OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing),

        // Branch on Result Not Zero
        OpCode::new(0xd0, "BNE", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Overflow Set
        OpCode::new(0x70, "BVS", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Overflow Clear
        OpCode::new(0x50, "BVC", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Result Minus
        OpCode::new(0x30, "BMI", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Result Zero
        OpCode::new(0xf0, "BEQ", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Carry Set
        OpCode::new(0xb0, "BCS", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Carry Clear
        OpCode::new(0x90, "BCC", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),
        // Branch on Result Plus
        OpCode::new(0x10, "BPL", 2, 2 /*+1 if branch succeeds +2 if to a new page (256 bytes)*/, AddressingMode::NoneAddressing),

        // Test Memory Bits with Accumulator
        OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x2c, "BIT", 3, 4, AddressingMode::Absolute),


        // Stores - Loads
        // Load Accumulator with Memory
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),

        // Load Index X with Memory
        OpCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbe, "LDX", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),

        // Load Index Y with Memory
        OpCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbc, "LDY", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),

        // Store Accumulator in Memory
        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        // Sote Index X in Memory
        OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8e, "STX", 3, 4, AddressingMode::Absolute),

        // Store Index Y in Memory
        OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8c, "STY", 3, 4, AddressingMode::Absolute),


        // Flags clear
        // Clear Decimal Mode
        OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),
        // Clear Interrupt Disable Bit
        OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing),
        // Clear Overflow Flag
        OpCode::new(0xb8, "CLV", 1, 2, AddressingMode::NoneAddressing),
        // Clear Carry Flag
        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),
        // Set Carry Flag
        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),
        // Set Interrupt Disable Bit
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing),
        // Set Decimal Mode
        OpCode::new(0xf8, "SED", 1, 2, AddressingMode::NoneAddressing),

        // Transfer Accumulator to Index X
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        // Transfer Accumulator to Index Y
        OpCode::new(0xa8, "TAY", 1, 2, AddressingMode::NoneAddressing),
        // Transfer Stack Pointer to Index X
        OpCode::new(0xba, "TSX", 1, 2, AddressingMode::NoneAddressing),
        // Transfer Index X to Accumulator
        OpCode::new(0x8a, "TXA", 1, 2, AddressingMode::NoneAddressing),
        // Transfer Index X to Stack Pointer
        OpCode::new(0x9a, "TXS", 1, 2, AddressingMode::NoneAddressing),
        // Trasnfer Index Y to Accumulator
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),

        // Stack
        // Push Accumulator on Stack
        OpCode::new(0x48, "PHA", 1, 3, AddressingMode::NoneAddressing),
        // Pull Accumulator from Stack
        OpCode::new(0x68, "PLA", 1, 4, AddressingMode::NoneAddressing),
        // Push Processor Status on Stack
        OpCode::new(0x08, "PHP", 1, 3, AddressingMode::NoneAddressing),
        // Pull Processor Status from Stack
        OpCode::new(0x28, "PLP", 1, 4, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
