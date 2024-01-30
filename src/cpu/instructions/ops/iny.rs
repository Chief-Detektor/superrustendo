use crate::{cpu::IndexRegister, mem::Bus};

pub fn iny(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let index: u16 = u16::from(regs.Y).wrapping_add(1);
    regs.Y = IndexRegister::from(index);
    if u16::from(regs.Y) == 0 {
        regs.P.z = 1;
    }
    if regs.P.x == 1 {
        if u16::from(regs.X.get_low()) >> 7 == 1 {
            regs.P.z = 1;
        }
    } else {
        if u16::from(regs.X) >> 15 == 1 {
            regs.P.z = 1;
        }
    }
}
