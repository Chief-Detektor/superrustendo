use crate::{mem::Bus, cpu::{address::Address, addressmodes::AddressModes}};

pub fn dec(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if  regs.P.m == 1 {
        let mut val;
        if address_mode == AddressModes::Accumulator {
            val = regs.C.A as u8;
        } else {
            val = bus.read(effective_address.unwrap()) as u8;
        }
        val = val.wrapping_sub(1);
        if val >> 7 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if val == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        if address_mode == AddressModes::Accumulator {
            regs.C.A = val as u16;
        } else {
            bus.write(effective_address.unwrap(), val);
        }
    } else {
        // TODO
        let mut val;
        if address_mode == AddressModes::Accumulator {
            val = regs.C.A as u8;
        } else {
            val = bus.read(effective_address.unwrap()) as u8;
        }
        val -= 1;
        if val >> 7 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if val == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        if address_mode == AddressModes::Accumulator {
            regs.C.set_C(val as u16);
        } else {
            bus.write(effective_address.unwrap(), val);
        }
    }
}
