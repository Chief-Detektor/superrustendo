use crate::mem::Bus;

pub fn phk(bus: &mut Bus) {
    let regs = bus.get_cpu().regs_ref().borrow().clone();
    bus.get_cpu().stack_push(regs.PBR);
}
