#![recursion_limit = "256"]

use crate::mem::WRAM;
use crate::ppu::PPU;

use std::env;

use std::io::{Error, ErrorKind};
use std::path::Path;
pub mod cartridge;
pub mod cpu;
pub mod mem;
pub mod ppu;
pub mod tooling;

use crate::cpu::decoder::*;
use crate::cpu::*;
use crate::mem::Bus;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "Please specify an sfc rom file",
        ));
    }

    let card = cartridge::Cartridge::load_rom(Path::new(&args[1])).expect("Error loading");
    println!("Loaded Cardidge: {:?}", card.header);

    let mut cpu = CPU::new();

    // TODO: Fix address offsets => rom mapping starts at 0x8000.. for bank 00
    let mut bus = Bus {
        cartridge: Some(card),
        wram: WRAM::new(),
        ppu: PPU::new(),
    };

    let decoder = Decoder::new(&mut cpu, &mut bus, true);

    for (_i, instr) in decoder.enumerate() {
        instr.print_info();
    }

    Ok(())
}
