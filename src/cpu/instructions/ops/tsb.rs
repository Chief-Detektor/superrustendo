use crate::{
    cpu::{addressmodes::AddressModes,  address::Address},
    mem::Bus,
};

pub fn tsb(bus: &mut Bus, address: Address, address_mode: AddressModes) /* -> usize */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let mut cycles = 0;
    if regs.P.m == 1 {
        let mut val = bus.read(address) as u8;
        println!("val: {:#x}", val);
        if address_mode == AddressModes::Absolute {
            cycles = 6;
        } else {
            cycles = 5;
        }
        println!("val: {:#b}", !(val as u8));
        val = val | regs.C.A as u8;
        println!("val: {:#b}", val);
        println!("val: {:#b}", !(val as u8));
        regs.P.z = (val == 0) as u8;
        bus.write(address, val);
    } else {
        let mut val = bus.read(address) as u16 | ((bus.read(address.add(1)) as u16) << 8);
        val = val | u16::from(regs.C);
        regs.P.z = (val == 0) as u8;
        bus.write(address, (val & 0x0f) as u8);
        bus.write(address.add(1), (val >> 8) as u8);
        if address_mode == AddressModes::Absolute {
            cycles = 6;
            if (regs.D & 0x00ff) != 0 {
                cycles += 1;
            }
        } else {
            cycles = 5;
        }
        cycles += 2;
    }
    //cycles
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::cpu::Accumulator;
    use crate::cpu::StatusRegister;
    use crate::{
        cpu::addressmodes::AddressModes,
        mem::Bus,
    };
    #[test]
    fn tsb_instruction_8_bit_zero() {
        let mut bus = Bus::new();
        let mut P = StatusRegister::default();
        P.set_m(1);
        P.set_z(0);
        let mut c = Accumulator::default();
        c.A = 0x00;
        bus.get_cpu().regs.borrow_mut().set_C(&c);
        bus.get_cpu().regs.borrow_mut().set_P(&P);
        let address = Address::new(0x0);
        bus.write(address, 0x00);

        let address_mode = AddressModes::Absolute;

        super::tsb(&mut bus, address, address_mode);

        println!("P: {:?}", bus.get_cpu().get_regs().get_P());
        assert!(bus.read(Address::new(0x0)) == 0x00);
        assert!(bus.get_cpu().get_regs().get_P().get_z() == 1);
    }
    #[test]
    fn tsb_instruction_8_bit() {
        let mut bus = Bus::new();
        let mut P = StatusRegister::default();
        P.set_m(1);
        P.set_z(0);
        let mut c = Accumulator::default();
        c.A = 0b10101010;
        bus.get_cpu().regs.borrow_mut().set_C(&c);
        bus.get_cpu().regs.borrow_mut().set_P(&P);
        let address = Address::new(0x0);
        bus.write(address, 0xff);
        let address_mode = AddressModes::Absolute;
        super::tsb(&mut bus, address, address_mode);
        println!("P: {:?}", bus.get_cpu().get_regs().get_P());
        println!("val: {:#b}", bus.read(Address::new(0x0)));
        assert!(bus.read(Address::new(0x0)) == 0b11111111);
        assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
    }
    #[test]
    fn tsb_instruction_16_bit_zero(){
        let mut bus = Bus::new();
        let mut P = StatusRegister::default();
        P.set_m(0);
        P.set_z(0);
        let mut c = Accumulator::default();
        c.set_C(0x00);
        bus.get_cpu().regs.borrow_mut().set_C(&c);
        bus.get_cpu().regs.borrow_mut().set_P(&P);
        let address = Address::new(0x0);
        bus.write(address, 0x00);
        bus.write(address.add(1), 0x00);
        let address_mode = AddressModes::Absolute;
        super::tsb(&mut bus, address, address_mode);
        println!("P: {:?}", bus.get_cpu().get_regs().get_P());
        assert!(bus.read(Address::new(0x0)) == 0x00);
        assert!(bus.read(Address::new(0x1)) == 0x00);
        assert!(bus.get_cpu().get_regs().get_P().get_z() == 1);
    }
    #[test]
    fn tsb_instruction_16_bit() {
        let mut bus = Bus::new();
        let mut P = StatusRegister::default();
        P.set_m(0);
        P.set_z(0);
        let mut c = Accumulator::default();
        c.set_C(0b1010101010101010);
        bus.get_cpu().regs.borrow_mut().set_C(&c);
        bus.get_cpu().regs.borrow_mut().set_P(&P);
        let address = Address::new(0x0);
        bus.write(address, 0xff);
        bus.write(address.add(1), 0xff);
        let address_mode = AddressModes::Absolute;
        super::tsb(&mut bus, address, address_mode);
        println!("P: {:?}", bus.get_cpu().get_regs().get_P());
        assert!(bus.read(Address::new(0x0)) == 0xf);
        assert!(bus.read(Address::new(0x1)) == 0xff);
        assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
            
    }
    
}
