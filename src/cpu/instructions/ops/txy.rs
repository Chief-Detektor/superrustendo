use crate::mem::Bus;

pub fn txy(bus: &mut Bus) /* -> u8  (for counting cycles) */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // 8-bit index registers
    if regs.P.x == 1 {
        regs.Y.low = regs.X.low;
        if regs.X.low == 0 {
            regs.P.z = 1;
        }
        if (regs.X.low as u8) >> 7 == 1 {
            regs.P.n = 1;
        }
    } else {
        regs.Y = regs.X;
        if regs.X.low == 0 && regs.X.high == 0 {
            regs.P.z = 1;
        }
        if regs.X.high >> 7 == 1 {
            regs.P.n = 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{decoder::Opcodes, IndexRegister, StatusRegister};
    use crate::cpu::instructions::Instruction;
    use crate::{
        cpu::{addressmodes::AddressModes, decoder::Decoder},
        mem::Bus,
    };
    #[test]
    fn txy_instruction() {
        let mut bus = Bus::new();
        let mut P = StatusRegister::default();

        let mut X = IndexRegister::from(0xf7f7u16);
        let mut Y = IndexRegister::from(0x0u16);
        {
            // 8-Bit copy msb = 1
            P.set_x(1);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_low() == bus.get_cpu().get_regs().get_Y().get_low());
            // The high byte must be diffrend
            assert!(bus.get_cpu().get_regs().get_X().get_high() != bus.get_cpu().get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 1);
            // The copied byte is not zero
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
        }
        {
            // 8-Bit copy of 0
            P.set_x(1);
            Y.set_low(0xff);
            X.set_low(0x0);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_low() == bus.get_cpu().get_regs().get_Y().get_low());
            // The high byte must be diffrend
            assert!(bus.get_cpu().get_regs().get_X().get_high() != bus.get_cpu().get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 1);
        }
        {
            // 8-Bit copy msb = 0 and X.low != 0
            P.set_x(1);
            X.set_low(0x12);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_low() == bus.get_cpu().get_regs().get_Y().get_low());
            // The high byte must be diffrend
            assert!(bus.get_cpu().get_regs().get_X().get_high() != bus.get_cpu().get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
        }
        {
            // 16-Bit copy
            P.set_x(0);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_low() == bus.get_cpu().get_regs().get_Y().get_low());
            // The high byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_high() == bus.get_cpu().get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 1);
            // The copied byte is not zero
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
        }
        {
            // 16-Bit copy with msb on high byte x = 0
            P.set_x(0);
            // To a number that has msb = 0
            X.set_high(0x12);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_low() == bus.get_cpu().get_regs().get_Y().get_low());
            // The high byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_high() == bus.get_cpu().get_regs().get_Y().get_high());
            // The copied byte is not negeative (MSB = 0)
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
        }
        {
            // 16-Bit copy with byte = 0
            P.set_x(0);
            X.set_high(0x0);
            X.set_low(0x0);
            Y.set_high(0xff);
            Y.set_low(0xff);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_low() == bus.get_cpu().get_regs().get_Y().get_low());
            // The high byte should be copied
            assert!(bus.get_cpu().get_regs().get_X().get_high() == bus.get_cpu().get_regs().get_Y().get_high());
            // The copied byte is not negeative (MSB = 0)
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 1);
        }
    }
}
