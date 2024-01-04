use crate::mem::Bus;

pub fn bcs(bus: &mut Bus, value: u8) /* -> u8  (for counting cycles) */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.c == 1 {
        let val = (value as i8) as i16 + regs.PC as i16;
        regs.PC = val as u16;
    }
    //else {
    //    regs.PC += 2;
    //}
}
