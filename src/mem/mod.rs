pub mod wram;

use crate::cartridge::{Cartridge, RomTypes};

#[derive(Debug)]
pub struct Mapper {
  pub cartridge: Option<Cartridge>,
}

impl Mapper {
  pub fn read(&self, address: usize) -> u8 {
    match address {
      0x2100..=0x21ff => {
        println!("=> Access to PPU1, APU, HW-Registers");
        // match address => {
        //   0x2100 =>
        // }
      }
      0x8000..=0xffff => {
        if self.cartridge.as_ref().unwrap().rom_type == Some(RomTypes::LowRom) {
          println!("=> Access to ROM at 0x{:x}", address ^ 0x8000);
          return self.cartridge.as_ref().unwrap().rom[address ^ 0x8000];
        } else {
          println!("=> Access to ROM at 0x{:x}", address);
          return self.cartridge.as_ref().unwrap().rom[address];
        }
      }
      _ => {}
    }
    return 0;
  }
  // TODO: Implement rom type agnostic writes
  pub fn write(&mut self, address: usize, data: u8) {
    match address {
      0x2100 => {
        print!("INIDISP - Screen Display: ");
        let force_blank = (data >> 7) != 0;
        let brightness = data & 0xf;
        println!(
          "force blank: {:?}, brightness: {:x}",
          force_blank, brightness
        );
      }
      0x2101 => {
        print!("OBSEL - Object Size and Character Address: ");
        let object_size = data >> 5;
        let name_select = (data & 0x18) >> 3;
        let name_base_select = data & 0x7;
        println!(
          "object_size: 0x{:x}, name_select: 0x{:x}, name_base_select: 0x{:x}",
          object_size, name_select, name_base_select
        );
      }
      0x2102 => {
        println!("OAMADDL - OAM Address low byte: 0x{:x}", data);
      }
      0x2103 => {
        print!("OAMADDH - OAM Address high bit and Obj Priority: ");

        let p = (data >> 7) != 0;
        let b = (data & 0x1) != 0;

        println!(
          "Obj priority activation bit: {:?}, table selector: {:?}",
          p, b
        );
      }
      _ => unimplemented!("Register {:x}", address),
    }
  }
  pub fn write_u16(&mut self, address: usize, data: u16) {
    // let mut card_ref = self.cartridge.as_ref().unwrap();
    // card_ref.rom[address] = data & 0x00ff;
    // card_ref.rom
    // (data & 0xff00) >> 8;
  }
}
