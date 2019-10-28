use byte_struct::*;
use constants::GI_MASK;
// enum Opcodes {}

pub mod constants;
pub mod instructions;

pub struct Stack {
  contents: [u8; 0xffff - 1],
  // constents: Vec<u8>,
}

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

impl Default for StatusRegister {
  fn default() -> StatusRegister {
    StatusRegister {
      c: 0,
      z: 0,
      i: 0,
      d: 0,
      x: 0,
      m: 0,
      v: 0,
      n: 0,
    }
  }
}

bitfields!(
  #[derive(PartialEq, Debug)]
  pub Accumulator: u16 {
    A: 8,
    B: 8,
  }
);

impl Default for Accumulator {
  fn default() -> Accumulator {
    Accumulator { A: 0, B: 0 }
  }
}

bitfields!(
  #[derive(PartialEq, Debug)]
  pub IndexRegister: u16 {
    register: 8,
    index: 8
  }
);

impl Default for IndexRegister {
  fn default() -> IndexRegister {
    IndexRegister {
      register: 0,
      index: 0,
    }
  }
}

// TODO: Proper inital state
#[derive(ByteStruct, PartialEq, Debug)]
#[byte_struct_le]
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

#[derive(Debug)]
pub struct CPU {
  regs: Registers,
}

impl Default for Registers {
  fn default() -> Registers {
    Registers {
      P: StatusRegister::default(),
      C: Accumulator::default(),
      X: IndexRegister::default(),
      Y: IndexRegister::default(),
      D: 0,
      S: 0,
      PBR: 0,
      DBR: 0,
      PC: 0,
    }
  }
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      regs: Registers::default(),
    }
  }
}
