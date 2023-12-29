// TODO: implement stepping debugger
//      1. step by pressing s once or x times (e.g. 5s)
//      2. goto address g #$address
//      3. print cpu/regs/stack by pressing p cpu,stack#index -a#cells_after -b#cells_before

// Non stable features
#![recursion_limit = "256"]

use rustyline;
use rustyline::error::ReadlineError;
use superrustendo::ppu::PPU;

use std::cell::RefCell;
use std::env;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::rc::Rc;
use std::string::String;
use superrustendo::cpu::decoder::Decoder;
use superrustendo::cpu::instructions::Instruction;
use superrustendo::cpu::*;
use superrustendo::mem::Bus;
use superrustendo::{cartridge::Cartridge, mem::WRAM};

extern crate hex;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "Please specify an sfc rom file",
        ));
    }

    // init stuff
    let card = Cartridge::load_rom(Path::new(&args[1])).expect("Error loading");
    println!("Loaded Cardidge: {:?}", card.header);

    // This translates addresses to components or correct memory locations
    let mut bus = Bus {
        cartridge: Some(card),
        wram: Rc::new(RefCell::new(WRAM::new())),
        ppu: PPU::new(),
        cpu: CPU::new(),
        mdr: Rc::new(RefCell::new(0)),
    };

    // pretty self explainatory

    // decoder is an iteratior that iterates over the program code
    let mut decoder = Decoder::new(&mut bus, true);

    // the readline handle
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // println!("Line: {:?}", line);
                if !eval_line(line, &mut decoder) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(_) => println!("No input"),
        }
    }

    Ok(())
}

fn print_instruction(inst: Option<Instruction>) {
    let i = inst.unwrap();
    println!(
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
                        println!("Value: {:?} hex: {:x}{:x}", v, v[0], v[1]);
                    }
                    Err(_) => {}
                }
                // println!("Adding breakpoint at {:x}", hex::encode(&addr).parse::<u64>().unwrap());
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
            // println!("Step: {:?}", command.next());
        }
        Some("p" | "print") => {
            match command.next() {
                Some(thing) if thing == "cpu" => println!("{:?}", decoder.bus.cpu),
                _ => {}
            }
            println!("Print: {:?}", command.next());
        }
        Some("q" | "quit") => {
            println!("Quit");
            return false;
        }
        Some("g" | "goto") => println!("Goto: {:?}", command.next()),
        Some(unknown) => println!("unknown command, {:?}", unknown),
        None => {}
    }

    return true;
}
