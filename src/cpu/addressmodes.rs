use super::address::Address;
use super::constants::*;
use super::decoder::Opcodes;
use super::Registers;
use super::CPU;
use crate::mem::Bus;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum AddressModes {
    Absolute,
    AbsoluteIndirectLong,
    AbsoluteIndexedIndirect,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    AbsoluteIndirect,
    AbsoluteLong,
    AbsoluteLongIndexedX,
    Accumulator,
    BlockMove,
    DirectPage,
    DirectPageIndexedIndirectX,
    DirectPageIndirectIndexedY,
    DirectPageIndexedX,
    DirectPageIndexedY,
    DirectPageIndirect,
    DirectPageIndirectLong,
    DirectPageIndirectLongIndexedY,
    Immediate,
    Implied,
    ProgrammCounterRelative,
    ProgrammCounterRelativeLong,
    StackAbsolute,
    StackDirectPageIndirect,
    StackInterrupt,
    StackPCRelativeLong,
    StackPull,
    StackPush,
    StackRTI,
    StackRTL,
    StackRTS,
    StackRelative,
    StackRelativeIndirectIndexedY,
    Unknown,
}

// TODO: Pass in CPU native or emulation mode in order to return correct len
impl AddressModes {
    pub fn len(&self, regs: &Registers, op: &Opcodes) -> usize {
        match self {
            AddressModes::Absolute => 3,
            AddressModes::AbsoluteIndirectLong => 3,
            AddressModes::AbsoluteIndexedIndirect => 3,
            AddressModes::AbsoluteIndexedX => 3,
            AddressModes::AbsoluteIndexedY => 3,
            AddressModes::AbsoluteIndirect => 3,
            AddressModes::AbsoluteLong => 4,
            AddressModes::AbsoluteLongIndexedX => 4,
            AddressModes::Accumulator => 1,
            AddressModes::BlockMove => 3,
            AddressModes::DirectPage => 2,
            AddressModes::DirectPageIndexedIndirectX => 2,
            AddressModes::DirectPageIndirectIndexedY => 2,
            AddressModes::DirectPageIndexedX => 2,
            AddressModes::DirectPageIndexedY => 2,
            AddressModes::DirectPageIndirect => 2,
            AddressModes::DirectPageIndirectLong => 2,
            AddressModes::DirectPageIndirectLongIndexedY => 2,
            AddressModes::Immediate => {
                match *op {
                    Opcodes::LDX | Opcodes::CPX | Opcodes::LDY => {
                        if regs.P.x != 1 {
                            return 3;
                        }
                    }
                    Opcodes::LDA | Opcodes::BIT => {
                        if regs.P.m == 0 {
                            return 3;
                        }
                    }
                    _ => {}
                }
                return 2;
            }
            AddressModes::Implied => 1,
            AddressModes::ProgrammCounterRelative => 2,
            AddressModes::ProgrammCounterRelativeLong => 3,
            AddressModes::StackAbsolute => 3,
            AddressModes::StackDirectPageIndirect => 2,
            AddressModes::StackInterrupt => 2,
            AddressModes::StackPCRelativeLong => 3,
            AddressModes::StackPull => 1,
            AddressModes::StackPush => 1,
            AddressModes::StackRTI => 1,
            AddressModes::StackRTL => 1,
            AddressModes::StackRTS => 1,
            AddressModes::StackRelative => 2,
            AddressModes::StackRelativeIndirectIndexedY => 2,
            AddressModes::Unknown => 2,
        }
    }

    pub fn get_effective_address(
        &self,
        cpu: &mut CPU,
        payload: &Vec<u8>,
        opcode: &Opcodes,
        bus: &Bus,
    ) -> Option<Address> {
        let mut address = Address {
            bank: 0,
            address: 0,
        };
        match &self {
            AddressModes::Absolute => {
                if *opcode == Opcodes::JMP || *opcode == Opcodes::JSR {
                    // println!("Transfer control");
                    address.bank = cpu.regs.PBR;
                } else {
                    // println!("Datamove");
                    address.bank = cpu.regs.DBR;
                }

                let data = payload.as_slice();
                address.address = (data[1] as u16) << 8 | data[0] as u16;
                // println!("### Data, yo: {:?} Bank: {:x}", data, bank);

                return Some(address);
            }
            AddressModes::AbsoluteIndexedX => {
                address.bank = cpu.regs.DBR;
                if cpu.regs.P.x == 1 {
                    if (payload[0] as u32 | (payload[1] as u32) << 8) + cpu.regs.X.low as u32
                        > 0xffff
                    {
                        address.bank += 1;
                        address.address = (payload[0] as u16 | (payload[1] as u16) << 8)
                            .wrapping_add(cpu.regs.X.low);
                    } else {
                        address.address =
                            (payload[0] as u16 | (payload[1] as u16) << 8) + cpu.regs.X.low;
                    }
                } else {
                    if (payload[0] as u32 | (payload[1] as u32) << 8) + u16::from(cpu.regs.X) as u32
                        > 0xffff
                    {
                        address.bank += 1;
                        address.address = (payload[0] as u16 | (payload[1] as u16) << 8)
                            .wrapping_add(u16::from(cpu.regs.X));
                    } else {
                        address.address =
                            (payload[0] as u16 | (payload[1] as u16) << 8) + u16::from(cpu.regs.X);
                    }
                }
                return Some(address);
            }
            AddressModes::AbsoluteIndexedY => {
                address.bank = cpu.regs.DBR;
                if cpu.regs.P.x == 1 {
                    if (payload[0] as u32 | (payload[1] as u32) << 8) + cpu.regs.Y.low as u32
                        > 0xffff
                    {
                        address.bank += 1;
                        address.address = (payload[0] as u16 | (payload[1] as u16) << 8)
                            .wrapping_add(cpu.regs.Y.low);
                    } else {
                        address.address =
                            (payload[0] as u16 | (payload[1] as u16) << 8) + cpu.regs.Y.low;
                    }
                } else {
                    if (payload[0] as u32 | (payload[1] as u32) << 8) + u16::from(cpu.regs.Y) as u32
                        > 0xffff
                    {
                        address.bank += 1;
                        address.address = (payload[0] as u16 | (payload[1] as u16) << 8)
                            .wrapping_add(u16::from(cpu.regs.Y));
                    } else {
                        address.address =
                            (payload[0] as u16 | (payload[1] as u16) << 8) + u16::from(cpu.regs.Y);
                    }
                }
                return Some(address);
            }
            AddressModes::AbsoluteLongIndexedX => {
                let data = payload.as_slice();

                let bank = data[2];
                let mut addr = (data[1] as u16) << 8 | data[0] as u16;

                // TODO: Wrapping? => Bank change?
                if cpu.regs.P.x == 0 {
                    addr = addr.wrapping_add(u16::from(cpu.regs.X));
                } else {
                    addr = addr.wrapping_add(cpu.regs.X.get_low() as u16);
                }

                address.bank = bank;
                address.address = addr;
                return Some(address);
            }
            AddressModes::AbsoluteIndexedIndirect => {
                let op_low = payload[0];
                let op_high = payload[1];

                let x;
                if cpu.regs.P.x == 1 || cpu.e {
                    x = cpu.regs.X.low;
                } else {
                    x = u16::from(cpu.regs.X);
                }
                // TODO: increment bank on overflow like AbsoluteIndexed/XY?
                let mut indirect_address = op_low as u16 | (op_high as u16) << 8;
                // indirect_address += x;
                if indirect_address as u32 + x as u32 > 0xffff {
                    address.bank = cpu.regs.PBR /*+ 1*/;
                    indirect_address = indirect_address.wrapping_add(x);
                } else {
                    address.bank = cpu.regs.PBR;
                    indirect_address += x;
                }

                let addresss_low = bus.read(address);
                let addresss_high = bus.read(address.add(1));

                // cpu.regs.PC = (cpu.regs.PBR as u32) << 16 | (addresss_high as u32) << 8 | addresss_low as u32;
                address.address = (addresss_high as u16) << 8 | addresss_low as u16;
                return Some(address);
            }
            AddressModes::AbsoluteLong => {
                let op_low = payload[0];
                let op_high = payload[1];
                address.address = (op_high as u16) << 8 | op_low as u16;
                address.bank = payload[2];
                return Some(address);
            }
            AddressModes::Implied => {
                println!("Implied addressing");
                return None;
            }
            AddressModes::Immediate => {
                println!("Immediate addressing");
                if !cpu.e && (cpu.regs.P.m == 0 || cpu.regs.P.x == 0) && payload.capacity() == 2 {
                    address.address = payload[0] as u16 | (payload[1] as u16) << 8;
                } else {
                    address.address = payload[0] as u16;
                }
                return Some(address);
            } // TODO: Return Payload as slice?
            AddressModes::ProgrammCounterRelative => {
                let offset: i8 = payload[0] as _;
                let foo = offset as i16;
                address.address = (foo as i32 + (cpu.regs.PC as i32)).try_into().unwrap();
                address.bank = cpu.regs.PBR;
                // return (((cpu.regs.PBR as u32) << 16) | address) as usize;
                return Some(address);
            }
            AddressModes::ProgrammCounterRelativeLong => {
                let offset = payload[0] as u16 | (payload[1] as u16) << 8;
                let sign_offest = offset as i16;
                address.address = (sign_offest as i32 + (cpu.regs.PC as i32))
                    .try_into()
                    .unwrap();
                address.bank = cpu.regs.PBR;
                // return (((cpu.regs.PBR as u32) << 16) | address) as usize;
                return Some(address);
            }
            AddressModes::StackPCRelativeLong => {
                let op_low = payload[0];
                let address = cpu.regs.PC + op_low as u16;
                cpu.stack_push((address & 0x00ff) as u8);
                cpu.stack_push(((address & 0xff00) >> 8) as u8);
            }
            AddressModes::StackRelative => {
                address.address = payload[0] as u16 + u16::from(cpu.regs.S);
                address.bank = 0;
                return Some(address);
            }
            AddressModes::StackRelativeIndirectIndexedY => {
                let val = payload[0] as u16 + u16::from(cpu.regs.S);
                let mut addr = bus.read(Address {
                    address: val,
                    bank: 0,
                }) as u16
                    | (bus.read(
                        Address {
                            address: val,
                            bank: 0,
                        }
                        .add(1),
                    ) as u16)
                        << 8;
                if cpu.e || cpu.regs.P.x == 1 {
                    addr += cpu.regs.Y.low;
                } else {
                    addr += u16::from(cpu.regs.Y);
                }
                address.address = addr;
                address.bank = cpu.regs.DBR;
                return Some(address);
            }
            AddressModes::StackInterrupt => {
                if !cpu.e {
                    cpu.stack_push(cpu.regs.PBR);
                }
                let pc_high = (cpu.regs.PC >> 8) as u8;
                let pc_low = (cpu.regs.PC & 0xff) as u8;
                cpu.stack_push(pc_high);
                cpu.stack_push(pc_low);
                cpu.stack_push(cpu.regs.P.into());

                // TODO: Eval this
                let interrupt_vector;
                if !cpu.e {
                    interrupt_vector = bus.cartridge.as_ref().unwrap().header.native_irq;
                } else {
                    interrupt_vector = bus.cartridge.as_ref().unwrap().header.emu_irq;
                }
                let load_address = Address {
                    bank: 0,
                    address: interrupt_vector,
                };
                let val_low = bus.read(load_address);
                let val_high = bus.read(load_address.add(1));

                address.address = (val_high as u16) << 8 | val_low as u16;
                return Some(address);
            }
            AddressModes::Accumulator => {}
            AddressModes::StackRTS => {}
            AddressModes::StackRTL => {}
            AddressModes::StackRTI => {}
            AddressModes::StackPush => {}
            AddressModes::StackPull => {}
            AddressModes::StackAbsolute => {}
            AddressModes::DirectPage => {
                address.address = u16::from(cpu.regs.D) + payload[0] as u16;
                return Some(address);
            }
            AddressModes::DirectPageIndirect => {
                let val = payload[0] as u16 + cpu.regs.D;
                let addr_low = bus.read(Address {
                    bank: 0,
                    address: val,
                });
                let addr_high = bus.read(
                    Address {
                        bank: 0,
                        address: val,
                    }
                    .add(1),
                );

                address.bank = cpu.regs.DBR;
                address.address = addr_low as u16 | (addr_high as u16) << 8;
                return Some(address);
            }
            AddressModes::DirectPageIndirectLong => {
                let val = payload[0] as u16 + cpu.regs.D;
                let addr_low = bus.read(Address {
                    bank: 0,
                    address: val,
                });
                let addr_high = bus.read(
                    Address {
                        bank: 0,
                        address: val,
                    }
                    .add(1),
                );
                let bank = bus.read(
                    Address {
                        bank: 0,
                        address: val,
                    }
                    .add(2),
                );
                address.bank = bank;
                address.address = addr_low as u16 | (addr_high as u16) << 8;
                return Some(address);
            }
            AddressModes::DirectPageIndirectIndexedY => {
                if cpu.e || cpu.regs.P.x == 1 {
                    let mut val;
                    val = cpu.regs.D + payload[0] as u16;
                    address.address = (bus.read(Address {
                        bank: 0,
                        address: val,
                    }) as u16
                        | (bus.read(
                            Address {
                                bank: 0,
                                address: val,
                            }
                            .add(1),
                        ) as u16)
                            << 8)
                        + cpu.regs.Y.low as u16;
                    address.bank = cpu.regs.DBR;
                } else {
                    let mut val;
                    val = cpu.regs.D + payload[0] as u16;
                    address.address = (bus.read(Address {
                        bank: 0,
                        address: val,
                    }) as u16
                        | (bus.read(
                            Address {
                                bank: 0,
                                address: val,
                            }
                            .add(1),
                        ) as u16)
                            << 8)
                        + u16::from(cpu.regs.Y);
                    address.bank = cpu.regs.DBR;
                }
                return Some(address);
            }
            AddressModes::DirectPageIndirectLongIndexedY => {
                let indirect = cpu.regs.D + payload[0] as u16;
                let addr_low = bus.read(Address {
                    address: indirect,
                    bank: 0,
                });
                let addr_high = bus.read(Address {
                    address: indirect,
                    bank: 0,
                });
                let addr_bank = bus.read(Address {
                    address: indirect,
                    bank: 0,
                });
                let index;
                if cpu.e || cpu.regs.P.x == 1 {
                    index = cpu.regs.Y.low;
                } else {
                    index = u16::from(cpu.regs.Y);
                }
                address.address = (addr_low as u16 | (addr_high as u16) << 8).wrapping_add(index);
                address.bank = addr_bank;
                return Some(address);
            }
            AddressModes::DirectPageIndexedX => {
                let addr;
                if cpu.e || cpu.regs.P.x == 1 {
                    let D;
                    if cpu.e {
                        D = 0;
                    } else {
                        D = cpu.regs.D;
                    }
                    addr = payload[0] as u16 + D + cpu.regs.X.low;
                } else {
                    addr = payload[0] as u16 + cpu.regs.D + u16::from(cpu.regs.X);
                }
                address.address = addr;
                address.bank = 0;
                return Some(address);
            }
            AddressModes::DirectPageIndexedIndirectX => {
                if cpu.e || cpu.regs.P.x == 1 {
                    let mut val;
                    // is it 0 in emu mode??
                    val = 0u8;
                    // val = cpu.regs.D;
                    val += val
                        .wrapping_add(cpu.regs.X.low as u8)
                        .wrapping_add(payload[0]);
                    let indirect = bus.read(Address {
                        bank: 0,
                        address: val as u16,
                    }) as u16
                        | (bus.read(
                            Address {
                                bank: 0,
                                address: val as u16,
                            }
                            .add(1),
                        ) as u16)
                            << 8;
                    address.bank = cpu.regs.DBR;
                } else {
                    let mut val;
                    val = cpu.regs.D;
                    val += u16::from(cpu.regs.X) + payload[0] as u16;
                    address.address = bus.read(Address {
                        bank: 0,
                        address: val,
                    }) as u16
                        | (bus.read(
                            Address {
                                bank: 0,
                                address: val,
                            }
                            .add(1),
                        ) as u16)
                            << 8;
                    address.bank = cpu.regs.DBR;
                }
                return Some(address);
            }
            AddressModes::BlockMove => {
                println!(
                    "AddressMode: {:?}, opcpode: {:?}, cpu-regs: {:?}",
                    self, opcode, cpu.regs
                );
            }
            _ => {
                unimplemented!(
                    "AddressMode: {:?}, opcpode: {:?}, cpu-regs: {:?}",
                    self,
                    opcode,
                    cpu.regs
                );
            }
        };
        None
    }
}

impl Default for AddressModes {
    fn default() -> AddressModes {
        AddressModes::Unknown
    }
}

pub fn get_gii_reg_load_addr_mode(opcode: u8) -> Option<AddressModes> {
    let mask = opcode & GII_MASK;
    let g2_mask = opcode & !GII_MASK;
    match mask {
        G2_REGLOAD_ADDR_MODE_IMMEDIATE => Some(AddressModes::Immediate),
        G2_REGLOAD_ADDR_MODE_ABSOLUTE => Some(AddressModes::Absolute),
        G2_REGLOAD_ADDR_MODE_DIRECT_PAGE => Some(AddressModes::DirectPage),
        G2_REGLOAD_ADDR_MODE_DIRECT_PAGE_INDEXED => match g2_mask {
            G2_OP_LDX => Some(AddressModes::DirectPageIndexedY),
            G2_OP_LDY => Some(AddressModes::DirectPageIndexedX),
            _ => None,
        },
        G2_REGLOAD_ADDR_MODE_ABSOLUTE_INDEXED => match g2_mask {
            G2_OP_LDX => Some(AddressModes::AbsoluteIndexedY),
            G2_OP_LDY => Some(AddressModes::AbsoluteIndexedX),
            _ => None,
        },
        _ => None,
    }
}

pub fn get_gii_addr_mode(opcode: u8) -> Option<AddressModes> {
    let mask = opcode & GII_MASK;

    match mask {
        G2_ADDR_MODE_ACCUMULATOR => Some(AddressModes::Accumulator),
        G2_ADDR_MODE_ABSOLUTE => Some(AddressModes::Absolute),
        G2_ADDR_MODE_DIRECT_ZERO_PAGE => Some(AddressModes::DirectPage),
        G2_ADDR_MODE_ABSOLUTE_INDEXED_X => Some(AddressModes::AbsoluteIndexedX),
        G2_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some(AddressModes::DirectPageIndexedX),
        _ => None,
    }
}

pub fn get_gi_addr_mode(opcode: u8) -> Option<AddressModes> {
    let mask = opcode & GI_MASK;

    match mask {
        GI_ADDR_MODE_INTERMEDIATE => Some(AddressModes::Immediate), // Add 1 byte if m = 0 (16Bit memory/accumulator)
        GI_ADDR_MODE_DIRECT_ZERO_PAGE => Some(AddressModes::DirectPage),
        GI_ADDR_MODE_ABSOLUTE => Some(AddressModes::Absolute),
        GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X => Some(AddressModes::DirectPageIndexedX),
        GI_ADDR_MODE_ABSOLUTE_INDEXED_X => Some(AddressModes::AbsoluteIndexedX),
        GI_ADDR_MODE_ABSOLUTE_INDEXED_Y => Some(AddressModes::AbsoluteIndexedY),
        GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_X => {
            Some(AddressModes::DirectPageIndexedIndirectX)
        }
        GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDIRECT_INDEXED_Y => {
            Some(AddressModes::DirectPageIndirectIndexedY)
        }
        GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG_INDEXED_Y => {
            Some(AddressModes::DirectPageIndirectLongIndexedY)
        }
        GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG => Some(AddressModes::DirectPageIndirectLong),
        GI_ADDR_MODE_ABSOLUTE_LONG => Some(AddressModes::AbsoluteLong),
        GI_ADDR_MODE_ABSOLUTE_LONG_INDEXED_X => Some(AddressModes::AbsoluteLongIndexedX),
        GI_ADDR_MODE_STACK_RELATIVE => Some(AddressModes::StackRelative),
        GI_ADDR_MODE_STACK_RELATIVE_INDIRECT_INDEXED_Y => {
            Some(AddressModes::StackRelativeIndirectIndexedY)
        }
        GI_ADDR_MODE_DIRECT_PAGE_INDIRECT => Some(AddressModes::DirectPageIndirect),
        _ => {
            return None;
        }
    }
}
