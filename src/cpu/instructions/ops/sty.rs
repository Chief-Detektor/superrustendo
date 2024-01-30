use crate::{cpu::address::Address, mem::Bus};

pub fn sty(bus: &mut Bus, effective_address: Option<Address>) {
    let regs = bus.get_cpu().regs_ref().borrow_mut();
    let Y = regs.Y;
    if regs.P.x == 1 {
        bus.write(effective_address.unwrap(), Y.low as u8);
    } else {
        bus.write(effective_address.unwrap(), Y.low as u8);
        bus.write(effective_address.unwrap().add(1), Y.high as u8);
    }
}
