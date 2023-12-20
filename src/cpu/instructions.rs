use crate::cpu::addressmodes::AddressModes;
use crate::cpu::decoder::Opcodes;
use crate::cpu::CPU;
// use crate::cpu::
use crate::cpu::{Accumulator, IndexRegister, StatusRegister};
use crate::mem::Bus;

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

#[derive(Debug)]
enum Operant_Value {
    short(u8),
    long(u32),
}

impl Operant_Value {
    fn lower_to_number(&self) -> u32 {
        match *self {
            Operant_Value::short(a) => {
                return a as u32;
            }
            Operant_Value::long(a) => {
                return a;
            }
        }
    }
}

fn is_16_bit_mem_and_accu(cpu: &CPU) -> bool {
    !cpu.e && cpu.regs.P.m != 1
}
fn is_16_bit_index(cpu: &CPU) -> bool {
    !cpu.e && cpu.regs.P.x != 1
}

impl Instruction {
    pub fn new() -> Instruction {
        let inst = Instruction::default();
        inst
    }

    fn load_value(&self, cpu: &mut CPU, bus: &Bus) -> Option<Operant_Value> {
        // if self.address_mode == AddressModes::Immediate {
        //     if is_16_bit_mem_and_accu(cpu) && self.address_mode.len(cpu.get_regs(), &self.opcode) > 3 {
        //         return Some(Operant_Value::long(self.payload[0] as u16 | (self.payload[1] as u16) << 8));
        //     } else {
        //         return Some(Operant_Value::short(self.payload[0]));
        //     }
        // } else {
        // get address
        let address =
            self.address_mode
                .get_effective_address(cpu, &self.payload, &self.opcode, bus);
        if address.is_some() {
            if !is_16_bit_mem_and_accu(cpu) {
                let val = bus.read(address.unwrap());
                return Some(Operant_Value::short(val));
            } else {
                let val =
                    bus.read(address.unwrap()) as u32 | (bus.read(address.unwrap()) as u32) << 8; //| (cpu.regs.PBR as u32) << 16;
                return Some(Operant_Value::long(val));
            }
        } else {
            return None;
        }
        // }
    }

    // TODO: Make this clean.
    // 1. Introduce variable value
    // 2. Is addressing mode immediate?
    // - if yes then payload is value
    // - if not then load value from address and save it to value
    pub fn execute(&mut self, cpu: &mut CPU, bus: &mut Bus, follow_jumps: bool) {
        // if this is None it's implied addressing
        let value;
        if self.address_mode != AddressModes::Immediate {
            value = self.load_value(cpu, bus);
        } else {
            let address =
                self.address_mode
                    .get_effective_address(cpu, &self.payload, &self.opcode, bus);

            if let Some(a) = address {
                print!("{:?}", <u16>::from(a));
                value = Some(Operant_Value::long(<u16>::from(a) as u32));
            } else {
                value = None;
            }
        }
        println!("{:?}", value);

        let effective_address =
            self.address_mode
                .get_effective_address(cpu, &self.payload, &self.opcode, bus);

        match &self.opcode {
            // TODO: handle immediate addressing early in such a way that the folloing patterns use it transparently/agnistically
            Opcodes::AND => {
                // TODO: This needs to become a function returning an enum having either a 8 or 16 bit value
                match value.unwrap() {
                    Operant_Value::short(val) => {
                        cpu.regs.C = Accumulator::from((cpu.regs.C.A as u8 & val) as u16);
                        if u16::from(cpu.regs.C) >> 8 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    }
                    Operant_Value::long(val) => {
                        cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) & val as u16);
                        if u16::from(cpu.regs.C) >> 15 == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    }
                }
                if u16::from(cpu.regs.C) == 0 {
                    cpu.regs.P.z = 1;
                } else {
                    cpu.regs.P.z = 0;
                }
            }
            // TODO: test this
            // Opcodes::ADC => {
            //     match operant.unwrap() {
            //         Operant_Value::short(value) => {
            //             if cpu.regs.C.A as u16 + (value as u16) > 255 {
            //                 cpu.regs.P.v = 1;
            //             }
            //             if cpu.regs.P.c == 0 {
            //                 value += 1;
            //             }
            //             if value == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if value >> 7 == 1 {
            //                 cpu.regs.P.n = 1
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //             cpu.regs.C.A = value as u16;
            //         }
            //         Operant_Value::long(value) => {
            //              if cpu.regs.C.A as u16 + (value as u16) > 255 {
            //                 cpu.regs.P.v = 1;
            //             }
            //             if cpu.regs.P.c == 0 {
            //                 value += 1;
            //             }
            //             if value == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if value >> 7 == 1 {
            //                 cpu.regs.P.n = 1
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //             cpu.regs.C.A = value as u16;
            //         }
            //     }
            //     // TODO: Decimal flag
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         // 8-Bit

            //         let mut data =
            //             (cpu.regs.C.A as u8).wrapping_add(bus.read(effective_address.unwrap()));
            //     } else {
            //         let mut data_low = bus.read(effective_address.unwrap());
            //         let mut data_high = bus.read(effective_address.unwrap().add(1));
            //         if cpu.regs.C.A as u16 + (data_low as u16) > 255 {
            //             // borrow required
            //             data_high += 1;
            //             cpu.regs.P.c = 0;
            //             cpu.regs.C.A = cpu.regs.C.A.wrapping_add(data_low.into());
            //             cpu.regs.C.B = cpu.regs.C.B.wrapping_add(data_high.into());

            //             if u16::from(cpu.regs.C) == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if u16::from(cpu.regs.C) >> 15 == 1 {
            //                 cpu.regs.P.n = 1;
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //             cpu.regs.P.v = 1;
            //         } else {
            //             cpu.regs.P.c = 1;
            //             cpu.regs.C.A = cpu.regs.C.A.wrapping_add(data_low.into());
            //             cpu.regs.C.B = cpu.regs.C.B.wrapping_add(data_high.into());

            //             if u16::from(cpu.regs.C) == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if u16::from(cpu.regs.C) >> 15 == 1 {
            //                 cpu.regs.P.n = 1;
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //             cpu.regs.P.v = 0;
            //         }
            //     }
            // }
            // Opcodes::ASL => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = cpu.regs.C.A as u8;
            //         } else {
            //             // shift val located at effective_address
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         val = val << 1;
            //         let msb = val >> 7;
            //         if msb == 1 {
            //             cpu.regs.P.n = 1;
            //         } else {
            //             cpu.regs.P.n = 0;
            //         }
            //         cpu.regs.P.c = msb;
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C.A = val as u16
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     } else {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(cpu.regs.C);
            //         } else {
            //             // shift val located at effective_address
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | ((bus.read(effective_address.unwrap().add(1)) as u16) << 8)
            //         }
            //         val = val << 1;
            //         let msb = val >> 15;
            //         if msb == 1 {
            //             cpu.regs.P.n = 1;
            //         } else {
            //             cpu.regs.P.n = 0;
            //         }
            //         cpu.regs.P.c = msb as u8;
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C = Accumulator::from(val);
            //         } else {
            //             bus.write(effective_address.unwrap(), (val & 0x0f) as u8);
            //             bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
            //         }
            //     }
            // }
            Opcodes::BIT => {
                if !is_16_bit_mem_and_accu(cpu) {
                    // if cpu.e || cpu.regs.P.m == 0 {
                    // if self.address_mode == AddressModes::Immediate {
                    //     if self.payload[0] == 0 {
                    //         cpu.regs.P.z = 1;
                    //     } else {
                    //         cpu.regs.P.z = 0;
                    //     }
                    // } else {
                    // let mut val = bus.read(effective_address.unwrap());
                    let mut val = value.unwrap().lower_to_number() as u8;
                    cpu.regs.P.n = (val >> 7) & 0x1;
                    cpu.regs.P.v = (val >> 6) & 0x1;
                    val = val & (cpu.regs.C.A as u8);
                    cpu.regs.P.z = !val;
                    // }
                } else {
                    // if self.address_mode == AddressModes::Immediate {
                    //     if self.payload[0] as u16 | (self.payload[1] as u16) << 8 == 0 {
                    //         cpu.regs.P.z = 1;
                    //     } else {
                    //         cpu.regs.P.z = 0;
                    //     }
                    // } else {
                    let mut val = value.unwrap().lower_to_number() as u16;
                    cpu.regs.P.n = ((val >> 15) & 0x1) as u8;
                    cpu.regs.P.v = ((val >> 14) & 0x1) as u8;
                    val = val & u16::from(cpu.regs.C);
                    cpu.regs.P.z = !(val as u8);
                    // }
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
                // let val;
                if !is_16_bit_mem_and_accu(cpu) {
                    let val = value.unwrap().lower_to_number() as u8;
                    let res = (u16::from(cpu.regs.C.A) as u8) - val;
                    if res >> 7 == 1 {
                        cpu.regs.P.n = 1;
                    } else {
                        cpu.regs.P.n = 0;
                    }
                    if res == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }
                    if u16::from(cpu.regs.C.A) as u8 >= val {
                        cpu.regs.P.c = 1;
                    } else {
                        cpu.regs.P.c = 0;
                    }
                } else {
                    let val = value.unwrap().lower_to_number();
                    let res = u16::from(cpu.regs.C) - val as u16;
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
                    if u16::from(cpu.regs.C) >= val as u16 {
                        cpu.regs.P.c = 1;
                    } else {
                        cpu.regs.P.c = 0;
                    }
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
                if !is_16_bit_index(cpu) {
                    // if cpu.e || cpu.regs.P.x == 1 {
                    let val;
                    // if self.address_mode != AddressModes::Immediate {
                    val = value.unwrap().lower_to_number() as u8;
                    // } else {
                    //     val = self.payload[0];
                    // }
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
                    // if self.address_mode != AddressModes::Immediate {
                    //     val = bus.read(effective_address.unwrap()) as u16
                    //         | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
                    // } else {
                    val = value.unwrap().lower_to_number();
                    // }
                    let bar = <u16>::from(cpu.regs.X).wrapping_sub(val as u16);
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
            // Opcodes::EOR => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         let val = bus.read(effective_address.unwrap());
            //         cpu.regs.C =
            //             Accumulator::from(cpu.regs.C.B | (cpu.regs.C.A as u8 ^ val) as u16);
            //     } else {
            //         let val = bus.read(effective_address.unwrap()) as u16
            //             | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //         cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) ^ val);
            //     }
            // }
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
                    // if self.address_mode == AddressModes::Immediate {
                    //     val = effective_address.unwrap().address;
                    // } else {
                    match value.unwrap() {
                        Operant_Value::short(v) => {
                            val = v as i32;
                        }
                        Operant_Value::long(v) => {
                            val = v as i32;
                        }
                    }
                    // }

                    // Set cpu flags accordingly
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }

                    if is_16_bit_mem_and_accu(cpu) {
                        if (val >> 15) == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    } else {
                        if (val >> 7) == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    }
                    if is_16_bit_index(cpu) {
                        cpu.regs.X = IndexRegister::from(val as u16);
                    } else {
                        cpu.regs.X = IndexRegister::from(val as u8);
                    }
                }
            }
            Opcodes::LDY => {
                if cpu.regs.P.x != 1 {
                    let mut val = 0;
                    // if self.address_mode == AddressModes::Immediate {
                    //     val = effective_address.unwrap().address;
                    // } else {
                    match value.unwrap() {
                        Operant_Value::short(v) => {
                            val = v as i32;
                        }
                        Operant_Value::long(v) => {
                            val = v as i32;
                        }
                    }
                    // }

                    // Set cpu flags accordingly
                    if val == 0 {
                        cpu.regs.P.z = 1;
                    } else {
                        cpu.regs.P.z = 0;
                    }

                    if is_16_bit_mem_and_accu(cpu) {
                        if (val >> 15) == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    } else {
                        if (val >> 7) == 1 {
                            cpu.regs.P.n = 1;
                        } else {
                            cpu.regs.P.n = 0;
                        }
                    }
                    if is_16_bit_index(cpu) {
                        cpu.regs.Y = IndexRegister::from(val as u16);
                    } else {
                        cpu.regs.Y = IndexRegister::from(val as u8);
                    }
                }
            }
            Opcodes::NOP => {
                // No need to increment.. this is done by the iterator
                // cpu.get_regs().PC += 1;
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
                if follow_jumps {
                    cpu.regs.PC = effective_address.unwrap().address;
                    if self.address_mode == AddressModes::AbsoluteLong {
                        cpu.regs.PBR = effective_address.unwrap().bank;
                    }
                    // cpu.
                    // | (cpu.regs.DBR as u16) << 16;
                    // cpu.regs.PC = value.unwrap().lower_to_number() as u16;
                }
            }
            Opcodes::JSR => {
                if follow_jumps {
                    let pc_low = (cpu.regs.PC & 0x00ff) as u8;
                    let pc_high = (cpu.regs.PC >> 8) as u8;
                    if self.address_mode == AddressModes::AbsoluteLong {
                        let bank = cpu.regs.PBR;
                        cpu.stack_push(bank);
                    }

                    cpu.stack_push(pc_high);
                    cpu.stack_push(pc_low);

                    // let address = effective_address
                    if self.address_mode == AddressModes::AbsoluteLong {
                        cpu.regs.PBR = effective_address.unwrap().bank;
                    }
                    cpu.regs.PC = effective_address.unwrap().address;
                    // cpu.regs.PC = value.unwrap().lower_to_number() as u16;
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
            // Opcodes::ROL => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = cpu.regs.C.A as u8
            //         } else {
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         let new_c = val >> 7;
            //         let old_c = cpu.regs.P.c;
            //         val = (val << 1) | old_c;
            //         cpu.regs.P.c = new_c;
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C.A = val as u16;
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     } else {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(cpu.regs.C);
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //         }
            //         let new_c = val >> 15;
            //         let old_c = cpu.regs.P.c as u16;
            //         val = (val << 1) | old_c;
            //         cpu.regs.P.c = new_c as u8;
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C = Accumulator::from(val);
            //         } else {
            //             bus.write(effective_address.unwrap(), (val & 0xf) as u8);
            //             bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
            //         }
            //     }
            // }
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
                // if !is_16_bit_mem_and_accu(cpu) {
                let val;
                let msb;
                if self.address_mode == AddressModes::Immediate {
                    if is_16_bit_mem_and_accu(cpu) {
                        val = self.payload[0] as u16 | (self.payload[1] as u16) << 8;
                        msb = val >> 15;
                        cpu.regs.C = Accumulator::from(val);
                    } else {
                        val = self.payload[0] as u16;
                        msb = val >> 7;
                        cpu.regs.C.A = val;
                    }
                } else {
                    match value.unwrap() {
                        Operant_Value::long(v) => {
                            val = v as u16;
                            msb = (v as u16) >> 15;
                            cpu.regs.C = Accumulator::from(v as u16)
                        }
                        Operant_Value::short(v) => {
                            msb = (v >> 7) as u16;
                            val = v as u16;
                            cpu.regs.C.A = v.into();
                        }
                    }
                }

                if val == 0 {
                    cpu.get_regs().P.z = 1;
                } else {
                    cpu.get_regs().P.z = 0;
                }
                if msb == 1 {
                    cpu.get_regs().P.n = 1;
                } else {
                    cpu.get_regs().P.n = 0;
                }
                // }
            }
            // Opcodes::LSR => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(cpu.regs.C) as u8;
            //         } else {
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         // set carry bit
            //         cpu.regs.P.c = val & 0x1;
            //         cpu.regs.C = Accumulator::from((val >> 1) as u16);
            //         if val == 0 {
            //             cpu.regs.P.z = 1;
            //         } else {
            //             cpu.regs.P.z = 0;
            //         }
            //     } else {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(cpu.regs.C);
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //         }
            //         // set carry bit
            //         cpu.regs.P.c = val as u8 & 1;
            //         cpu.regs.C = Accumulator::from((val >> 1) as u16);
            //         if val == 0 {
            //             cpu.regs.P.z = 1;
            //         } else {
            //             cpu.regs.P.z = 0;
            //         }
            //     }
            //     cpu.regs.P.n = 0;
            // }
            // TODO: TEST this!
            Opcodes::SBC => {
                // TODO: Decimal flag
                if cpu.regs.P.m == 1 || cpu.e {
                    // 8-Bit
                    if cpu.regs.C.A as i8 - (bus.read(effective_address.unwrap()) as i8) < 0 {
                        cpu.regs.P.v = 1;
                    }
                    let mut data =
                        (cpu.regs.C.A as u8).wrapping_sub(bus.read(effective_address.unwrap()));
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
                    let data_low = bus.read(effective_address.unwrap());
                    let mut data_high = bus.read(effective_address.unwrap().add(1));
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
                if is_16_bit_mem_and_accu(cpu) {
                    bus.write(effective_address.unwrap(), cpu.regs.C.A as u8);
                } else {
                    bus.write(effective_address.unwrap(), cpu.regs.C.A as u8);
                    bus.write(effective_address.unwrap().add(1), cpu.regs.C.B as u8);
                    // bus.write_u16(effective_address, cpu.regs.C.try_into().unwrap());
                }
            }
            Opcodes::STZ => {
                bus.write(effective_address.unwrap(), 0x0);
                // reset zero flag
                cpu.get_regs().P.z = 0;
            }
            Opcodes::STX => {
                if is_16_bit_index(cpu) {
                    bus.write(effective_address.unwrap(), cpu.regs.X.low as u8);
                } else {
                    bus.write(effective_address.unwrap(), cpu.regs.X.low as u8);
                    bus.write(effective_address.unwrap().add(1), cpu.regs.X.high as u8);
                }
            }
            Opcodes::STY => {
                if is_16_bit_index(cpu) {
                    bus.write(effective_address.unwrap(), cpu.regs.Y.low as u8);
                } else {
                    bus.write(effective_address.unwrap(), cpu.regs.Y.low as u8);
                    bus.write(effective_address.unwrap().add(1), cpu.regs.Y.high as u8);
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
            // Opcodes::DEC => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = cpu.regs.C.A as u8;
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u8;
            //         }
            //         val = val.wrapping_sub(1);
            //         if val >> 7 == 1 {
            //             cpu.regs.P.n = 1;
            //         } else {
            //             cpu.regs.P.n = 0;
            //         }
            //         if val == 0 {
            //             cpu.regs.P.z = 1;
            //         } else {
            //             cpu.regs.P.z = 0;
            //         }
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C.A = val as u16;
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     } else {
            //         // TODO
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = cpu.regs.C.A as u8;
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u8;
            //         }
            //         val -= 1;
            //         if val >> 7 == 1 {
            //             cpu.regs.P.n = 1;
            //         } else {
            //             cpu.regs.P.n = 0;
            //         }
            //         if val == 0 {
            //             cpu.regs.P.z = 1;
            //         } else {
            //             cpu.regs.P.z = 0;
            //         }
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C.A = val as u16;
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     }
            // }
            // TODO: SET STATUS FLAGS!!!!
            Opcodes::DEX => {
                let index: u16 = u16::from(cpu.regs.X).wrapping_sub(1);
                cpu.regs.X = IndexRegister::from(index);
                if u16::from(cpu.get_regs().X) == 0 {
                    cpu.get_regs().P.z = 1;
                }
                if is_16_bit_index(cpu) {
                    if u16::from(cpu.get_regs().X.get_low()) >> 7 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                } else {
                    if u16::from(cpu.get_regs().X) >> 15 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                }
            }
            Opcodes::DEY => {
                let index: u16 = u16::from(cpu.regs.Y).wrapping_sub(1);
                cpu.regs.Y = IndexRegister::from(index);
                if u16::from(cpu.get_regs().Y) == 0 {
                    cpu.get_regs().P.z = 1;
                }
                if is_16_bit_index(cpu) {
                    if u16::from(cpu.get_regs().Y.get_low()) >> 7 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                } else {
                    if u16::from(cpu.get_regs().Y) >> 15 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                }
            }
            Opcodes::INX => {
                let index: u16 = u16::from(cpu.regs.X).wrapping_add(1);
                cpu.regs.X = IndexRegister::from(index);
                if u16::from(cpu.get_regs().X) == 0 {
                    cpu.get_regs().P.z = 1;
                }
                if is_16_bit_index(cpu) {
                    if u16::from(cpu.get_regs().X.get_low()) >> 7 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                } else {
                    if u16::from(cpu.get_regs().X) >> 15 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                }
            }
            Opcodes::INY => {
                let index: u16 = u16::from(cpu.regs.Y).wrapping_add(1);
                cpu.regs.Y = IndexRegister::from(index);
                if u16::from(cpu.get_regs().Y) == 0 {
                    cpu.get_regs().P.z = 1;
                }
                if is_16_bit_index(cpu) {
                    if u16::from(cpu.get_regs().X.get_low()) >> 7 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                } else {
                    if u16::from(cpu.get_regs().X) >> 15 == 1 {
                        cpu.get_regs().P.z = 1;
                    }
                }
            }
            // Opcodes::INC => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C.A += 1;
            //             if u16::from(cpu.regs.C.A as u8) == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if u16::from(cpu.regs.C.A as u8) >> 7 == 1 {
            //                 cpu.regs.P.n = 1;
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //         } else {
            //             // TODO: Wrapping?
            //             let val = bus.read(effective_address.unwrap()) + 1;
            //             bus.write(effective_address.unwrap(), val);
            //             if val == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if val >> 7 == 1 {
            //                 cpu.regs.P.n = 1;
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //         }
            //     } else {
            //         if self.address_mode == AddressModes::Accumulator {
            //             cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) + 1);
            //             if u16::from(cpu.regs.C) == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if u16::from(cpu.regs.C) >> 15 == 1 {
            //                 cpu.regs.P.n = 1;
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //         } else {
            //             // TODO: Wrapping?
            //             let val_low = bus.read(effective_address.unwrap());
            //             let val_high = bus.read(effective_address.unwrap().add(1));
            //             bus.write(effective_address.unwrap(), val_low);
            //             bus.write(effective_address.unwrap().add(1), val_high);

            //             if val_low as u16 | (val_high as u16) << 8 == 0 {
            //                 cpu.regs.P.z = 1;
            //             } else {
            //                 cpu.regs.P.z = 0;
            //             }
            //             if val_low as u16 | (val_high as u16) >> 15 == 1 {
            //                 cpu.regs.P.n = 1;
            //             } else {
            //                 cpu.regs.P.n = 0;
            //             }
            //         }
            //     }
            // }
            Opcodes::BNE => {
                if cpu.regs.P.z == 1 {
                    return;
                } else {
                    // cpu.regs.PC = effective_address.unwrap().address as _;
                    cpu.regs.PC = (cpu.regs.PC as i16 + (self.payload[0] as i8) as i16) as u16;
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
                    let _length = u16::from(cpu.regs.C);

                    let src_address = Address {
                        bank: src_bnk,
                        address: source,
                    };
                    let val = bus.read(src_address);
                    let dest_address = Address {
                        bank: dest_bnk,
                        address: dest,
                    };
                    bus.write(dest_address, val);

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
            // Opcodes::ORA => {
            //     if cpu.e || cpu.regs.P.m == 1 {
            //         let val;
            //         if self.address_mode == AddressModes::Immediate {
            //             val = self.payload[0];
            //         } else {
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         cpu.regs.C.A = cpu.regs.C.A | val as u16;
            //         if u16::from(cpu.regs.C.A) as u8 >> 7 == 1 {
            //             cpu.regs.P.n = 1;
            //         } else {
            //             cpu.regs.P.n = 0;
            //         }
            //         if cpu.regs.C.A == 0 {
            //             cpu.regs.P.z = 1;
            //         } else {
            //             cpu.regs.P.z = 0;
            //         }
            //     } else {
            //         // 16 bit
            //         let val;
            //         if self.address_mode == AddressModes::Immediate {
            //             val = self.payload[0] as u16 | (self.payload[1] as u16) << 8;
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //         }
            //         cpu.regs.C = Accumulator::from(u16::from(cpu.regs.C) | val as u16);
            //         if u16::from(cpu.regs.C) >> 15 == 1 {
            //             cpu.regs.P.n = 1;
            //         } else {
            //             cpu.regs.P.n = 0;
            //         }

            //         if u16::from(cpu.regs.C) == 0 {
            //             cpu.regs.P.z = 1;
            //         } else {
            //             cpu.regs.P.z = 0;
            //         }
            //     }
            // }
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
