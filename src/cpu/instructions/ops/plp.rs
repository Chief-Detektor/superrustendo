use crate::{cpu::StatusRegister, mem::Bus};

pub fn plp(bus: &mut Bus) {
    let stack_item = bus.get_cpu().stack_pull();
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.P = StatusRegister::from(stack_item);
}
