use crate::{
    cpu::{instructions::Operant_Value, IndexRegister},
    mem::Bus,
};

pub fn ldy(bus: &mut Bus, value: Option<Operant_Value>) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.x != 1 {
        let val;
        // if self.address_mode == AddressModes::Immediate {
        //     val = effective_address.unwrap().address;
        // } else {
        match value.unwrap() {
            Operant_Value::short(v) => {
                val = v as i32;
            }
            Operant_Value::long(v) => {
                val = v as i32;
            }
        }
        // }

        // Set cpu flags accordingly
        if val == 0 {
            regs.P.z = 1;
        } else {
            regs.P.z = 0;
        }

        if regs.P.m == 1 {
            if (val >> 15) == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        } else {
            if (val >> 7) == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        }
        if regs.P.x == 1 {
            regs.Y = IndexRegister::from(val as u16);
        } else {
            regs.Y = IndexRegister::from(val as u8);
        }
    }
}
