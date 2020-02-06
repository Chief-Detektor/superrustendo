pub mod wram;

use crate::cartridge::{Cartridge, RomTypes};
// use std::convert::TryInto;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Mapper {
  pub cartridge: Option<Cartridge>,
}

// impl Mapper {
//   pub fn read_byte(&self, address: usize) -> u8 {
//     self.cartridge.read_byte(address - 0x8000)
//   }

//   pub fn read_u16(&self, address: usize) -> u8 {
//     self.cartridge.read_u16(address - 0x8000)
//   }
// }

impl Index<usize> for Mapper {
  type Output = u8;

  fn index(&self, address: usize) -> &u8 {
    println!("### MemMapper read");
    match address {
      0x2100..=0x21ff => println!("=> Access to PPU1, APU, HW-Registers"),
      0x8000..=0xffff => {
        if self.cartridge.as_ref().unwrap().rom_type == Some(RomTypes::LowRom) {
          println!("=> Access to ROM at 0x{:x}", address - 0x8000);
          // let ret = (address as u8) - 0x8000;
          return &self.cartridge.as_ref().unwrap().rom[address - 0x8000];
        } else {
          println!("=> Access to ROM at 0x{:x}", address);
          // let ret = (address as u8) - 0x8000;
          // return &((self.cartridge.header.emu_res - 0x8000) as u8);
          return &self.cartridge.as_ref().unwrap().rom[address];
        }
      }
      _ => {}
    }
    return &0;
  }
}
