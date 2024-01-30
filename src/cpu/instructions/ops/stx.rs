use crate::{cpu::address::Address, mem::Bus};

pub fn stx(bus: &mut Bus, effective_address: Option<Address>) {
    let regs = bus.get_cpu().regs_ref().borrow_mut();
    let X = regs.X;
    if regs.P.x == 1 {
        bus.write(effective_address.unwrap(), X.low as u8);
    } else {
        bus.write(effective_address.unwrap(), X.low as u8);
        bus.write(effective_address.unwrap().add(1), X.high as u8);
    }
}
