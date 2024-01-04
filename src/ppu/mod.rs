use byte_struct::bitfields;
use byte_struct::ByteStructLen;
use byte_struct::ByteStructUnspecifiedByteOrder;

use crate::cpu::address::Address;

mod sprite;

#[derive(Debug)]
struct VRAM {
    mem: [u8; 64 * 1024],
}

impl VRAM {
    fn new() -> Self {
        VRAM {
            mem: [0; 64 * 1024],
        }
    }
}

bitfields!(
  #[derive(PartialEq, Debug, Copy, Clone)]
  pub ColorEntry: u16 {
    red: 5,
    green: 5,
    blue: 5,
    bit_of_confusion: 1 // reading this bit causes PPU2 open bus
}
);

#[derive(Debug)]
struct CGRAM {
    mem: [ColorEntry; 256],
}

impl CGRAM {
    fn new() -> Self {
        CGRAM {
            mem: [ColorEntry {
                red: 0,
                green: 0,
                blue: 0,
                bit_of_confusion: 0,
            }; 256],
        }
    }
}

bitfields!(
#[derive(PartialEq, Debug, Copy, Clone, Default)]
  pub LowOAMLow: u16 {
    obj_h_pos: 8,
    obj_v_pos: 8
}
);

bitfields!(
#[derive(PartialEq, Debug, Copy, Clone, Default)]
  pub LowOAMHigh: u16 {
    name: 9,
    color: 3,
    obj: 2,
    flip: 2
}
);

#[derive(PartialEq, Debug, Copy, Clone, Default)]
struct LowOAMEntry {
    low: LowOAMLow,
    high: LowOAMHigh,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
struct ObjAttribute {
    h_pos_msb: bool,
    size_large_small: bool,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
struct HighOAMEntry {
    mem: [ObjAttribute; 8],
}

#[derive(PartialEq, Debug)]
struct OAM {
    lowoam: [LowOAMEntry; 128],
    hioam: [HighOAMEntry; 16],
}

impl OAM {
    fn new() -> Self {
        OAM {
            lowoam: [LowOAMEntry::default(); 128],
            hioam: [HighOAMEntry::default(); 16],
        }
    }
}

#[derive(Debug)]
pub struct PPU {
    vram: VRAM,
    cgram: CGRAM,
    oam: OAM,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: VRAM::new(),
            cgram: CGRAM::new(),
            oam: OAM::new(),
        }
    }
    pub fn read(&self, _address: Address) -> u8 {
        !unimplemented!();
        0
    }
}
