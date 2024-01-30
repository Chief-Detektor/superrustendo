use crate::{cpu::instructions::Operant_Value, mem::Bus};

pub fn cpy(bus: &mut Bus, value: Option<Operant_Value>) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // 8 Bit registers
    if regs.P.x == 0 {
        // if bus.get_cpu().e || regs.P.x == 1 {
        let val;
        // if self.address_mode != AddressModes::Immediate {
        val = value.unwrap().lower_to_number() as u8;
        // } else {
        //     val = self.payload[0];
        // }
        let bar = (regs.Y.low as u8).wrapping_sub(val);
        if bar >> 7 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if bar == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        // TODO: double check this.
        if regs.Y.low as u8 >= bar {
            regs.P.c = 1;
        } else {
            regs.P.c = 0;
        }
    } else {
        let val;
        // if self.address_mode != AddressModes::Immediate {
        //     val = bus.read(effective_address.unwrap()) as u16
        //         | (bus.read(effective_address.unwrap().add(1)) as u16) << 8;
        // } else {
        val = value.unwrap().lower_to_number();
        // }
        let bar = <u16>::from(regs.Y).wrapping_sub(val as u16);
        if bar >> 15 == 1 {
            regs.P.n = 1;
        } else {
            regs.P.n = 0;
        }
        if bar == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }
        // TODO: double check this.
        if <u16>::from(regs.Y) >= bar {
            regs.P.c = 1;
        } else {
            regs.P.c = 0;
        }
    }
}
