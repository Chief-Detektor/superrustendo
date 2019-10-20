use super::constants::*;

#[derive(Debug, Clone)]
pub enum AddressModes {
  Unknown,
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
  // Group II
  Accumulator,
  // Group III
  StackInterrupt,
}

impl Default for AddressModes {
  fn default() -> AddressModes {
    AddressModes::Unknown
  }
}

fn get_gii_addr_mode(opcode: u8) -> Option<(AddressModes, usize)> {
  let mask = opcode & GII_MASK;

  println!("get_ii_addr_mode {:b}, opcode: {:b}", mask, opcode);
  match mask {
    G2_ADDR_MODE_ACCUMULATOR => Some((AddressModes::Accumulator, 1)),
    G2_ADDR_MODE_ABSOLUTE => Some((AddressModes::Absolute, 3)),
    G2_ADDR_MODE_DIRECT_ZERO_PAGE => Some((AddressModes::DirectZeroPage, 2)),
    G2_ADDR_MODE_ABSOLUTE_INDEXED_X => Some((AddressModes::AbsoluteIndexedX, 3)),
    G2_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some((AddressModes::DirectZeroPageIndexedX, 2)),
    _ => None,
  }
}

fn get_gi_addr_mode(opcode: u8) -> Option<(AddressModes, usize)> {
  let mask = opcode & GI_MASK;

  match mask {
    GI_ADDR_MODE_INTERMEDIATE => Some((AddressModes::Intermediate, 2)), // Add 1 byte if m = 0 (16Bit memory/accumulator)
    GI_ADDR_MODE_DIRECT_ZERO_PAGE => Some((AddressModes::DirectZeroPage, 2)),
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

#[derive(Debug, Default, Clone)]
pub struct Operants {
  operant_low: u8,
  operant_high: u8,
  operant_bank: u8,
}

#[derive(Debug, Default, Clone)]
pub struct Instruction {
  address: u32,
  opcode: Opcodes,
  address_mode: AddressModes,
  lenght: usize,
  pub operants: Operants,
  cycles: usize,
}

// impl Instruction {
//   fn execute() {

//   }
// }

#[derive(Debug, Clone)]
pub enum Opcodes {
  // Group I Opcodes
  Unknown,
  ADC,
  AND,
  CMP,
  EOR,
  LDA,
  ORA,
  SBC,
  STA,
  // Group II opcodes
  ASL,
  DEC,
  INC,
  LSR,
  ROL,
  ROR,
  STX,
  STY,
  // Group III opcodes
  BRK,
  COP,
  TSB,
}

impl Default for Opcodes {
  fn default() -> Opcodes {
    Opcodes::Unknown
  }
}

pub fn decode_group_III(opcode: u8) -> Option<(Opcodes, AddressModes, usize)> {
  // println!("Decode group III: {:X}, {:b}", opcode, G3_OP_TSB);
  match opcode {
    G3_OP_BRK => Some((Opcodes::BRK, AddressModes::StackInterrupt, 2)),
    G3_OP_COP => Some((Opcodes::COP, AddressModes::StackInterrupt, 2)),
    G3_OP_TSB => Some((Opcodes::TSB, AddressModes::Absolute, 3)),
    G3_OP_TSB_DIRECT => Some((Opcodes::TSB, AddressModes::DirectZeroPage, 2)),
    _ => None,
  }
  // Some((Opcodes::BRK, AddressModes::StackInterrupt, 2))
}

pub fn decode_group_II(opcode: u8) -> Option<(Opcodes, AddressModes, usize)> {
  let group_2_mask: u8 = !GII_MASK;
  // let group_2_mask4addr_mode: u8 = !GII_MASK_4_ADDR_MODES;
  let g2_mask = opcode & group_2_mask;
  //  | opcode & group_2_mask4addr_mode;

  println!(
    "Decode group II: {:b}, {:b}, {:b}",
    opcode,
    g2_mask,
    opcode & g2_mask
  );

  // Edge Cases...
  match opcode {
    0x1a => {
      // println!("{}", opcode);
      return Some((Opcodes::INC, AddressModes::Accumulator, 1));
    }
    0x3a => {
      return Some((Opcodes::DEC, AddressModes::Accumulator, 1));
    }
    0x96 => return Some((Opcodes::STX, AddressModes::DirectPageIndexedIndirectY, 2)),
    // 0x94 => return Some((Opcodes::STY, AddressModes::DirectPageIndexedIndirectX, 2)), // This is needed because of the
    // 0x
    _ => {}
  }

  // println!("{:b}  {:b}", g2_mask, opcode);
  match g2_mask {
    G2_OP_ASL => {
      if let Some(addr_mode) = get_gii_addr_mode(opcode) {
        Some((Opcodes::ASL, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G2_OP_DEC => {
      if let Some(address_mode) = get_gii_addr_mode(opcode | opcode & 0b1000) {
        Some((Opcodes::DEC, address_mode.0, address_mode.1))
      } else {
        None
      }
    }
    G2_OP_INC => {
      if let Some(addr_mode) = get_gii_addr_mode(opcode | opcode & 0b1000) {
        Some((Opcodes::INC, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G2_OP_LSR => {
      if let Some(addr_mode) = get_gii_addr_mode(opcode) {
        Some((Opcodes::LSR, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G2_OP_ROL => {
      if let Some(addr_mode) = get_gii_addr_mode(opcode) {
        Some((Opcodes::ROL, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G2_OP_ROR => {
      if let Some(address_mode) = get_gii_addr_mode(opcode) {
        Some((Opcodes::ROR, address_mode.0, address_mode.1))
      } else {
        None
      }
    }
    G2_OP_STX => {
      if let Some(address_mode) = get_gii_addr_mode(opcode | opcode & 0b1000) {
        Some((Opcodes::STX, address_mode.0, address_mode.1))
      } else {
        None
      }
    }
    G2_OP_STY => {
      if let Some(address_mode) = get_gii_addr_mode(opcode | opcode & 0b1000) {
        Some((Opcodes::STY, address_mode.0, address_mode.1))
      } else {
        None
      }
    }
    _ => None,
  }
}

pub fn decode_group_I(opcode: u8) -> Option<(Opcodes, AddressModes, usize)> {
  let group_1_mask: u8 = !GI_MASK;
  let g1_mask = opcode & group_1_mask;

  // println!("{:b}  {:b}", g1_mask, opcode);
  match g1_mask {
    G1_OP_ADC => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::ADC, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_AND => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::AND, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_CMP => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::CMP, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_EOR => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::EOR, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_LDA => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::LDA, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_ORA => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::ORA, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_SBC => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::SBC, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    G1_OP_STA => {
      if let Some(addr_mode) = get_gi_addr_mode(opcode) {
        Some((Opcodes::STA, addr_mode.0, addr_mode.1))
      } else {
        None
      }
    }
    _ => {
      // println!("No Group I opcode");
      None
    }
  }
}

#[derive(Debug)]
pub struct Decoder {
  fetching_instruction: bool,
  instructions: Vec<Instruction>,
}

impl Decoder {
  pub fn new() -> Decoder {
    Decoder {
      fetching_instruction: false,
      instructions: Vec::new(),
    }
  }

  // fn read_instruction(byte: u8)

  pub fn read_instructions(&mut self, bytes: &Vec<u8>) /*-> Instruction*/
  {
    let mut bytes_to_read = 0;
    let mut inst = Instruction::default();
    for (i, byte) in bytes.iter().enumerate() {
      // let mut operants = Operants::default();
      if !self.fetching_instruction {
        if let Some(foo) = decode_group_I(*byte) {
          self.fetching_instruction = true;
          bytes_to_read = foo.2 - 1;
          println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
          println!("{:?}", foo);
          inst.address = i as u32;
          inst.address_mode = foo.1;
          inst.opcode = foo.0;
          inst.lenght = foo.2;
        // inst.cycles = foo;
        } else if let Some(foo) = decode_group_II(*byte) {
          self.fetching_instruction = true;
          bytes_to_read = foo.2 - 1;
          println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
          println!("{:?}", foo);
          inst.address = i as u32;
          inst.address_mode = foo.1;
          inst.opcode = foo.0;
          inst.lenght = foo.2;
        } else if let Some(foo) = decode_group_III(*byte) {
          self.fetching_instruction = true;
          bytes_to_read = foo.2 - 1;
          println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
          println!("{:?}", foo);
          inst.address = i as u32;
          inst.address_mode = foo.1;
          inst.opcode = foo.0;
          inst.lenght = foo.2;
        } else {
          println!("Nothing found... {:x}", &byte);
        }
      } else {
        // bytes_to_read -= 1;
        if bytes_to_read < 1 {
          println!("Pushing new instruction, {:?}", inst);
          self.fetching_instruction = false;
          self.instructions.push(inst.clone());
        } else if bytes_to_read == 1 {
          inst.operants.operant_bank = *byte;
          bytes_to_read -= 1;
        } else if bytes_to_read == 2 {
          inst.operants.operant_high = *byte;
          bytes_to_read -= 1;
        } else if bytes_to_read == 3 {
          inst.operants.operant_low = *byte;
          bytes_to_read -= 1;
        }
      }
    }
  }

  pub fn decode(&self, opcode: u8) {
    // Group I decode

    if let Some(instr) = decode_group_I(opcode) {
      println!("Group I: {:?}", instr);
    } else if let Some(instr) = decode_group_II(opcode) {
      println!("Group II: {:?}", instr);
    } else if let Some(instr) = decode_group_III(opcode) {
      println!("Group III: {:?}", instr);
    }
    // println!("Nothing found! Opcode 0x{:x}", opcode)
    // }

    println!("{:?}", opcode);

    // Group II decode
    // let group_2_mask: u8 = 0b1110_0011;
    // if opcode & group_2_mask == 0b0000_0010 {
    //   println!("Arithmetic Shift Left (ASL)")
    // }
  }
}
