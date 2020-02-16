use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};
use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use raw_string::RawStr;

const SMC_OFFSET: usize = 0x200;

#[derive(Debug)]
struct FastRom(bool);

// TODO: Once we know what rom type it is (hi rom, low rom) get if fastrom etc from header

// TODO: This is crap.. I think
#[derive(Debug)]
enum MakeupType {
  HiRom(FastRom),
  LowRom(FastRom),
}

impl MakeupType {
  pub fn from_byte(byte: u8) -> MakeupType {
    let fastrom = byte & 0b00110000 == 0b00110000;

    if byte & 0b00000001 == 1 {
      MakeupType::HiRom(FastRom(fastrom))
    } else {
      MakeupType::LowRom(FastRom(fastrom))
    }

    // match byte {
    //   (byte & 0b00000001 == true) => {
    //     if byte & 0b00110000 {
    //       return HiRom(FastRom(true));
    //     } else {
    //       return LowRom(FastRom(true));
    //     }
    //   }
    //   _ => {}
    // }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RomTypes {
  LowRom,
  HiRom,
}

// TODO: Use byte_struct here and also padding
#[derive(ByteStruct, Copy, Clone, Default)]
#[byte_struct_le]
pub struct SnesHeader {
  title: [u8; 21],
  rom_makeup: u8,
  rom_type: u8,
  rom_size: u8,
  sram_size: u8,
  creator_id: u16,
  version: u8,
  checksum_compl: u16,
  checksum: u16,
  _padding: [u8; 4],
  // 4 bytes 0xff
  // Native Mode Vectors
  native_cop: u16,   // unused (0x97, 0xff)
  native_brk: u16,   // (0x97, 0xff) ??
  native_abort: u16, // (0x97, 0xff) ??
  native_nmi: u16,   // 0x9b, 0xff
  native_reset: u16, // 0x90, 0xff
  native_irq: u16,   // 0x97, 0xff
  _padding2: [u8; 4],
  // Emulation Mode Vectors
  // 4 bytes 0xff
  emu_cop: u16, // (0x97, 0xff)
  // emu_brk: u16, // (0x97, 0xff) // fixed here: this is wrong at this side (https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map#Interrupt_vectors)
  _padding3: u16,
  emu_abort: u16,   // (0x97, 0xff)
  emu_nmi: u16,     // (0x9b, 0xff)
  pub emu_res: u16, // 0x98, 0xff [ Offset 0x8000  => 0x7f90 instead of ff90 International Superstar Soccer ]
  emu_irq: u16,     // (0x97, 0xff) // NOTE: emu_brk is same as this
}

impl fmt::Debug for SnesHeader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let makeup = MakeupType::from_byte(self.rom_makeup);
    // MakeupType::from_byte(0b00000000);

    let string = raw_string::RawStr::from_bytes(&self.title)
      .to_str()
      .unwrap();
    write!(
      f,
      "SnesHeader {{
       title: {}
       reset_vector: {:x} makeup_type: {:?} rom_type: {:x} rom_size: {:} bytes sram_size: {:} creator: {:x},
       version: {:x} checksum_comp: {:x} checksum: {:x},
       padding: {:?}
       native_cop: {:x}  native_brk: {:x} native_abort: {:x} native_nmi: {:x} native_reset: {:x} native_irq: {:x}
       padding: {:?}
       emu_cop: {:x} emu_abort: {:x} emu_nmi: {:x} emu_reset: {:x} emu_irq: {:x}
      }}",
      string,
      self.emu_res,
      makeup,
      self.rom_type,
      0x400 << self.rom_size,
      self.sram_size,
      self.creator_id,
      self.version,
      self.checksum_compl,
      self.checksum,
      self._padding,
      self.native_cop,
      self.native_brk,
      self.native_abort,
      self.native_nmi,
      self.native_reset,
      self.native_irq,
      self._padding2,
      self.emu_cop,
      self.emu_abort,
      self.emu_nmi,
      self.emu_res,
      self.emu_irq
    )
  }
}

#[derive(Debug, Clone)]
pub struct Cartridge {
  pub rom: Vec<u8>,
  pub header: SnesHeader,
  pub rom_type: Option<RomTypes>,
  pub size: usize,
}

impl Cartridge {
  pub fn get_emu_reset_vector(&self) -> u16 {
    self.header.emu_res
  }
  pub fn load_rom(path: &Path) -> Cartridge {
    let mut file = File::open(path).unwrap();
    let mut rom = Vec::new();
    let size = file.read_to_end(&mut rom).unwrap();

    let hi_rom = 0xffc0;
    let low_rom = 0x7fc0;

    // TODO: Check header for correctness at both addresses

    let mut card = Cartridge {
      rom,
      size,
      rom_type: None,
      header: SnesHeader::default(),
    };

    if let Some(header) = card.load_header(hi_rom) {
      println!("Hi Rom Detected");
      card.header = header;
      card.rom_type = Some(RomTypes::HiRom);
    } else if let Some(header) = card.load_header(low_rom) {
      println!("Low Rom Detected");
      card.header = header;
      card.rom_type = Some(RomTypes::LowRom);
    } else {
      println!("No header found");
    }
    card
  }

  fn load_header(&self, address: usize) -> Option<SnesHeader> {
    let mut header = self.read_bytes(address, 0x40);
    let raw = header.as_mut_slice();
    let snes_header = SnesHeader::read_bytes(&raw[..]);

    // TODO: This looks awfull
    let mut i = 0;
    for b in &raw[..] {
      i += 1;
      if *b > 0x1f && *b <= 0x7f && i <= 21 {
        println!("#====> Snes Title is all asci at: {}", i);
      } else if i <= 21 {
        println!("Error! {} is not ascii", *b);
        return None;
      }
    }

    if raw_string::RawStr::from_bytes(&snes_header.title).is_ascii() {
      println!("String is ascii");
    } else {
      println!("title no ascii");
    }

    // TODO: Find more critieria to check if loaded header is valid
    if snes_header.rom_size == 0 {
      println!("Invailid rom_size");
      return None;
    }

    Some(snes_header)
  }

  pub fn read_byte(&self, address: usize) -> u8 {
    self.rom[address]
  }

  pub fn read_u16(&self, address: usize) -> u16 {
    let ret = self.read_bytes_reverse(address, 2);
    ret[1] as u16 | ((ret[0] as u16) << 8)
  }

  pub fn read_bytes(&self, address: usize, length: usize) -> Vec<u8> {
    let mut ret = Vec::with_capacity(length);
    if self.rom_type == Some(RomTypes::LowRom) {
      for i in 0..length {
        // Is this correct?
        ret.push(self.read_byte((address ^ 0x8000) + i));
      }
    } else {
      for i in 0..length {
        ret.push(self.read_byte(address + i));
      }
    }
    ret
  }
  pub fn read_bytes_reverse(&self, address: usize, length: usize) -> Vec<u8> {
    let mut ret = self.read_bytes(address, length);
    ret.reverse();
    ret
  }
}

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
