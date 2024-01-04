use crate::{
    cpu::{address::Address, addressmodes::AddressModes, Accumulator},
    mem::Bus,
};

// TODO: This used to be commented out so it needs to be verified
pub fn asl(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) /*-> u8*/
{
    // NOTE: this is a Copilot implementation of the asl instruction
    //    let mut effective_address = None;
    //    let mut cycles = 0;
    //    match address_mode {
    //        AddressModes::Accumulator => {
    //            let val = bus.get_cpu().regs.C.A;
    //            let msb = val >> 7;
    //            if msb == 1 {
    //                bus.get_cpu().regs.P.n = 1;
    //            } else {
    //                bus.get_cpu().regs.P.n = 0;
    //            }
    //            bus.get_cpu().regs.P.c = msb;
    //            bus.get_cpu().regs.C.A = val << 1;
    //            cycles = 2;
    //        }
    //        _ => {
    //            effective_address = Some(bus.get_address(address_mode));
    //            let mut val = bus.read(effective_address.unwrap());
    //            let msb = val >> 7;
    //            if msb == 1 {
    //                bus.get_cpu().regs.P.n = 1;
    //            } else {
    //                bus.get_cpu().regs.P.n = 0;
    //            }
    //            bus.get_cpu().regs.P.c = msb;
    //            val = val << 1;
    //            bus.write(effective_address.unwrap(), val);
    //            cycles = 6;
    //        }
    //    }
    //    if bus.get_cpu().regs.P.m == 1 {
    //        bus.get_cpu().regs.P.z = bus.get_cpu().regs.C.A == 0;
    //    } else {
    //        bus.get_cpu().regs.P.z = bus.get_cpu().regs.C == Accumulator::from(0);
    //    }
    //    cycles
    //}

    if bus.get_cpu().regs.borrow().P.m == 1 {
        let mut val;
        if address_mode == AddressModes::Accumulator {
            val = bus.get_cpu().regs.borrow().C.A as u8;
        } else {
            // shift val located at effective_address
            val = bus.read(effective_address.unwrap());
        }
        val = val << 1;
        let msb = val >> 7;
        if msb == 1 {
            bus.get_cpu().regs.borrow_mut().P.n = 1;
        } else {
            bus.get_cpu().regs.borrow_mut().P.n = 0;
        }
        bus.get_cpu().regs.borrow_mut().P.c = msb;
        if address_mode == AddressModes::Accumulator {
            bus.get_cpu().regs.borrow_mut().C.A = val as u16
        } else {
            bus.write(effective_address.unwrap(), val);
        }
    } else {
        let mut val;
        if address_mode == AddressModes::Accumulator {
            val = u16::from(bus.get_cpu().regs.borrow_mut().C);
        } else {
            // shift val located at effective_address
            val = bus.read(effective_address.unwrap()) as u16
                | ((bus.read(effective_address.unwrap().add(1)) as u16) << 8)
        }
        val = val << 1;
        let msb = val >> 15;
        if msb == 1 {
            bus.get_cpu().regs.borrow_mut().P.n = 1;
        } else {
            bus.get_cpu().regs.borrow_mut().P.n = 0;
        }
        bus.get_cpu().regs.borrow_mut().P.c = msb as u8;
        if address_mode == AddressModes::Accumulator {
            bus.get_cpu().regs.borrow_mut().C = Accumulator::from(val);
        } else {
            bus.write(effective_address.unwrap(), (val & 0x0f) as u8);
            bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
        }
    }
}
