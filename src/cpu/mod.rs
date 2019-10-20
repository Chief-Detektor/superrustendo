use constants::GI_MASK;
use byte_struct::*;
// enum Opcodes {}

pub mod constants;
pub mod instructions;

bitfields!(
  #[derive(PartialEq, Debug)]
  pub StatusRegister: u8 {
    c: 1,   // CarryBit / Emulation Mode
    z: 1,   // Result Zero
    i: 1,   // IRQ Disable
    d: 1,   // Decimal Mde
    x: 1,   // Index Register Select/Break Instruction
    m: 1,   // Memory/Accumulator Select
    v: 1,   // Overflow
    n: 1,    // Negative
  }
);

bitfields!(
  #[derive(PartialEq, Debug)]
  pub Accumulator: u16 {
    A: 8,
    B: 8,
  }
);

bitfields!(
  #[derive(PartialEq, Debug)]
  pub IndexRegister: u16 {
    register: 8,
    index: 8
  }
);

// TODO: Proper inital state
// #[derive(ByteStzruct, PartialEq, Debug)]
pub struct Registers {
  P: StatusRegister,
  C: Accumulator,
  X: IndexRegister, // X Index Register,
  Y: IndexRegister, // Y Index Register,
  D: u16,           // Direct Page Register
  S: u8,            // Stack Pointer
  PBR: u8,          // Programm Bank Register
  DBR: u8,          // Data Bank Register
  PC: u16,          // Programm Counter
}

pub struct CPU {
  regs: Registers,
  emulation_mode: bool,
}

