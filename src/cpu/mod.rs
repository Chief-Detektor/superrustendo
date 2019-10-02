use constants::GI_MASK;
use byte_struct::*;
// enum Opcodes {}

pub mod constants;

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

#[derive(Debug)]
pub enum AddressModes {
  Intermediate,
  DirectZeroPage,
  Absolute,
  DirectZeroPageIndexedX,
  AbsoluteIndexedX,
  AbsoluteIndexedY,
  DirectPageIndexedIndirectX,
  DirectPageIndexedIndirectY,
  DirectPageIndirectLongIndexedY,
  DirectPageIndirectLong,
  AbsoluteLong,
  AbsoluteLongIndexedX,
  StackRelative,
  StackRelativeIndirectIndexedY,
  DirectPageIndirect,
}

pub struct Decoder {}

fn get_gi_addr_mode(opcode: u8) -> Option<(AddressModes, usize)> {
  let mask = opcode & GI_MASK;

  match mask {
    GI_ADDR_MODE_INTERMEDIATE => Some((AddressModes::Intermediate, 2)), // Add 1 byte if m = 0 (16Bit memory/accumulator)
    GI_ADDR_MODE_DIRECT_ZERO_PAGE => Some((AddressModes::DirectZeroPage, 3)),
    GI_ADDR_MODE_ABSOLUTE => Some((AddressModes::Absolute, 3)),
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some((AddressModes::DirectZeroPageIndexedX, 2)),
    GI_ADDR_MODE_ABSOLUTE_INDEXED_X => Some((AddressModes::AbsoluteIndexedX, 3)),
    GI_ADDR_MODE_ABSOLUTE_INDEXED_Y => Some((AddressModes::AbsoluteIndexedY, 3)),
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_X => {
      Some((AddressModes::DirectPageIndexedIndirectX, 2))
    }
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_Y => {
      Some((AddressModes::DirectPageIndexedIndirectY, 2))
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG_INDEXED_Y => {
      Some((AddressModes::DirectPageIndirectLongIndexedY, 2))
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG => Some((AddressModes::DirectPageIndirectLong, 2)),
    GI_ADDR_MODE_ABSOLUTE_LONG => Some((AddressModes::AbsoluteLong, 4)),
    GI_ADDR_MODE_ABSOLUTE_LONG_INDEXED_X => Some((AddressModes::AbsoluteLongIndexedX, 4)),
    GI_ADDR_MODE_STACK_RELATIVE => Some((AddressModes::StackRelative, 2)),
    GI_ADDR_MODE_STACK_RELATIVE_INDIRECT_INDEXED_Y => {
      Some((AddressModes::StackRelativeIndirectIndexedY, 2))
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT => Some((AddressModes::DirectPageIndirect, 2)),
    _ => {
      return None;
    }
  }
  // return true;
}

#[derive(Debug)]
struct Instruction {
  Opcode: Opcodes,
  AddressMode: AddressModes,
  lenght: usize,
  bytes: [u8; 4],
  cycles: usize,
}

// impl Instruction {
//   fn execute() {

//   }
// }

#[derive(Debug)]
enum Opcodes {
  // Group I Opcodes
  ADC,
  AND,
  CMP,
  EOR,
  LDA,
  ORA,
  SBC,
  STA,
}

fn decode_group_I(opcode: u8) -> Option<Instruction> {
  let group_1_mask: u8 = 0b1110_0000;
  let g1_mask = opcode & group_1_mask;

  match g1_mask {
    G1_OP_ADC => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        // println!("Add with Carry to Acc (ADC), AddressMode: {:?}", addr_mode);
        Some(Instruction {
          Opcode: Opcodes::ADC,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_AND => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        // println!("Add with Carry to Acc (ADC), AddressMode: {:?}", addr_mode);
        Some(Instruction {
          Opcode: Opcodes::AND,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_CMP => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some(Instruction {
          Opcode: Opcodes::CMP,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_EOR => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some(Instruction {
          Opcode: Opcodes::EOR,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_LDA => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some(Instruction {
          Opcode: Opcodes::LDA,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_ORA => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some(Instruction {
          Opcode: Opcodes::ORA,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_SBC => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some(Instruction {
          Opcode: Opcodes::SBC,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    G1_OP_STA => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some(Instruction {
          Opcode: Opcodes::STA,
          AddressMode: addr_mode.0,
          cycles: 0,
          lenght: addr_mode.1,
          bytes: [0 as u8; 4],
        })
      } else {
        None
      }
    }
    _ => {
      println!("No Group I opcode");
      None
    }
  }
}

impl Decoder {
  pub fn decode(opcode: u8) {
    // Group I decode

    let instr = decode_group_I(opcode);

    println!("{:?}", instr);

    // Group II decode
    let group_2_mask: u8 = 0b1110_0011;
    if opcode & group_2_mask == 0b0000_0010 {
      println!("Arithmetic Shift Left (ASL)")
    }
  }
}
