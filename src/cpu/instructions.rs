use super::constants::*;

#[derive(Debug, Clone)]
pub enum AddressModes {
  Unknown,
  Immediate,
  DirectPage,
  Absolute,
  DirectPageIndexedX,
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
  StackPush,
  ProgrammCounterRelative,
  Implied,
  StackPull,
  StackRTI,
  BlockMove,
  AbsoluteIndirect,
  AbsoluteIndexedIndirect,
  StackRTS,
  StackPCRelativeLong,
  StackRTL,
  ProgrammCounterRelativeLong,
  StackDirectPageIndirect,
  StackAbsolute,
}

impl Default for AddressModes {
  fn default() -> AddressModes {
    AddressModes::Unknown
  }
}

fn get_gii_reg_load_addr_mode(opcode: u8) -> Option<(AddressModes, usize)> {
  let mask = (opcode & GII_MASK);
  match mask {
    G2_REGLOAD_ADDR_MODE_IMMEDIATE => Some((AddressModes::Immediate, 2)),
    G2_REGLOAD_ADDR_MODE_DIRECT_PAGE => Some((AddressModes::DirectPage, 2)),
    G2_REGLOAD_ADDR_MODE_ABSOLUTE => Some((AddressModes::Absolute, 3)),
    G2_REGLOAD_ADDR_MODE_DIRECT_PAGE_INDEXED => Some((AddressModes::DirectPageIndexedX, 2)),
    G2_REGLOAD_ADDR_MODE_ABSOLUTE_INDEXED => Some((AddressModes::AbsoluteIndexedX, 3)),
    _ => None,
  }
}

fn get_gii_addr_mode(opcode: u8) -> Option<(AddressModes, usize)> {
  let mask = (opcode & GII_MASK);
  //  & (opcode & GII_MASK2);

  // println!("get_ii_addr_mode {:b}, opcode: {:b}", mask, opcode);
  match mask {
    G2_ADDR_MODE_ACCUMULATOR => Some((AddressModes::Accumulator, 1)),
    G2_ADDR_MODE_ABSOLUTE => Some((AddressModes::Absolute, 3)),
    G2_ADDR_MODE_DIRECT_ZERO_PAGE => Some((AddressModes::DirectPage, 2)),
    G2_ADDR_MODE_ABSOLUTE_INDEXED_X => Some((AddressModes::AbsoluteIndexedX, 3)),
    G2_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some((AddressModes::DirectPageIndexedX, 2)),
    _ => None,
  }
}

fn get_gi_addr_mode(opcode: u8) -> Option<(AddressModes, usize)> {
  let mask = opcode & GI_MASK;

  // println!("G1 opcode: {:b}, mask: {:b}", opcode, mask);

  match mask {
    GI_ADDR_MODE_INTERMEDIATE => Some((AddressModes::Immediate, 2)), // Add 1 byte if m = 0 (16Bit memory/accumulator)
    GI_ADDR_MODE_DIRECT_ZERO_PAGE => Some((AddressModes::DirectPage, 2)),
    GI_ADDR_MODE_ABSOLUTE => Some((AddressModes::Absolute, 3)),
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some((AddressModes::DirectPageIndexedX, 2)),
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
  pub opcode: Opcodes,
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
  PHP,
  PHD,
  BLP,
  TRB,
  CLC,
  TCS,
  JSR,
  BIT,
  PLP,
  PLD,
  BMI,
  SEC,
  TSC,
  RTI,
  WDM,
  MVP,
  PHA,
  PHK,
  JMP,
  BVC,
  MVN,
  CLI,
  PHY,
  TCD,
  RTS,
  PER,
  STZ,
  PLA,
  RTL,
  BVS,
  SEI,
  PLY,
  TDC,
  BRA,
  BRL,
  DEY,
  TXA,
  PHB,
  BCC,
  TYA,
  TXY,
  TXS,
  LDX,
  LDY,
  TAY,
  TAX,
  PLB,
  BCS,
  CLV,
  TSX,
  TYX,
  CPY,
  REP,
  INY,
  DEX,
  WAI,
  BNE,
  PEI,
  CLD,
  PHX,
  STP,
  CPX,
  INX,
  NOP,
  XBA,
  BEQ,
  PEA,
  SED,
  PLX,
  XCE,
}

impl Default for Opcodes {
  fn default() -> Opcodes {
    Opcodes::Unknown
  }
}

pub fn decode_group_III(opcode: u8) -> Option<(Opcodes, AddressModes, usize)> {
  // println!("Decode group III: {:X}, {:b}", opcode, G3_OP_TSB);
  // println!(
  //   "G III {:b}, {:b}, {:x}",
  //   G3_OP_TSB,
  //   opcode | 0x4 | 0xc,
  //   opcode
  // );

  match opcode | G3_OP_TSB {
    G3_OP_TSB => match opcode {
      0xc => return Some((Opcodes::TSB, AddressModes::Absolute, 3)),
      0x4 => return Some((Opcodes::TSB, AddressModes::DirectPage, 2)),
      _ => {}
    },
    _ => {}
  }
  match opcode | G3_OP_TRB {
    G3_OP_TRB => match opcode {
      0x1c => return Some((Opcodes::TRB, AddressModes::Absolute, 3)),
      0x14 => return Some((Opcodes::TRB, AddressModes::DirectPage, 2)),
      _ => {}
    },
    _ => {}
  }
  match opcode | G3_OP_JSR {
    G3_OP_JSR => match opcode {
      0x20 => return Some((Opcodes::JSR, AddressModes::Absolute, 3)),
      0xfc => return Some((Opcodes::JSR, AddressModes::AbsoluteIndexedIndirect, 3)),
      0x22 => return Some((Opcodes::JSR, AddressModes::AbsoluteLong, 4)),
      _ => {}
    },
    _ => {}
  }
  match opcode | G3_OP_BIT {
    G3_OP_BIT => match opcode {
      0x24 => return Some((Opcodes::BIT, AddressModes::DirectPage, 3)),
      0x2c => return Some((Opcodes::BIT, AddressModes::Absolute, 3)),
      0x34 => return Some((Opcodes::BIT, AddressModes::DirectPageIndexedX, 2)),
      0x3c => return Some((Opcodes::BIT, AddressModes::AbsoluteIndexedX, 3)),
      0x89 => return Some((Opcodes::BIT, AddressModes::Immediate, 2)), // bytes length can be 3 if m = 0 (16 bit accumulator)
      _ => {}
    },
    _ => {}
  }
  match opcode | G3_OP_JMP {
    G3_OP_JMP => match opcode {
      0x4c => return Some((Opcodes::JMP, AddressModes::Absolute, 3)),
      0x5c => return Some((Opcodes::JMP, AddressModes::AbsoluteLong, 4)),
      0x6c => return Some((Opcodes::JMP, AddressModes::AbsoluteIndirect, 3)),
      0x7c => return Some((Opcodes::JMP, AddressModes::AbsoluteIndexedIndirect, 3)),
      0xdc => return Some((Opcodes::JMP, AddressModes::Immediate, 2)), // bytes length can be 3 if m = 0 (16 bit accumulator)
      _ => {}
    },
    _ => {}
  }
  match opcode | G3_OP_STZ {
    G3_OP_STZ => match opcode {
      0x64 => return Some((Opcodes::STZ, AddressModes::DirectPage, 2)),
      0x74 => return Some((Opcodes::STZ, AddressModes::DirectPageIndexedIndirectX, 2)), // TODO: check address modes
      0x9c => return Some((Opcodes::STZ, AddressModes::Absolute, 3)),
      0x9e => return Some((Opcodes::STZ, AddressModes::AbsoluteIndexedIndirect, 3)),
      _ => {}
    },
    _ => {}
  }

  match opcode | G3_OP_CPY {
    G3_OP_CPY => match opcode {
      0xc0 => return Some((Opcodes::CPY, AddressModes::Immediate, 2)),
      0xc4 => return Some((Opcodes::CPY, AddressModes::DirectPage, 2)),
      0xcc => return Some((Opcodes::CPY, AddressModes::Absolute, 3)),
      _ => {}
    },
    _ => {}
  }

  match opcode | G3_OP_CPX {
    G3_OP_CPX => match opcode {
      0xe0 => return Some((Opcodes::CPX, AddressModes::Immediate, 2)),
      0xe4 => return Some((Opcodes::CPX, AddressModes::DirectPage, 2)),
      0xec => return Some((Opcodes::CPX, AddressModes::Absolute, 3)),
      _ => {}
    },
    _ => {}
  }
  match opcode {
    G3_OP_BRK => Some((Opcodes::BRK, AddressModes::StackInterrupt, 2)),
    G3_OP_COP => Some((Opcodes::COP, AddressModes::StackInterrupt, 2)),
    G3_OP_PHP => Some((Opcodes::PHP, AddressModes::StackPush, 1)),
    G3_OP_PHD => Some((Opcodes::PHD, AddressModes::StackPush, 1)),
    G3_OP_BLP => Some((Opcodes::BLP, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_CLC => Some((Opcodes::CLC, AddressModes::Implied, 1)),
    G3_OP_TCS => Some((Opcodes::TCS, AddressModes::Implied, 1)),
    G3_OP_PLP => Some((Opcodes::PLP, AddressModes::StackPull, 1)),
    G3_OP_PLD => Some((Opcodes::PLD, AddressModes::StackPull, 1)),
    G3_OP_BMI => Some((Opcodes::BMI, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_SEC => Some((Opcodes::SEC, AddressModes::Implied, 1)),
    G3_OP_TSC => Some((Opcodes::TSC, AddressModes::Implied, 1)),
    G3_OP_RTI => Some((Opcodes::RTI, AddressModes::StackRTI, 1)),
    G3_OP_WDM => Some((Opcodes::WDM, AddressModes::Unknown, 1)),
    G3_OP_MVP => Some((Opcodes::MVP, AddressModes::BlockMove, 3)),
    G3_OP_PHA => Some((Opcodes::PHA, AddressModes::StackPush, 1)),
    G3_OP_PHK => Some((Opcodes::PHK, AddressModes::StackPush, 1)),
    G3_OP_BVC => Some((Opcodes::BVC, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_MVN => Some((Opcodes::MVN, AddressModes::BlockMove, 3)),
    G3_OP_CLI => Some((Opcodes::CLI, AddressModes::Implied, 1)),
    G3_OP_PHY => Some((Opcodes::PHY, AddressModes::StackPush, 1)),
    G3_OP_TCD => Some((Opcodes::TCD, AddressModes::Implied, 1)),
    G3_OP_RTS => Some((Opcodes::RTS, AddressModes::StackRTS, 1)),
    G3_OP_PER => Some((Opcodes::PER, AddressModes::StackPCRelativeLong, 3)),
    G3_OP_PLA => Some((Opcodes::PLA, AddressModes::StackPull, 1)),
    G3_OP_RTL => Some((Opcodes::RTL, AddressModes::StackRTL, 1)),
    G3_OP_BVS => Some((Opcodes::BVS, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_SEI => Some((Opcodes::SEI, AddressModes::Implied, 1)),
    G3_OP_PLY => Some((Opcodes::PLY, AddressModes::StackPull, 1)),
    G3_OP_TDC => Some((Opcodes::TDC, AddressModes::Implied, 1)),
    G3_OP_BRA => Some((Opcodes::BRA, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_BRL => Some((Opcodes::BRL, AddressModes::ProgrammCounterRelativeLong, 3)),
    G3_OP_DEY => Some((Opcodes::DEY, AddressModes::Implied, 1)),
    G3_OP_TXA => Some((Opcodes::TXA, AddressModes::Implied, 1)),
    G3_OP_PHB => Some((Opcodes::PHB, AddressModes::StackPush, 1)),
    G3_OP_BCC => Some((Opcodes::BCC, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_TYA => Some((Opcodes::TYA, AddressModes::Implied, 1)),
    G3_OP_TXS => Some((Opcodes::TXS, AddressModes::Implied, 1)),
    G3_OP_TXY => Some((Opcodes::TXY, AddressModes::Implied, 1)),
    G3_OP_TAY => Some((Opcodes::TAY, AddressModes::Implied, 1)),
    G3_OP_TAX => Some((Opcodes::TAX, AddressModes::Implied, 1)),
    G3_OP_PLB => Some((Opcodes::PLB, AddressModes::StackPull, 1)),
    G3_OP_BCS => Some((Opcodes::BCS, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_CLV => Some((Opcodes::CLV, AddressModes::Implied, 1)),
    G3_OP_TSX => Some((Opcodes::TSX, AddressModes::Implied, 1)),
    G3_OP_TYX => Some((Opcodes::TYX, AddressModes::Implied, 1)),
    G3_OP_REP => Some((Opcodes::REP, AddressModes::Immediate, 2)),
    G3_OP_INY => Some((Opcodes::INY, AddressModes::Implied, 1)),
    G3_OP_DEX => Some((Opcodes::DEX, AddressModes::Implied, 1)),
    G3_OP_WAI => Some((Opcodes::WAI, AddressModes::Implied, 1)),
    G3_OP_BNE => Some((Opcodes::BNE, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_PEI => Some((Opcodes::PEI, AddressModes::StackDirectPageIndirect, 2)),
    G3_OP_CLD => Some((Opcodes::CLD, AddressModes::Implied, 1)),
    G3_OP_PHX => Some((Opcodes::PHX, AddressModes::StackPush, 1)),
    G3_OP_STP => Some((Opcodes::STP, AddressModes::Implied, 1)),
    G3_OP_INX => Some((Opcodes::INX, AddressModes::Implied, 1)),
    G3_OP_NOP => Some((Opcodes::NOP, AddressModes::Implied, 1)),
    G3_OP_XBA => Some((Opcodes::XBA, AddressModes::Implied, 1)),
    G3_OP_BEQ => Some((Opcodes::BEQ, AddressModes::ProgrammCounterRelative, 2)),
    G3_OP_PEA => Some((Opcodes::PEA, AddressModes::StackAbsolute, 3)),
    G3_OP_SED => Some((Opcodes::SED, AddressModes::Implied, 1)),
    G3_OP_PLX => Some((Opcodes::PLX, AddressModes::StackPull, 1)),
    G3_OP_XCE => Some((Opcodes::XCE, AddressModes::Implied, 1)),
    _ => None,
  }
  // Some((Opcodes::BRK, AddressModes::StackInterrupt, 2))
}

pub fn decode_group_II(opcode: u8) -> Option<(Opcodes, AddressModes, usize)> {
  // let group_2_mask: u8 = !GII_MASK;
  // let group_2_mask4addr_mode: u8 = !GII_MASK_4_ADDR_MODES;
  let g2_mask = opcode & !GII_MASK;
  let g2_mask2 = opcode & !GII_MASK2;
  // | (opcode & !GII_MASK2);
  //  | opcode & group_2_mask4addr_mode;

  // println!(
  //   "Decode group II: {:b}, {:b}, {:b}",
  //   opcode,
  //   g2_mask,
  //   opcode & g2_mask
  // );

  // LDX LDY

  // Edge Cases...
  match opcode {
    0x1a => {
      return Some((Opcodes::INC, AddressModes::Accumulator, 1));
    }
    0x3a => {
      return Some((Opcodes::DEC, AddressModes::Accumulator, 1));
    }
    0xe2 => {
      return Some((Opcodes::CPX, AddressModes::Immediate, 2)); // Only on 65816...
    }
    _ => {}
  }

  match g2_mask2 {
    G2_OP_DEC => {
      // println!("Test for DEC {:b}", opcode);
      if let Some(address_mode) = get_gii_addr_mode(opcode) {
        return Some((Opcodes::DEC, address_mode.0, address_mode.1));
      } else {
        return None;
      }
    }
    G2_OP_INC => {
      if let Some(addr_mode) = get_gii_addr_mode(opcode) {
        return Some((Opcodes::INC, addr_mode.0, addr_mode.1));
      } else {
        return None;
      }
    }
    G2_OP_STX => {
      if let Some(address_mode) = get_gii_addr_mode(opcode) {
        return Some((Opcodes::STX, address_mode.0, address_mode.1));
      } else {
        return None;
      }
    }
    G2_OP_STY => {
      if let Some(address_mode) = get_gii_addr_mode(opcode) {
        return Some((Opcodes::STY, address_mode.0, address_mode.1));
      } else {
        return None;
      }
    }
    _ => {}
  }

  match g2_mask {
    G2_OP_ASL => {
      if let Some(addr_mode) = get_gii_addr_mode(opcode) {
        Some((Opcodes::ASL, addr_mode.0, addr_mode.1))
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
    // Diffrend addressing here
    G2_OP_LDX => {
      if let Some(address_mode) = get_gii_reg_load_addr_mode(opcode) {
        Some((Opcodes::LDX, address_mode.0, address_mode.1))
      } else {
        None
      }
    }
    G2_OP_LDY => {
      if let Some(address_mode) = get_gii_reg_load_addr_mode(opcode) {
        Some((Opcodes::LDY, address_mode.0, address_mode.1))
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
      println!("{:x}", *byte);
      if let Some(i) = self.decode(*byte) {
        println!("{:?}", i);
      }
    }
    // for (i, byte) in bytes.iter().enumerate() {
    //   // let mut operants = Operants::default();
    //   // TODO: reduce code dublication!<
    //   // TODO: Fix this loop... it's crap
    //   if !self.fetching_instruction {
    //     if let Some(foo) = decode_group_III(*byte) {
    //       self.fetching_instruction = true;
    //       bytes_to_read = foo.2 - 1;
    //       // println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
    //       println!("{:?}", foo);
    //       inst.address = i as u32;
    //       inst.address_mode = foo.1;
    //       inst.opcode = foo.0;
    //       inst.lenght = foo.2;
    //     // inst.cycles = foo;
    //     } else if let Some(foo) = decode_group_II(*byte) {
    //       self.fetching_instruction = true;
    //       bytes_to_read = foo.2 - 1;
    //       // println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
    //       println!("{:?}", foo);
    //       inst.address = i as u32;
    //       inst.address_mode = foo.1;
    //       inst.opcode = foo.0;
    //       inst.lenght = foo.2;
    //     } else if let Some(foo) = decode_group_I(*byte) {
    //       self.fetching_instruction = true;
    //       bytes_to_read = foo.2 - 1;
    //       // println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
    //       println!("{:?}", foo);
    //       inst.address = i as u32;
    //       inst.address_mode = foo.1;
    //       inst.opcode = foo.0;
    //       inst.lenght = foo.2;
    //     } else {
    //       println!("Nothing found... {:x}", &byte);
    //     }
    //   } else {
    //     // bytes_to_read -= 1;
    //     if bytes_to_read < 1 {
    //       println!("Pushing new instruction, {:?}", inst);
    //       self.fetching_instruction = false;
    //       self.instructions.push(inst.clone());
    //     } else if bytes_to_read == 3 {
    //       inst.operants.operant_bank = *byte;
    //       bytes_to_read -= 1;
    //     } else if bytes_to_read == 2 {
    //       inst.operants.operant_high = *byte;
    //       bytes_to_read -= 1;
    //     } else if bytes_to_read == 1 {
    //       inst.operants.operant_low = *byte;
    //       bytes_to_read -= 1;
    //     }
    //   }
    // }
  }

  pub fn decode(&self, opcode: u8) -> Option<(Opcodes, AddressModes, usize)> {
    // Group I decode

    if let Some(instr) = decode_group_III(opcode) {
      // println!("Group III: {:?}", instr);
      return Some(instr);
    } else if let Some(instr) = decode_group_II(opcode) {
      // println!("Group II: {:?}", instr);
      return Some(instr);
    } else if let Some(instr) = decode_group_I(opcode) {
      // println!("Group I: {:?}", instr);
      return Some(instr);
    }
    None
  }

  pub fn printInstructions(&self) {
    for i in &self.instructions {
      // println!("{:?}", i);
      println!("{:?} {:?}", i.opcode, i.operants);
    }
  }
}
