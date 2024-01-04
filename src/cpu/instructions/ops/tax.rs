use crate::mem::Bus;

pub fn tax(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    if !bus.get_cpu().get_emulation_mode() {
        // native mode
        // 8 Bit accumulator, 8 bit index registers
        regs.X.low = regs.C.A;
    } else {
        // 8 bit accumulator, 16 bit index registers
        if regs.P.m == 1 && regs.P.x == 0 {
            regs.X.low = regs.C.A;
            regs.X.high = regs.C.B;
        }
        // 16 bit accumulator, 8 bit index registers
        if regs.P.m == 0 && regs.P.x == 1 {
            regs.X.low = regs.C.A;
        }
        if regs.P.m == 0 && regs.P.x == 0 {
            regs.X.low = regs.C.A;
            regs.X.high = regs.C.B;
        }
    }
    if (regs.C.A >> 7) == 1 {
        regs.P.n = 1;
    } else {
        regs.P.n = 0;
    }
    if regs.C.A == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
}
