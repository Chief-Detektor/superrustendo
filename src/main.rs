use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;
pub mod cartridge;
pub mod cpu;
pub mod mem;
pub mod tooling;

use crate::cpu::decoder::*;
use crate::cpu::*;
use crate::mem::Mapper;
use crate::tooling::disassembler::PrintToken;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    return Err(Error::new(
      ErrorKind::Other,
      "Please specify an sfc rom file",
    ));
  }

  let mut card = cartridge::Cartridge::load_rom(Path::new(&args[1])).expect("Error loading");
  println!("Loaded Cardidge: {:?}", card.header);

  let mut cpu = CPU::new();

  // TODO: Fix address offsets => rom mapping starts at 0x8000.. for bank 00
  // cpu.regs.PC = 0x4;
  let mut mapper = Mapper {
    cartridge: Some(card),
  };

  let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

  let mut labels = HashMap::new();
  let mut decoded_asm = Vec::new();

  for (i, instr) in decoder.enumerate() {
    instr.print_info();
    decoded_asm.push((instr.address, instr.print(&mut labels)));
    if i == 200 {
      break;
    }
  }

  println!();
  println!("Dissassembled code:");

  // for (address, line) in decoded_asm.iter_mut() {
  //   if labels.contains_key(&(*address as usize)) {
  //     let label = labels.get(&(*address as usize)).unwrap();
  //     *line = format!("{}:\n {}", label, line);
  //     // line = line + labels.get(&(*address as usize));
  //   }
  //   // Don't print, yet
  //   // println!("{:#x}: {}", address, line);
  // }

  println!("Labels:");
  for (k, l) in &labels {
    println!("At {:0x}: {}", k, l);
  }

  for (address, line) in decoded_asm {
    // labels
    if labels.contains_key(&(address as usize)) {
      let label = labels.get(&(address as usize)).unwrap();
      println!("{}:", label);
    }
    println!("\t{:#x}:\t{}", address, line);
  }

  Ok(())
}
