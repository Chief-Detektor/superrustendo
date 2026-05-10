// TODO: implement stepping debugger
//      1. step by pressing s once or x times (e.g. 5s)
//      2. goto address g #$address
//      3. print cpu/regs/stack by pressing p cpu,stack#index -a#cells_after -b#cells_before

// Non stable features
#![recursion_limit = "256"]

use rustyline;
use rustyline::error::ReadlineError;

use std::env;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::string::String;
use superrustendo::cpu::decoder::Decoder;
use superrustendo::cpu::instructions::Instruction;

use superrustendo::mem::Bus;
use superrustendo::{cartridge::Cartridge};

extern crate hex;

extern crate log;

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "Please specify an sfc rom file",
        ));
    }

    // init stuff
    let card = Cartridge::load_rom(Path::new(&args[1])).expect("Error loading");
    log::info!("Loaded Cartridge: {:?}", card.header);

    // This translates addresses to components or correct memory locations
    let mut bus = Bus::new();
    bus.load_cartridge(card);

    //bus.insert_cartridge(card);
    // pretty self explanatory

    // decoder is an iterator that iterates over the program code
    let mut decoder = Decoder::new(&mut bus, true);

    // the readline handle
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // log::debug!("Line: {:?}", line);
                if !eval_line(line, &mut decoder) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(_) => log::warn!("No input"),
        }
    }

    Ok(())
}

fn print_instruction(inst: Option<Instruction>) {
    let i = inst.unwrap();
    log::info!(
        "{:?}, {:?}, payload: {:?}",
        i.opcode, i.address_mode, i.payload
    );
}

fn eval_line(line: String, decoder: &mut Decoder) -> bool {
    let mut command = line.split_whitespace();
    match command.next() {
        Some("b" | "break") => {
            if let Some(addr) = command.next() {
                let val = hex::decode(addr);
                match val {
                    Ok(v) => {
                        log::info!("Value: {:?} hex: {:x}{:x}", v, v[0], v[1]);
                    }
                    Err(_) => {}
                }
                // log::debug!("Adding breakpoint at {:x}", hex::encode(&addr).parse::<u64>().unwrap());
            }
        }
        Some("s" | "step") => {
            match command.next() {
                Some(steps) => {
                    // step multiple times
                    let s = steps.parse().unwrap();
                    for i in 1..=s {
                        print!("Step {}: ", i);
                        print_instruction(decoder.next());
                    }
                }
                _ => {
                    print_instruction(decoder.next());
                }
            }
            // log::debug!("Step: {:?}", command.next());
        }
        Some("p" | "print") => {
            match command.next() {
                Some(thing) if thing == "cpu" => log::info!("{:?}", decoder.get_bus().get_cpu()),
                _ => {}
            }
            log::debug!("Print: {:?}", command.next());
        }
        Some("q" | "quit") => {
            log::info!("Quit");
            return false;
        }
        Some("g" | "goto") => log::debug!("Goto: {:?}", command.next()),
        Some(unknown) => log::warn!("unknown command, {:?}", unknown),
        None => {}
    }

    return true;
}
