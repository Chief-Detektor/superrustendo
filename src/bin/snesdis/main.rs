use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind};
use std::path::Path;

use superrustendo::cpu::decoder::*;
use superrustendo::cpu::*;
use superrustendo::mem::Mapper;
use superrustendo::tooling::disassembler::PrintToken;
use superrustendo::{cartridge::Cartridge, mem::WRAM};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "Please specify an sfc rom file",
        ));
    }

    let card = Cartridge::load_rom(Path::new(&args[1])).expect("Error loading");
    println!("Loaded Cardidge: {:?}", card.header);

    let mut cpu = CPU::new();

    // TODO: Fix address offsets => rom mapping starts at 0x8000.. for bank 00
    let mut mapper = Mapper {
        cartridge: Some(card),
        wram: WRAM::new(),
    };

    let decoder = Decoder::new(&mut cpu, &mut mapper, true);

    let mut labels = HashMap::new();
    let mut decoded_asm = Vec::new();

    for (i, instr) in decoder.enumerate() {
        instr.print_info();
        decoded_asm.push((instr.address, instr.print(&mut labels)));
        if i == 125 {
            break;
        }
    }

    println!();
    println!("Dissassembled code:");

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
        println!("{:x}:{:x}:\t{}", cpu.get_regs().get_PBR(), address, line);
    }

    Ok(())
}
