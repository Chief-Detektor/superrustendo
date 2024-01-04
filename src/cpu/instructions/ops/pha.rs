use crate::mem::Bus;

pub fn pha(bus: &mut Bus) {
    let regs = bus.get_cpu().regs.borrow().clone();
    let A = regs.C.A;
    let B = regs.C.B;
    if bus.get_cpu().get_emulation_mode() || regs.P.m == 1 {
        bus.get_cpu().stack_push(A as u8);
    } else {
        bus.get_cpu().stack_push(B as u8);
        bus.get_cpu().stack_push(A as u8);
    }
}
