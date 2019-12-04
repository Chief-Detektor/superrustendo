use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};

use std::fmt;

pub mod addressmodes;
pub mod constants;
pub mod instructions;

// #[derive(Debug)]

// in emulation mode $100 to $1FF
pub struct Stack {
  content: [u8; 0xffff - 1],
  // constents: Vec<u8>,
}

impl fmt::Debug for Stack {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Stack {{ too large to print }}")
  }
}

impl Default for Stack {
  fn default() -> Stack {
    Stack {
      content: [0; 0xffff - 1],
    }
  }
}

bitfields!(
  #[derive(PartialEq)]
  pub StatusRegister: u8 {
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

impl IndexRegister {
  pub fn new() -> IndexRegister {
    IndexRegister {
      low: 0xff,
      high: 0xff,
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
  S: IndexRegister, // Stack Pointer (or 24 bits?)
  PBR: u8,          // Programm Bank Register
  DBR: u8,          // Data Bank Register
  pub PC: u16,      // Programm Counter
}

#[derive(Debug)]
pub struct CPU {
  pub regs: Registers,
  pub stack: Stack,
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
      S: IndexRegister::new(),
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
      stack: Stack::default(),
      e: true,
    }
  }

  // This looks horrible..
  // TODO: helper functions to deal with byte_struct to u16 and back
  // pub fn stack_push(&mut self, data: u8) {
  //   // decrease stack pointer
  //   let mut stack_pointer = [0x0, 0x0];
  //   self.regs.S.write_bytes_default_le(&mut stack_pointer);
  //   let mut foo = (stack_pointer[1] as u16) << 8 | stack_pointer[0] as u16;
  //   foo += 1;
  //   self.regs.S = byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&foo);

  //   self.stack.content[stack_pointer[0] as usize | (stack_pointer[1] as usize) << 8] = data;
  // }

  // pub fn stack_pull(&mut self) -> u8 {
  //   // increase stack pointer
  //   let mut stack_pointer = [0x0, 0x0];
  //   self.regs.S.write_bytes_default_le(&mut stack_pointer);
  //   stack_pointer[0] = stack_pointer[0] - 1;
  //   self.regs.S =
  //     byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&stack_pointer);

  //     byte_struct::ByteStructUnspecifiedByteOrder::

  //   // let mut index = [0x0, 0x0];
  //   // self.regs.P.write_bytes_default_le(&mut index);
  //   self.stack.content[stack_pointer[0] as usize | (stack_pointer[1] as usize) << 8]
  // }
}
