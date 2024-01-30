use crate::mem::Bus;

pub fn tdc(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.C = regs.D.into();
    if regs.C.to_raw() == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
    if regs.C.to_raw() >> 15 == 1 {
        regs.P.n = 1;
    } else {
        regs.P.n = 0;
    }
}
