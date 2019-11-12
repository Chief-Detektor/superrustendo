use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};

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
      n: 1, // Negative
      v: 1, // Overflow
      m: 1, // Memory/Accumulator Select
      x: 1, // Index Register Select/Break Instruction
      d: 1, // Decimal Mde
      i: 1, // IRQ Disable
      z: 1, // Result Zero
      c: 1, // CarryBit / Emulation Mode
  }
);

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
  #[derive(PartialEq, Debug)]
  pub Accumulator: u16 {
    pub B: 8,
    pub A: 8,
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
      e: true,
    }
  }
}
