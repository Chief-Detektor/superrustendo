use crate::mem::Bus;

pub fn bne(bus: &mut Bus, value: u8) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.z == 1 {
        return;
    } else {
        //regs.PC = effective_address.unwrap().address as _;
        regs.PC = (regs.PC as i16 + (value as i8) as i16) as u16;
    }
}
