use byte_struct::{ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};
use std::error::Error;

use std::fmt;
use std::fs::File;

use std::io::prelude::*;
use std::path::Path;

use crate::cpu::address::Address;

const SMC_OFFSET: usize = 0x200;

#[derive(Debug)]
struct FastRom(bool);

// TODO: Once we know what rom type it is (hi rom, low rom) get if fastrom etc from header

// TODO: This is crap.. I think
#[derive(Debug)]
enum MakeupType {
    HiRom(FastRom),
    LowRom(FastRom),
    SA1Rom,
    ExLoRom,
    ExHiRom,
    Unknown(u8),
}

impl MakeupType {
    pub fn from_byte(byte: u8) -> Result<MakeupType, String> {
        match byte {
            0b00100000 => Ok(MakeupType::LowRom(FastRom(false))),
            0b00100001 => Ok(MakeupType::HiRom(FastRom(false))),
            0b00100011 => Ok(MakeupType::SA1Rom),
            0b00110000 => Ok(MakeupType::LowRom(FastRom(true))),
            0b00110001 => Ok(MakeupType::HiRom(FastRom(true))),
            0b00110010 => Ok(MakeupType::ExLoRom),
            0b00110101 => Ok(MakeupType::ExHiRom),
            _ => {
                // Err(format!("Unknown Makeup type: {:b}", byte))
                Ok(MakeupType::Unknown(byte))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RomTypes {
    LowRom,
    HiRom, // ,
           // HiRomFast,
           // LowRomFast,
           // SA1Rom,
           // ExLoRom,
           // ExHiRom
}

// TODO: Use byte_struct here and also padding
// TODO: Evaluate this..
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
    native_cop: u16,     // unused (0x97, 0xff)
    native_brk: u16,     // (0x97, 0xff) ??
    native_abort: u16,   // (0x97, 0xff) ??
    native_nmi: u16,     // 0x9b, 0xff
    native_reset: u16,   // 0x90, 0xff
    pub native_irq: u16, // 0x97, 0xff
    _padding2: [u8; 4],
    // Emulation Mode Vectors
    // 4 bytes 0xff
    emu_cop: u16, // (0x97, 0xff)
    // emu_brk: u16, // (0x97, 0xff) // fixed here: this is wrong at this side (https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map#Interrupt_vectors)
    _padding3: u16,
    emu_abort: u16,   // (0x97, 0xff)
    emu_nmi: u16,     // (0x9b, 0xff)
    pub emu_res: u16, // 0x98, 0xff [ Offset 0x8000  => 0x7f90 instead of ff90 International Superstar Soccer ]
    pub emu_irq: u16, // (0x97, 0xff) // NOTE: emu_brk is same as this
}

impl fmt::Debug for SnesHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let makeup;
        if let Ok(makeup_type) = MakeupType::from_byte(self.rom_makeup) {
            makeup = makeup_type;
        } else {
            // makeup = MakeupType::SA1Rom;//? What is 1001001?
            unreachable!();
        }

        let string = raw_string::RawStr::from_bytes(&self.title)
            .to_str()
            .unwrap_or("");
        write!(
            f,
            r#"SnesHeader {{
       title: {}
       reset_vector: {:x} makeup_type: {:?} rom_type: {:x} rom_size: {:} bytes sram_size: {:} creator: {:x},
       version: {:x} checksum_comp: {:x} checksum: {:x},
       padding: {:?}
       native_cop: {:x}  native_brk: {:x} native_abort: {:x} native_nmi: {:x} native_reset: {:x} native_irq: {:x}
       padding: {:?}
       emu_cop: {:x} emu_abort: {:x} emu_nmi: {:x} emu_reset: {:x} emu_irq: {:x}
      }}"#,
            string,
            self.emu_res,
            makeup,
            self.rom_type,
            self.rom_size,
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
    // This function is an rust port of cpp code from the higan emulator by byuu et al. to be found at
    // "https://github.com/higan-emu/higan/blob/4fa09731307bb3fb47627e3e0154225661b168f9/icarus/cartridge/super-famicom.cpp"
    // This also implies that the licence of this project has to be GLPv3
    fn scoreHeader(&self, address: usize) -> i32 {
        let mut score: i32 = 0;

        if self.rom.len() < address + 0x50 {
            return score;
        }

        let map_mode = self.rom[address + 0x25] & !0x10;
        let complement = (self.rom[address + 0x2c] as u16) << 0
            | ((self.rom[address + 0x2d] as u16) << 8) as u16;
        let checksum = (self.rom[address + 0x2e] as u16) << 0
            | ((self.rom[address + 0x2f] as u16) << 8) as u16;
        let reset_vector = (self.rom[address + 0x4c] as u16) << 0
            | ((self.rom[address + 0x4d] as u16) << 8) as u16;

        if reset_vector < 0x8000 {
            return score;
        }

        let opcode = self.rom[(address & !0x7FFF) | (reset_vector & 0x7FFF) as usize];

        //most likely opcodes
        if opcode == 0x78 // sei
        || opcode == 0x18  //clc (clc; xce)
        || opcode == 0x38  //sec (sec; xce)
        || opcode == 0x9c  //stz $nnnn (stz $4200)
        || opcode == 0x4c  //jmp $nnnn
        || opcode == 0x5c
        {
            score += 8;
        }

        //plausible opcodes
        if opcode == 0xc2  //rep #$nn
        || opcode == 0xe2  //sep #$nn
        || opcode == 0xad  //lda $nnnn
        || opcode == 0xae  //ldx $nnnn
        || opcode == 0xac  //ldy $nnnn
        || opcode == 0xaf  //lda $nnnnnn
        || opcode == 0xa9  //lda #$nn
        || opcode == 0xa2  //ldx #$nn
        || opcode == 0xa0  //ldy #$nn
        || opcode == 0x20  //jsr $nnnn
        || opcode == 0x22
        //jsl $nnnnnn
        {
            score += 4;
        }

        //implausible opcodes
        if opcode == 0x40  //rti
        || opcode == 0x60  //rts
        || opcode == 0x6b  //rtl
        || opcode == 0xcd  //cmp $nnnn
        || opcode == 0xec  //cpx $nnnn
        || opcode == 0xcc
        //cpy $nnnn
        {
            score -= 4;
        }

        //least likely opcodes
        if opcode == 0x00  //brk #$nn
        || opcode == 0x02  //cop #$nn
        || opcode == 0xdb  //stp
        || opcode == 0x42  //wdm
        || opcode == 0xff
        //sbc $nnnnnn,x
        {
            score -= 8;
        }

        if (checksum as u32) + (complement as u32) == 0xFFFF {
            score += 4;
        }

        if address == 0x7fb0 && map_mode == 0x20 {
            score += 2;
        }
        if address == 0xffb0 && map_mode == 0x21 {
            score += 2;
        }

        return score.max(0);
    }

    fn heuristics(&mut self) -> Option<Address> {
        if self.size < 0x8000 {
            return None;
        }

        let LoRom = self.scoreHeader(0x7FB0);
        let HiRom = self.scoreHeader(0xFFB0);
        let mut ExLoRom = self.scoreHeader(0x407FB0);
        let mut ExHiRom = self.scoreHeader(0x40FFB0);
        if ExLoRom > 0 {
            ExLoRom += 4;
        }
        if ExHiRom > 0 {
            ExHiRom += 4;
        }

        let header_address: u32;

        if LoRom >= HiRom && LoRom >= ExLoRom && LoRom >= ExHiRom {
            header_address = 0x7FB0;
            self.rom_type = Some(RomTypes::LowRom);
        } else if HiRom >= ExLoRom && HiRom >= ExHiRom {
            self.rom_type = Some(RomTypes::HiRom);
            header_address = 0xFFB0;
        } else if ExLoRom >= ExHiRom {
            // TODO: Verify this (add more Types?)
            self.rom_type = Some(RomTypes::LowRom);
            header_address = 0x407FB0;
        } else {
            // TODO: Verify this
            self.rom_type = Some(RomTypes::HiRom);
            header_address = 0x40FFB0;
        }
        Some(Address {
            bank: (header_address >> 16) as u8,
            address: (header_address & 0x00FFFF) as u16,
        })
    }

    pub fn get_emu_reset_vector(&self) -> u16 {
        self.header.emu_res
    }
    pub fn load_rom(path: &Path) -> Result<Cartridge, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut rom = Vec::new();
        let size = file.read_to_end(&mut rom)?;

        // let hi_rom = 0xffc0;
        // let low_rom = 0x7fc0;

        // TODO: Check header for correctness at both addresses

        let mut card = Cartridge {
            rom,
            size,
            rom_type: None,
            header: SnesHeader::default(),
        };

        if let Some(header_address) = card.heuristics() {
            println!("{:?}", header_address);
            if let Some(header) = card
                .load_header(header_address.address as usize | (header_address.bank as usize) << 16)
            {
                card.header = header;
                // card.header.emu_res =
                return Ok(card);
            }
        }
        return Err("No Header found!".into());
    }

    fn load_header(&self, address: usize) -> Option<SnesHeader> {
        // let foo = self.heuristics();
        // TODO: Find out why I had to add this offset? The addresses come form higan's scoring heuristics
        let mut header = self.read_bytes(address + 0x10, 0x40);
        let raw = header.as_mut_slice();
        let snes_header = SnesHeader::read_bytes(&raw[..]);

        // println!("{:?}", snes_header);

        // TODO: This looks awfull
        let mut i = 0;
        for b in &raw[..] {
            i += 1;
            if *b > 0x1f && *b <= 0x7f && i <= 21 {
                println!("#====> Snes Title is all asci at: {}, {}", i, *b);
            } else if i <= 21 {
                println!("Error! {} is not ascii", *b);
                if snes_header.emu_res < 0x8000 {
                    return Some(snes_header);
                } else {
                    return None;
                }
            }
        }

        // if raw_string::RawStr::from_bytes(&snes_header.title).is_ascii() {
        //   println!("String is ascii");
        // } else {
        //   println!("title no ascii");
        // }

        // TODO: Find more critieria to check if loaded header is valid
        // if snes_header.rom_size == 0 {
        //     println!("Invailid rom_size");
        //     return None;
        // }

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
        // if self.rom_type == Some(RomTypes::LowRom) {
        //     for i in 0..length {
        //         // Is this correct?
        //         ret.push(self.read_byte((address % 0x8000) + i));
        //     }
        // } else {
        for i in 0..length {
            ret.push(self.read_byte(address + i));
        }
        // }
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
