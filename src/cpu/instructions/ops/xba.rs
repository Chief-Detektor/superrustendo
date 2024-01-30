use crate::mem::Bus;

pub fn xba(bus: &mut Bus) /* -> u8  (for counting cycles) */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let temp = regs.C.B;
    regs.C.B = regs.C.A;
    regs.C.A = temp;
}
