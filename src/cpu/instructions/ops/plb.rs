use crate::mem::Bus;

pub fn plb(bus: &mut Bus) /* -> u8  (for counting cycles) */
{
    let stack_item = bus.get_cpu().stack_pull();
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.DBR = stack_item;
    if regs.DBR >> 7 == 1 {
        regs.P.n = 1;
    } else {
        regs.P.n = 0;
    }
    if regs.DBR == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
}
