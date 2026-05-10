use crate::{
    cpu::address::Address,
    mem::Bus,
};
use log;

pub fn sta(bus: &mut Bus, effective_address: Option<Address>) {
    let regs = bus.get_cpu().get_regs(); // log::debug!("STA ====>{:?}", self.payload);
    if regs.P.m == 1 {
        bus.write(effective_address.unwrap(), regs.C.A as u8);
    } else {
        bus.write(effective_address.unwrap(), regs.C.A as u8);
        bus.write(effective_address.unwrap().add(1), regs.C.B as u8);
        // bus.write_u16(effective_address,regs.C.try_into().unwrap());
    }
}
