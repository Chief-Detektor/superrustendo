use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub mod cartridge;
pub mod cpu;
pub mod mem;

use crate::cpu::decoder::*;
use crate::cpu::*;
use crate::mem::Mapper;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    return Err(Error::new(
      ErrorKind::Other,
      "Please specify an sfc rom file",
    ));
  }

  let mut card = cartridge::Cartridge::load_rom(Path::new(&args[1]));
  println!("Loaded Cardidge: {:?}", card.header);

  let mut cpu = CPU::new();

  // TODO: Fix address offsets => rom mapping starts at 0x8000.. for bank 00
  // cpu.regs.PC = 0x4;
  let mut mapper = Mapper {
    cartridge: Some(card),
  };

  let mut decoder = Decoder::new(&mut cpu, &mut mapper);

  for i in decoder {
    // println!("{:?}", i);
    i.print();
  }
  Ok(())
}
