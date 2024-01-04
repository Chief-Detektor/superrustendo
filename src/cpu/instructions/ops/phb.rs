use crate::mem::Bus;

pub fn phb(bus: &mut Bus) {
    let regs = bus.get_cpu().regs.borrow().clone();
    bus.get_cpu().stack_push(regs.DBR);
}
