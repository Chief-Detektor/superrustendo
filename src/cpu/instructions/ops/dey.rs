use crate::{cpu::IndexRegister, mem::Bus};

pub fn dey(bus: &mut Bus) {
    let regs_r = bus.get_cpu().get_regs();
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if  regs_r.P.x == 1 {
        regs.Y.low = regs_r.Y.low.wrapping_sub(1);
        if regs.Y.low as u8 >> 7 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if regs.Y.low as u8 == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
    } else {
        regs.Y = IndexRegister::from(u16::from(regs_r.Y).wrapping_sub(1));
        if u16::from(regs.Y) >> 15 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if u16::from(regs.Y) == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
    }
}
 
//            Opcodes::DEY => {
//                let mut regs = bus.get_cpu().regs_ref().borrow_mut();
//                let index: u16 = u16::from(regs.Y).wrapping_sub(1);
//                regs.Y = IndexRegister::from(index);
//                if u16::from(regs.Y) == 0 {
//                    regs.P.z = 1;
//                }
//                if is_16_bit_index_register {
//                    if u16::from(regs.Y.get_low()) >> 7 == 1 {
//                        regs.P.z = 1;
//                    }
//                } else {
//                    if u16::from(regs.Y) >> 15 == 1 {
//                        regs.P.z = 1;
//                    }
//                }
//            }
