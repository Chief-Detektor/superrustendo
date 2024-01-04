use crate::mem::Bus;

pub fn pea(bus: &mut Bus, low: u8, high: u8) {
    bus.get_cpu().stack_push(high);
    bus.get_cpu().stack_push(low);
}
