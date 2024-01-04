use crate::mem::Bus;

pub fn brl(bus: &mut Bus, low: u8, high: u8) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let val = low as u16 | (high as u16) << 8;
    regs.PC += val;
}
