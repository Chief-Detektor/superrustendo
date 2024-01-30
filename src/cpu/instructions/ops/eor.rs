use crate::{
    cpu::{address::Address, Accumulator},
    mem::Bus,
};

pub fn eor(bus: &mut Bus, effective_address: Option<Address>) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if bus.get_cpu().get_emulation_mode() || regs.P.m == 1 {
        let val = bus.read(effective_address.unwrap());
        regs.C = Accumulator::from(regs.C.B | (regs.C.A as u8 ^ val) as u16);
    } else {
        let val = bus.read(effective_address.unwrap()) as u16
            | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
        regs.C = Accumulator::from(u16::from(regs.C) ^ val);
    }
}
