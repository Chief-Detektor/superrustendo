use crate::mem::Bus;

pub fn tyx(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // 8-bit index registers
    if regs.P.x == 1 {
        regs.X.low = regs.Y.low;
        if regs.Y.low == 0 {
            regs.P.z = 1;
        }
        if (regs.Y.low as u8) >> 7 == 1 {
            regs.P.n = 1;
        }
    } else {
        regs.X = regs.Y;
        if regs.Y.low == 0 && regs.Y.high == 0 {
            regs.P.z = 1;
        }
        if (regs.Y.high as u8) >> 7 == 1 {
            regs.P.n = 1;
        }
    }
}
