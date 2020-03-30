use crate::cpu::addressmodes::AddressModes;
use crate::cpu::decoder::Opcodes;
use crate::cpu::CPU;
// use crate::cpu::
use crate::cpu::{Accumulator, IndexRegister, Registers, StatusRegister};
use crate::mem::Mapper;

use std::convert::TryInto;

#[derive(Debug, Default, Clone)]
pub struct Instruction {
  pub address: u32,
  pub opcode: Opcodes,
  pub address_mode: AddressModes,
  pub(crate) length: usize,
  pub payload: Vec<u8>,
  cycles: usize,
  follow_jumps: bool,
}

impl Instruction {
  pub fn new(follow_jumps: bool) -> Instruction {
    let mut inst = Instruction::default();
    inst.follow_jumps = follow_jumps;
    inst
  }
  // pub fn new(opcode: u8) -> Instruction {}
  pub fn execute(&mut self, mut cpu: &mut CPU, mapper: &mut Mapper) {
    // Get the correct address for instruction
    let effective_address =
      self
        .address_mode
        .get_effective_address(&mut cpu, &self.payload, &self.opcode, &mapper);

    println!("Calculated effective address: {:?}", effective_address);
    match &self.opcode {
      Opcodes::BRK => {
        // cpu.regs.PC += 2;
        // TODO: Eval this..
        cpu.regs.PC = effective_address.unwrap().address;
      }
      Opcodes::CLD => cpu.regs.P.d = 0,
      Opcodes::SEI => {
        cpu.regs.P.i = 1;
      }
      Opcodes::CLC => {
        cpu.regs.P.c = 0;
      }
      Opcodes::CPX => {
        // 8 Bit registers
        if cpu.regs.P.x == 1 {
          let val;
          if self.address_mode != AddressModes::Immediate {
            val = mapper.read(effective_address.unwrap());
          } else {
            val = self.payload[0];
          }
          let bar = cpu.regs.X.low as u8 - val;
          if bar >> 7 == 1 {
            cpu.regs.P.n = 1;
          } else {
            cpu.regs.P.n = 0;
          }
          if bar == 0 {
            cpu.regs.P.z = 1;
          } else {
            cpu.regs.P.z = 0;
          }
          // TODO: double check this.
          if cpu.regs.X.low as u8 >= bar {
            cpu.regs.P.c = 1;
          } else {
            cpu.regs.P.c = 0;
          }
        } else {
          let val;
          if self.address_mode != AddressModes::Immediate {
            val = mapper.read(effective_address.unwrap()) as u16
              | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
          } else {
            val = self.payload[1] as u16 | ((self.payload[0] as u16) << 8);
          }
          let bar = <u16>::from(cpu.regs.X) - val;
          if bar >> 15 == 1 {
            cpu.regs.P.n = 1;
          } else {
            cpu.regs.P.n = 0;
          }
          if bar == 0 {
            cpu.regs.P.z = 1;
          } else {
            cpu.regs.P.z = 0;
          }
          // TODO: double check this.
          if <u16>::from(cpu.regs.X) >= bar {
            cpu.regs.P.c = 1;
          } else {
            cpu.regs.P.c = 0;
          }
        }
      }
      Opcodes::XCE => {
        // Exchange carry with phantom emulation flag
        // TODO: Reset programm bank register
        let temp = cpu.e;
        cpu.e = cpu.regs.P.c != 0;
        cpu.regs.P.c = temp as _;
      }
      Opcodes::SEP => {
        // Set Status Bits
        let tmp = <u8>::from(cpu.regs.P);
        let next = tmp | self.payload[0]; // Set bits
        cpu.regs.P = StatusRegister::from(next);
      }
      Opcodes::REP => {
        // Reset Status Bits
        let tmp = <u8>::from(cpu.regs.P);
        let next = tmp & !self.payload[0]; // Clear bits
        cpu.regs.P = StatusRegister::from(next);
      }
      Opcodes::PEA => {
        let low = self.payload[0];
        let high = self.payload[1];
        cpu.stack_push(high);
        cpu.stack_push(low);
      }
      Opcodes::PHB => cpu.stack_push(cpu.regs.DBR),
      Opcodes::LDX => {
        if cpu.regs.P.x != 1 {
          // TODO: use effective_address here
          let load_address = self.payload[1] as u16 | (self.payload[0] as u16) << 8;

          let mut val = 0;
          if self.address_mode == AddressModes::Immediate {
            val = load_address;
          } else {
            val = mapper
              .cartridge
              .as_ref()
              .unwrap()
              .read_u16(load_address.try_into().unwrap());
          }
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

          cpu.regs.X = IndexRegister::from(val);
        } else {
          let load_address = self.payload[0];

          let val;
          if self.address_mode == AddressModes::Immediate {
            val = load_address;
          } else {
            val = mapper
              .cartridge
              .as_ref()
              .unwrap()
              .read_byte(load_address.try_into().unwrap());
          }
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
          cpu.regs.X = IndexRegister::from(val);
        }
      }
      Opcodes::TXS => {
        if cpu.e {
          // TXS emu
          cpu.regs.S.high = 1; // High byte stack pointer is always 1
          if cpu.regs.P.x != 1 {
            // 16Bit index
            cpu.regs.S.low = cpu.regs.X.low;
          } else {
            cpu.regs.S.low = cpu.regs.X.low;
            // 8Bit index
          }
        } else {
          // TXS native
          if cpu.regs.P.x != 1 {
            // 16Bit index
            cpu.regs.S.high = cpu.regs.X.high;
            cpu.regs.S.low = cpu.regs.X.low;
          } else {
            // 8Bit index
            cpu.regs.S.high = 0;
            cpu.regs.S.low = cpu.regs.X.low;
          }
        }
      }
      Opcodes::JMP => {
        // TODO: At long jumping: Bank Mapping e.g. in HiRom is bank 80 - 9f  = 00 - 1f etc
        //  also do this in AddressMode module
        // TODO: Use Memmaper to handle program/databank register update and returning 16 Bit pc
        if self.follow_jumps {
          cpu.regs.PC = effective_address.unwrap().address;
        }
      }
      Opcodes::JSR => {
        if self.follow_jumps {
          let pc_low = (cpu.regs.PC & 0x00ff) as u8;
          let pc_high = (cpu.regs.PC >> 8) as u8;

          cpu.stack_push(pc_high);
          cpu.stack_push(pc_low);

          // let address = effective_address;
          cpu.regs.PC = effective_address.unwrap().address;
        }
      }
      Opcodes::RTS => {
        let op_low = cpu.stack_pull();
        let op_high = cpu.stack_pull();
        cpu.regs.PC = ((op_high as u16) << 8) | op_low as u16;
      }
      Opcodes::LDA => {
        if cpu.regs.P.m != 1 {
          // println!("### 16 Bit accumulator");
          let val = (self.payload[1] as u16) << 8 | self.payload[0] as u16;
          cpu.regs.C = Accumulator::from(val);
        } else {
          // println!("### 8 Bit accumulator");
          let val = self.payload[0] as u16;
          cpu.regs.C = Accumulator::from(val);
        }
        // TODO: Better naming
        let foo = mapper
          .cartridge
          .as_ref()
          .unwrap()
          .read_byte(self.payload[0] as _);
        // println!("### Yo check da LDA, Bro: {:?}", foo);
      }
      Opcodes::STA => {
        // println!("STA ====>{:?}", self.payload);
        if cpu.regs.P.m == 1
        /*&& cpu.e*/
        {
          mapper.write(effective_address.unwrap(), cpu.regs.C.A.try_into().unwrap());
        } else {
          mapper.write(effective_address.unwrap(), cpu.regs.C.A.try_into().unwrap());
          // mapper.write(effective_address + 1, cpu.regs.C.B.try_into().unwrap());
          // mapper.write_u16(effective_address, cpu.regs.C.try_into().unwrap());
        }
      }
      Opcodes::STZ => {
        mapper.write(effective_address.unwrap(), 0x0);
      }
      Opcodes::STX => {
        if !cpu.e {
          mapper.write(effective_address.unwrap(), cpu.regs.X.low as u8);
        } else if cpu.regs.P.x == 1 {
          mapper.write(effective_address.unwrap(), cpu.regs.X.low as u8);
          mapper.write(effective_address.unwrap().add(1), cpu.regs.X.high as u8);
        }
      }
      Opcodes::TAX => {
        if !cpu.e {
          // native mode
          // 8 Bit accumulator, 8 bit index registers
          cpu.regs.X.low = cpu.regs.C.A;
        } else {
          // 8 bit accumulator, 16 bit index registers
          if cpu.regs.P.m == 1 && cpu.regs.P.x == 0 {
            cpu.regs.X.low = cpu.regs.C.A;
            cpu.regs.X.high = cpu.regs.C.B;
          }
          // 16 bit accumulator, 8 bit index registers
          if cpu.regs.P.m == 0 && cpu.regs.P.x == 1 {
            cpu.regs.X.low = cpu.regs.C.A;
          }
          if cpu.regs.P.m == 0 && cpu.regs.P.x == 0 {
            cpu.regs.X.low = cpu.regs.C.A;
            cpu.regs.X.high = cpu.regs.C.B;
          }
        }
        if (cpu.regs.C.A >> 7) == 1 {
          cpu.regs.P.n = 1;
        } else {
          cpu.regs.P.n = 0;
        }
        if cpu.regs.C.A == 0 {
          cpu.regs.P.z = 1;
        } else {
          cpu.regs.P.z = 0;
        }
      }
      Opcodes::INX => {
        let index: u16 = u16::from(cpu.regs.X) + 1;
        cpu.regs.X = IndexRegister::from(index);
      }
      Opcodes::BNE => {
        if cpu.regs.P.z == 1 {
          return;
        } else {
          cpu.regs.PC = effective_address.unwrap().address as _;
        }
      }
      Opcodes::PHB => {
        cpu.stack_push(cpu.regs.DBR);
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

  pub fn print_info(&self) {
    println!(
      "0x{:x}: {:?} {:?} {:?}",
      self.address, self.opcode, self.payload, self.address_mode
    );
  }
}
