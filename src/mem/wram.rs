use crate::cpu::address::Address;

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

    pub fn write(&mut self, address: Address, byte: u8) {
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

    pub fn read(&self, address: Address) -> Option<u8> {
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
