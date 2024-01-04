use crate::mem::Bus;

pub fn php(bus: &mut Bus) {
    let regs = bus.get_cpu().get_regs();
    let p = u8::from(regs.P);
    bus.get_cpu().stack_push(p);
}
