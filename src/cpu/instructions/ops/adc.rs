use std::{num::Wrapping, usize, u16};

use crate::{
    cpu::{instructions::Operant_Value, Accumulator},
    mem::Bus,
};

use super::utils::{
    check_negative_u16, check_negative_u8, check_signed_overflow_u16, check_signed_overflow_u8,
    check_zero,
};

pub fn adc(bus: &mut Bus, value: Operant_Value) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();

    // TODO: Implement decimal mode
    if regs.P.d == 1 {
        panic!("Addition on BCD mode not implemented");
    }

    let carry = regs.P.c;

    let val = value.lower_to_number();

    // 8 Bit addition
    if regs.P.m == 1 {
        match value {
            Operant_Value::short(_) => {}
            Operant_Value::long(_) => panic!("16 Bit addition with 8 Bit accumulator not possible"),
        }

        let res = Wrapping(regs.C.A as u8) + Wrapping(val as u8) + Wrapping(carry as u8);
        if (regs.C.A + val as u16 + carry as u16) > 0xFF {
            regs.C.A = res.0 as u16;
            regs.P.c = 1;
        } else {
            // No carry
            regs.C.A = res.0 as u16;
            regs.P.c = 0;
        }
        check_negative_u8(&mut regs, res.0 as u8);
        check_zero(&mut regs, res.0 as u16);
        check_signed_overflow_u8(&mut regs, res.0 as u8);
    } else {
        // 16 Bit addition
        let res = Wrapping(regs.C.get_C()) + Wrapping(val as u16) + Wrapping(carry as u16);
        if (<u16>::from(regs.C) as u32 + val as u32 + carry as u32) > 0xFFFF {
            regs.C = Accumulator::from(res.0);
            regs.P.c = 1;
        } else {
            // No carry
            regs.C.set_C(res.0);
            regs.P.c = 0;
        }
        check_negative_u16(&mut regs, res.0);
        check_zero(&mut regs, res.0);
        check_signed_overflow_u16(&mut regs, res.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::instructions::Operant_Value, mem::Bus};

    #[test]
    fn test_adc_emu() {
        // 8-Bit ADC Test
        // 0x01 (Accumulator C.A) + 0x01 at Memory location 0x0000 (WRAM) = 0x02
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 0, N = 0

        //use super::super::utils::Bcd;

        //let a: u8 = 99u8.to_bcd();

        // println!("A: {:x}", a);
        // println!("A: {:b}", a);
        // println!("A: {}", a);

        // let b = a.from_bcd();

        // println!("B: {:x}", b);
        // println!("B: {:b}", b);
        // println!("B: {}", b);

        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(true);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.A = 0x01;
            regs.C.B = 0x00;
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
        }

        // We don't use decoder and instruction for now, since they assume the instruction is already fetched
        // which is not the case in unit tests
        // But we can write some component tests that uses this opcode. E.g. Assembler or a vector of instructions
        //let mut decoder = Decoder::new(&mut bus, true);
        //let mut inst = Instruction::new(&mut decoder);
        //inst.opcode = Opcodes::ADC;
        //let i = decoder.peek();
        //println!("i: {:?}",inst);

        super::adc(&mut bus, Operant_Value::short(0x01));

        let regs = bus.get_cpu().regs_ref().borrow();

        println!("A: {:X}", regs.C.A);

        assert_eq!(regs.C.A, 0x02);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_carry_emu() {
        // 8-Bit ADC Test with Carry
        // 0x01 (Accumulator C.A) + 0x01 at Memory location 0x0000 (WRAM) = 0x03
        // Flags before operation:
        // C = 1, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 0, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(true);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.A = 0x01;
            regs.C.B = 0x00;
            regs.P.c = 1;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
        }
        super::adc(&mut bus, Operant_Value::short(0x01));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("A: {:X}", regs.C.A);
        assert_eq!(regs.C.A, 0x03);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_overflow_emu() {
        // 8-Bit ADC Test with Overflow
        // 0x01 (Accumulator C.A) + 0xff at Memory location 0x0000 (WRAM) = 0x00
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 1, V = 0, Z = 1, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(true);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.A = 0x01;
            regs.C.B = 0x00;
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
        }
        //let address = Address::new(0 as u32);
        //bus.write(address, 0xff);
        super::adc(&mut bus, Operant_Value::short(0xff));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("A: {:X}", regs.C.A);
        assert_eq!(regs.C.A, 0x00);
        assert_eq!(regs.P.c, 1);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 1);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_zero_emu() {
        // 8-Bit ADC Test with Zero
        // 0x00 (Accumulator C.A) + 0x00 at Memory location 0x0000 (WRAM) = 0x00
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 1, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(true);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.A = 0x00;
            regs.C.B = 0x00;
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
        }
        //let address = Address::new(0 as u32);
        //bus.write(address, 0x00);
        super::adc(&mut bus, Operant_Value::short(0x00));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("A: {:X}", regs.C.A);
        assert_eq!(regs.C.A, 0x00);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 1);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_negative_emu() {
        // 8-Bit ADC Test with Negative
        // 0x00 (Accumulator C.A) + 0xff at Memory location 0x0000 (WRAM) = 0xff
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 0, N = 1
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(true);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.A = 0x80;
            regs.C.B = 0x00;
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
            regs.P.m = 1;
            regs.P.x = 1;
        }
        //let address = Address::new(0 as u32);
        //bus.write(address, 0xff);
        super::adc(&mut bus, Operant_Value::short(0x01));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("A: {:X}", regs.C.A);
        assert_eq!(regs.C.A, 0x81);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 1);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 1);
    }

    #[test]
    fn test_adc_native() {
        // 16-Bit ADC Test
        // 0x0001 (Accumulator C) + 0x0001 at Memory location 0x0000 (WRAM) = 0x0002
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 0, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(false);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.set_C(0x0001);
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
            regs.P.m = 0;
            regs.P.x = 0;
        }
        super::adc(&mut bus, Operant_Value::long(0x0001));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("C: {:X}", regs.C.get_C());
        assert_eq!(regs.C.get_C(), 0x0002);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_carry_native() {
        // 16-Bit ADC Test with Carry
        // 0x0001 (Accumulator C) + 0x0001 at Memory location 0x0000 (WRAM) = 0x0003
        // Flags before operation:
        // C = 1, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 0, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(false);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.set_C(0x0001);
            regs.P.c = 1;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
            regs.P.m = 0;
            regs.P.x = 0;
        }
        super::adc(&mut bus, Operant_Value::long(0x0001));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("C: {:X}", regs.C.get_C());
        assert_eq!(regs.C.get_C(), 0x0003);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_overflow_native() {
        // 16-Bit ADC Test with Overflow
        // 0x0001 (Accumulator C) + 0xffff at Memory location 0x0000 (WRAM) = 0x0000
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 1, V = 0, Z = 1, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(false);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.set_C(0x0001);
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
            regs.P.m = 0;
            regs.P.x = 0;
        }
        //let address = Address::new(0 as u32);
        //bus.write(address, 0xffff);
        super::adc(&mut bus, Operant_Value::long(0xffff));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("C: {:X}", regs.C.get_C());
        assert_eq!(regs.C.get_C(), 0x0000);
        assert_eq!(regs.P.c, 1);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 1);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_zero_native() {
        // 16-Bit ADC Test with Zero
        // 0x0000 (Accumulator C) + 0x0000 at Memory location 0x0000 (WRAM) = 0x0000
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 1, N = 0
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(false);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.set_C(0x0000);
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
            regs.P.m = 0;
            regs.P.x = 0;
        }
        //let address = Address::new(0 as u32);
        //bus.write(address, 0x0000);
        super::adc(&mut bus, Operant_Value::long(0x0000));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("C: {:X}", regs.C.get_C());
        assert_eq!(regs.C.get_C(), 0x0000);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 0);
        assert_eq!(regs.P.z, 1);
        assert_eq!(regs.P.n, 0);
    }

    #[test]
    fn test_adc_negative_native() {
        // 16-Bit ADC Test with Negative
        // 0x00 (Accumulator C.A) + 0xff at Memory location 0x0000 (WRAM) = 0xff
        // Flags before operation:
        // C = 0, V = 0, Z = 0, N = 0
        // Flags after operation:
        // C = 0, V = 0, Z = 0, N = 1
        let mut bus = Bus::new();
        bus.get_cpu().set_emulation_mode(false);
        {
            let mut regs = bus.get_cpu().regs_ref().borrow_mut();
            regs.C.A = 0x00;
            regs.C.B = 0x80;
            regs.P.c = 0;
            regs.P.v = 0;
            regs.P.z = 0;
            regs.P.n = 0;
            regs.P.m = 0;
            regs.P.x = 0;
        }
        //let address = Address::new(0 as u32);
        //bus.write(address, 0xff);
        super::adc(&mut bus, Operant_Value::long(0x0001));
        let regs = bus.get_cpu().regs_ref().borrow();
        println!("C: {:X}", regs.C.get_C());
        assert_eq!(regs.C.get_C(), 0x8001);
        assert_eq!(regs.P.c, 0);
        assert_eq!(regs.P.v, 1);
        assert_eq!(regs.P.z, 0);
        assert_eq!(regs.P.n, 1);
    }
}
