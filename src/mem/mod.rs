use std::cell::{RefCell};
use std::rc::Rc;

use crate::cartridge::{Cartridge, RomTypes};
use crate::cpu::address::{Address};
// TODO: Use superrustendo lib path
use crate::cpu::CPU;
use crate::apu::Apu;
use crate::ppu::PPU;

use self::wram::WRAM;

use hw_registers::HWRegister;

pub mod wram;

pub mod hw_registers;

//#[derive(Debug)]
pub struct Bus {
    apu: RefCell<Apu>,
    cartridge: Option<Cartridge>,
    wram: Rc<RefCell<WRAM>>,
    ppu: PPU,
    cpu: CPU,
    mdr: RefCell<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            apu: RefCell::new(Apu::new()),
            cartridge: None,
            wram: Rc::new(RefCell::new(WRAM::new())),
            ppu: PPU::new(),
            cpu: CPU::new(),
            mdr: RefCell::new(0),
        }
    }

    pub fn get_apu(&self) -> &RefCell<Apu> {
        &self.apu
    }

    pub fn get_cartridge(&self) -> &Option<Cartridge> {
        &self.cartridge
    }

    pub fn get_cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn get_ppu(&self) -> &PPU {
        &self.ppu
    }

    //    pub fn get_wram(&self) -> &WRAM {
    //        &self.wram
    //    }

    pub fn get_mdr(&self) -> u8 {
        *self.mdr.borrow()
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

    // TODO: put this on the cartridge type
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

                let address2: u32 = (bank as u32) << 16 | offset as u32;
                //                println!("Address: {:x}, Resolved: {:x}", address.address, address2);
                // TODO: Mirroring
                return Some(Address::new(address2).mirror(self));
            }
            _ => panic!("Unsupported rom type"),
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

        if let Some(val) = HWRegister::dispatch_read(&self, address) {
            self.set_mdr(val);
            return val;
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
    // TODO: Write to mdr as well?
    pub fn write(&self, address: Address, value: u8) {
        HWRegister::dispatch_write(self, address, value);

        println!(
            "[BUS Write] {:x}, to Bank {:x}, Address: {:x}",
            value, address.bank, address.address
        );
        //        unimplemented!("Bus write not implemented");
    }
}
