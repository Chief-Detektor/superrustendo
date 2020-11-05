use crate::cpu::addressmodes::AddressModes;
use crate::cpu::decoder::Opcodes;
use crate::cpu::CPU;
// use crate::cpu::
use crate::cpu::{Accumulator, IndexRegister, Registers, StatusRegister};
use crate::mem::Mapper;

use std::convert::TryInto;

use super::address::Address;

#[derive(Debug, Default, Clone)]
pub struct Instruction {
    pub address: u32,
    pub opcode: Opcodes,
    pub address_mode: AddressModes,
    pub length: usize,
    pub payload: Vec<u8>,
    pub cycles: usize,
}

impl Instruction {
    pub fn new() -> Instruction {
        let inst = Instruction::default();
        inst
    }
    // pub fn new(opcode: u8) -> Instruction {}
    pub fn execute(&mut self, mut cpu: &mut CPU, mapper: &mut Mapper, follow_jumps: bool) {
        // Get the correct address for instruction
        let effective_address =
            self.address_mode
                .get_effective_address(&mut cpu, &self.payload, &self.opcode, &mapper);

        if effective_address.is_some() {
            println!("Calculated effective address: {:?}", effective_address);
        }

        match &self.opcode {
            Opcodes::AND => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let val = mapper.read(effective_address.unwrap());
                    cpu.regs.C = Accumulator::from(cpu.regs.C.A & val as u16);
                } else {
                    let low = mapper.read(effective_address.unwrap());
                    let high = mapper.read(effective_address.unwrap().add(1));
                    cpu.regs.C = Accumulator::from(
                        u16::from(cpu.regs.C) & (low as u16 | (high as u16) << 8),
                    );
                }
                if u16::from(cpu.regs.C) >> 15 == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
                if u16::from(cpu.regs.C) == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
            }
            // TODO: test this
            Opcodes::ADC => {
                // TODO: Decimal flag
                if cpu.e || cpu.regs.P.m == 1 {
                    // 8-Bit
                    if cpu.regs.C.A as u16 + (mapper.read(effective_address.unwrap()) as u16) > 255
                    {
                        cpu.regs.P.v = 1;
                    }
                    let mut data =
                        (cpu.regs.C.A as u8).wrapping_add(mapper.read(effective_address.unwrap()));
                    if cpu.regs.P.c == 0 {
                        data += 1;
                    }
                    if data == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    if data >> 7 == 1 {
                        cpu.regs.P.n = 1
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    cpu.regs.C.A = data as u16;
                } else {
                    let mut data_low = mapper.read(effective_address.unwrap());
                    let mut data_high = mapper.read(effective_address.unwrap().add(1));
                    if cpu.regs.C.A as u16 + (data_low as u16) > 255 {
                        // borrow required
                        data_high += 1;
                        cpu.regs.P.c = 0;
                        cpu.regs.C.A = cpu.regs.C.A.wrapping_add(data_low.into());
                        cpu.regs.C.B = cpu.regs.C.B.wrapping_add(data_high.into());

                        if u16::from(cpu.regs.C) == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if u16::from(cpu.regs.C) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                        cpu.regs.P.v = 1;
                    } else {
                        cpu.regs.P.c = 1;
                        cpu.regs.C.A = cpu.regs.C.A.wrapping_add(data_low.into());
                        cpu.regs.C.B = cpu.regs.C.B.wrapping_add(data_high.into());

                        if u16::from(cpu.regs.C) == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if u16::from(cpu.regs.C) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                        cpu.regs.P.v = 0;
                    }
                }
            }
            Opcodes::ASL => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = cpu.regs.C.A as u8;
                    } else {
                        // shift val located at effective_address
                        val = mapper.read(effective_address.unwrap());
                    }
                    val = val << 1;
                    let msb = val >> 7;
                    if msb == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    cpu.regs.P.c = msb;
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C.A = val as u16
                    } else {
                        mapper.write(effective_address.unwrap(), val);
                    }
                } else {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = u16::from(cpu.regs.C);
                    } else {
                        // shift val located at effective_address
                        val = mapper.read(effective_address.unwrap()) as u16
                            | ((mapper.read(effective_address.unwrap().add(1)) as u16) << 8)
                    }
                    val = val << 1;
                    let msb = val >> 15;
                    if msb == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    cpu.regs.P.c = msb as u8;
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C = Accumulator::from(val);
                    } else {
                        mapper.write(effective_address.unwrap(), (val & 0x0f) as u8);
                        mapper.write(effective_address.unwrap().add(1), (val >> 8) as u8);
                    }
                }
            }
            Opcodes::BIT => {
                if cpu.e || cpu.regs.P.m == 0 {
                    if self.address_mode == AddressModes::Immediate {
                        if self.payload[0] == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                    } else {
                        let mut val = mapper.read(effective_address.unwrap());
                        cpu.regs.P.n = (val >> 7) & 0x1;
                        cpu.regs.P.v = (val >> 6) & 0x1;
                        val = val & (cpu.regs.C.A as u8);
                        cpu.regs.P.z = !val;
                    }
                } else {
                    if self.address_mode == AddressModes::Immediate {
                        if self.payload[0] as u16 | (self.payload[1] as u16) << 8 == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                    } else {
                        let mut val = mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                        cpu.regs.P.n = ((val >> 15) & 0x1) as u8;
                        cpu.regs.P.v = ((val >> 14) & 0x1) as u8;
                        val = val & u16::from(cpu.regs.C);
                        cpu.regs.P.z = !(val as u8);
                    }
                }
            }
            Opcodes::BRA => {
                let val = (self.payload[0] as i8) as i16;
                let foo = (cpu.regs.PC as i32 + val as i32) as i32;
                cpu.regs.PC = foo as u16;
            }
            Opcodes::BRL => {
                let val = self.payload[0] as u16 | (self.payload[1] as u16) << 8;
                cpu.regs.PC += val;
            }
            Opcodes::BEQ => {
                if cpu.regs.P.z == 1 {
                    let mut addr = (self.payload[0] as i8) as i16;
                    addr += cpu.regs.PC as i16;
                    cpu.regs.PC = addr as u16;
                }
            }
            Opcodes::BCC => {
                if cpu.regs.P.c == 0 {
                    let mut addr = (self.payload[0] as i8) as i16;
                    addr += cpu.regs.PC as i16;
                    cpu.regs.PC = addr as u16;
                }
            }
            Opcodes::BCS => {
                if cpu.regs.P.c == 1 {
                    let val = (self.payload[0] as i8) as i16 + cpu.regs.PC as i16;
                    cpu.regs.PC = val as u16;
                }
            }
            Opcodes::BPL => {
                if cpu.regs.P.n == 0 {
                    let val = (self.payload[0] as i8) as i16 + cpu.regs.PC as i16;
                    cpu.regs.PC = val as u16;
                }
            }
            Opcodes::BMI => {
                if cpu.regs.P.n == 1 {
                    let val = (self.payload[0] as i8) as i16 + cpu.regs.PC as i16;
                    cpu.regs.PC = val as u16;
                }
            }
            Opcodes::BRK => {
                cpu.regs.PC += 2;
                // TODO: Eval this..
                // cpu.regs.PC = effective_address.unwrap().address;
            }
            Opcodes::BVS => {
                if cpu.regs.P.v == 1 {
                    let val = (self.payload[0] as i8) as i16 + cpu.regs.PC as i16;
                    cpu.regs.PC = val as u16;
                }
            }
            Opcodes::CLD => cpu.regs.P.d = 0,
            Opcodes::CMP => {
                let val;
                if cpu.e || cpu.regs.P.m == 1 {
                    val = u16::from(cpu.regs.C) - mapper.read(effective_address.unwrap()) as u16;
                } else {
                    val = u16::from(cpu.regs.C)
                        - (mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8);
                }
                let res = u16::from(cpu.regs.C) - val;

                if res >> 15 == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
                if res == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
                if u16::from(cpu.regs.C) >= val {
                    cpu.regs.P.c = 1;
                } else {
                    cpu.regs.P.c = 0;
                }
            }
            Opcodes::SEI => {
                cpu.regs.P.i = 1;
            }
            Opcodes::CLC => {
                cpu.regs.P.c = 0;
            }
            Opcodes::CPX => {
                // 8 Bit registers
                if cpu.e || cpu.regs.P.x == 1 {
                    let val;
                    if self.address_mode != AddressModes::Immediate {
                        val = mapper.read(effective_address.unwrap());
                    } else {
                        val = self.payload[0];
                    }
                    let bar = (cpu.regs.X.low as u8).wrapping_sub(val);
                    if bar >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if bar == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    // TODO: double check this.
                    if cpu.regs.X.low as u8 >= bar {
                        cpu.regs.P.c = 1;
                    } else {
                        cpu.regs.P.c = 0;
                    }
                } else {
                    let val;
                    if self.address_mode != AddressModes::Immediate {
                        val = mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                    } else {
                        val = self.payload[0] as u16 | ((self.payload[1] as u16) << 8);
                    }
                    let bar = <u16>::from(cpu.regs.X).wrapping_sub(val);
                    if bar >> 15 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if bar == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    // TODO: double check this.
                    if <u16>::from(cpu.regs.X) >= bar {
                        cpu.regs.P.c = 1;
                    } else {
                        cpu.regs.P.c = 0;
                    }
                }
            }
            Opcodes::DEY => {
                if cpu.e || cpu.regs.P.m == 1 {
                    cpu.regs.Y.low = cpu.regs.Y.low.wrapping_sub(1);
                    if cpu.regs.Y.low as u8 >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if cpu.regs.Y.low as u8 == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                } else {
                    cpu.regs.Y = IndexRegister::from(u16::from(cpu.regs.Y).wrapping_sub(1));
                    if u16::from(cpu.regs.Y) >> 15 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if u16::from(cpu.regs.Y) == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                }
            }
            Opcodes::EOR => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let val = mapper.read(effective_address.unwrap());
                    cpu.regs.C =
                        Accumulator::from(cpu.regs.C.B | (cpu.regs.C.A as u8 ^ val) as u16);
                } else {
                    let val = mapper.read(effective_address.unwrap()) as u16
                        | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                    cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) ^ val);
                }
            }
            Opcodes::XCE => {
                // Exchange carry with phantom emulation flag
                // TODO: Reset programm bank register
                let temp = cpu.e;
                cpu.e = cpu.regs.P.c != 0;
                cpu.regs.P.c = temp as _;
            }
            Opcodes::SEP => {
                // Set Status Bits
                let tmp = <u8>::from(cpu.regs.P);
                let next = tmp | self.payload[0]; // Set bits
                cpu.regs.P = StatusRegister::from(next);
            }
            Opcodes::REP => {
                // Reset Status Bits
                let tmp = <u8>::from(cpu.regs.P);
                let next = tmp & !self.payload[0]; // Clear bits
                cpu.regs.P = StatusRegister::from(next);
            }
            Opcodes::PEA => {
                let low = self.payload[0];
                let high = self.payload[1];
                cpu.stack_push(high);
                cpu.stack_push(low);
            }
            Opcodes::PHB => cpu.stack_push(cpu.regs.DBR),
            Opcodes::LDX => {
                if cpu.regs.P.x != 1 {
                    let mut val = 0;
                    if self.address_mode == AddressModes::Immediate {
                        val = effective_address.unwrap().address;
                    } else {
                        val = mapper
                            .cartridge
                            .as_ref()
                            .unwrap()
                            .read_u16(effective_address.unwrap().address as usize);
                    }

                    // Set cpu flags accordingly
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }

                    if (val >> 7) == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }

                    cpu.regs.X = IndexRegister::from(val);
                } else {
                    let load_address = self.payload[0];

                    let val;
                    if self.address_mode == AddressModes::Immediate {
                        val = load_address;
                    } else {
                        val = mapper
                            .cartridge
                            .as_ref()
                            .unwrap()
                            .read_byte(load_address.try_into().unwrap());
                    }
                    // Set cpu flags accordingly
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }

                    if (val >> 7) == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    cpu.regs.X = IndexRegister::from(val);
                }
            }
            Opcodes::LDY => {
                if cpu.regs.P.x != 1 {
                    let mut val = 0;
                    if self.address_mode == AddressModes::Immediate {
                        val = effective_address.unwrap().address;
                    } else {
                        val = mapper
                            .cartridge
                            .as_ref()
                            .unwrap()
                            .read_u16(effective_address.unwrap().address as usize);
                    }

                    // Set cpu flags accordingly
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }

                    if (val >> 7) == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }

                    cpu.regs.Y = IndexRegister::from(val);
                } else {
                    let load_address = self.payload[0];

                    let val;
                    if self.address_mode == AddressModes::Immediate {
                        val = load_address;
                    } else {
                        val = mapper
                            .cartridge
                            .as_ref()
                            .unwrap()
                            .read_byte(load_address.try_into().unwrap());
                    }
                    // Set cpu flags accordingly
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }

                    if (val >> 7) == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    cpu.regs.Y = IndexRegister::from(val);
                }
            }
            Opcodes::TXS => {
                if cpu.e {
                    // TXS emu
                    cpu.regs.S.high = 1; // High byte stack pointer is always 1
                    if cpu.regs.P.x != 1 {
                        // 16Bit index
                        cpu.regs.S.low = cpu.regs.X.low;
                    } else {
                        cpu.regs.S.low = cpu.regs.X.low;
                        // 8Bit index
                    }
                } else {
                    // TXS native
                    if cpu.regs.P.x != 1 {
                        // 16Bit index
                        cpu.regs.S.high = cpu.regs.X.high;
                        cpu.regs.S.low = cpu.regs.X.low;
                    } else {
                        // 8Bit index
                        cpu.regs.S.high = 0;
                        cpu.regs.S.low = cpu.regs.X.low;
                    }
                }
            }
            Opcodes::TXY => {
                // 8-bit index registers
                if cpu.regs.P.x == 1 {
                    cpu.regs.Y.low = cpu.regs.X.low;
                    if cpu.regs.X.low == 0 {
                        cpu.regs.P.z = 1;
                    }
                    if (cpu.regs.X.low as u8) >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    }
                } else {
                    cpu.regs.Y = cpu.regs.X;
                    if cpu.regs.X.low == 0 && cpu.regs.X.high == 0 {
                        cpu.regs.P.z = 1;
                    }
                    if cpu.regs.X.high >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    }
                }
            }
            Opcodes::TYX => {
                // 8-bit index registers
                if cpu.regs.P.x == 1 {
                    cpu.regs.X.low = cpu.regs.Y.low;
                    if cpu.regs.Y.low == 0 {
                        cpu.regs.P.z = 1;
                    }
                    if (cpu.regs.Y.low as u8) >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    }
                } else {
                    cpu.regs.X = cpu.regs.Y;
                    if cpu.regs.Y.low == 0 && cpu.regs.Y.high == 0 {
                        cpu.regs.P.z = 1;
                    }
                    if (cpu.regs.Y.high as u8) >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    }
                }
            }
            Opcodes::JMP => {
                // TODO: At long jumping: Bank Mapping e.g. in HiRom is bank 80 - 9f  = 00 - 1f etc
                //  also do this in AddressMode module
                // TODO: Use Memmaper to handle program/databank register update and returning 16 Bit pc
                if follow_jumps {
                    cpu.regs.PC = effective_address.unwrap().address;
                }
            }
            Opcodes::JSR => {
                if follow_jumps {
                    let pc_low = (cpu.regs.PC & 0x00ff) as u8;
                    let pc_high = (cpu.regs.PC >> 8) as u8;

                    cpu.stack_push(pc_high);
                    cpu.stack_push(pc_low);

                    // let address = effective_address;
                    cpu.regs.PC = effective_address.unwrap().address;
                }
            }
            // TODO:
            Opcodes::RTI => {
                if cpu.e {
                    cpu.regs.P = StatusRegister::from(cpu.stack_pull());
                    let high = cpu.stack_pull();
                    let low = cpu.stack_pull();
                    cpu.regs.PC = low as u16 | (high as u16) << 8;
                } else {
                    cpu.regs.P = StatusRegister::from(cpu.stack_pull());
                    let bank = cpu.stack_pull();
                    let high = cpu.stack_pull();
                    let low = cpu.stack_pull();
                    cpu.regs.PC = low as u16 | (high as u16) << 8;
                    cpu.regs.PBR = bank;
                }
            }
            Opcodes::ROL => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = cpu.regs.C.A as u8
                    } else {
                        val = mapper.read(effective_address.unwrap());
                    }
                    let new_c = val >> 7;
                    let old_c = cpu.regs.P.c;
                    val = (val << 1) | old_c;
                    cpu.regs.P.c = new_c;
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C.A = val as u16;
                    } else {
                        mapper.write(effective_address.unwrap(), val);
                    }
                } else {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = u16::from(cpu.regs.C);
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                    }
                    let new_c = val >> 15;
                    let old_c = cpu.regs.P.c as u16;
                    val = (val << 1) | old_c;
                    cpu.regs.P.c = new_c as u8;
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C = Accumulator::from(val);
                    } else {
                        mapper.write(effective_address.unwrap(), (val & 0xf) as u8);
                        mapper.write(effective_address.unwrap().add(1), (val >> 8) as u8);
                    }
                }
            }
            Opcodes::RTS => {
                let op_low = cpu.stack_pull();
                let op_high = cpu.stack_pull();
                cpu.regs.PC = ((op_high as u16) << 8) | op_low as u16;
            }
            Opcodes::RTL => {
                let op_low = cpu.stack_pull();
                let op_high = cpu.stack_pull();
                let pbr = cpu.stack_pull();
                cpu.regs.PC = ((op_high as u16) << 8) | op_low as u16;
                cpu.regs.PBR = pbr;
            }
            Opcodes::LDA => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let val;
                    if self.address_mode == AddressModes::Immediate {
                        val = self.payload[0] as u16;
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u16;
                    }
                    cpu.regs.C.A = val;
                } else {
                    let val;
                    if self.address_mode == AddressModes::Immediate {
                        val = (self.payload[1] as u16) << 8 | self.payload[0] as u16;
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                    }
                    cpu.regs.C = Accumulator::from(val);
                }
            }
            Opcodes::LSR => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = u16::from(cpu.regs.C) as u8;
                    } else {
                        val = mapper.read(effective_address.unwrap());
                    }
                    // set carry bit
                    cpu.regs.P.c = val & 0x1;
                    cpu.regs.C = Accumulator::from((val >> 1) as u16);
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                } else {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = u16::from(cpu.regs.C);
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                    }
                    // set carry bit
                    cpu.regs.P.c = val as u8 & 1;
                    cpu.regs.C = Accumulator::from((val >> 1) as u16);
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                }
                cpu.regs.P.n = 0;
            }
            // TODO: TEST this!
            Opcodes::SBC => {
                // TODO: Decimal flag
                if cpu.regs.P.m == 1 || cpu.e {
                    // 8-Bit
                    if cpu.regs.C.A as i8 - (mapper.read(effective_address.unwrap()) as i8) < 0 {
                        cpu.regs.P.v = 1;
                    }
                    let mut data =
                        (cpu.regs.C.A as u8).wrapping_sub(mapper.read(effective_address.unwrap()));
                    if cpu.regs.P.c == 0 {
                        data = data.wrapping_sub(1);
                    }
                    if data == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    if data >> 7 == 1 {
                        cpu.regs.P.n = 1
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    cpu.regs.C.A = data as u16;
                } else {
                    let mut data_low = mapper.read(effective_address.unwrap());
                    let mut data_high = mapper.read(effective_address.unwrap().add(1));
                    if cpu.regs.C.A as i8 - (data_low as i8) < 0 {
                        // borrow required
                        data_high -= 1;
                        cpu.regs.P.c = 0;
                        cpu.regs.C.A = cpu.regs.C.A.wrapping_sub(data_low.into());
                        cpu.regs.C.B = cpu.regs.C.B.wrapping_sub(data_high.into());

                        if u16::from(cpu.regs.C) == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if u16::from(cpu.regs.C) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                        cpu.regs.P.v = 1;
                    } else {
                        cpu.regs.P.c = 1;
                        cpu.regs.C.A = cpu.regs.C.A.wrapping_sub(data_low.into());
                        cpu.regs.C.B = cpu.regs.C.B.wrapping_sub(data_high.into());

                        if u16::from(cpu.regs.C) == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if u16::from(cpu.regs.C) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                        cpu.regs.P.v = 0;
                    }
                }
            }
            Opcodes::STA => {
                // println!("STA ====>{:?}", self.payload);
                if cpu.e || cpu.regs.P.m == 1
                /*&& cpu.e*/
                {
                    mapper.write(effective_address.unwrap(), cpu.regs.C.A as u8);
                } else {
                    mapper.write(effective_address.unwrap(), cpu.regs.C.A as u8);
                    mapper.write(effective_address.unwrap().add(1), cpu.regs.C.B as u8);
                    // mapper.write_u16(effective_address, cpu.regs.C.try_into().unwrap());
                }
            }
            Opcodes::STZ => {
                mapper.write(effective_address.unwrap(), 0x0);
            }
            Opcodes::STX => {
                if cpu.e || cpu.regs.P.m == 1 {
                    mapper.write(effective_address.unwrap(), cpu.regs.X.low as u8);
                } else {
                    mapper.write(effective_address.unwrap(), cpu.regs.X.low as u8);
                    mapper.write(effective_address.unwrap().add(1), cpu.regs.X.high as u8);
                }
            }
            Opcodes::STY => {
                if cpu.e || cpu.regs.P.m == 1 {
                    mapper.write(effective_address.unwrap(), cpu.regs.Y.low as u8);
                } else {
                    mapper.write(effective_address.unwrap(), cpu.regs.Y.low as u8);
                    mapper.write(effective_address.unwrap().add(1), cpu.regs.Y.high as u8);
                }
            }
            Opcodes::TCS => {
                if cpu.e {
                    cpu.regs.S = IndexRegister::from(cpu.regs.C.A as u16);
                } else {
                    cpu.regs.S = IndexRegister::from(u16::from(cpu.regs.C));
                }
            }
            Opcodes::TAX => {
                if !cpu.e {
                    // native mode
                    // 8 Bit accumulator, 8 bit index registers
                    cpu.regs.X.low = cpu.regs.C.A;
                } else {
                    // 8 bit accumulator, 16 bit index registers
                    if cpu.regs.P.m == 1 && cpu.regs.P.x == 0 {
                        cpu.regs.X.low = cpu.regs.C.A;
                        cpu.regs.X.high = cpu.regs.C.B;
                    }
                    // 16 bit accumulator, 8 bit index registers
                    if cpu.regs.P.m == 0 && cpu.regs.P.x == 1 {
                        cpu.regs.X.low = cpu.regs.C.A;
                    }
                    if cpu.regs.P.m == 0 && cpu.regs.P.x == 0 {
                        cpu.regs.X.low = cpu.regs.C.A;
                        cpu.regs.X.high = cpu.regs.C.B;
                    }
                }
                if (cpu.regs.C.A >> 7) == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
                if cpu.regs.C.A == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
            }
            Opcodes::TAY => {
                if !cpu.e {
                    // native mode
                    // 8 Bit accumulator, 8 bit index registers
                    cpu.regs.Y.low = cpu.regs.C.A;
                } else {
                    // 8 bit accumulator, 16 bit index registers
                    if cpu.regs.P.m == 1 && cpu.regs.P.x == 0 {
                        cpu.regs.Y.low = cpu.regs.C.A;
                        cpu.regs.Y.high = cpu.regs.C.B;
                    }
                    // 16 bit accumulator, 8 bit index registers
                    if cpu.regs.P.m == 0 && cpu.regs.P.x == 1 {
                        cpu.regs.Y.low = cpu.regs.C.A;
                    }
                    if cpu.regs.P.m == 0 && cpu.regs.P.x == 0 {
                        cpu.regs.Y.low = cpu.regs.C.A;
                        cpu.regs.Y.high = cpu.regs.C.B;
                    }
                }
                if (cpu.regs.C.A >> 7) == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
                if cpu.regs.C.A == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
            }
            Opcodes::TCD => {
                cpu.regs.D = u16::from(cpu.regs.C);
                if cpu.regs.D >> 7 == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
                if cpu.regs.D == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
            }
            Opcodes::DEC => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = cpu.regs.C.A as u8;
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u8;
                    }
                    val = val.wrapping_sub(1);
                    if val >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C.A = val as u16;
                    } else {
                        mapper.write(effective_address.unwrap(), val);
                    }
                } else {
                    // TODO
                    let mut val;
                    if self.address_mode == AddressModes::Accumulator {
                        val = cpu.regs.C.A as u8;
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u8;
                    }
                    val -= 1;
                    if val >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C.A = val as u16;
                    } else {
                        mapper.write(effective_address.unwrap(), val);
                    }
                }
            }
            Opcodes::DEX => {
                let index: u16 = u16::from(cpu.regs.X).wrapping_sub(1);
                cpu.regs.X = IndexRegister::from(index);
            }
            Opcodes::DEY => {
                let index: u16 = u16::from(cpu.regs.Y).wrapping_sub(1);
                cpu.regs.Y = IndexRegister::from(index);
            }
            Opcodes::INX => {
                let index: u16 = u16::from(cpu.regs.X).wrapping_add(1);
                cpu.regs.X = IndexRegister::from(index);
            }
            Opcodes::INY => {
                let index: u16 = u16::from(cpu.regs.Y).wrapping_add(1);
                cpu.regs.Y = IndexRegister::from(index);
            }
            Opcodes::INC => {
                if cpu.e || cpu.regs.P.m == 1 {
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C.A += 1;
                        if u16::from(cpu.regs.C.A as u8) == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if u16::from(cpu.regs.C.A as u8) >> 7 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    } else {
                        // TODO: Wrapping?
                        let val = mapper.read(effective_address.unwrap()) + 1;
                        mapper.write(effective_address.unwrap(), val);
                        if val == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if val >> 7 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    }
                } else {
                    if self.address_mode == AddressModes::Accumulator {
                        cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) + 1);
                        if u16::from(cpu.regs.C) == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if u16::from(cpu.regs.C) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    } else {
                        // TODO: Wrapping?
                        let val_low = mapper.read(effective_address.unwrap());
                        let val_high = mapper.read(effective_address.unwrap().add(1));
                        mapper.write(effective_address.unwrap(), val_low);
                        mapper.write(effective_address.unwrap().add(1), val_high);

                        if val_low as u16 | (val_high as u16) << 8 == 0 {
                            cpu.regs.P.z = 1;
                        } else {
                            cpu.regs.P.z = 0;
                        }
                        if val_low as u16 | (val_high as u16) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    }
                }
            }
            Opcodes::BNE => {
                if cpu.regs.P.z == 1 {
                    return;
                } else {
                    cpu.regs.PC = effective_address.unwrap().address as _;
                }
            }
            Opcodes::PLD => {
                let low = cpu.stack_pull();
                let high = cpu.stack_pull();
                cpu.regs.D = low as u16 | (high as u16) << 8;

                if cpu.regs.D == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
                if cpu.regs.D >> 7 == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
            }
            Opcodes::PLP => {
                cpu.regs.P = StatusRegister::from(cpu.stack_pull());
            }
            Opcodes::PLX => {
                if cpu.e || cpu.regs.P.x == 1 {
                    cpu.regs.C.A = cpu.stack_pull() as u16;
                    cpu.regs.P.n = (cpu.regs.C.A as u8) >> 7;
                } else {
                    let low = cpu.stack_pull();
                    let high = cpu.stack_pull();
                    cpu.regs.C = Accumulator::from(u16::from(low as u16 | (high as u16) << 8));
                    if u16::from(cpu.regs.C) == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    if u16::from(cpu.regs.C) >> 15 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                }
            }
            Opcodes::PHB => {
                cpu.stack_push(cpu.regs.DBR);
            }
            Opcodes::PHP => {
                cpu.stack_push(u8::from(cpu.regs.P));
            }
            Opcodes::PHD => {
                cpu.stack_push(u8::from((cpu.regs.D >> 8) as u8));
                cpu.stack_push(u8::from((cpu.regs.D & 0x0f) as u8));
            }
            Opcodes::PHA => {
                if cpu.e || cpu.regs.P.m == 1 {
                    cpu.stack_push(cpu.regs.C.A as u8);
                } else {
                    cpu.stack_push(cpu.regs.C.B as u8);
                    cpu.stack_push(cpu.regs.C.A as u8);
                }
            }
            Opcodes::PHK => {
                cpu.stack_push(cpu.regs.PBR);
            }
            Opcodes::PHX => {
                if cpu.e || cpu.regs.P.x == 1 {
                    cpu.stack_push(cpu.regs.X.low as u8);
                } else {
                    cpu.stack_push(cpu.regs.X.high as u8);
                    cpu.stack_push(cpu.regs.X.low as u8);
                }
            }
            Opcodes::PLA => {
                if cpu.e || cpu.regs.P.m == 1 {
                    cpu.regs.C.A = cpu.stack_pull() as u16;
                    if (cpu.regs.C.A as u8) >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if cpu.regs.C.A == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                } else {
                    let low = cpu.stack_pull();
                    let high = cpu.stack_pull();
                    cpu.regs.C = Accumulator::from(low as u16 | (high as u16) << 8);
                    if u16::from(cpu.regs.C) >> 15 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if u16::from(cpu.regs.C) == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                }
            }
            Opcodes::PLB => {
                cpu.regs.DBR = cpu.stack_pull();
                if cpu.regs.DBR >> 7 == 1 {
                    cpu.regs.P.n = 1;
                } else {
                    cpu.regs.P.n = 0;
                }
                if cpu.regs.DBR == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
            }
            Opcodes::MVN => {
                let (src_bnk, dest_bnk) = (self.payload[1], self.payload[0]);

                loop {
                    if cpu.regs.C == Accumulator::from(0xffffu16) {
                        break;
                    }
                    let source = u16::from(cpu.regs.X);
                    let dest = u16::from(cpu.regs.Y);
                    let length = u16::from(cpu.regs.C);

                    let src_address = Address {
                        bank: src_bnk,
                        address: source,
                    };
                    let val = mapper.read(src_address);
                    let dest_address = Address {
                        bank: dest_bnk,
                        address: dest,
                    };
                    mapper.write(dest_address, val);

                    // print!("{:x} : {:?}|", val, address);
                    cpu.regs.X = IndexRegister::from(u16::from(cpu.regs.X).wrapping_add(1));
                    cpu.regs.Y = IndexRegister::from(u16::from(cpu.regs.Y).wrapping_add(1));
                    cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C).wrapping_sub(1));
                }

                // panic!("src: {} : {} dest: {} : {} count: {}", src_bnk, source, dest_bnk, dest, length);/
            }
            Opcodes::XBA => {
                let temp = cpu.regs.C.B;
                cpu.regs.C.B = cpu.regs.C.A;
                cpu.regs.C.A = temp;
            }
            Opcodes::ORA => {
                if cpu.e || cpu.regs.P.m == 1 {
                    let val;
                    if self.address_mode == AddressModes::Immediate {
                        val = self.payload[0];
                    } else {
                        val = mapper.read(effective_address.unwrap());
                    }
                    cpu.regs.C.A = cpu.regs.C.A | val as u16;
                    if u16::from(cpu.regs.C.A) as u8 >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if cpu.regs.C.A == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                } else {
                    // 16 bit
                    let val;
                    if self.address_mode == AddressModes::Immediate {
                        val = self.payload[0] as u16 | (self.payload[1] as u16) << 8;
                    } else {
                        val = mapper.read(effective_address.unwrap()) as u16
                            | (mapper.read(effective_address.unwrap().add(1)) as u16) << 8;
                    }
                    cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) | val as u16);
                    if u16::from(cpu.regs.C) >> 15 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }

                    if u16::from(cpu.regs.C) == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                }
            }
            _ => {
                unimplemented!(
                    "{:?} {:?} payload: {:?}",
                    &self.opcode,
                    &self.address_mode,
                    &self.payload,
                );
            }
        }
    }

    pub fn print_info(&self) {
        println!(
            "0x{:x}: {:?} {:?} {:?}",
            self.address, self.opcode, self.payload, self.address_mode
        );
    }
}
