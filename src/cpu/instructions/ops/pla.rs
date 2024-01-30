use std::u16;

use crate::{cpu::Accumulator, mem::Bus};

pub fn pla(bus: &mut Bus) {
    let regs = bus.get_cpu().get_regs();
    if regs.P.m == 1 {
        let low = bus.get_cpu().stack_pull() as u16;
        let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.C.A = low;
        if (regs.C.A as u8) >> 7 == 1 {
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
        let low = bus.get_cpu().stack_pull();
        let high = bus.get_cpu().stack_pull();
        let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.C = Accumulator::from(low as u16 | (high as u16) << 8);
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
