use crate::mem::Bus;

use super::utils::{check_zero, check_negative_u16};

pub fn tsx(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let stack_pointer = regs.get_S().clone();
    if regs.P.x == 0 {
        regs.X = stack_pointer.into();
    } else {
        let low_byte = stack_pointer.low;
        regs.X.low = low_byte.into();
    }
    check_zero(&mut regs, stack_pointer.into());
    check_negative_u16(&mut regs, stack_pointer.into());
}
