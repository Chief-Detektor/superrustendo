use crate::cpu::address::Address;
use crate::cpu::addressmodes::AddressModes;
use crate::mem::Bus;

use super::utils::{check_negative_u16, check_negative_u8, check_zero};

fn rotate_right_u8(bus: &mut Bus, data: u8) -> u8 {
    let new_carry = data & 0x1;
    let mut val = data >> 1;
    let temp = bus.get_cpu().get_regs().get_P().get_c();
    println!("ROR: temp: {:b}", temp);
    val = (temp << 7) | val;
    let mut binding = bus.get_cpu().regs_ref().borrow_mut();
    binding.get_P_mut().set_c(new_carry);
    val
}

fn rotate_right_u16(bus: &mut Bus, data: u16) -> u16 {
    let new_carry = data & 0x1;
    let mut val = data >> 1;
    val = ((bus.get_cpu().get_regs().get_P().get_c() as u16) << 15) | val;
    let mut binding = bus.get_cpu().regs_ref().borrow_mut();
    binding.get_P_mut().set_c(new_carry.try_into().unwrap());
    val
}

pub fn ror(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) {
    // 8 bit operation
    if bus.get_cpu().get_regs().get_P().m == 1 {
        let mut val: u8;
        if address_mode == AddressModes::Accumulator {
            val = bus.get_cpu().get_regs().get_C().get_A();
            println!("Before ROR: val: {:b}, {:x}", val, val);
            val = rotate_right_u8(bus, val);
            println!("ROR: val: {:b}, {:x}", val, val);
            bus.get_cpu().regs_ref().borrow_mut().get_C_mut().set_A(val);
        } else {
            val = bus.read(effective_address.unwrap());
            val = rotate_right_u8(bus, val);
            bus.write(effective_address.unwrap(), val);
        }
        check_negative_u8(&mut bus.get_cpu().regs_ref().borrow_mut(), val);
        check_zero(&mut bus.get_cpu().regs_ref().borrow_mut(), val.into());
    } else {
        let mut val: u16;
        if address_mode == AddressModes::Accumulator {
            val = <u16>::from(*bus.get_cpu().get_regs().get_C());
            val = rotate_right_u16(bus, val);
            println!("ROR 16Bit val: {:b}", val);
            bus.get_cpu()
                .regs_ref()
                .borrow_mut()
                .get_C_mut()
                .set_C(val.into());
        } else {
            let low = bus.read(effective_address.unwrap());
            let high = bus.read(effective_address.unwrap().add(1));
            val = rotate_right_u16(bus, (high as u16) << 8 | low as u16);
            bus.write(effective_address.unwrap(), val as u8);
            bus.write(effective_address.unwrap().add(1), (val >> 8) as u8);
        }
        check_negative_u16(&mut bus.get_cpu().regs_ref().borrow_mut(), val);
        check_zero(&mut bus.get_cpu().regs_ref().borrow_mut(), val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::addressmodes::AddressModes;
    #[test]
    fn test_ror() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.P.set_c(0);
            regs.P.set_m(1);
            regs.C.set_A(0x66);
        }
        ror(&mut bus, None, AddressModes::Accumulator);
        let a = bus.get_cpu().get_regs().get_C().get_A();
        println!("A accumulator: {:b}, {:x}, {}", a, a, a);
        assert_eq!(bus.get_cpu().get_regs().get_C().get_A(), 0x33);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_c(), 0);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_z(), 0);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_n(), 0);
        ror(&mut bus, None, AddressModes::Accumulator);
        let a = bus.get_cpu().get_regs().get_C().get_A();
        println!("A accumulator: {:b}, {:x}, {}", a, a, a);
        assert_eq!(bus.get_cpu().get_regs().get_C().get_A(), 0x19);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_c(), 1);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_z(), 0);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_n(), 0);
        ror(&mut bus, None, AddressModes::Accumulator);
        let a = bus.get_cpu().get_regs().get_C().get_A();
        println!("A accumulator: {:b}, {:x}, {}", a, a, a);
        assert_eq!(bus.get_cpu().get_regs().get_C().get_A(), 0x8c);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_c(), 1);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_z(), 0);
        assert_eq!(bus.get_cpu().get_regs().get_P().get_n(), 1);
    }
}
