use crate::{cpu::address::Address, mem::Bus};

pub fn pei(bus: &mut Bus, value: u8, effective_address: Option<Address>) {
    let address = effective_address.unwrap().add(value.into());
    let low = bus.read(address);
    let high = bus.read(address.add(1));
    bus.get_cpu().stack_push(high);
    bus.get_cpu().stack_push(low);
}
