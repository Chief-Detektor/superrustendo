use crate::{cpu::address::Address, mem::Bus};

pub fn stz(bus: &mut Bus, effective_address: Option<Address>) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    bus.write(effective_address.unwrap(), 0x0);
    // reset zero flag
    regs.P.z = 0;
}
