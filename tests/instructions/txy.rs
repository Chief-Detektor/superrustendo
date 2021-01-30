#[cfg(test)]
// #![feature(or_patterns)]

mod tests {
    extern crate superrustendo;

    use superrustendo::cpu::{decoder::Opcodes, IndexRegister, StatusRegister};
    use superrustendo::{cpu::instructions::Instruction, mem::WRAM};
    use superrustendo::{
        cpu::{addressmodes::AddressModes, decoder::Decoder, CPU},
        mem::Bus,
    };
    #[test]
    fn txy_instruction() {
        let mut cpu = CPU::new();
        let mut bus = Bus {
            cartridge: None,
            wram: WRAM::new(),
        };
        let mut P = StatusRegister::default();

        let mut X = IndexRegister::from(0xf7f7u16);
        let mut Y = IndexRegister::from(0x0u16);
        {
            // 8-Bit copy msb = 1
            P.set_x(1);
            cpu.get_regs().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(cpu.get_regs().get_X().get_low() == cpu.get_regs().get_Y().get_low());
            // The high byte must be diffrend
            assert!(cpu.get_regs().get_X().get_high() != cpu.get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(cpu.get_regs().get_P().get_n() == 1);
            // The copied byte is not zero
            assert!(cpu.get_regs().get_P().get_z() == 0);
        }
        {
            // 8-Bit copy of 0
            P.set_x(1);
            Y.set_low(0xff);
            X.set_low(0x0);
            cpu.get_regs().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(cpu.get_regs().get_X().get_low() == cpu.get_regs().get_Y().get_low());
            // The high byte must be diffrend
            assert!(cpu.get_regs().get_X().get_high() != cpu.get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(cpu.get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(cpu.get_regs().get_P().get_z() == 1);
        }
        {
            // 8-Bit copy msb = 0 and X.low != 0
            P.set_x(1);
            X.set_low(0x12);
            cpu.get_regs().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(cpu.get_regs().get_X().get_low() == cpu.get_regs().get_Y().get_low());
            // The high byte must be diffrend
            assert!(cpu.get_regs().get_X().get_high() != cpu.get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(cpu.get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(cpu.get_regs().get_P().get_z() == 0);
        }
        {
            // 16-Bit copy
            P.set_x(0);
            cpu.get_regs().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(cpu.get_regs().get_X().get_low() == cpu.get_regs().get_Y().get_low());
            // The high byte should be copied
            assert!(cpu.get_regs().get_X().get_high() == cpu.get_regs().get_Y().get_high());
            // The copied byte is negeative (MSB = 1)
            assert!(cpu.get_regs().get_P().get_n() == 1);
            // The copied byte is not zero
            assert!(cpu.get_regs().get_P().get_z() == 0);
        }
        {
            // 16-Bit copy with msb on high byte x = 0
            P.set_x(0);
            // To a number that has msb = 0
            X.set_high(0x12);
            cpu.get_regs().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(cpu.get_regs().get_X().get_low() == cpu.get_regs().get_Y().get_low());
            // The high byte should be copied
            assert!(cpu.get_regs().get_X().get_high() == cpu.get_regs().get_Y().get_high());
            // The copied byte is not negeative (MSB = 0)
            assert!(cpu.get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(cpu.get_regs().get_P().get_z() == 0);
        }
        {
            // 16-Bit copy with byte = 0
            P.set_x(0);
            X.set_high(0x0);
            X.set_low(0x0);
            Y.set_high(0xff);
            Y.set_low(0xff);
            cpu.get_regs().set_X(&X).set_Y(&Y).set_P(&P);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXY,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
            decoder.execute_instruction(&mut i);

            // The low byte should be copied
            assert!(cpu.get_regs().get_X().get_low() == cpu.get_regs().get_Y().get_low());
            // The high byte should be copied
            assert!(cpu.get_regs().get_X().get_high() == cpu.get_regs().get_Y().get_high());
            // The copied byte is not negeative (MSB = 0)
            assert!(cpu.get_regs().get_P().get_n() == 0);
            // The copied byte is not zero
            assert!(cpu.get_regs().get_P().get_z() == 1);
        }
    }
}
