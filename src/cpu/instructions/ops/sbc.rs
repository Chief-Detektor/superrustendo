use crate::{cpu::address::Address, mem::Bus};

pub fn sbc(bus: &mut Bus, effective_address: Option<Address>) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut(); // TODO: Decimal flag
    if regs.P.m == 1 || bus.get_cpu().get_emulation_mode() {
        // 8-Bit
        if regs.C.A as i8 - (bus.read(effective_address.unwrap()) as i8) < 0 {
            regs.P.v = 1;
        }
        let mut data = (regs.C.A as u8).wrapping_sub(bus.read(effective_address.unwrap()));
        if regs.P.c == 0 {
            data = data.wrapping_sub(1);
        }
        if data == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        if data >> 7 == 1 {
            regs.P.n = 1
        } else {
            regs.P.n = 0;
        }
        regs.C.A = data as u16;
    } else {
        let data_low = bus.read(effective_address.unwrap());
        let mut data_high = bus.read(effective_address.unwrap().add(1));
        if regs.C.A as i8 - (data_low as i8) < 0 {
            // borrow required
            data_high -= 1;
            regs.P.c = 0;
            regs.C.A = regs.C.A.wrapping_sub(data_low.into());
            regs.C.B = regs.C.B.wrapping_sub(data_high.into());

            if u16::from(regs.C) == 0 {
                regs.P.z = 1;
            } else {
                regs.P.z = 0;
            }
            if u16::from(regs.C) >> 15 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
            regs.P.v = 1;
        } else {
            regs.P.c = 1;
            regs.C.A = regs.C.A.wrapping_sub(data_low.into());
            regs.C.B = regs.C.B.wrapping_sub(data_high.into());

            if u16::from(regs.C) == 0 {
                regs.P.z = 1;
            } else {
                regs.P.z = 0;
            }
            if u16::from(regs.C) >> 15 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
            regs.P.v = 0;
        }
    }
}
