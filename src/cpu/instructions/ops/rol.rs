use crate::{cpu::{addressmodes::AddressModes, address::Address, Accumulator}, mem::Bus};

pub fn rol(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if bus.get_cpu().get_emulation_mode() || regs.P.m == 1 {
        let mut val;
        if address_mode == AddressModes::Accumulator {
            val = regs.C.A as u8
        } else {
            val = bus.read(effective_address.unwrap());
        }
        let new_c = val >> 7;
        let old_c = regs.P.c;
        val = (val << 1) | old_c;
        regs.P.c = new_c;
        if address_mode == AddressModes::Accumulator {
            regs.C.A = val as u16;
        } else {
            bus.write(effective_address.unwrap(), val);
        }
    } else {
        let mut val;
        if address_mode == AddressModes::Accumulator {
            val = u16::from(regs.C);
        } else {
            val = bus.read(effective_address.unwrap()) as u16
                | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
        }
        let new_c = val >> 15;
        let old_c = regs.P.c as u16;
        val = (val << 1) | old_c;
        regs.P.c = new_c as u8;
        if address_mode == AddressModes::Accumulator {
            regs.C = Accumulator::from(val);
        } else {
            bus.write(effective_address.unwrap(), (val & 0xf) as u8);
            bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
        }
    }
}
