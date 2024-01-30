use crate::mem::Bus;

pub fn rtl(bus: &mut Bus) {
    let op_low = bus.get_cpu().stack_pull();
    let op_high = bus.get_cpu().stack_pull();
    let pbr = bus.get_cpu().stack_pull();
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.PC = ((op_high as u16) << 8) | op_low as u16;
    regs.PBR = pbr;
}
