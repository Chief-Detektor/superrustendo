use crate::mem::Bus;

pub fn per(bus: &mut Bus, high: u8, low: u8) {
    let displacement = ((high as u16) << 8 | low as u16) as i16;
    let address = bus.get_cpu().regs.borrow().PC as i16 + displacement;
    let high = (address >> 8) as u8;
    let low = (address & 0xFF) as u8;
    bus.get_cpu().stack_push(high);
    bus.get_cpu().stack_push(low);
}
