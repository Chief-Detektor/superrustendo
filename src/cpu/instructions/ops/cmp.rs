use std::num::Wrapping;

use crate::{cpu::instructions::Operant_Value, mem::Bus};

pub fn cmp(bus: &mut Bus, value: Option<Operant_Value>) {
    //    let (value, cycles) = cpu.read_value(mode);
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.m == 1 {
        let val = value.unwrap().lower_to_number() as u8;
//        let res = (u16::from(regs.C.A) as u8) - val;
        let res = Wrapping(u16::from(regs.C.A) as u8) - Wrapping(val);
        if res.0 >> 7 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if res.0 == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        if u16::from(regs.C.A) as u8 >= val {
            regs.P.c = 1;
        } else {
            regs.P.c = 0;
        }
    } else {
        let val = value.unwrap().lower_to_number();
//        let res = u16::from(regs.C) - val as u16;
        let res = Wrapping(u16::from(regs.C.A)) - Wrapping(val as u16);
        if res.0 >> 15 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if res.0 == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        if u16::from(regs.C) >= val as u16 {
            regs.P.c = 1;
        } else {
            regs.P.c = 0;
        }
    }
}
