use crate::mem::Bus;

pub fn tcd(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    regs.D = u16::from(regs.C);
    if regs.D >> 7 == 1 {
        regs.P.n = 1;
    } else {
        regs.P.n = 0;
    }
    if regs.D == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
}
