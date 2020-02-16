use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};

use std::convert::From;
use std::fmt;

pub mod addressmodes;
pub mod constants;
pub mod decoder;
pub mod instructions;

// in emulation mode $100 to $1FF
#[derive(Copy, Clone)]
pub struct Stack {
  content: [u8; 0x10000],
}

impl fmt::Debug for Stack {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Stack {{ too large to print }}")
  }
}

impl Default for Stack {
  fn default() -> Stack {
    Stack {
      content: [0; 0x10000],
    }
  }
}

bitfields!(
  #[derive(PartialEq, Copy, Clone)]
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

// Conversion helper functions
impl From<StatusRegister> for u8 {
  fn from(p: StatusRegister) -> Self {
    let mut number = [0];
    p.write_bytes_default_le(&mut number);
    return number[0];
  }
}

impl From<u8> for StatusRegister {
  fn from(byte: u8) -> Self {
    byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[byte])
  }
}

impl From<u8> for IndexRegister {
  fn from(number: u8) -> Self {
    let high = 0x0;
    let low = number;
    byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
  }
}

impl From<IndexRegister> for u16 {
  fn from(register: IndexRegister) -> Self {
    let mut number = [0; 2];
    register.write_bytes_default_le(&mut number);
    return (number[1] as u16) << 8 | number[0] as u16;
  }
}

impl From<u16> for IndexRegister {
  fn from(number: u16) -> Self {
    let high = (number >> 8) as u8;
    let low = (number & 0xff) as u8;
    byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
  }
}

impl From<Accumulator> for u16 {
  fn from(register: Accumulator) -> Self {
    let mut number = [0; 2];
    register.write_bytes_default_le(&mut number);
    return (number[1] as u16) << 8 | number[0] as u16;
  }
}

impl From<u16> for Accumulator {
  fn from(number: u16) -> Self {
    let high = (number >> 8) as u8;
    let low = (number & 0xff) as u8;
    byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
  }
}

impl From<Accumulator> for usize {
  fn from(register: Accumulator) -> Self {
    let mut number = [0; 2];
    register.write_bytes_default_le(&mut number);
    return (number[1] as usize) << 8 | number[0] as usize;
  }
}

impl From<usize> for Accumulator {
  fn from(number: usize) -> Self {
    let high = (number >> 8) as u8;
    let low = (number & 0xff) as u8;
    byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
  }
}

// NOTE: Verify if this is correct
impl Default for StatusRegister {
  fn default() -> StatusRegister {
    StatusRegister {
      n: 0,
      v: 0,
      m: 1,
      x: 1,
      d: 0,
      i: 1,
      z: 0,
      c: 0,
    }
  }
}

bitfields!(
  #[derive(PartialEq, Copy, Clone)]
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
  #[derive(PartialEq, Debug, Copy, Clone)]
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
#[derive(ByteStruct, PartialEq, Debug, Clone, Copy)]
#[byte_struct_le]
pub struct Registers {
  P: StatusRegister,
  C: Accumulator,
  X: IndexRegister, // X Index Register,
  Y: IndexRegister, // Y Index Register,
  D: u16,           // Direct Page Register
  S: IndexRegister, // Stack Pointer (or 24 bits?)
  pub PBR: u8,      // Programm Bank Register
  pub DBR: u8,      // Data Bank Register
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
      S: byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[0xff, 0xf2]),
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

  pub fn stack_push(&mut self, payload: u8) {
    let index = <u16>::from(self.regs.S);
    println!("Pushing {:x} to address {:x}", payload, index);

    let mut new_index: i32 = index as i32 - 1;

    if new_index == -1 {
      new_index = 0xffff
    }
    self.stack.content[(new_index) as usize] = payload;
    self.regs.S = IndexRegister::from(new_index as u16);
  }

  pub fn stack_pull(&mut self) -> u8 {
    let index = <u16>::from(self.regs.S);
    let ret = self.stack.content[index as usize];

    let mut new_index: i32 = index as i32 + 1;
    if new_index == 0x10000 {
      new_index = 0;
    }
    self.regs.S = IndexRegister::from(new_index as u16);
    ret
  }
}
