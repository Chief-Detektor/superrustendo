use super::constants::*;
use super::decoder::Opcodes;
use super::Registers;
use super::CPU;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum AddressModes {
  Absolute,
  AbsoluteIndirectLong,
  AbsoluteIndexedIndirect,
  AbsoluteIndexedX,
  AbsoluteIndexedY,
  AbsoluteIndirect,
  AbsoluteLong,
  AbsoluteLongIndexedX,
  Accumulator,
  BlockMove,
  DirectPage,
  DirectPageIndexedIndirectX,
  DirectPageIndexedIndirectY,
  DirectPageIndexedX,
  DirectPageIndexedY,
  DirectPageIndirect,
  DirectPageIndirectLong,
  DirectPageIndirectLongIndexedY,
  Immediate,
  Implied,
  ProgrammCounterRelative,
  ProgrammCounterRelativeLong,
  StackAbsolute,
  StackDirectPageIndirect,
  StackInterrupt,
  StackPCRelativeLong,
  StackPull,
  StackPush,
  StackRTI,
  StackRTL,
  StackRTS,
  StackRelative,
  StackRelativeIndirectIndexedY,
  Unknown,
}

// TODO: Pass in CPU native or emulation mode in order to return correct len
impl AddressModes {
  pub fn len(&self, regs: &Registers, op: &Opcodes) -> usize {
    match self {
      AddressModes::Absolute => 3,
      AddressModes::AbsoluteIndirectLong => 3,
      AddressModes::AbsoluteIndexedIndirect => 3,
      AddressModes::AbsoluteIndexedX => 3,
      AddressModes::AbsoluteIndexedY => 3,
      AddressModes::AbsoluteIndirect => 3,
      AddressModes::AbsoluteLong => 4,
      AddressModes::AbsoluteLongIndexedX => 4,
      AddressModes::Accumulator => 1,
      AddressModes::BlockMove => 3,
      AddressModes::DirectPage => 2,
      AddressModes::DirectPageIndexedIndirectX => 2,
      AddressModes::DirectPageIndexedIndirectY => 2,
      AddressModes::DirectPageIndexedX => 2,
      AddressModes::DirectPageIndexedY => 2,
      AddressModes::DirectPageIndirect => 2,
      AddressModes::DirectPageIndirectLong => 2,
      AddressModes::DirectPageIndirectLongIndexedY => 2,
      AddressModes::Immediate => {
        match *op {
          Opcodes::LDX | Opcodes::CPX => {
            if regs.P.x != 1 {
              return 3;
            }
          }
          Opcodes::LDA => {
            if regs.P.m != 1 {
              return 3;
            }
          }
          _ => {}
        }
        return 2;
      }
      AddressModes::Implied => 1,
      AddressModes::ProgrammCounterRelative => 2,
      AddressModes::ProgrammCounterRelativeLong => 3,
      AddressModes::StackAbsolute => 3,
      AddressModes::StackDirectPageIndirect => 2,
      AddressModes::StackInterrupt => 2,
      AddressModes::StackPCRelativeLong => 3,
      AddressModes::StackPull => 1,
      AddressModes::StackPush => 1,
      AddressModes::StackRTI => 1,
      AddressModes::StackRTL => 1,
      AddressModes::StackRTS => 1,
      AddressModes::StackRelative => 2,
      AddressModes::StackRelativeIndirectIndexedY => 2,
      AddressModes::Unknown => 2,
    }
  }

  pub fn get_effective_address(&self, cpu: &mut CPU, payload: &Vec<u8>, opcode: &Opcodes) -> usize {
    match &self {
      AddressModes::Absolute => {
        let bank;
        if *opcode == Opcodes::JMP || *opcode == Opcodes::JSR {
          println!("Transfer control");
          bank = cpu.regs.PBR;
        } else {
          println!("Datamove");
          bank = cpu.regs.DBR;
        }

        let data = payload.as_slice();
        println!("### Data, yo: {:?} Bank: {:x}", data, bank);

        return ((bank as usize) << 16 | (data[1] as usize) << 8 | data[0] as usize) as usize;
      }
      // AddressModes::AbsoluteIndexedX => {
      // unimplemented!();
      // let data = payload.as_slice();

      // let mut number = (cpu.regs.DBR as u32) << 16 | (data[1] as u32) << 8 | data[0] as u32;

      // if !cpu.e && cpu.regs.P.x == 0 {
      //   // 16Bit
      //   // TODO: byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le
      //   number += cpu.regs.X as u16;
      // } else {
      //   // 8Bit
      //   number += cpu.regs.X.low as u32;
      // }
      // return number as usize;
      // }
      AddressModes::AbsoluteLong => {
        let op_low = payload[0];
        let op_high = payload[1];
        let op_bank = payload[2];
        return ((op_bank as u32) << 16 | (op_high as u32) << 8 | op_low as u32)
          .try_into()
          .unwrap();
      }
      AddressModes::Implied => println!("Implied addressing"),
      AddressModes::Immediate => println!("Immediate addressing"), // TODO: Return Payload as slice?
      AddressModes::ProgrammCounterRelative => {
        let offset: i8 = payload[0] as _;
        let foo = offset as i16;
        let address: u32 = (foo as i32 + (cpu.regs.PC as i32)).try_into().unwrap();
        return (((cpu.regs.PBR as u32) << 16) | address) as usize;
      }
      AddressModes::StackPCRelativeLong => {
        let op_low = payload[0];
        let address = cpu.regs.PC + op_low as u16;
        cpu.stack_push((address & 0x00ff) as u8);
        cpu.stack_push(((address & 0xff00) >> 8) as u8);
      }
      // TODO: Should this go to RTS instruction instead?
      AddressModes::StackRTS => {
        let op_low = cpu.stack_pull();
        let op_high = cpu.stack_pull();
        cpu.regs.PC = ((op_high as u16) << 8) | op_low as u16;
      }
      // AddressModes::StackInterrupt => {
      //   // TODO
      // }
      _ => {
        unimplemented!(
          "AddressMode: {:?}, opcpode: {:?}, cpu-regs: {:?}",
          self,
          opcode,
          cpu.regs
        );
      }
    };
    0
  }
}

impl Default for AddressModes {
  fn default() -> AddressModes {
    AddressModes::Unknown
  }
}

pub fn get_gii_reg_load_addr_mode(opcode: u8) -> Option<AddressModes> {
  let mask = opcode & GII_MASK;
  let g2_mask = opcode & !GII_MASK;
  match mask {
    G2_REGLOAD_ADDR_MODE_IMMEDIATE => Some(AddressModes::Immediate),
    G2_REGLOAD_ADDR_MODE_ABSOLUTE => Some(AddressModes::Absolute),
    G2_REGLOAD_ADDR_MODE_DIRECT_PAGE => Some(AddressModes::DirectPage),
    G2_REGLOAD_ADDR_MODE_DIRECT_PAGE_INDEXED => match g2_mask {
      G2_OP_LDX => Some(AddressModes::DirectPageIndexedY),
      G2_OP_LDY => Some(AddressModes::DirectPageIndexedX),
      _ => None,
    },
    G2_REGLOAD_ADDR_MODE_ABSOLUTE_INDEXED => match g2_mask {
      G2_OP_LDX => Some(AddressModes::AbsoluteIndexedY),
      G2_OP_LDY => Some(AddressModes::AbsoluteIndexedX),
      _ => None,
    },
    _ => None,
  }
}

pub fn get_gii_addr_mode(opcode: u8) -> Option<AddressModes> {
  let mask = opcode & GII_MASK;

  match mask {
    G2_ADDR_MODE_ACCUMULATOR => Some(AddressModes::Accumulator),
    G2_ADDR_MODE_ABSOLUTE => Some(AddressModes::Absolute),
    G2_ADDR_MODE_DIRECT_ZERO_PAGE => Some(AddressModes::DirectPage),
    G2_ADDR_MODE_ABSOLUTE_INDEXED_X => Some(AddressModes::AbsoluteIndexedX),
    G2_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some(AddressModes::DirectPageIndexedX),
    _ => None,
  }
}

pub fn get_gi_addr_mode(opcode: u8) -> Option<AddressModes> {
  let mask = opcode & GI_MASK;

  match mask {
    GI_ADDR_MODE_INTERMEDIATE => Some(AddressModes::Immediate), // Add 1 byte if m = 0 (16Bit memory/accumulator)
    GI_ADDR_MODE_DIRECT_ZERO_PAGE => Some(AddressModes::DirectPage),
    GI_ADDR_MODE_ABSOLUTE => Some(AddressModes::Absolute),
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some(AddressModes::DirectPageIndexedX),
    GI_ADDR_MODE_ABSOLUTE_INDEXED_X => Some(AddressModes::AbsoluteIndexedX),
    GI_ADDR_MODE_ABSOLUTE_INDEXED_Y => Some(AddressModes::AbsoluteIndexedY),
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_X => {
      Some(AddressModes::DirectPageIndexedIndirectX)
    }
    GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_Y => {
      Some(AddressModes::DirectPageIndexedIndirectY)
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG_INDEXED_Y => {
      Some(AddressModes::DirectPageIndirectLongIndexedY)
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG => Some(AddressModes::DirectPageIndirectLong),
    GI_ADDR_MODE_ABSOLUTE_LONG => Some(AddressModes::AbsoluteLong),
    GI_ADDR_MODE_ABSOLUTE_LONG_INDEXED_X => Some(AddressModes::AbsoluteLongIndexedX),
    GI_ADDR_MODE_STACK_RELATIVE => Some(AddressModes::StackRelative),
    GI_ADDR_MODE_STACK_RELATIVE_INDIRECT_INDEXED_Y => {
      Some(AddressModes::StackRelativeIndirectIndexedY)
    }
    GI_ADDR_MODE_DIRECT_PAGE_INDIRECT => Some(AddressModes::DirectPageIndirect),
    _ => {
      return None;
    }
  }
}
