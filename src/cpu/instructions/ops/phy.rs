use crate::mem::Bus;

pub fn phy(bus: &mut Bus) {
    let regs = bus.get_cpu().regs.borrow().clone();
    if bus.get_cpu().get_emulation_mode() || regs.P.x == 1 {
        bus.get_cpu().stack_push(regs.Y.low as u8);
    } else {
        bus.get_cpu().stack_push(regs.Y.high as u8);
        bus.get_cpu().stack_push(regs.Y.low as u8);
    }
}
