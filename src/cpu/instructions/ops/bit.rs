use crate::{cpu::{addressmodes::AddressModes, instructions::Operant_Value}, mem::Bus};

pub fn bit(bus: &mut Bus, value: Option<Operant_Value>, address_mode: AddressModes) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.m == 1 {
        let mut val = value.unwrap().lower_to_number() as u8;
        if address_mode == AddressModes::Immediate {
            regs.P.n = (val >> 7) & 0x1;
            regs.P.v = (val >> 6) & 0x1;
        }
        val = val & (regs.C.A as u8);
        regs.P.z = !val;
    } else {
        let mut val = value.unwrap().lower_to_number() as u16;
        if address_mode == AddressModes::Immediate {
            regs.P.n = ((val >> 15) & 0x1) as u8;
            regs.P.v = ((val >> 14) & 0x1) as u8;
        }
        val = val & u16::from(regs.C);
        regs.P.z = !(val as u8);
    }
}
