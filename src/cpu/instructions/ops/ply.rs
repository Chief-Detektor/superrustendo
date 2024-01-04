use crate::{cpu::IndexRegister, mem::Bus};

pub fn ply(bus: &mut Bus) {
    let regs = bus.get_cpu().get_regs();
    if regs.P.m == 1 || regs.P.x == 1 {
        let s = bus.get_cpu().stack_pull() as u16;
        let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.Y.low = s;
        regs.P.n = (regs.Y.low as u8) >> 7;
    } else {
        let low = bus.get_cpu().stack_pull();
        let high = bus.get_cpu().stack_pull();
        let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.Y = IndexRegister::from(u16::from(low as u16 | (high as u16) << 8));
        if u16::from(regs.Y) == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        if u16::from(regs.Y) >> 15 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
    }
}
