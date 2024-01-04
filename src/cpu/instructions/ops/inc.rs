use crate::{mem::Bus, cpu::{address::Address, addressmodes::AddressModes, Accumulator}};

pub fn inc(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if bus.get_cpu().get_emulation_mode() || regs.P.m == 1 {
        if address_mode == AddressModes::Accumulator {
            regs.C.A += 1;
            if u16::from(regs.C.A as u8) == 0 {
                regs.P.z = 1;
            } else {
                regs.P.z = 0;
            }
            if u16::from(regs.C.A as u8) >> 7 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        } else {
            // TODO: Wrapping?
            let val = bus.read(effective_address.unwrap()) + 1;
            bus.write(effective_address.unwrap(), val);
            if val == 0 {
                regs.P.z = 1;
            } else {
                regs.P.z = 0;
            }
            if val >> 7 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        }
    } else {
        if address_mode == AddressModes::Accumulator {
            regs.C = Accumulator::from(u16::from(regs.C) + 1);
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
        } else {
            // TODO: Wrapping?
            let val_low = bus.read(effective_address.unwrap());
            let val_high = bus.read(effective_address.unwrap().add(1));
            bus.write(effective_address.unwrap(), val_low);
            bus.write(effective_address.unwrap().add(1), val_high);

            if val_low as u16 | (val_high as u16) << 8 == 0 {
                regs.P.z = 1;
            } else {
                regs.P.z = 0;
            }
            if val_low as u16 | (val_high as u16) >> 15 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        }
    }
}
