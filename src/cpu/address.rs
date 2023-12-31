use std::fmt;

use crate::{cartridge::RomTypes, mem::Bus};

#[derive(Clone, Copy)]
pub struct Address {
    pub bank: u8,
    pub address: u16,
}

impl Address {
    pub fn new(address: u32) -> Self {
        let bank = (address >> 16) as u8;
        let address = address as u16;
        Address { bank, address }
    }
    pub fn add(&self, offset: usize) -> Address {
        Address {
            bank: self.bank,
            // TODO: wrapping add?
            address: self.address.wrapping_add(offset as u16),
        }
    }

    fn mirror_lowrom(&self) -> Address {
        match self.bank {
            0x00..=0x3f => {
                return *self;
            }
            0x40..=0x7f => {
                // Mirroring
                return Address {
                    bank: self.bank ^ 0x40,
                    address: self.address,
                };
            }
            0x80..=0xbf => {
                return Address {
                    bank: self.bank ^ 0x80,
                    address: self.address,
                };
            }
            0xc0..=0xef => {
                return Address {
                    bank: self.bank ^ 0xc0,
                    address: self.address,
                };
            }
            _ => {
                panic!("Invalid bank for rom access");
                return *self;
            }
        }
    }

    pub fn mirror(&self, bus: &Bus) -> Address {
        if let Some(rom_type) = bus.get_rom_type() {
            match rom_type {
                RomTypes::LowRom => self.mirror_lowrom(),
                _ => unimplemented!("Mirror not implemented for rom type {:?}", rom_type),
            }
        } else {
            panic!("No rom type specified");
        }

        //        Address {
        //            bank: self.bank ^ mirror,
        //            address: self.address,
        //        }
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address {{ bank: {:x}, address: {:x} }}",
            self.bank, self.address
        )
    }
}

impl From<Address> for u16 {
    fn from(a: Address) -> Self {
        a.address
    }
}

impl From<Address> for usize {
    fn from(a: Address) -> Self {
        (a.bank as usize) << 16 | a.address as usize
    }
}
