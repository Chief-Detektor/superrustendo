use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod cartridge;
mod cpu;
mod mem;

use crate::cpu::decoder::*;
use crate::cpu::instructions::*;
use crate::cpu::*;
use crate::mem::Mapper;

fn main() -> std::io::Result<()> {
  // TODO: load rom via command line
  let mut card = cartridge::Cartridge::load_rom(Path::new("elix-nu-pal.sfc"));

  println!("Loaded Cardidge: {:?}", card.header);

  let mut reset_vector = card.read_u16(0x7ffc);

  println!("reset vector: {:x}", reset_vector);

  let mut cpu = CPU::new();

  // TODO: Fix address offsets => rom mapping starts at 0x8000.. for bank 00
  // cpu.regs.PC = 0x4;
  let mut mapper = Mapper { cartridge: card };
  // cpu.regs.PC = m[reset_vector as usize].into();
  // let test = m[reset_vector as usize] as u16;

  // cpu.regs.PC = m[reset_vector as usize] as u16;

  // println!("TEST: {:x}", test);

  let mut decoder = Decoder::new(&mut cpu, &mut mapper);

  for i in decoder {
    // println!("{:?}", i);
    i.print();
  }
  Ok(())
}
