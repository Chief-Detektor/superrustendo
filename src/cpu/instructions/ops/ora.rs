use crate::{cpu::{addressmodes::AddressModes, address::Address, Accumulator}, mem::Bus};

pub fn ora(bus: &mut Bus, effective_address:  Option<Address>, address_mode: AddressModes, low: u8, high: u8) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if bus.get_cpu().get_emulation_mode() || regs.P.m == 1 {
        let val;
        if address_mode == AddressModes::Immediate {
            val = low;
        } else {
            val = bus.read(effective_address.unwrap());
        }
        regs.C.A = regs.C.A | val as u16;
        if u16::from(regs.C.A) as u8 >> 7 == 1 {
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
        // 16 bit
        let val;
        if address_mode == AddressModes::Immediate {
            val = low as u16 | (high as u16) << 8;
        } else {
            val = bus.read(effective_address.unwrap()) as u16
                | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
        }
        regs.C = Accumulator::from(u16::from(regs.C) | val as u16);
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
