use crate::mem::Bus;

use super::utils::{check_negative_u8, check_zero, check_negative_u16};

pub fn tya(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();

    let y_u8 = regs.get_Y().get_low();
    let y_u16 = regs.get_Y().to_raw();

    if regs.P.x == 1 && regs.P.m == 1 {
        regs.C.set_A(y_u8);
        check_negative_u8(&mut regs, y_u8);
        check_zero(&mut regs, y_u8.into());
    } else if regs.P.x == 0 && regs.P.m == 1 {
        regs.C.set_A(y_u8);
        check_negative_u8(&mut regs, y_u8);
        check_zero(&mut regs, y_u8.into());
    } else if regs.P.x == 1 && regs.P.m == 0 {
        regs.C.set_A(y_u8);
        regs.C.set_B(0);
        check_negative_u8(&mut regs, y_u8);
        check_zero(&mut regs, y_u8.into());
    } else if regs.P.x == 0 && regs.P.m == 0 {
        regs.C.set_C(y_u16);
        check_negative_u16(&mut regs, y_u16);
        check_zero(&mut regs, y_u16);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::IndexRegister;

    use super::*;
    #[test]
    fn tya_instruction_0() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.x = 0;
            regs.P.m = 0;
            regs.C.set_C(0x0000);
            regs.set_Y(&IndexRegister::from(0x000f as u16));
        }
        tya(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x000f);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }
    #[test]
    fn tya_instruction_1() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.x = 0;
            regs.P.m = 1;
            regs.C.set_C(0x7700);
            regs.set_Y(&IndexRegister::from(0x0f0f as u16));
        }
        tya(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x770f);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }
    #[test]
    fn tya_instruction_2() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.x = 1;
            regs.P.m = 0;
            regs.C.set_C(0x7700);
            regs.set_Y(&IndexRegister::from(0x000f as u16));
        }
        tya(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x000f);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn tya_instruction_3() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.x = 0;
            regs.P.m = 0;
            regs.C.set_C(0xffff);
            regs.set_Y(&IndexRegister::from(0x0000 as u16));
        }
        tya(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x0000);
        assert_eq!(regs.P.z, 1);
        assert_eq!(regs.P.n, 0);
    }
    #[test]
    fn tya_instruction_4() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.x = 0;
            regs.P.m = 0;
            regs.C.set_C(0x0000);
            regs.set_Y(&IndexRegister::from(0x800f as u16));
        }
        tya(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.C.get_C(), 0x800f);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 1);
    }
}
