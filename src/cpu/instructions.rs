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

//fn is_16_bit_mem_and_accu(cpu: &CPU) -> bool {
//    !cpu.e && cpu.regs.borrow().P.m != 1
//}
//fn is_16_bit_index(cpu: &CPU) -> bool {
//    !cpu.e && cpu.regs.borrow().P.x != 1
//}

impl Instruction {
    pub fn new() -> Instruction {
        let inst = Instruction::default();
        inst
    }

    pub fn load_value(&self, bus: &Bus) -> Option<Operant_Value> {
        // if self.address_mode == AddressModes::Immediate {
        //     if is_16_bit_mem_and_accu(bus.cpu) && self.address_mode.len(regs, &self.opcode) > 3 {
        //         return Some(Operant_Value::long(self.payload[0] as u16 | (self.payload[1] as u16) << 8));
        //     } else {
        //         return Some(Operant_Value::short(self.payload[0]));
        //     }
        // } else {
        // get address
        let address =
            self.address_mode
                .get_effective_address(&bus.cpu, &self.payload, &self.opcode, bus);
        if address.is_some() {
            if !&bus.cpu.is_16_bit_mem_and_accu() {
                let val = bus.read(address.unwrap());
                return Some(Operant_Value::short(val));
            } else {
                let val =
                    bus.read(address.unwrap()) as u32 | (bus.read(address.unwrap()) as u32) << 8; //| (regs.PBR as u32) << 16;
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
    pub fn execute(&mut self, bus: &mut Bus, follow_jumps: bool) {
        // if this is None it's implied addressing
        let value;
        if self.address_mode != AddressModes::Immediate {
            value = self.load_value(bus);
        } else {
            let address =
                self.address_mode
                    .get_effective_address(&bus.cpu, &self.payload, &self.opcode, bus);

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
                .get_effective_address(&bus.cpu, &self.payload, &self.opcode, bus);

        let is_16_bit_mem_and_accu = bus.cpu.is_16_bit_mem_and_accu();
        let is_16_bit_index_register = bus.cpu.is_16_bit_index();
        //        let regs = bus.cpu.regs.borrow();

        match &self.opcode {
            // TODO: handle immediate addressing early in such a way that the folloing patterns use it transparently/agnistically
            Opcodes::AND => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // TODO: This needs to become a function returning an enum having either a 8 or 16 bit value
                match value.unwrap() {
                    Operant_Value::short(val) => {
                        regs.C = Accumulator::from((bus.cpu.regs.borrow().C.A as u8 & val) as u16);
                        if u16::from(regs.C) >> 8 == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                    }
                    Operant_Value::long(val) => {
                        regs.C = Accumulator::from(u16::from(bus.cpu.regs.borrow().C) & val as u16);
                        if u16::from(regs.C) >> 15 == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                    }
                }
                if u16::from(regs.C) == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
            }
            // TODO: test this
            // Opcodes::ADC => {
            //     match operant.unwrap() {
            //         Operant_Value::short(value) => {
            //             if regs.C.A as u16 + (value as u16) > 255 {
            //                regs.P.v = 1;
            //             }
            //             if regs.P.c == 0 {
            //                 value += 1;
            //             }
            //             if value == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if value >> 7 == 1 {
            //                regs.P.n = 1
            //             } else {
            //                regs.P.n = 0;
            //             }
            //            regs.C.A = value as u16;
            //         }
            //         Operant_Value::long(value) => {
            //              if regs.C.A as u16 + (value as u16) > 255 {
            //                regs.P.v = 1;
            //             }
            //             if regs.P.c == 0 {
            //                 value += 1;
            //             }
            //             if value == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if value >> 7 == 1 {
            //                regs.P.n = 1
            //             } else {
            //                regs.P.n = 0;
            //             }
            //            regs.C.A = value as u16;
            //         }
            //     }
            //     // TODO: Decimal flag
            //     if bus.cpu.e || regs.P.m == 1 {
            //         // 8-Bit

            //         let mut data =
            //             (regs.C.A as u8).wrapping_add(bus.read(effective_address.unwrap()));
            //     } else {
            //         let mut data_low = bus.read(effective_address.unwrap());
            //         let mut data_high = bus.read(effective_address.unwrap().add(1));
            //         if regs.C.A as u16 + (data_low as u16) > 255 {
            //             // borrow required
            //             data_high += 1;
            //            regs.P.c = 0;
            //            regs.C.A = bus.cpu.regs.borrow().C.A.wrapping_add(data_low.into());
            //            regs.C.B = bus.cpu.regs.borrow().C.B.wrapping_add(data_high.into());

            //             if u16::from(regs.C) == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if u16::from(regs.C) >> 15 == 1 {
            //                regs.P.n = 1;
            //             } else {
            //                regs.P.n = 0;
            //             }
            //            regs.P.v = 1;
            //         } else {
            //            regs.P.c = 1;
            //            regs.C.A = bus.cpu.regs.borrow().C.A.wrapping_add(data_low.into());
            //            regs.C.B = bus.cpu.regs.borrow().C.B.wrapping_add(data_high.into());

            //             if u16::from(regs.C) == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if u16::from(regs.C) >> 15 == 1 {
            //                regs.P.n = 1;
            //             } else {
            //                regs.P.n = 0;
            //             }
            //            regs.P.v = 0;
            //         }
            //     }
            // }
            // Opcodes::ASL => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val =regs.C.A as u8;
            //         } else {
            //             // shift val located at effective_address
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         val = val << 1;
            //         let msb = val >> 7;
            //         if msb == 1 {
            //            regs.P.n = 1;
            //         } else {
            //            regs.P.n = 0;
            //         }
            //        regs.P.c = msb;
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C.A = val as u16
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     } else {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(regs.C);
            //         } else {
            //             // shift val located at effective_address
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | ((bus.read(effective_address.unwrap().add(1)) as u16) << 8)
            //         }
            //         val = val << 1;
            //         let msb = val >> 15;
            //         if msb == 1 {
            //            regs.P.n = 1;
            //         } else {
            //            regs.P.n = 0;
            //         }
            //        regs.P.c = msb as u8;
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C = Accumulator::from(val);
            //         } else {
            //             bus.write(effective_address.unwrap(), (val & 0x0f) as u8);
            //             bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
            //         }
            //     }
            // }
            Opcodes::BIT => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if !is_16_bit_mem_and_accu {
                    // if bus.cpu.e || regs.P.m == 0 {
                    // if self.address_mode == AddressModes::Immediate {
                    //     if self.payload[0] == 0 {
                    //        regs.P.z = 1;
                    //     } else {
                    //        regs.P.z = 0;
                    //     }
                    // } else {
                    // let mut val = bus.read(effective_address.unwrap());
                    let mut val = value.unwrap().lower_to_number() as u8;
                    regs.P.n = (val >> 7) & 0x1;
                    regs.P.v = (val >> 6) & 0x1;
                    val = val & (regs.C.A as u8);
                    regs.P.z = !val;
                    // }
                } else {
                    // if self.address_mode == AddressModes::Immediate {
                    //     if self.payload[0] as u16 | (self.payload[1] as u16) << 8 == 0 {
                    //        regs.P.z = 1;
                    //     } else {
                    //        regs.P.z = 0;
                    //     }
                    // } else {
                    let mut val = value.unwrap().lower_to_number() as u16;
                    regs.P.n = ((val >> 15) & 0x1) as u8;
                    regs.P.v = ((val >> 14) & 0x1) as u8;
                    val = val & u16::from(regs.C);
                    regs.P.z = !(val as u8);
                    // }
                }
            }
            Opcodes::BRA => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let val = (self.payload[0] as i8) as i16;
                let foo = (regs.PC as i32 + val as i32) as i32;
                regs.PC = foo as u16;
            }
            Opcodes::BRL => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let val = self.payload[0] as u16 | (self.payload[1] as u16) << 8;
                regs.PC += val;
            }
            Opcodes::BEQ => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.z == 1 {
                    let mut addr = (self.payload[0] as i8) as i16;
                    addr += regs.PC as i16;
                    regs.PC = addr as u16;
                }
            }
            Opcodes::BCC => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.c == 0 {
                    let mut addr = (self.payload[0] as i8) as i16;
                    addr += regs.PC as i16;
                    regs.PC = addr as u16;
                }
            }
            Opcodes::BCS => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.c == 1 {
                    let val = (self.payload[0] as i8) as i16 + regs.PC as i16;
                    regs.PC = val as u16;
                }
            }
            Opcodes::BPL => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.n == 0 {
                    let val = (self.payload[0] as i8) as i16 + regs.PC as i16;
                    regs.PC = val as u16;
                }
            }
            Opcodes::BMI => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.n == 1 {
                    let val = (self.payload[0] as i8) as i16 + regs.PC as i16;
                    regs.PC = val as u16;
                }
            }
            Opcodes::BRK => {
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.PC += 2;
                // TODO: Eval this..
                //regs.PC = effective_address.unwrap().address;
            }
            Opcodes::BVS => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.v == 1 {
                    let val = (self.payload[0] as i8) as i16 + regs.PC as i16;
                    regs.PC = val as u16;
                }
            }
            Opcodes::CLD => {
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.P.d = 0;
            }
            Opcodes::CMP => {
                // let val;
                let mut regs = bus.cpu.regs.borrow_mut();
                if !is_16_bit_mem_and_accu {
                    let val = value.unwrap().lower_to_number() as u8;
                    let res = (u16::from(regs.C.A) as u8) - val;
                    if res >> 7 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if res == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                    if u16::from(regs.C.A) as u8 >= val {
                        regs.P.c = 1;
                    } else {
                        regs.P.c = 0;
                    }
                } else {
                    let val = value.unwrap().lower_to_number();
                    let res = u16::from(regs.C) - val as u16;
                    if res >> 15 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if res == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                    if u16::from(regs.C) >= val as u16 {
                        regs.P.c = 1;
                    } else {
                        regs.P.c = 0;
                    }
                }
            }
            Opcodes::SEI => {
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.P.i = 1;
            }
            Opcodes::CLC => {
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.P.c = 0;
            }
            Opcodes::CPX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // 8 Bit registers
                if !is_16_bit_index_register {
                    // if bus.cpu.e || regs.P.x == 1 {
                    let val;
                    // if self.address_mode != AddressModes::Immediate {
                    val = value.unwrap().lower_to_number() as u8;
                    // } else {
                    //     val = self.payload[0];
                    // }
                    let bar = (regs.X.low as u8).wrapping_sub(val);
                    if bar >> 7 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if bar == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                    // TODO: double check this.
                    if regs.X.low as u8 >= bar {
                        regs.P.c = 1;
                    } else {
                        regs.P.c = 0;
                    }
                } else {
                    let val;
                    // if self.address_mode != AddressModes::Immediate {
                    //     val = bus.read(effective_address.unwrap()) as u16
                    //         | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
                    // } else {
                    val = value.unwrap().lower_to_number();
                    // }
                    let bar = <u16>::from(regs.X).wrapping_sub(val as u16);
                    if bar >> 15 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if bar == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                    // TODO: double check this.
                    if <u16>::from(regs.X) >= bar {
                        regs.P.c = 1;
                    } else {
                        regs.P.c = 0;
                    }
                }
            }
            Opcodes::DEY => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e || regs.P.m == 1 {
                    regs.Y.low = bus.cpu.regs.borrow().Y.low.wrapping_sub(1);
                    if regs.Y.low as u8 >> 7 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if regs.Y.low as u8 == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                } else {
                    regs.Y =
                        IndexRegister::from(u16::from(bus.cpu.regs.borrow().Y).wrapping_sub(1));
                    if u16::from(regs.Y) >> 15 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if u16::from(regs.Y) == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                }
            }
            // Opcodes::EOR => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         let val = bus.read(effective_address.unwrap());
            //        regs.C =
            //             Accumulator::from(regs.C.B | (cpu.regs.C.A as u8 ^ val) as u16);
            //     } else {
            //         let val = bus.read(effective_address.unwrap()) as u16
            //             | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //        regs.C = Accumulator::from(u16::from(bus.cpu.regs.borrow().C) ^ val);
            //     }
            // }
            Opcodes::XCE => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // Exchange carry with phantom emulation flag
                // TODO: Reset programm bank register
                let temp = bus.cpu.e;
                bus.cpu.e = regs.P.c != 0;
                regs.P.c = temp as _;
            }
            Opcodes::SEP => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // Set Status Bits
                let tmp = <u8>::from(regs.P);
                let next = tmp | self.payload[0]; // Set bits
                regs.P = StatusRegister::from(next);
            }
            Opcodes::REP => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // Reset Status Bits
                let tmp = <u8>::from(regs.P);
                let next = tmp & !self.payload[0]; // Clear bits
                regs.P = StatusRegister::from(next);
            }
            Opcodes::PEA => {
                let low = self.payload[0];
                let high = self.payload[1];
                bus.cpu.stack_push(high);
                bus.cpu.stack_push(low);
            }
            Opcodes::PHB => {
                let regs = bus.cpu.regs.borrow().clone();
                bus.cpu.stack_push(regs.DBR);
            }
            Opcodes::LDX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.x != 1 {
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
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }

                    if is_16_bit_mem_and_accu {
                        if (val >> 15) == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                    } else {
                        if (val >> 7) == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                    }
                    if is_16_bit_index_register {
                        regs.X = IndexRegister::from(val as u16);
                    } else {
                        regs.X = IndexRegister::from(val as u8);
                    }
                }
            }
            Opcodes::LDY => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.x != 1 {
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
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }

                    if is_16_bit_mem_and_accu {
                        if (val >> 15) == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                    } else {
                        if (val >> 7) == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                    }
                    if is_16_bit_index_register {
                        regs.Y = IndexRegister::from(val as u16);
                    } else {
                        regs.Y = IndexRegister::from(val as u8);
                    }
                }
            }
            Opcodes::NOP => {
                // No need to increment.. this is done by the iterator
                //regs.PC += 1;
            }
            Opcodes::TXS => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e {
                    // TXS emu
                    regs.S.high = 1; // High byte stack pointer is always 1
                    if regs.P.x != 1 {
                        // 16Bit index
                        regs.S.low = regs.X.low;
                    } else {
                        regs.S.low = regs.X.low;
                        // 8Bit index
                    }
                } else {
                    // TXS native
                    if regs.P.x != 1 {
                        // 16Bit index
                        regs.S.high = regs.X.high;
                        regs.S.low = regs.X.low;
                    } else {
                        // 8Bit index
                        regs.S.high = 0;
                        regs.S.low = regs.X.low;
                    }
                }
            }
            Opcodes::TXY => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // 8-bit index registers
                if regs.P.x == 1 {
                    regs.Y.low = regs.X.low;
                    if regs.X.low == 0 {
                        regs.P.z = 1;
                    }
                    if (regs.X.low as u8) >> 7 == 1 {
                        regs.P.n = 1;
                    }
                } else {
                    regs.Y = regs.X;
                    if regs.X.low == 0 && regs.X.high == 0 {
                        regs.P.z = 1;
                    }
                    if regs.X.high >> 7 == 1 {
                        regs.P.n = 1;
                    }
                }
            }
            Opcodes::TYX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                // 8-bit index registers
                if regs.P.x == 1 {
                    regs.X.low = regs.Y.low;
                    if regs.Y.low == 0 {
                        regs.P.z = 1;
                    }
                    if (regs.Y.low as u8) >> 7 == 1 {
                        regs.P.n = 1;
                    }
                } else {
                    regs.X = regs.Y;
                    if regs.Y.low == 0 && regs.Y.high == 0 {
                        regs.P.z = 1;
                    }
                    if (regs.Y.high as u8) >> 7 == 1 {
                        regs.P.n = 1;
                    }
                }
            }
            Opcodes::JMP => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if follow_jumps {
                    regs.PC = effective_address.unwrap().address;
                    if self.address_mode == AddressModes::AbsoluteLong {
                        regs.PBR = effective_address.unwrap().bank;
                    }
                    //bus.cpu.
                    // | (regs.DBR as u16) << 16;
                    //regs.PC = value.unwrap().lower_to_number() as u16;
                }
            }
            Opcodes::JSR => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if follow_jumps {
                    let pc_low = (regs.PC & 0x00ff) as u8;
                    let pc_high = (regs.PC >> 8) as u8;
                    if self.address_mode == AddressModes::AbsoluteLong {
                        let bank = regs.PBR;
                        bus.cpu.stack_push(bank);
                    }

                    bus.cpu.stack_push(pc_high);
                    bus.cpu.stack_push(pc_low);

                    // let address = effective_address
                    if self.address_mode == AddressModes::AbsoluteLong {
                        regs.PBR = effective_address.unwrap().bank;
                    }
                    regs.PC = effective_address.unwrap().address;
                    //regs.PC = value.unwrap().lower_to_number() as u16;
                }
            }
            // TODO:
            Opcodes::RTI => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e {
                    regs.P = StatusRegister::from(bus.cpu.stack_pull());
                    let high = bus.cpu.stack_pull();
                    let low = bus.cpu.stack_pull();
                    regs.PC = low as u16 | (high as u16) << 8;
                } else {
                    regs.P = StatusRegister::from(bus.cpu.stack_pull());
                    let bank = bus.cpu.stack_pull();
                    let high = bus.cpu.stack_pull();
                    let low = bus.cpu.stack_pull();
                    regs.PC = low as u16 | (high as u16) << 8;
                    regs.PBR = bank;
                }
            }
            // Opcodes::ROL => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val =regs.C.A as u8
            //         } else {
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         let new_c = val >> 7;
            //         let old_c =regs.P.c;
            //         val = (val << 1) | old_c;
            //        regs.P.c = new_c;
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C.A = val as u16;
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     } else {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(regs.C);
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //         }
            //         let new_c = val >> 15;
            //         let old_c =regs.P.c as u16;
            //         val = (val << 1) | old_c;
            //        regs.P.c = new_c as u8;
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C = Accumulator::from(val);
            //         } else {
            //             bus.write(effective_address.unwrap(), (val & 0xf) as u8);
            //             bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
            //         }
            //     }
            // }
            Opcodes::RTS => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let op_low = bus.cpu.stack_pull();
                let op_high = bus.cpu.stack_pull();
                regs.PC = ((op_high as u16) << 8) | op_low as u16;
            }
            Opcodes::RTL => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let op_low = bus.cpu.stack_pull();
                let op_high = bus.cpu.stack_pull();
                let pbr = bus.cpu.stack_pull();
                regs.PC = ((op_high as u16) << 8) | op_low as u16;
                regs.PBR = pbr;
            }

            Opcodes::LDA => {
                let is_16_bit_mem_and_accu = bus.cpu.is_16_bit_mem_and_accu();
                let mut regs = bus.cpu.regs.borrow_mut();
                // if !is_16_bit_mem_and_accu(bus.cpu) {
                let val;
                let msb;

                if self.address_mode == AddressModes::Immediate {
                    if is_16_bit_mem_and_accu {
                        val = self.payload[0] as u16 | (self.payload[1] as u16) << 8;
                        msb = val >> 15;
                        regs.C = Accumulator::from(val);
                    } else {
                        val = self.payload[0] as u16;
                        msb = val >> 7;
                        regs.C.A = val;
                    }
                } else {
                    match value.unwrap() {
                        Operant_Value::long(v) => {
                            val = v as u16;
                            msb = (v as u16) >> 15;
                            regs.C = Accumulator::from(v as u16)
                        }
                        Operant_Value::short(v) => {
                            msb = (v >> 7) as u16;
                            val = v as u16;
                            regs.C.A = v.into();
                        }
                    }
                }

                if val == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
                if msb == 1 {
                    regs.P.n = 1;
                } else {
                    regs.P.n = 0;
                }
                // }
            }
            // Opcodes::LSR => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(regs.C) as u8;
            //         } else {
            //             val = bus.read(effective_address.unwrap());
            //         }
            //         // set carry bit
            //        regs.P.c = val & 0x1;
            //        regs.C = Accumulator::from((val >> 1) as u16);
            //         if val == 0 {
            //            regs.P.z = 1;
            //         } else {
            //            regs.P.z = 0;
            //         }
            //     } else {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val = u16::from(regs.C);
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u16
            //                 | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
            //         }
            //         // set carry bit
            //        regs.P.c = val as u8 & 1;
            //        regs.C = Accumulator::from((val >> 1) as u16);
            //         if val == 0 {
            //            regs.P.z = 1;
            //         } else {
            //            regs.P.z = 0;
            //         }
            //     }
            //    regs.P.n = 0;
            // }
            // TODO: TEST this!
            Opcodes::SBC => {
                let mut regs = bus.cpu.regs.borrow_mut(); // TODO: Decimal flag
                if regs.P.m == 1 || bus.cpu.e {
                    // 8-Bit
                    if regs.C.A as i8 - (bus.read(effective_address.unwrap()) as i8) < 0 {
                        regs.P.v = 1;
                    }
                    let mut data =
                        (regs.C.A as u8).wrapping_sub(bus.read(effective_address.unwrap()));
                    if regs.P.c == 0 {
                        data = data.wrapping_sub(1);
                    }
                    if data == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                    if data >> 7 == 1 {
                        regs.P.n = 1
                    } else {
                        regs.P.n = 0;
                    }
                    regs.C.A = data as u16;
                } else {
                    let data_low = bus.read(effective_address.unwrap());
                    let mut data_high = bus.read(effective_address.unwrap().add(1));
                    if regs.C.A as i8 - (data_low as i8) < 0 {
                        // borrow required
                        data_high -= 1;
                        regs.P.c = 0;
                        regs.C.A = regs.C.A.wrapping_sub(data_low.into());
                        regs.C.B = regs.C.B.wrapping_sub(data_high.into());

                        if u16::from(regs.C) == 0 {
                            regs.P.z = 1;
                        } else {
                            regs.P.z = 0;
                        }
                        if u16::from(regs.C) >> 15 == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                        regs.P.v = 1;
                    } else {
                        regs.P.c = 1;
                        regs.C.A = regs.C.A.wrapping_sub(data_low.into());
                        regs.C.B = regs.C.B.wrapping_sub(data_high.into());

                        if u16::from(regs.C) == 0 {
                            regs.P.z = 1;
                        } else {
                            regs.P.z = 0;
                        }
                        if u16::from(regs.C) >> 15 == 1 {
                            regs.P.n = 1;
                        } else {
                            regs.P.n = 0;
                        }
                        regs.P.v = 0;
                    }
                }
            }
            Opcodes::STA => {
                let regs = bus.cpu.regs.borrow(); // println!("STA ====>{:?}", self.payload);
                if is_16_bit_mem_and_accu {
                    bus.write(effective_address.unwrap(), regs.C.A as u8);
                } else {
                    bus.write(effective_address.unwrap(), regs.C.A as u8);
                    bus.write(effective_address.unwrap().add(1), regs.C.B as u8);
                    // bus.write_u16(effective_address,regs.C.try_into().unwrap());
                }
            }
            Opcodes::STZ => {
                let mut regs = bus.cpu.regs.borrow_mut();
                bus.write(effective_address.unwrap(), 0x0);
                // reset zero flag
                regs.P.z = 0;
            }
            Opcodes::STX => {
                let regs = bus.cpu.regs.borrow();
                let X = regs.X;
                if bus.cpu.is_16_bit_index() {
                    bus.write(effective_address.unwrap(), X.low as u8);
                } else {
                    bus.write(effective_address.unwrap(), X.low as u8);
                    bus.write(effective_address.unwrap().add(1), X.high as u8);
                }
            }
            Opcodes::STY => {
                let regs = bus.cpu.regs.borrow();
                let Y = regs.Y;
                if bus.cpu.is_16_bit_index() {
                    bus.write(effective_address.unwrap(), Y.low as u8);
                } else {
                    bus.write(effective_address.unwrap(), Y.low as u8);
                    bus.write(effective_address.unwrap().add(1), Y.high as u8);
                }
            }
            Opcodes::TCS => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e {
                    regs.S = IndexRegister::from(regs.C.A as u16);
                } else {
                    regs.S = IndexRegister::from(u16::from(regs.C));
                }
            }
            Opcodes::TAX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if !bus.cpu.e {
                    // native mode
                    // 8 Bit accumulator, 8 bit index registers
                    regs.X.low = regs.C.A;
                } else {
                    // 8 bit accumulator, 16 bit index registers
                    if regs.P.m == 1 && regs.P.x == 0 {
                        regs.X.low = regs.C.A;
                        regs.X.high = regs.C.B;
                    }
                    // 16 bit accumulator, 8 bit index registers
                    if regs.P.m == 0 && regs.P.x == 1 {
                        regs.X.low = regs.C.A;
                    }
                    if regs.P.m == 0 && regs.P.x == 0 {
                        regs.X.low = regs.C.A;
                        regs.X.high = regs.C.B;
                    }
                }
                if (regs.C.A >> 7) == 1 {
                    regs.P.n = 1;
                } else {
                    regs.P.n = 0;
                }
                if regs.C.A == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
            }
            Opcodes::TAY => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if !bus.cpu.e {
                    // native mode
                    // 8 Bit accumulator, 8 bit index registers
                    regs.Y.low = regs.C.A;
                } else {
                    // 8 bit accumulator, 16 bit index registers
                    if regs.P.m == 1 && regs.P.x == 0 {
                        regs.Y.low = regs.C.A;
                        regs.Y.high = regs.C.B;
                    }
                    // 16 bit accumulator, 8 bit index registers
                    if regs.P.m == 0 && regs.P.x == 1 {
                        regs.Y.low = regs.C.A;
                    }
                    if regs.P.m == 0 && regs.P.x == 0 {
                        regs.Y.low = regs.C.A;
                        regs.Y.high = regs.C.B;
                    }
                }
                if (regs.C.A >> 7) == 1 {
                    regs.P.n = 1;
                } else {
                    regs.P.n = 0;
                }
                if regs.C.A == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
            }
            Opcodes::TCD => {
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.D = u16::from(regs.C);
                if regs.D >> 7 == 1 {
                    regs.P.n = 1;
                } else {
                    regs.P.n = 0;
                }
                if regs.D == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
            }
            // Opcodes::DEC => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val =regs.C.A as u8;
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u8;
            //         }
            //         val = val.wrapping_sub(1);
            //         if val >> 7 == 1 {
            //            regs.P.n = 1;
            //         } else {
            //            regs.P.n = 0;
            //         }
            //         if val == 0 {
            //            regs.P.z = 1;
            //         } else {
            //            regs.P.z = 0;
            //         }
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C.A = val as u16;
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     } else {
            //         // TODO
            //         let mut val;
            //         if self.address_mode == AddressModes::Accumulator {
            //             val =regs.C.A as u8;
            //         } else {
            //             val = bus.read(effective_address.unwrap()) as u8;
            //         }
            //         val -= 1;
            //         if val >> 7 == 1 {
            //            regs.P.n = 1;
            //         } else {
            //            regs.P.n = 0;
            //         }
            //         if val == 0 {
            //            regs.P.z = 1;
            //         } else {
            //            regs.P.z = 0;
            //         }
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C.A = val as u16;
            //         } else {
            //             bus.write(effective_address.unwrap(), val);
            //         }
            //     }
            // }
            // TODO: SET STATUS FLAGS!!!!
            Opcodes::DEX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let index: u16 = u16::from(regs.X).wrapping_sub(1);
                regs.X = IndexRegister::from(index);
                if u16::from(regs.X) == 0 {
                    regs.P.z = 1;
                }
                if is_16_bit_index_register {
                    if u16::from(regs.X.get_low()) >> 7 == 1 {
                        regs.P.z = 1;
                    }
                } else {
                    if u16::from(regs.X) >> 15 == 1 {
                        regs.P.z = 1;
                    }
                }
            }
            Opcodes::DEY => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let index: u16 = u16::from(regs.Y).wrapping_sub(1);
                regs.Y = IndexRegister::from(index);
                if u16::from(regs.Y) == 0 {
                    regs.P.z = 1;
                }
                if is_16_bit_index_register {
                    if u16::from(regs.Y.get_low()) >> 7 == 1 {
                        regs.P.z = 1;
                    }
                } else {
                    if u16::from(regs.Y) >> 15 == 1 {
                        regs.P.z = 1;
                    }
                }
            }
            Opcodes::INX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let index: u16 = u16::from(regs.X).wrapping_add(1);
                regs.X = IndexRegister::from(index);
                if u16::from(regs.X) == 0 {
                    regs.P.z = 1;
                }
                if is_16_bit_index_register {
                    if u16::from(regs.X.get_low()) >> 7 == 1 {
                        regs.P.z = 1;
                    }
                } else {
                    if u16::from(regs.X) >> 15 == 1 {
                        regs.P.z = 1;
                    }
                }
            }
            Opcodes::INY => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let index: u16 = u16::from(regs.Y).wrapping_add(1);
                regs.Y = IndexRegister::from(index);
                if u16::from(regs.Y) == 0 {
                    regs.P.z = 1;
                }
                if is_16_bit_index_register {
                    if u16::from(regs.X.get_low()) >> 7 == 1 {
                        regs.P.z = 1;
                    }
                } else {
                    if u16::from(regs.X) >> 15 == 1 {
                        regs.P.z = 1;
                    }
                }
            }
            // Opcodes::INC => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C.A += 1;
            //             if u16::from(regs.C.A as u8) == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if u16::from(regs.C.A as u8) >> 7 == 1 {
            //                regs.P.n = 1;
            //             } else {
            //                regs.P.n = 0;
            //             }
            //         } else {
            //             // TODO: Wrapping?
            //             let val = bus.read(effective_address.unwrap()) + 1;
            //             bus.write(effective_address.unwrap(), val);
            //             if val == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if val >> 7 == 1 {
            //                regs.P.n = 1;
            //             } else {
            //                regs.P.n = 0;
            //             }
            //         }
            //     } else {
            //         if self.address_mode == AddressModes::Accumulator {
            //            regs.C = Accumulator::from(u16::from(bus.cpu.regs.borrow().C) + 1);
            //             if u16::from(regs.C) == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if u16::from(regs.C) >> 15 == 1 {
            //                regs.P.n = 1;
            //             } else {
            //                regs.P.n = 0;
            //             }
            //         } else {
            //             // TODO: Wrapping?
            //             let val_low = bus.read(effective_address.unwrap());
            //             let val_high = bus.read(effective_address.unwrap().add(1));
            //             bus.write(effective_address.unwrap(), val_low);
            //             bus.write(effective_address.unwrap().add(1), val_high);

            //             if val_low as u16 | (val_high as u16) << 8 == 0 {
            //                regs.P.z = 1;
            //             } else {
            //                regs.P.z = 0;
            //             }
            //             if val_low as u16 | (val_high as u16) >> 15 == 1 {
            //                regs.P.n = 1;
            //             } else {
            //                regs.P.n = 0;
            //             }
            //         }
            //     }
            // }
            Opcodes::BNE => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if regs.P.z == 1 {
                    return;
                } else {
                    //regs.PC = effective_address.unwrap().address as _;
                    regs.PC = (regs.PC as i16 + (self.payload[0] as i8) as i16) as u16;
                }
            }
            Opcodes::PLD => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let low = bus.cpu.stack_pull();
                let high = bus.cpu.stack_pull();
                regs.D = low as u16 | (high as u16) << 8;

                if regs.D == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
                if regs.D >> 7 == 1 {
                    regs.P.n = 1;
                } else {
                    regs.P.n = 0;
                }
            }
            Opcodes::PLP => {
                let stack_item = bus.cpu.stack_pull();
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.P = StatusRegister::from(stack_item);
            }
            Opcodes::PLX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e || regs.P.x == 1 {
                    regs.C.A = bus.cpu.stack_pull() as u16;
                    regs.P.n = (regs.C.A as u8) >> 7;
                } else {
                    let low = bus.cpu.stack_pull();
                    let high = bus.cpu.stack_pull();
                    regs.C = Accumulator::from(u16::from(low as u16 | (high as u16) << 8));
                    if u16::from(regs.C) == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                    if u16::from(regs.C) >> 15 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                }
            }
            Opcodes::PHB => {
                let mut regs = bus.cpu.regs.borrow_mut();
                bus.cpu.stack_push(regs.DBR);
            }
            Opcodes::PHP => {
                let mut regs = bus.cpu.regs.borrow_mut();
                bus.cpu.stack_push(u8::from(regs.P));
            }
            Opcodes::PHD => {
                let mut regs = bus.cpu.regs.borrow_mut();
                bus.cpu.stack_push(u8::from((regs.D >> 8) as u8));
                bus.cpu.stack_push(u8::from((regs.D & 0x0f) as u8));
            }
            Opcodes::PHA => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e || regs.P.m == 1 {
                    bus.cpu.stack_push(regs.C.A as u8);
                } else {
                    bus.cpu.stack_push(regs.C.B as u8);
                    bus.cpu.stack_push(regs.C.A as u8);
                }
            }
            Opcodes::PHK => {
                let mut regs = bus.cpu.regs.borrow_mut();
                bus.cpu.stack_push(regs.PBR);
            }
            Opcodes::PHX => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e || regs.P.x == 1 {
                    bus.cpu.stack_push(regs.X.low as u8);
                } else {
                    bus.cpu.stack_push(regs.X.high as u8);
                    bus.cpu.stack_push(regs.X.low as u8);
                }
            }
            Opcodes::PLA => {
                let mut regs = bus.cpu.regs.borrow_mut();
                if bus.cpu.e || regs.P.m == 1 {
                    regs.C.A = bus.cpu.stack_pull() as u16;
                    if (regs.C.A as u8) >> 7 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if regs.C.A == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                } else {
                    let low = bus.cpu.stack_pull();
                    let high = bus.cpu.stack_pull();
                    regs.C = Accumulator::from(low as u16 | (high as u16) << 8);
                    if u16::from(regs.C) >> 15 == 1 {
                        regs.P.n = 1;
                    } else {
                        regs.P.n = 0;
                    }
                    if u16::from(regs.C) == 0 {
                        regs.P.z = 1;
                    } else {
                        regs.P.z = 0;
                    }
                }
            }
            Opcodes::PLB => {
                let stack_item = bus.cpu.stack_pull();
                let mut regs = bus.cpu.regs.borrow_mut();
                regs.DBR = stack_item;
                if regs.DBR >> 7 == 1 {
                    regs.P.n = 1;
                } else {
                    regs.P.n = 0;
                }
                if regs.DBR == 0 {
                    regs.P.z = 1;
                } else {
                    regs.P.z = 0;
                }
            }
            Opcodes::MVN => {
                let (src_bnk, dest_bnk) = (self.payload[1], self.payload[0]);
                let mut regs = bus.cpu.regs.borrow_mut();
                loop {
                    if regs.C == Accumulator::from(0xffffu16) {
                        break;
                    }
                    let source = u16::from(regs.X);
                    let dest = u16::from(regs.Y);
                    let _length = u16::from(regs.C);

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
                    regs.X = IndexRegister::from(u16::from(regs.X).wrapping_add(1));
                    regs.Y = IndexRegister::from(u16::from(regs.Y).wrapping_add(1));
                    regs.C = Accumulator::from(u16::from(regs.C).wrapping_sub(1));
                }

                // panic!("src: {} : {} dest: {} : {} count: {}", src_bnk, source, dest_bnk, dest, length);/
            }
            Opcodes::XBA => {
                let mut regs = bus.cpu.regs.borrow_mut();
                let temp = regs.C.B;
                regs.C.B = regs.C.A;
                regs.C.A = temp;
            }
            // Opcodes::ORA => {
            //     if bus.cpu.e || regs.P.m == 1 {
            //         let val;
            //         if self.address_mode == AddressModes::Immediate {
            //             val = self.payload[0];
            //         } else {
            //             val = bus.read(effective_address.unwrap());
            //         }
            //        regs.C.A = bus.cpu.regs.borrow().C.A | val as u16;
            //         if u16::from(regs.C.A) as u8 >> 7 == 1 {
            //            regs.P.n = 1;
            //         } else {
            //            regs.P.n = 0;
            //         }
            //         if regs.C.A == 0 {
            //            regs.P.z = 1;
            //         } else {
            //            regs.P.z = 0;
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
            //        regs.C = Accumulator::from(u16::from(bus.cpu.regs.borrow().C) | val as u16);
            //         if u16::from(regs.C) >> 15 == 1 {
            //            regs.P.n = 1;
            //         } else {
            //            regs.P.n = 0;
            //         }

            //         if u16::from(regs.C) == 0 {
            //            regs.P.z = 1;
            //         } else {
            //            regs.P.z = 0;
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
