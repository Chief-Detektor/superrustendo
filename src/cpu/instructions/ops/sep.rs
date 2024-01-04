use crate::{cpu::StatusRegister, mem::Bus};

pub fn sep(bus: &mut Bus, value: u8){
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // Set Status Bits
    let tmp = <u8>::from(regs.P);
    let next = tmp | value; // Set bits
    regs.P = StatusRegister::from(next);
}
