#![recursion_limit = "256"]
use superrustendo::ppu::PPU;

use crate::mem::WRAM;
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
pub mod ppu;

use crate::cpu::decoder::*;
use crate::cpu::*;
use crate::mem::Bus;
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
    let mut bus = Bus {
        cartridge: Some(card),
        wram: WRAM::new(),
        // ppu: ppu::PPU::new(),
    };

    let mut decoder = Decoder::new(&mut cpu, &mut bus, true);

    for (i, instr) in decoder.enumerate() {
        instr.print_info();
    }

    Ok(())
}
