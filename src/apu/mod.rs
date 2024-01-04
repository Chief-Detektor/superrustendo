use std::{
    thread::{self, sleep},
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct Apu {
    // TODO
    memory: [u8; 0x10000], // 64K
    initialized: bool,
    input_buffer: [u8; 4],
    output_buffer: [u8; 4],
}

//unsafe impl Send for Apu {}

//unsafe impl Sync for Apu {}

enum MemDestination {
    DirectPage0,
    HardwareRegisters,
    DirectPage1,
    IPLROM,
    Unmapped,
}

impl<'s, 't> Apu {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            initialized: true,
            input_buffer: [0; 4],
            output_buffer: [0; 4],
        }
    }

    pub fn tick(&self) {
        // TODO
        sleep(Duration::from_millis(1));
            
    }

    fn resolve(address: u16) -> MemDestination {
        match address {
            0x0000..=0x00EF => MemDestination::DirectPage0,
            0x00F0..=0x00FF => MemDestination::HardwareRegisters,
            0x0100..=0x01FF => MemDestination::DirectPage1,
            0xFFC0..=0xFFFF => MemDestination::IPLROM,
            _ => MemDestination::Unmapped,
        }
    }

    // Write to the APU from the CPU
    pub fn bus_write(&mut self, address: u16, value: u8) {
        match address {
            0x2140 => {
                self.input_buffer[0] = value;
            }
            0x2141 => {
                self.input_buffer[1] = value;
            }
            0x2142 => {
                self.input_buffer[2] = value;
            }
            0x2143 => {
                self.input_buffer[3] = value;
            }
            _ => {}
        }
    }

    // Read from the APU to the CPU
    pub fn bus_read(&self, address: u16) -> u8 {
        match address {
            0x2140 => self.output_buffer[0],
            0x2141 => self.output_buffer[1],
            0x2142 => self.output_buffer[2],
            0x2143 => self.output_buffer[3],
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        println!("[APU Write] {:x} {:x}", address, value);
        match Self::resolve(address) {
            MemDestination::DirectPage0 => self.memory[address as usize] = value,
            MemDestination::HardwareRegisters => {
                match address {
                    0x00F0 => {
                        // TODO
                    }
                    0x00F1 => {
                        // TODO
                    }
                    0x00F2 => {
                        // TODO
                    }
                    0x00F3 => {
                        // TODO
                    }
                    0x00F4 => {
                        self.output_buffer[0] = value;
                    }
                    0x00F5 => {
                        self.output_buffer[1] = value;
                    }
                    0x00F6 => {
                        self.output_buffer[2] = value;
                    }
                    0x00F7 => {
                        self.output_buffer[3] = value;
                    }
                    0x00F8 => {
                        // TODO
                    }
                    0x00F9 => {
                        // TODO
                    }
                    0x00FA => {
                        // TODO
                    }
                    0x00FB => {
                        // TODO
                    }
                    0x00FC => {
                        // TODO
                    }
                    0x00FD => {
                        // TODO
                    }
                    0x00FE => {
                        // TODO
                    }
                    0x00FF => {
                        // TODO
                    }
                    _ => {}
                }
            }

            MemDestination::DirectPage1 => self.memory[address as usize] = value,
            MemDestination::IPLROM => self.memory[address as usize] = value,
            MemDestination::Unmapped => {}
        }
        self.memory[address as usize] = value;
    }

    fn read(&self, address: u16) -> u8 {
        println!("[APU Read] {:x}", address);
        return match Self::resolve(address) {
            MemDestination::DirectPage0 => self.memory[address as usize],
            MemDestination::HardwareRegisters => match address {
                0x00F0 => 0,
                0x00F1 => 0,
                0x00F2 => 0,
                0x00F3 => 0,
                0x00F4 => 0,
                0x00F5 => 0,
                0x00F6 => 0,
                0x00F7 => 0,
                0x00F8 => 0,
                0x00F9 => 0,
                0x00FA => 0,
                0x00FB => 0,
                0x00FC => 0,
                0x00FD => 0,
                0x00FE => 0,
                0x00FF => 0,
                _ => 0,
            },

            MemDestination::DirectPage1 => self.memory[address as usize],
            MemDestination::IPLROM => self.memory[address as usize],
            MemDestination::Unmapped => 0,
        };
        //self.memory[address as usize]
    }
}
