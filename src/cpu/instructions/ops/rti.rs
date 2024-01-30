use crate::{mem::Bus, cpu::StatusRegister};

pub fn rti(bus: &mut Bus) {
    if bus.get_cpu().get_emulation_mode() {
        let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.P = StatusRegister::from(bus.get_cpu().stack_pull());
        let high = bus.get_cpu().stack_pull();
        let low = bus.get_cpu().stack_pull();
        regs.PC = low as u16 | (high as u16) << 8;
    } else {
        let p = StatusRegister::from(bus.get_cpu().stack_pull());
        let bank = bus.get_cpu().stack_pull();
        let high = bus.get_cpu().stack_pull();
        let low = bus.get_cpu().stack_pull();
        let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.P = p;
        regs.PC = low as u16 | (high as u16) << 8;
        regs.PBR = bank;
    }
}
