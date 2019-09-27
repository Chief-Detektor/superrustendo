use byte_struct::*;
#[macro_use]
extern crate bitpat;

bitfields!(
  #[derive(PartialEq, Debug)]
  StatusRegister: u8 {
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
  Accumulator: u16 {
    A: 8,
    B: 8,
  }
);

bitfields!(
  #[derive(PartialEq, Debug)]
  IndexRegister: u16 {
    register: 8,
    index: 8
  }
);

// TODO: Proper inital state
// #[derive(ByteStzruct, PartialEq, Debug)]
struct Registers {
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

struct CPU {
  regs: Registers,
}

// enum Opcodes {}

const GI_ADDR_MODE_INTERMEDIATE: u8 = 0b0_1001;
const GI_ADDR_MODE_DIRECT_ZERO_PAGE: u8 = 0b0_0101;
const GI_ADDR_MODE_ABSOLUTE: u8 = 0b0_1101;
const GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X: u8 = 0b1_0101;
const GI_ADDR_MODE_ABSOLUTE_INDEXED_X: u8 = 0b1_1101;
const GI_ADDR_MODE_ABSOLUTE_INDEXED_Y: u8 = 0b1_1001;
const GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_X: u8 = 0b0_0001;
const GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_Y: u8 = 0b1_0001;

const GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG_INDEXED_Y: u8 = 0b1_0111;
const GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG: u8 = 0b0_0111;
const GI_ADDR_MODE_ABSOLUTE_LONG: u8 = 0b0_1111;
const GI_ADDR_MODE_ABSOLUTE_LONG_INDEXED_X: u8 = 0b1_1111;
const GI_ADDR_MODE_STACK_RELATIVE: u8 = 0b0_0011;
const GI_ADDR_MODE_STACK_RELATIVE_INDIRECT_INDEXED_Y: u8 = 0b1_0011;
const GI_ADDR_MODE_DIRECT_PAGE_INDIRECT: u8 = 0b1_0010;

struct Decoder {}

fn GroupIModeDecoder(opcode: u8) -> bool {
  let address_mode_mask: u8 = 0b0001_1111;

  let mask = opcode & address_mode_mask;

  // println!("Mask: {:b}", mask);

  match mask {
    GI_ADDR_MODE_INTERMEDIATE => {
      println!("Intermediate");
    }
    GI_ADDR_MODE_DIRECT_ZERO_PAGE => {
      println!("Direct (Zero) Page");
    }
    GI_ADDR_MODE_ABSOLUTE => {
      println!("Absolute");
    }
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => {
      println!("Direct (Zero) Page Indexed by X");
    }
    GI_ADDR_MODE_ABSOLUTE_INDEXED_X => {
      println!("Absolute Indexed by X");
    }
    GI_ADDR_MODE_ABSOLUTE_INDEXED_Y => {
      println!("Absolute Indexed by Y");
    }
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_X => {
      println!("Direct (Zero) Page Indexed Indirect with X (pre-indexed)");
    }
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_Y => {
      println!("Direct (Zero) Page Indexed Indirect with Y (post-indexed)");
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG_INDEXED_Y => {
      println!("Direct Page Long indexed with Y (post index long)");
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG => {
      println!("Direct Page Long");
    }
    GI_ADDR_MODE_ABSOLUTE_LONG => {
      println!("Absolute Long");
    }
    GI_ADDR_MODE_ABSOLUTE_LONG_INDEXED_X => {
      println!("Absolute Long Indexed with X");
    }
    GI_ADDR_MODE_STACK_RELATIVE => {
      println!("Stack Relative");
    }
    GI_ADDR_MODE_STACK_RELATIVE_INDIRECT_INDEXED_Y => {
      println!("Stack Relative Indirect Indexed with Y");
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT => {
      // also in 65816?
      println!("Direct Page Indirect");
      // panic!("Stopping, yo!");
      // return false;
    }
    _ => {
      return false;
    }
  }

  return true;
}

impl Decoder {
  fn decode(opcode: u8) {
    let group_1_mask: u8 = 0b1110_0000;

    // Group I decode

    if opcode & group_1_mask == 0b0110_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Add with Carry to Acc (ADC)");
      }
    } else if opcode & group_1_mask == 0b0010_0000 {
      if GroupIModeDecoder(opcode) {
        println!("And the Accumulator (AND)");
      }
    } else if opcode & group_1_mask == 0b1100_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Compare the Accumulator (CMP)");
      }
    } else if opcode & group_1_mask == 0b0100_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Exclusive Or the Accumulator (EOR)");
      }
    } else if opcode & group_1_mask == 0b1010_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Load the Accumulator (LDA)");
      }
    } else if opcode & group_1_mask == 0b0000_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Or the Accumulator (ORA)");
      }
    } else if opcode & group_1_mask == 0b1110_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Substract with Borrow from the Accumulator (SBC)");
      }
    } else if opcode & group_1_mask == 0b1000_0000 {
      if GroupIModeDecoder(opcode) {
        println!("Store the Accumulator (STA)");
      }
    }

    // Group II decode
    let group_2_mask: u8 = 0b1110_0011;
    if opcode & group_2_mask == 0b0000_0010 {
      println!("Arithmetic Shift Left (ASL)")
    }
  }
}

fn main() {
  // ADC opcode test
  Decoder::decode(0x69);
  Decoder::decode(0x6D);
  Decoder::decode(0x6F);
  Decoder::decode(0x65);

  // println!("{:b}", 0x72);
  Decoder::decode(0x72);
  Decoder::decode(0x67);
  Decoder::decode(0x7D);
  Decoder::decode(0x7F);
  Decoder::decode(0x79);
  Decoder::decode(0x75);
  Decoder::decode(0x61);
  Decoder::decode(0x71);
  Decoder::decode(0x77);
  Decoder::decode(0x63);
  Decoder::decode(0x73);

  // ASL
  Decoder::decode(0x0A);
  Decoder::decode(0x0E);
  // Decoder::decode(0x6F);
  // Decoder::decode(0x6F);
}
