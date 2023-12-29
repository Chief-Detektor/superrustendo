use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::{Cartridge, RomTypes};
use crate::cpu::address::{self, Address};
use crate::cpu::CPU;
use crate::ppu::PPU;

pub struct WRAM {
    lowRam: [u8; 0x2000],  // bank: 0x0-3f (shadowed from 0x7e) 0x0000-0x1fff
    highRam: [u8; 0x6000], // 0x7e
    extendedRam: [u8; 0x10000],
    //    extendedRam_2: [u8; 0x10000],
}

impl WRAM {
    pub fn new() -> Self {
        WRAM {
            lowRam: [0xf; 0x2000],
            highRam: [0; 0x6000],
            extendedRam: [0; 0x10000],
            //            extendedRam_2: [0; 0x10000],
        }
    }
    pub fn is_wram(address: Address) -> bool {
        match address.bank {
            0x00..=0x3f => match address.address {
                0x0000..=0x1fff => {
                    return true;
                }
                _ => {
                    return false;
                }
            },
            0x7e => {
                match address.address {
                    0x0000..=0x1fff => {
                        return true;
                    }
                    0x2000..=0x7fff => {
                        // This xor is needed otherwise access would exceed the memory (highRam
                        // size is 0x6000
                        return true;
                    }
                    0x8000..=0xffff => {
                        return true;
                    }
                }
            }
            0x7f => return true,
            0x80..=0xbf => match address.address {
                0x0000..=0x1fff => {
                    return true;
                }
                _ => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }
    }

    fn write(&mut self, address: Address, byte: u8) {
        match address.bank {
            0x00..=0x3f => match address.address {
                0x0000..=0x1fff => {
                    self.lowRam[address.address as usize] = byte;
                }
                _ => {}
            },
            0x7e => {
                match address.address {
                    0x0000..=0x1fff => {
                        self.lowRam[address.address as usize] = byte;
                    }
                    0x2000..=0x7fff => {
                        // This xor is needed otherwise access would exceed the memory (highRam
                        // size is 0x6000
                        self.highRam[(address.address - 0x2000) as usize] = byte;
                    }
                    0x8000..=0xffff => {
                        self.extendedRam[(address.address) as usize] = byte;
                    }
                }
            }
            0x7f => {
                self.extendedRam[address.address as usize] = byte;
            }
            0x80..=0xbf => match address.address {
                0x0000..=0x1fff => {
                    self.lowRam[address.address as usize] = byte;
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn read(&self, address: Address) -> Option<u8> {
        match address.bank {
            0x00..=0x3f => match address.address {
                0x0000..=0x1fff => {
                    return Some(self.lowRam[address.address as usize]);
                }
                _ => {
                    return None;
                }
            },
            0x7e => {
                match address.address {
                    0x0000..=0x1fff => {
                        return Some(self.lowRam[address.address as usize]);
                    }
                    0x2000..=0x7fff => {
                        // This xor is needed otherwise access would exceed the memory (highRam
                        // size is 0x6000
                        // substract the offset from the address
                        return Some(self.highRam[(address.address - 0x2000) as usize]);
                    }
                    0x8000..=0xffff => {
                        // TODO: is (address ^ 0x8000) needed here?
                        // I assume not because the address is already mapped to 0x8000..0xffff
                        return Some(self.extendedRam[(address.address) as usize]);
                    }
                }
            }
            0x7f => return Some(self.extendedRam[address.address as usize]),
            0x80..=0xbf => match address.address {
                0x0000..=0x1fff => {
                    return Some(self.lowRam[address.address as usize]);
                }
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        }
    }
}
/*

LowRom = Address % 8000 + 8000 * Bank ( + mirror testing)


*/

//#[derive(Debug)]
pub struct Bus {
    pub cartridge: Option<Cartridge>,
    pub wram: Rc<RefCell<WRAM>>,
    pub ppu: PPU,
    pub cpu: CPU,
    pub mdr: Rc<RefCell<u8>>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cartridge: None,
            wram: Rc::new(RefCell::new(WRAM::new())),
            ppu: PPU::new(),
            cpu: CPU::new(),
            mdr: Rc::new(RefCell::new(0)),
        }
    }

    pub fn set_mdr(&self, byte: u8) {
        *self.mdr.borrow_mut() = byte;
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }

    pub fn get_rom_type(&self) -> Option<RomTypes> {
        match self.cartridge {
            Some(ref cartridge) => cartridge.rom_type.clone(),
            None => panic!("No cartridge loaded"),
        }
    }

    fn resolve_rom_address(&self, address: Address) -> Option<Address> {
        // TODO: Count cycles for rom access
        // TODO: Hardware registers
        match self.get_rom_type() {
            Some(RomTypes::LowRom) => {
                let bank = address.bank;
                // Why is this ^ 0x8000 needed?
                // NOTE: This is because the rom is mapped to 0x8000..0xffff. only half of the
                // address space is available. (Address line A15 snes is not connected - A16 snes is A15 cardridge and so on shifted by one)
                let offset = address.address ^ 0x8000;
                let address_raw: u32 = (bank as u32) << 16 | offset as u32;
                //println!("Address_raw: {:x}", address_raw);
                let new_address = Address::new(address_raw);
                return Some(new_address.mirror(self));
            }
            Some(RomTypes::HiRom) => {
                // TODO: Fix that stuff
                let bank = address.bank;
                let offset = address.address;
                let mirror = bank % 0x80;
                let bank = bank - mirror;
                let address2: u32 = bank as u32 * 0x8000 + offset as u32;
                println!("Address: {:x}, Resolved: {:x}", address.address, address2);
                // TODO: Mirroring
                return Some(Address::new(address2));
            }
            _ => panic!("Unsupported rom type"),
        }
    }

    fn is_wram(&self, address: Address) -> bool {
        match address.bank {
            0x00..=0x3f => match address.address {
                0x0000..=0x1fff => {
                    return true;
                }
                _ => {
                    return false;
                }
            },
            0x7e => {
                match address.address {
                    0x0000..=0x1fff => {
                        return true;
                    }
                    0x2000..=0x7fff => {
                        // This xor is needed otherwise access would exceed the memory (highRam
                        // size is 0x6000
                        return true;
                    }
                    0x8000..=0xffff => {
                        return true;
                    }
                }
            }
            0x7f => return true,
            0x80..=0xbf => match address.address {
                0x0000..=0x1fff => {
                    return true;
                }
                _ => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }
    }

    fn write_wram(&self, address: Address, byte: u8) {
        println!("[WRAM WRITE] Bank: {:x} Address: {:x} Value: {:x}", address.bank, address.address, byte);
        self.wram.borrow_mut().write(address, byte);
    }

    fn read_wram(&self, address: Address) -> Option<u8> {
        print!("[WRAM READ] Bank: {:x} Address: {:x}", address.bank, address.address);
        if let Some(byte) = self.wram.borrow().read(address) {
            print!(" {:x}\n", byte);
            return Some(byte);
        } else {
            return None;
        }
    }

    // Read a single byte from the bus
    pub fn read(&self, address: Address) -> u8 {
        // TODO: First check for WRAM Access
        //
        println!(
            "[BUS Read] Bank {:x}, Address: {:x}",
            address.bank, address.address
        );
        if WRAM::is_wram(address) {
            if let Some(byte) = self.read_wram(address) {
                self.set_mdr(byte);
                return byte;
            } else {
                return self.mdr.borrow().clone();
            }
        }
        if let Some(address) = self.resolve_rom_address(address) {
            let ret = self.cartridge.as_ref().unwrap().read_byte(address.into());
            self.set_mdr(ret);
            return ret;
        }
        unimplemented!("Bus read not implemented");
    }
    // Read n bytes from the bus
    pub fn read_bytes(&self, address: Address, length: usize) -> Vec<u8> {
        println!("Reading {} bytes from address: {:?}", length, address);
        let mut bytes: Vec<u8> = Vec::with_capacity(length);
        for i in 0..length {
            bytes.push(self.read(address.add(i)));
        }
        bytes
    }
    // Write a single byte to the bus
    pub fn write(&self, address: Address, value: u8) {
        if WRAM::is_wram(address) {
            self.write_wram(address, value);
            return;
        }
        println!(
            "[BUS Write] {:x}, to Bank {:x}, Address: {:x}",
            value, address.bank, address.address
        );
        unimplemented!("Bus write not implemented");
    }
}
