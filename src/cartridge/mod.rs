use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const SMC_OFFSET: usize = 0x200;

struct SnesHeader {
  title: [u8; 21],
  rom_makeup: u8,
  rom_type: u8,
  sram_size: u8,
  creator_id: u16,
  version: u8,
  checksum_compl: u16,
  checksum: u16,
  // 4 bytes 0xff
  // Native Mode Vectors
  native_cop: u16,   // unused (0x97, 0xff)
  native_brk: u16,   // (0x97, 0xff) ??
  native_abort: u16, // (0x97, 0xff) ??
  native_nmi: u16,   // 0x9b, 0xff
  native_reset: u16, // 0x90, 0xff
  native_irq: u16,   // 0x97, 0xff
  // Emulation Mode Vectors
  // 4 bytes 0xff
  emu_cop: u16,   // (0x97, 0xff)
  emu_abort: u16, // (0x97, 0xff)
  emu_nmi: u16,   // (0x9b, 0xff)
  emu_res: u16, // 0x98, 0xff [ Offset 0x8000  => 0x7f90 instead of ff90 International Superstar Soccer ]
  emu_brk: u16, // (0x97, 0xff) // The last two point to the same location? (https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map#Interrupt_vectors)
  emu_irq: u16, // (0x97, 0xff)
}

pub struct Cardridge {
  rom: Vec<u8>,
  pub size: usize,
}

impl Cardridge {
  pub fn load_rom(path: &Path) -> Cardridge {
    let mut file = File::open(path).unwrap();
    let mut rom = Vec::new();
    let size = file.read_to_end(&mut rom).unwrap();

    Cardridge { rom, size }
  }

  pub fn read_byte(&self, address: usize) -> u8 {
    self.rom[address]
  }

  pub fn read_u16(&self, address: usize) -> u16 {
    let ret = self.read_bytes(address, 2);
    ret[0] as u16 | ((ret[1] as u16) << 8)
  }

  pub fn read_bytes(&self, address: usize, length: usize) -> Vec<u8> {
    let mut ret = Vec::new();
    for i in 0..length {
      ret.push(self.read_byte(address + i));
    }
    // ret.reverse(); ? bc of little endian
    ret
  }
}

// pub fn read_header(path: &Path) -> SnesHeader {}

// pub fn get_rom_type() {}

pub fn test_for_smc_header(path: &Path) -> Result<bool, &'static str> {
  let mut f = File::open(&path).unwrap();

  let mut buffer = Vec::new();
  let size = f.read_to_end(&mut buffer).unwrap();

  let header = size % 0x400;

  if header == 0 {
    Ok(false)
  } else if header == 512 {
    Ok(true)
  } else {
    Err("Malformed SMC Header or ROM")
  }
}
