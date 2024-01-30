use crate::mem::Bus;

pub fn phd(bus: &mut Bus) {
    let regs = bus.get_cpu().regs_ref().borrow().clone();
    bus.get_cpu().stack_push(u8::from((regs.D >> 8) as u8));
    bus.get_cpu().stack_push(u8::from((regs.D & 0x0f) as u8));
}
