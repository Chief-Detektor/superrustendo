use crate::{cpu::{addressmodes::AddressModes, address::Address, Accumulator}, mem::Bus};

pub fn lsr(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if bus.get_cpu().get_emulation_mode() || regs.P.m == 1 {
        let val;
        if address_mode == AddressModes::Accumulator {
            val = u16::from(regs.C) as u8;
        } else {
            val = bus.read(effective_address.unwrap());
        }
        // set carry bit
        regs.P.c = val & 0x1;
        regs.C = Accumulator::from((val >> 1) as u16);
        if val == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
    } else {
        let val;
        if address_mode == AddressModes::Accumulator {
            val = u16::from(regs.C);
        } else {
            val = bus.read(effective_address.unwrap()) as u16
                | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
        }
        // set carry bit
        regs.P.c = val as u8 & 1;
        regs.C = Accumulator::from((val >> 1) as u16);
        if val == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
    }
    regs.P.n = 0;
}
