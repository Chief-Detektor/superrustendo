use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};

// enum Opcodes {}

use std::fmt;

pub mod constants;
pub mod instructions;

pub struct Stack {
  contents: [u8; 0xffff - 1],
  // constents: Vec<u8>,
}

bitfields!(
  #[derive(PartialEq)]
  pub StatusRegister: u8 {
      // n: 1, // Negative
      // v: 1, // Overflow
      // m: 1, // Memory/Accumulator Select
      // x: 1, // Index Register Select/Break Instruction
      // d: 1, // Decimal Mde
      // i: 1, // IRQ Disable
      // z: 1, // Result Zero
      // c: 1, // CarryBit / Emulation Mode

      c: 1, // CarryBit / Emulation Mode
      z: 1, // Result Zero
      i: 1, // IRQ Disable
      d: 1, // Decimal Mde
      x: 1, // Index Register Select/Break Instruction
      m: 1, // Memory/Accumulator Select
      v: 1, // Overflow
      n: 1, // Negative
   }
);

// NOTE: This is because default Debug prints the single bits in reverse order, which sucks to debug
impl fmt::Debug for StatusRegister {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "StatusRegister {{ n: {}, v: {}, m: {}, x: {}, d: {}, i: {}, z: {}, c: {} }}",
      self.n, self.v, self.m, self.x, self.d, self.i, self.z, self.c
    )
  }
}

impl Default for StatusRegister {
  fn default() -> StatusRegister {
    StatusRegister {
      n: 0,
      v: 0,
      m: 0,
      x: 0,
      d: 0,
      i: 0,
      z: 0,
      c: 0,
    }
  }
}

bitfields!(
  // TODO: Verify order of A and B concerning byte order
  #[derive(PartialEq)]
  pub Accumulator: u16 {
    pub A: 8,
    pub B: 8,
  }
);

impl fmt::Debug for Accumulator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Accumulator {{ B: {:x}, A: {:x} }}", self.B, self.A)
  }
}

impl Default for Accumulator {
  fn default() -> Accumulator {
    Accumulator { A: 0, B: 0 }
  }
}

bitfields!(
  #[derive(PartialEq, Debug)]
  pub IndexRegister: u16 {
    low: 8,
    high: 8
  }
);

impl Default for IndexRegister {
  fn default() -> IndexRegister {
    IndexRegister { low: 0, high: 0 }
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
  S: IndexRegister, // Stack Pointer (or 24 bits?)
  PBR: u8,          // Programm Bank Register
  DBR: u8,          // Data Bank Register
  pub PC: u16,      // Programm Counter
}

#[derive(Debug)]
pub struct CPU {
  pub regs: Registers,
  pub e: bool, // Emulation mode
}

impl Default for Registers {
  fn default() -> Registers {
    Registers {
      P: StatusRegister::default(),
      C: Accumulator::default(),
      X: IndexRegister::default(),
      Y: IndexRegister::default(),
      D: 0,
      S: IndexRegister::default(),
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
      e: true,
    }
  }
}
