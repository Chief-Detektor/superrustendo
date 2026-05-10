use crate::{cpu::{addressmodes::AddressModes, Accumulator, instructions::OperantValue, address::Address}, mem::Bus};
//
//// TODO: unify OperantValue and payload
pub fn lda(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes, payload: &[u8], value: Option<OperantValue>) {
    let is_16_bit_mem_and_accu = bus.get_cpu().is_16_bit_mem_and_accu();
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // if !is_16_bit_mem_and_accu(bus.get_cpu(). {
    let val;
    let msb;

    if address_mode == AddressModes::Immediate {
        if is_16_bit_mem_and_accu {
            val = payload[0] as u16 | (payload[1] as u16) << 8;
            msb = val >> 15;
            regs.C = Accumulator::from(val);
        } else {
            val = payload[0] as u16;
            msb = val >> 7;
            regs.C.A = val;
        }
    } else {
        match value.unwrap() {
            OperantValue::Long(v) => {
                val = v as u16;
                msb = (v as u16) >> 15;
                regs.C = Accumulator::from(v as u16)
            }
            OperantValue::Short(v) => {
                msb = (v >> 7) as u16;
                val = v as u16;
                regs.C.A = v.into();
            }
        }
    }

    if val == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
    if msb == 1 {
        regs.P.n = 1;
    } else {
        regs.P.n = 0;
    }
    // }
}
