pub mod wram;

use crate::cartridge::{Cartridge, RomTypes};
use crate::cpu::address::Address;
use crate::ppu::PPU;

pub struct WRAM {
    lowRam: [u8; 0x2000],  // bank: 0x0-3f (shadowed from 0x7e) 0x0000-0x1fff
    highRam: [u8; 0x6000], // 0x7e
    extendedRam_1: [u8; 0x8000],
    extendedRam_2: [u8; 0x10000],
}

impl WRAM {
    pub fn new() -> Self {
        WRAM {
            lowRam: [0xf; 0x2000],
            highRam: [0; 0x6000],
            extendedRam_1: [0; 0x8000],
            extendedRam_2: [0; 0x10000],
        }
    }
}
/*

LowRom = Address % 8000 + 8000 * Bank ( + mirror testing)


*/

//#[derive(Debug)]
pub struct Bus {
    pub cartridge: Option<Cartridge>,
    pub wram: WRAM,
    // pub ppu: PPU,
}

impl Bus {
    pub fn read_bytes(&self, address: Address, length: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(length);
        for i in 0..length {
            ret.push(self.read(address.add(i)));
        }
        ret
    }
    pub fn read(&self, address: Address) -> u8 {
        // let mut address = address;
        if let Some(card) = self.cartridge.as_ref() {
            if let Some(rom_type) = &card.rom_type {
                match rom_type {
                    RomTypes::LowRom => {
                        print!("[BUS]: Mapped read ");
                        // Mirroring!
                        match address.bank {
                            0x00..=0x3F => {
                                match address.address {
                                    0x0000..=0x1FFF => {
                                        println!(
                                            "access to {:x}:{:x} [WRAM]",
                                            address.bank, address.address
                                        );
                                        // println!("WRAM READ")
                                        return self.wram.lowRam[address.address as usize];
                                    }
                                    0x2100..=0x21FF => {
                                        println!(
                                            "access to {:x}:{:x} [PPU1, APU, HW-Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x3000..=0x3FFF => {
                                        println!("access to {:x}:{:x} [DSP, SuperFX, hardware registers]", address.bank, address.address);
                                    }
                                    0x4000..=0x40FF => {
                                        println!(
                                            "access to {:x}:{:x} [Old Style Joypad Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x4200..=0x44FF => {
                                        println!(
                                            "access to {:x}:{:x} [DMA, PPU2, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x6000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [RESERVED (enhancement chips memory])", address.bank, address.address);
                                    }
                                    0x800..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [LoROM section (program memory])",
                                            address.bank, address.address
                                        );
                                        // Convert to low-rom mapping:
                                        let new = address.address as u32 % 0x8000 as u32
                                            + address.bank as u32 * 0x8000 as u32;
                                        // let bank = (new >> 16) as u8;
                                        // let address = (new & 0xffffu32) as u16;
                                        return self
                                            .cartridge
                                            .as_ref()
                                            .unwrap()
                                            .read_byte(new as usize);
                                    }

                                    _ => {
                                        println!(
                                            " access to {:x}:{:x} is not defined",
                                            address.bank, address.address
                                        );
                                    }
                                }
                            }
                            0x40..=0x6F => {
                                match address.address {
                                    0x0000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [May be mapped as the higher bank ($8000 - $FFFF) if chip is not MAD-1. Otherwise this area is unused.]", address.bank, address.address);
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [LoROM section (program memory])",
                                            address.bank, address.address
                                        );
                                        // Convert to low-rom mapping:
                                        let new = address.address as u32 % 0x8000 as u32
                                            + address.bank as u32 * 0x8000 as u32;
                                        return self
                                            .cartridge
                                            .as_ref()
                                            .unwrap()
                                            .read_byte(new as usize);
                                    }
                                }
                            }
                            0x70..=0x7D => {
                                match address.address {
                                    0x0000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [Cartridge SRAM - 448 Kilobytes maximum]", address.bank, address.address);
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [LoROM section (program memory])",
                                            address.bank, address.address
                                        );
                                        // Convert to low-rom mapping:
                                        let new = address.address as u32 % 0x8000 as u32
                                            + address.bank as u32 * 0x8000 as u32;
                                        return self
                                            .cartridge
                                            .as_ref()
                                            .unwrap()
                                            .read_byte(new as usize);
                                    }
                                }
                            }
                            0x7E => match address.address {
                                0x0000..=0x1FFF => {
                                    println!(
                                        "access to {:x}:{:x} [LowRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.lowRam[address.address as usize];
                                }
                                0x2000..=0x7FFF => {
                                    println!(
                                        "access to {:x}:{:x} [HighRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.highRam[(address.address - 0x2000) as usize];
                                }
                                0x8000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.extendedRam_1
                                        [(address.address - 0x8000) as usize];
                                }
                            },
                            0x7F => match address.address {
                                0x0000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.extendedRam_2[address.address as usize];
                                }
                            },
                            0x80..=0xBF => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $00–$3F]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                return self.read(a);
                            }
                            0xC0..=0xEF => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $40–$6F]",
                                    address.bank, address.address
                                );
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                return self.read(a);
                            }
                            0xF0..=0xFD => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $70–$7D]",
                                    address.bank, address.address
                                );
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                return self.read(a);
                            }
                            0xFE..=0xFF => {
                                match address.address {
                                    0x0000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [Cartridge SRAM - 64 Kilobytes (512 KB total)]", address.bank, address.address);
                                        // TODO: Implement Sram (persist as file)
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [LoROM section (program memory)]",
                                            address.bank, address.address
                                        );
                                        println!("access to {:x}:{:x} [Mirror of $7E–$7F (overridden by WRAM in the <$7F range)]", address.bank, address.address);
                                        let mut a = address;
                                        a.bank = a.bank - 0x80;
                                        self.read(a);
                                    }
                                }
                            }
                        }
                    }
                    RomTypes::HiRom => {
                        // Mirroring!
                        // address
                        match address.bank {
                            0x00..=0x1F => {
                                match address.address {
                                    0x0000..=0x1FFF => {
                                        println!(
                                            "access to {:x}:{:x} [WRAM]",
                                            address.bank, address.address
                                        );
                                        // println!("WRAM READ")
                                        return self.wram.lowRam[address.address as usize];
                                    }
                                    0x2100..=0x21FF => {
                                        println!(
                                            "access to {:x}:{:x} [PPU1, APU, HW-Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x3000..=0x3FFF => {
                                        println!("access to {:x}:{:x} [DSP, SuperFX, hardware registers]", address.bank, address.address);
                                    }
                                    0x4000..=0x40FF => {
                                        println!(
                                            "access to {:x}:{:x} [Old Style Joypad Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x4200..=0x44FF => {
                                        println!(
                                            "access to {:x}:{:x} [DMA, PPU2, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x6000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [RESERVED (enhancement chips memory])", address.bank, address.address);
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [HiROM section (program memory])",
                                            address.bank, address.address
                                        );
                                        // Convert to low-rom mapping:
                                        let new =
                                            address.address as u32 + ((address.bank as u32) << 16);
                                        return self
                                            .cartridge
                                            .as_ref()
                                            .unwrap()
                                            .read_byte(new as usize);
                                    }

                                    _ => {
                                        println!(
                                            " access to {:x}:{:x} is not defined",
                                            address.bank, address.address
                                        );
                                    }
                                }
                            }
                            0x20..=0x3F => {
                                match address.address {
                                    0x0000..=0x1FFF => {
                                        println!(
                                            "access to {:x}:{:x} [LowRAM, shadowed from bank $7E]",
                                            address.bank, address.address
                                        );
                                        return self.wram.lowRam[address.address as usize];
                                    }
                                    0x2100..=0x21FF => {
                                        println!(
                                            "access to {:x}:{:x} [PPU1, APU, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x3000..=0x3FFF => {
                                        println!("access to {:x}:{:x} [DSP, SuperFX, hardware registers]", address.bank, address.address);
                                    }
                                    0x4000..=0x40FF => {
                                        println!(
                                            "access to {:x}:{:x} [Old Style Joypad Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x4200..=0x44FF => {
                                        println!(
                                            "access to {:x}:{:x} [DMA, PPU2, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x6000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [Cartridge SRAM - 8 Kilobytes (256 KB total)]", address.bank, address.address);
                                        // TODO: Implement SRAM (As a seperate file!)
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [HiROM section (program memory)]",
                                            address.bank, address.address
                                        );
                                    }
                                    _ => {
                                        println!(
                                            "access to {:x}:{:x} is not defined",
                                            address.bank, address.address
                                        );
                                    }
                                }
                            }
                            0x40..=0x7D => {
                                println!(
                                    "access to {:x}:{:x} [HiROM section (program memory)]",
                                    address.bank, address.address
                                );

                                let new = address.address as u32
                                    | ((address.bank as u32 - 0x40) << 16 as u32);
                                println!("Page {:x}", address.bank);
                                println!("Page {:x}", address.bank - 0x40);
                                println!("addr {:x}", new);
                                // let bank = (new >> 16) as u8;
                                // let address = (new & 0xffffu32) as u16;
                                return self.cartridge.as_ref().unwrap().read_byte(new as usize);
                            }
                            0x7E => match address.address {
                                0x0000..=0x1FFF => {
                                    println!(
                                        "access to {:x}:{:x} [LowRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.lowRam[address.address as usize];
                                }
                                0x2000..=0x7FFF => {
                                    println!(
                                        "access to {:x}:{:x} [HighRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.highRam[(address.address - 0x2000) as usize];
                                }
                                0x8000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.extendedRam_1
                                        [(address.address - 0x8000) as usize];
                                }
                            },
                            0x7F => match address.address {
                                0x0000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    return self.wram.extendedRam_2[address.address as usize];
                                }
                            },
                            0x80..=0x9F => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $00–$1F]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                return self.read(a);
                            }
                            0xA0..=0xBF => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $20–$3F]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                return self.read(a);
                            }
                            0xC0..=0xFD => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $40–$7D]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                return self.read(a);
                            }
                            0xFE..=0xFF => {
                                // let mut a = address;
                                // a.bank = a.bank - 0xC0;
                                let a = address.address as usize
                                    + ((address.bank as usize) - 0xC0) * 0x8000;
                                // self.read
                                return self.cartridge.as_ref().unwrap().read_byte(a);
                            }
                        }
                    }
                };
            }
        }
        return 0;
    }

    pub fn write(&mut self, address: Address, data: u8) {
        if let Some(card) = self.cartridge.as_ref() {
            if let Some(rom_type) = &card.rom_type {
                match rom_type {
                    RomTypes::LowRom => {
                        print!("[BUS]: Mapped write ");
                        // Mirroring!
                        match address.bank {
                            0x00..=0x3F => match address.address {
                                0x0000..=0x1fff => {
                                    println!("WRAM write");
                                    self.wram.lowRam[address.address as usize] = data;
                                }
                                0x2100 => {
                                    print!("INIDISP - Screen Display: ");
                                    let force_blank = (data >> 7) != 0;
                                    let brightness = data & 0xf;
                                    println!(
                                        "force blank: {:?}, brightness: {:x}",
                                        force_blank, brightness
                                    );
                                }
                                0x2101 => {
                                    print!("OBSEL - Object Size and Character Address: ");
                                    let object_size = data >> 5;
                                    let name_select = (data & 0x18) >> 3;
                                    let name_base_select = data & 0x7;
                                    println!(
                    "object_size: 0x{:x}, name_select: 0x{:x}, name_base_select: 0x{:x}",
                    object_size, name_select, name_base_select
                );
                                }
                                0x2102 => {
                                    println!("OAMADDL - OAM Address low byte: 0x{:x}", data);
                                }
                                0x2103 => {
                                    print!("OAMADDH - OAM Address high bit and Obj Priority: ");

                                    let p = (data >> 7) != 0;
                                    let b = (data & 0x1) != 0;

                                    println!(
                                        "Obj priority activation bit: {:?}, table selector: {:?}",
                                        p, b
                                    );
                                }
                                0x2104 => {
                                    println!("OAMDATA - Data for OAM write {:b}", data);
                                }
                                0x2105 => {
                                    // TODO: extract data from payload
                                    println!("BGMODE - BG Mode and Character Size, {:b}", data);
                                }
                                0x2106 => {
                                    println!("MOSAIC - Screen Pixelation: {:b}", data);
                                }
                                0x2107 => {
                                    println!("BG1SC - BG1 Tilemap Address and Size: {:b}", data);
                                }
                                0x2108 => {
                                    println!("BG2SC - BG2 Tilemap Address and Size: {:b}", data);
                                }
                                0x2109 => {
                                    println!("BG3SC - BG3 Tilemap Address and Size: {:b}", data);
                                }
                                0x210a => {
                                    println!("BG4SC - BG4 Tilemap Address and Size: {:b}", data);
                                }
                                0x210b => {
                                    println!("BG12NBA - BG1 and 2 Chr Address {:b}", data);
                                }
                                0x210c => {
                                    println!("BG34NBA - BG3 and 4 Chr Address {:b}", data);
                                }
                                0x210d => {
                                    println!(
                    "BG1HOFS - BG1 Horizontal Scroll || M7HOFS  - Mode 7 BG Horizontal Scroll {:b}",
                    data
                );
                                }
                                0x210e => {
                                    println!(
                    "BG1HOFS - BG1 Vertical Scroll || M7HOFS  - Mode 7 BG Vertical Scroll {:b}",
                    data
                );
                                }
                                0x210f => {
                                    println!("BG2HOFS - BG2 Horizontal Scroll {:b}", data);
                                }
                                0x2110 => {
                                    println!("BG2VOFS - BG2 Vertical Scroll {:b}", data);
                                }
                                0x2111 => {
                                    println!("BG3HOFS - BG3 Horizontal Scroll {:b}", data);
                                }
                                0x2112 => {
                                    println!("BG3VOFS - BG3 Vertical Scroll {:b}", data);
                                }
                                0x2113 => {
                                    println!("BG4HOFS - BG4 Horizontal Scroll {:b}", data);
                                }
                                0x2114 => {
                                    println!("BG4VOFS - BG4 Vertical Scroll {:b}", data);
                                }
                                0x2115 => {
                                    println!("VMAIN - Video Port Control {:b}", data);
                                }
                                0x2116 => {
                                    println!("VMADDL - VRAM Address low byte {:b}", data);
                                }
                                0x2117 => {
                                    println!("VMADDH - VRAM Address high byte {:b}", data);
                                }
                                0x2118 => {
                                    println!("VMDATAL - VRAM Data Write low byte {:b}", data);
                                }
                                0x2119 => {
                                    println!("VMDATAH - VRAM Data Write high byte {:b}", data);
                                }
                                0x211a => {
                                    println!("M7SEL - Mode 7 Settings {:b}", data);
                                }
                                0x211b => {
                                    println!(
                                        "M7A - Mode 7 Matrix A (also used with $2134/6) {:b}",
                                        data
                                    );
                                }
                                0x211c => {
                                    println!(
                                        "M7B - Mode 7 Matrix B (also used with $2134/6) {:b}",
                                        data
                                    );
                                }
                                0x211d => {
                                    println!("M7C - Mode 7 Matrix C {:b}", data);
                                }
                                0x211e => {
                                    println!("M7D - Mode 7 Matrix D {:b}", data);
                                }
                                0x211f => {
                                    println!("M7X - Mode 7 Center X {:b}", data);
                                }
                                0x2120 => {
                                    println!("M7Y - Mode 7 Center Y {:b}", data);
                                }
                                0x2121 => {
                                    println!("CGRAM Address {:b}", data);
                                }
                                0x2122 => {
                                    println!("CGDATA - CGRAM Data write {:b}", data);
                                }
                                0x2123 => {
                                    println!(
                                        "W12SEL - Window Mask Settings for BG1 and BG2 {:b}",
                                        data
                                    );
                                }
                                0x2124 => {
                                    println!(
                                        "W34SEL - Window Mask Settings for BG3 and BG4 {:b}",
                                        data
                                    );
                                }
                                0x2125 => {
                                    println!(
                    "WOBJSEL - Window Mask Settings for OBJ and Color Window {:b}",
                    data
                );
                                }
                                0x2126 => {
                                    println!("WH0 - Window 1 Left Position {:b}", data);
                                }
                                0x2127 => {
                                    println!("WH1 - Window 1 Right Position {:b}", data);
                                }
                                0x2128 => {
                                    println!("WH2 - Window 2 Left Position {:b}", data);
                                }
                                0x2129 => {
                                    println!("WH3 - Window 2 Right Position {:b}", data);
                                }
                                0x212a => {
                                    println!("WBGLOG - Window mask logic for BGs {:b}", data);
                                }
                                0x212b => {
                                    println!(
                    "WOBJLOG - Window mask logic for OBJs and Color Window {:b}",
                    data
                );
                                }
                                0x212c => {
                                    println!("TM - Main Screen Designation {:b}", data);
                                }
                                0x212d => {
                                    println!("TS - Subscreen Designation {:b}", data);
                                }
                                0x212e => {
                                    println!(
                                        "TMW - Window Mask Designation for the Main Screen {:b}",
                                        data
                                    );
                                }
                                0x212f => {
                                    println!(
                                        "TSW - Window Mask Designation for the Subscreen {:b}",
                                        data
                                    );
                                }
                                0x2130 => {
                                    println!("CGWSEL - Color Addition Select {:b}", data);
                                }
                                0x2131 => {
                                    println!("CGADSUB - Color math designation {:b}", data);
                                }
                                0x2132 => {
                                    println!("COLDATA - Fixed Color Data {:b}", data);
                                }
                                0x2133 => {
                                    println!("SETINI - Screen Mode/Video Select {:b}", data);
                                }
                                // TODO: Missing Regs
                                0x4200 => {
                                    println!("NMITIMEN - Interrupt Enable Flags {:b}", data);
                                    // TODO: Power on and reset => 0x00
                                }
                                0x4201 => {
                                    println!("WRIO - Programmable I/O port (out-port) {:b}", data);
                                }
                                0x4202 => {
                                    println!("WRMPYA - Multiplicand A {:b}", data);
                                    // TODO: 0xff on powerup/reset
                                }
                                0x4203 => {
                                    println!("WRMPYB - Multiplicand B {:b}", data);
                                }
                                0x4204 => {
                                    println!("WRDIVL - Dividend C low byte {:b}", data);
                                }
                                0x4205 => {
                                    println!("WRDIVH - Dividend C high byte {:b}", data);
                                }
                                0x4206 => {
                                    println!("WRDIVB - Divisor B {:b}", data);
                                }
                                0x4207 => {
                                    println!("HTIMEL - H Timer low byte {:b}", data);
                                }
                                0x4208 => {
                                    println!("HTIMEH - H Timer high byte {:b}", data);
                                }
                                0x4209 => {
                                    println!("VTIMEL - V Timer low byte {:b}", data);
                                }
                                0x420a => {
                                    println!("VTIMEH - V Timer high byte {:b}", data);
                                }
                                0x420b => {
                                    println!("MDMAEN - DMA Enable {:b}", data);
                                }
                                0x420c => {
                                    println!("HDMAEN - HDMA Enable {:b}", data);
                                }
                                0x420d => {
                                    println!("MEMSEL - ROM Access Speed {:b}", data);
                                }
                                // 0x420e => {
                                //   println!("RDNMI - NMI Flag and 5A22 Version {:b}", data);
                                // }
                                // 0x210d => {
                                //   println!("BG1HOFS - BG1 Horizontal Scroll {:b}", data);
                                // }
                                // 0x210d => {
                                //   println!("BG1HOFS - BG1 Horizontal Scroll {:b}", data);
                                // }
                                // 0x210d => {
                                //   println!("BG1HOFS - BG1 Horizontal Scroll {:b}", data);
                                // }
                                // 0x210d => {
                                //   println!("BG1HOFS - BG1 Horizontal Scroll {:b}", data);
                                // }
                                _ => println!(
                                    "Unimpl Register {:x}:{:x}",
                                    address.bank, address.address
                                ), //unimplemented!("Register {:x}", address),
                            },
                            0x40..=0x6F => {}
                            0x70..=0x7D => match address.address {
                                0x0000..=0x7FFF => {
                                    println!("access to {:x}:{:x} [Cartridge SRAM - 448 Kilobytes maximum]", address.bank, address.address);
                                }
                                _ => {
                                    println!(
                                        "access to {:x}:{:x} [Program ROM -------IGNORED]",
                                        address.bank, address.address
                                    );
                                }
                            },
                            0x7E => match address.address {
                                0x0000..=0x1FFF => {
                                    println!(
                                        "access to {:x}:{:x} [LowRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.lowRam[address.address as usize] = data;
                                }
                                0x2000..=0x7FFF => {
                                    println!(
                                        "access to {:x}:{:x} [HighRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.highRam[(address.address - 0x2000) as usize] = data;
                                }
                                0x8000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.extendedRam_1[(address.address - 0x8000) as usize] =
                                        data;
                                }
                            },
                            0x7F => match address.address {
                                0x0000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.extendedRam_2[address.address as usize] = data;
                                }
                            },
                            0x80..=0xBF => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $00–$3F]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                self.write(a, data);
                            }
                            0xC0..=0xEF => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $40–$6F]",
                                    address.bank, address.address
                                );
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                self.write(a, data);
                            }
                            0xF0..=0xFD => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $70–$7D]",
                                    address.bank, address.address
                                );
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                self.write(a, data);
                            }
                            0xFE..=0xFF => {}
                        }
                    }
                    RomTypes::HiRom => {
                        match address.bank {
                            0x00..=0x1F => {
                                match address.address {
                                    0x0000..=0x1FFF => {
                                        println!(
                                            "access to {:x}:{:x} [WRAM]",
                                            address.bank, address.address
                                        );
                                        // println!("WRAM READ")
                                        self.wram.lowRam[address.address as usize] = data;
                                    }
                                    0x2100..=0x21FF => {
                                        println!(
                                            "access to {:x}:{:x} [PPU1, APU, HW-Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x3000..=0x3FFF => {
                                        println!("access to {:x}:{:x} [DSP, SuperFX, hardware registers]", address.bank, address.address);
                                    }
                                    0x4000..=0x40FF => {
                                        println!(
                                            "access to {:x}:{:x} [Old Style Joypad Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x4200..=0x44FF => {
                                        println!(
                                            "access to {:x}:{:x} [DMA, PPU2, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x6000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [RESERVED (enhancement chips memory])", address.bank, address.address);
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [HiROM section (program memory]) IGNORED",
                                            address.bank, address.address
                                        );
                                    }

                                    _ => {
                                        println!(
                                            " access to {:x}:{:x} is not defined",
                                            address.bank, address.address
                                        );
                                    }
                                }
                            }
                            0x20..=0x3F => {
                                match address.address {
                                    0x0000..=0x1FFF => {
                                        println!(
                                            "access to {:x}:{:x} [LowRAM, shadowed from bank $7E]",
                                            address.bank, address.address
                                        );
                                        self.wram.lowRam[address.address as usize] = data;
                                    }
                                    0x2100..=0x21FF => {
                                        println!(
                                            "access to {:x}:{:x} [PPU1, APU, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x3000..=0x3FFF => {
                                        println!("access to {:x}:{:x} [DSP, SuperFX, hardware registers]", address.bank, address.address);
                                    }
                                    0x4000..=0x40FF => {
                                        println!(
                                            "access to {:x}:{:x} [Old Style Joypad Registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x4200..=0x44FF => {
                                        println!(
                                            "access to {:x}:{:x} [DMA, PPU2, hardware registers]",
                                            address.bank, address.address
                                        );
                                    }
                                    0x6000..=0x7FFF => {
                                        println!("access to {:x}:{:x} [Cartridge SRAM - 8 Kilobytes (256 KB total)]", address.bank, address.address);
                                        // TODO: Implement SRAM (As a seperate file!)
                                    }
                                    0x8000..=0xFFFF => {
                                        println!(
                                            "access to {:x}:{:x} [HiROM section (program memory)]",
                                            address.bank, address.address
                                        );
                                    }
                                    _ => {
                                        println!(
                                            "access to {:x}:{:x} is not defined",
                                            address.bank, address.address
                                        );
                                    }
                                }
                            }
                            0x40..=0x7D => {
                                println!(
                                    "access to {:x}:{:x} [HiROM section (program memory)] IGNORED",
                                    address.bank, address.address
                                );
                            }
                            0x7E => match address.address {
                                0x0000..=0x1FFF => {
                                    println!(
                                        "access to {:x}:{:x} [LowRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.lowRam[address.address as usize] = data;
                                }
                                0x2000..=0x7FFF => {
                                    println!(
                                        "access to {:x}:{:x} [HighRAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.highRam[(address.address - 0x2000) as usize] = data;
                                }
                                0x8000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.extendedRam_1[(address.address - 0x8000) as usize] =
                                        data;
                                }
                            },
                            0x7F => match address.address {
                                0x0000..=0xFFFF => {
                                    println!(
                                        "access to {:x}:{:x} [Extended RAM (WRAM)]",
                                        address.bank, address.address
                                    );
                                    self.wram.extendedRam_2[address.address as usize] = data;
                                }
                            },
                            0x80..=0x9F => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $00–$1F]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                self.write(a, data);
                            }
                            0xA0..=0xBF => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $20–$3F]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                self.write(a, data);
                            }
                            0xC0..=0xFD => {
                                println!(
                                    "access to {:x}:{:x} [Mirror of $40–$7D]",
                                    address.bank, address.address
                                );
                                // This is just a mirror. redirect to the address
                                let mut a = address;
                                a.bank = a.bank - 0x80;
                                self.write(a, data);
                            }
                            0xFE..=0xFF => {
                                let mut a = address;
                                a.bank = address.bank - 0xC0;

                                // self.read
                                self.write(a, data);
                            }
                        }
                    }
                }
            }
        }
    }
}
