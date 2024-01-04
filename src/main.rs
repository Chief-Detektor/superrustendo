#![recursion_limit = "256"]

use std::{env, thread};

use std::io::{Error, ErrorKind};
use std::path::Path;
pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod mem;
pub mod ppu;
pub mod tooling;

use crate::cpu::decoder::*;
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

    let mut bus = Bus::new();

    bus.load_cartridge(card);
    let decoder = Decoder::new(&mut bus, true);

    for (_i, instr) in decoder.enumerate() {
        instr.print_info();
    }

    Ok(())
}
