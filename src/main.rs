use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use std::io::SeekFrom;

mod cartridge;
mod cpu;

// use crate::cpu::constants::GI_MASK;
use crate::cpu::decoder::*;
use crate::cpu::instructions::*;
use crate::cpu::*;

// #[macro_use]
// extern crate bitpat;

fn main() -> std::io::Result<()> {
  let mut file = File::open("test.sfc")?;

  // let mut buf = [0 as u8; 10];
  let mut buf = Vec::new();
  // file.seek(SeekFrom::Start(0))
  let size = file.read_to_end(&mut buf)?;

  // Decoder::decode(buf[3]);
  // let mut decoder = Decoder::new();

  // decoder.read_instructions(&buf);

  // println!("{:?}", decoder);

  // if cartridge::test_for_smc_header(Path::new("International Superstar Soccer Deluxe (U).smc"))
  //   .unwrap()
  // {
  //   println!("SMC Header found!");
  // } else {
  //   println!("No SMC Header");
  // }

  // let mut rom = Vec::new();
  // let mut f = File::open(Path::new("International Superstar Soccer Deluxe (U).smc")).unwrap();
  // f.read_to_end(&mut rom);

  let mut card = cartridge::Cartridge::load_rom(Path::new("elix-nu-pal.sfc"));

  println!("Loaded Cardidge: {:?}", card.header);

  // println!("{:x} at {:x}", card.read_byte(0xffa0), 0xff0a);

  // println!("{:x} at {:x}", card.read_u16(0x7ffc), 0x7ffc);

  let mut reset_vector = card.read_u16(0x7ffc) - 0x8000;

  println!("reset vector: {:x}", reset_vector);

  let mut cpu = CPU::new();

  // TODO: Fix address offsets => rom mapping starts at 0x8000.. for bank 00
  cpu.regs.PC = 0x4;
  // cpu.regs.PC = card.header.

  println!("{:?}", cpu);

  let mut decoder = Decoder::new(&mut cpu, &mut card);

  for i in decoder {
    // println!("{:?}", i);
    i.print();
  }
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // decoder.next().unwrap().print();
  // // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());
  // println!("{:?}", &decoder.next());

  // TODO: give read instructions the bytearray and the reset vector.
  // Also it might be better to store the interrupt vectors on the cpu or the cartridge..

  // decoder.read_instructions(&mut cpu, &mut card.read_bytes(reset_vector as usize, 0x00ff));

  // decoder.printInstructions();

  // println!("CPU {:?}", cpu);

  // println!("{:?}", decoder);

  // Test all opcodes

  // let mut unknown = 0;
  // let mut found_instructions = Vec::new();

  // for i in 0x00..=0xff {
  //   //0xff {
  //   if let Some(foo) = decoder.decode(i) {
  //     println!("{:x}, is {:?}", i, foo);
  //     found_instructions.push(foo.0);
  //   } else {
  //     println!("unkown: {:x}", i);
  //     unknown += 1;
  //   }
  //   // println!("{:x}: {:?}", i, decoder.decode(i));
  // }

  // println!("{:} unknown ops", unknown);
  // println!("found: {:?}", found_instructions);
  // println!(
  //   "first instruction: {:x}",
  //   card.read_byte(reset_vector as usize)
  // );

  //  TODO: Make proper tests
  // INC test
  // println!("{:b}", 0x1a);
  // // assert_eq!(decoder.decode(0x1a), )
  // println!("{:?}", decoder.decode(0x1a));
  // println!("{:?}", decoder.decode(0xee));
  // println!("{:?}", decoder.decode(0xe6));
  // println!("{:?}", decoder.decode(0xfe));
  // println!("{:?}", decoder.decode(0xf6));

  // // DEC
  // println!("{:?}", decoder.decode(0x3a));
  // println!("{:?}", decoder.decode(0xce));
  // println!("{:?}", decoder.decode(0xc6));
  // println!("{:?}", decoder.decode(0xde));
  // println!("{:?}", decoder.decode(0xd6));

  // // STX
  // println!("{:?}", decoder.decode(0x8e));
  // println!("{:?}", decoder.decode(0x86));
  // println!("{:?}", decoder.decode(0x96));

  // println!("{:?}", decoder.decode(0x8c));
  // println!("{:?}", decoder.decode(0x84));
  // println!("{:?}", decoder.decode(0x94));

  // for i in 0..card.size {
  //   println!(
  //     "{:?}: {:x}",
  //     decoder.decode(card.read_byte(i * 8)),
  //     card.read_byte(i * 8)
  //   );
  // }

  // println!(
  //   "{:x}, {:x}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?} ",
  //   decoder.decode(card.read_byte(reset_vector as usize + 0)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 1)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 2)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 3)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 4)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 5)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 6)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 7)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 8)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 9)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 10)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 11)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 12)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 13)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 14)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 15)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 16)),
  //   decoder.decode(card.read_byte(reset_vector as usize + 17))
  // );

  // let mut low = 0xff;
  // let mut high = 0xa0;

  // let foo = rom[0xffa0] | (low << 8);

  // let foo = 0xffa0 -

  // println!("Da address is: {:x}", foo);

  // let mut instruction_fetched = false;
  // let mut bytes_to_read = 0;
  // for (i, byte) in buf.iter().enumerate() {
  //   if !instruction_fetched {
  //     if let Some(foo) = decode_group_I(*byte) {
  //       instruction_fetched = true;
  //       bytes_to_read = foo.2 - 1;
  //       println!("Addr: {:x},{:?}, {:?}, {:}", i, foo.0, foo.1, foo.2);
  //     } else {
  //       println!("Nothing found...");
  //     }
  //   } else {
  //     println!("Will add {:x} at {:}", byte, i);
  //     bytes_to_read -= 1;
  //     if bytes_to_read < 1 {
  //       instruction_fetched = false;
  //     }
  //   }
  //   // println!("{:}:{:x}", i, byte);
  // }

  // println!("Filesize: {:} Bytes", size);

  Ok(())
}
