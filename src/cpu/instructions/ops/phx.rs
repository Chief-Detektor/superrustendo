use crate::mem::Bus;

pub fn phx(bus: &mut Bus) {
    let regs = bus.get_cpu().regs.borrow().clone();
    if bus.get_cpu().get_emulation_mode() || regs.P.x == 1 {
        bus.get_cpu().stack_push(regs.X.low as u8);
    } else {
        bus.get_cpu().stack_push(regs.X.high as u8);
        bus.get_cpu().stack_push(regs.X.low as u8);
    }
}
