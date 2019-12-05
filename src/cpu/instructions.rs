use crate::addressmodes::{
  get_gi_addr_mode, get_gii_addr_mode, get_gii_reg_load_addr_mode, AddressModes,
};
use crate::cartridge::Cartridge;
use crate::constants::*;
use crate::decoder::Opcodes;
use crate::Registers;
use crate::CPU;
use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};
use std::convert::TryInto;

#[derive(Debug, Default, Clone)]
pub struct Instruction {
  pub address: u32,
  pub opcode: Opcodes,
  pub address_mode: AddressModes,
  lenght: usize,
  pub payload: Vec<u8>,
  cycles: usize,
  // follow_jumps: bool,
}

impl Instruction {
  pub fn execute(&mut self, mut cpu: &mut CPU, cartridge: &Cartridge) {
    // Get the correct address for instruction
    let effective_address =
      self
        .address_mode
        .get_effective_address(&mut cpu, &self.payload, &self.opcode);

    println!("Calculated effective address: {:x}", effective_address);
    match &self.opcode {
      Opcodes::BRK => {
        cpu.regs.PC += 2;
      }
      Opcodes::SEI => {
        cpu.regs.P.i = 1;
      }
      Opcodes::CLC => {
        cpu.regs.P.c = 0;
      }
      Opcodes::XCE => {
        // Exchange carry with phantom emulation flag
        // TODO: Reset programm bank register
        let temp = cpu.e;
        cpu.e = cpu.regs.P.c != 0;
        cpu.regs.P.c = temp as _;
      }
      Opcodes::REP => {
        // cpu.regs.P.x = 1;
        // println!("REP before {:?}", cpu.regs.P);
        // println!("Payload: {:b}", self.payload[0]);
        let mut next = [0x00];
        // byte_struct::ByteStructUnspecifiedByteOrder::write_bytes_default_le(&cpu.regs.P, &mut next);
        cpu.regs.P.write_bytes_default_le(&mut next);
        let foo = next[0] & !self.payload[0]; // Clear bits
        cpu.regs.P = byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[foo]);
        // println!("REP1: {:?}", cpu.regs);
        // println!("REP after {:?}", cpu.regs.P);
      }
      Opcodes::LDX => {
        if cpu.regs.P.x != 1 {
          let load_address = self.payload[1] as u16 | (self.payload[0] as u16) << 8;

          let val = cartridge.read_u16(load_address.try_into().unwrap());

          // Set cpu flags accordingly
          if val == 0 {
            cpu.regs.P.z = 1;
          } else {
            cpu.regs.P.z = 0;
          }

          if (val >> 7) == 1 {
            cpu.regs.P.n = 1;
          } else {
            cpu.regs.P.n = 0;
          }

          // TODO: Verify if this is correct
          cpu.regs.X = byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[
            (val >> 8) as u8,
            (val & 0xff00) as u8,
          ]);
        } else {
          let load_address = self.payload[0];
          let val = cartridge.read_byte(load_address.try_into().unwrap());

          // Set cpu flags accordingly
          if val == 0 {
            cpu.regs.P.z = 1;
          } else {
            cpu.regs.P.z = 0;
          }

          if (val >> 7) == 1 {
            cpu.regs.P.n = 1;
          } else {
            cpu.regs.P.n = 0;
          }

          // cpu.regs.X.low = val.into();

          cpu.regs.X.low =
            byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[val]);
        }
        // println!("{:?}", cpu.regs.C);
      }
      Opcodes::TXS => {
        if cpu.e {
          // println!("TXS emu");
          cpu.regs.S.high = 1; // High byte stack pointer is alsways 1
          if cpu.regs.P.x != 1 {
            // println!("16Bit index");
            cpu.regs.S.low = cpu.regs.X.low;
          } else {
            cpu.regs.S.low = cpu.regs.X.low;
            // println!("8Bit index");
          }
        } else {
          // println!("TXS native");
          if cpu.regs.P.x != 1 {
            // println!("16Bit index");
            cpu.regs.S.high = cpu.regs.X.high;
            cpu.regs.S.low = cpu.regs.X.low;
          } else {
            // println!("8Bit index");
            cpu.regs.S.high = 0;
            cpu.regs.S.low = cpu.regs.X.low;
          }
        }
        // println!("TXS: cpu {:?}", cpu.regs);
      }
      // TODO: implement Stack
      Opcodes::JSR => {
        // println!("Going to jump, yo!");
        // println!("JSR: CPU {:?}", cpu);

        let pc_low = (cpu.regs.PC & 0x00ff) as u8;
        let pc_high = (cpu.regs.PC >> 8) as u8;

        // cpu.stack_push(pc_high);
        // cpu.stack_push(pc_low);

        let address =
          ((cpu.regs.PBR as u32) << 16 | (self.payload[1] as u32) << 8 | self.payload[0] as u32)
            - 0x8000;
        println!("Jump to: {:x}", address);
        // panic!("FUUUUUUU");
        cpu.regs.PC = address.try_into().unwrap();
        // println!("JSR CPU: {:?} ", cpu);
      }
      // Opcodes::ORA => {
      //   if cpu.regs.P.m != 1 {
      //     println!("{:?}", self.payload);
      //     //16 bit

      //     let mut foo = [0x00, 0x00];
      //     let mut address =
      //       self.payload[0] as u32 | (self.payload[1] as u32) << 8 | (self.payload[2] as u32) << 16;
      //     cpu.regs.C.write_bytes_default_le(&mut foo);

      //     // address += foo[1] as u32 | (foo[0] as u32) << 8 as u32;
      //     address += 255;

      //     println!(
      //       "Addresss to load: {:x}, {:x}, {:x}",
      //       address, foo[0], foo[1]
      //     );
      //   } else {
      //     // 8 bit
      //     println!("8Bit ORA");
      //   }
      // }
      _ => {
        unimplemented!(
          "{:?} {:?} payload: {:?}",
          &self.opcode,
          &self.address_mode,
          &self.payload,
        );
      }
    }
  }

  pub fn print(&self) {
    println!(
      "0x{:x}: {:?} {:?} {:?}",
      self.address, self.opcode, self.payload, self.address_mode
    );
  }
}

// impl std::fmt::LowerHex for std::vec::Vec<u8> {}
