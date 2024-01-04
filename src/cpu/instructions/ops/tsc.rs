use crate::mem::Bus;

use super::utils::{check_zero, check_negative_u16};


pub fn tsc(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let stack_pointer = regs.get_S().clone();
    regs.C.set_C(<u16>::from(stack_pointer));
    check_zero(&mut regs, stack_pointer.into());
    check_negative_u16(&mut regs, stack_pointer.into());
}

#[cfg(test)]
mod tests{
    use crate::cpu::IndexRegister;

    use super::*;
    #[test]
    fn tsc_instruction_0() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.set_S(&IndexRegister::from(0xff77 as u16));
        }
        tsc(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0xff77);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 1);
    }

    #[test]
    fn tsc_instruction_1() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.set_S(&IndexRegister::from(0x0 as u16));
        }
        tsc(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x0);
        assert_eq!(regs.P.z, 1);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn tsc_instruction_2() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.set_S(&IndexRegister::from(0x77 as u16));
        }
        tsc(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x77);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }
}
