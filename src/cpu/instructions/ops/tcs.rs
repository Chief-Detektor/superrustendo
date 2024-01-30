use crate::{mem::Bus, cpu::IndexRegister};

pub fn tcs(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if bus.get_cpu().get_emulation_mode() {
        regs.S = IndexRegister::from(regs.C.A as u16);
    } else {
        regs.S = IndexRegister::from(u16::from(regs.C));
    }
}
