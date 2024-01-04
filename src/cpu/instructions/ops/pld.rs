use crate::mem::Bus;

pub fn pld(bus: &mut Bus) {
    let low = bus.get_cpu().stack_pull().clone();
    let high = bus.get_cpu().stack_pull().clone();
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.D = low as u16 | (high as u16) << 8;

    if regs.D == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
    if regs.D >> 7 == 1 {
        regs.P.n = 1;
    } else {
        regs.P.n = 0;
    }
}
