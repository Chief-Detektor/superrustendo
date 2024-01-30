use crate::mem::Bus;

pub fn beq(bus: &mut Bus, value: u8) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.z == 1 {
        let mut addr = (value as i8) as i16;
        addr += regs.PC as i16;
        regs.PC = addr as u16;
    }
    //else {
    //    regs.PC += 2;
    //}
}
