#[cfg(test)]
// #![feature(or_patterns)]

mod tests {
    extern crate superrustendo;

    use superrustendo::{cpu::instructions::Instruction, mem::WRAM};
    use superrustendo::cpu::{decoder::Opcodes, IndexRegister, StatusRegister};
    use superrustendo::{
        cpu::{addressmodes::AddressModes, decoder::Decoder, CPU},
        mem::Mapper,
    };

    #[test]
    fn sei_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };
        // let mut mapper = Mapper { cartridge: None };
        let mut P = StatusRegister::default();
        P.set_i(0);
        cpu.get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::SEI,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };

        decoder.execute_instruction(&mut i);

        assert!(cpu.get_regs().get_P().get_i() == 1);
    }

    #[test]
    fn clc_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };
        // let mut mapper = Mapper { cartridge: None };
        let mut P = StatusRegister::default();
        P.set_c(1);
        cpu.get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::CLC,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };

        decoder.execute_instruction(&mut i);

        assert!(cpu.get_regs().get_P().get_c() == 0);
    }

    #[test]
    fn cld_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };
        // let mut mapper = Mapper { cartridge: None };
        let mut P = StatusRegister::default();

        P.set_d(1);
        cpu.get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::CLD,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };

        decoder.execute_instruction(&mut i);

        assert!(cpu.get_regs().get_P().get_d() == 0);
    }
    #[test]
    fn xce_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };
        // let mut mapper = Mapper { cartridge: None };
        let mut P = StatusRegister::default();

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::XCE,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };
        // Test emulation mode -> native mode
        {
            P.set_c(0);
            println!("{:?}", cpu);
            cpu.get_regs().set_P(&P);
            println!("{:?}", cpu);

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
            decoder.execute_instruction(&mut i);

            println!("{:?}", cpu);

            assert!(cpu.get_emulation_mode() == false);
            assert!(cpu.get_regs().get_P().get_m() == 1);
            assert!(cpu.get_regs().get_P().get_c() == 1);
            assert!(cpu.get_regs().get_P().get_x() == 1);
        }
        // Test emulation mode <- native mode
        {
            P.set_c(1);
            cpu.get_regs().set_P(&P);

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
            decoder.execute_instruction(&mut i);

            assert!(cpu.get_emulation_mode() == true);
            // TODO: In emulation mode m should not be accessible
            assert!(cpu.get_regs().get_P().get_m() == 1);
            assert!(cpu.get_regs().get_P().get_c() == 0);
        }
    }
    #[test]
    fn rep_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };
        // let mut mapper = Mapper { cartridge: None };
        let mut P = StatusRegister::from(0b11111111);

        cpu.get_regs().set_P(&P);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::REP,
            payload: vec![0b11111111], // Reset all flags
            address_mode: AddressModes::Implied,
            length: 2,
            cycles: 3,
        };

        let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
        decoder.execute_instruction(&mut i);

        assert!(u8::from(*cpu.get_regs().get_P()) == 0);
    }

    #[test]
    fn txy_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };
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

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
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

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
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

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
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

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
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

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
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

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
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

    #[test]
    fn txs_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None, wram: WRAM::new() };

        let mut X = IndexRegister::from(0xffu16);
        {
            // Emulation mode
            cpu.get_regs().set_X(&X);

            cpu.set_emulation_mode(true);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXS,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

            decoder.execute_instruction(&mut i);

            // println!("{:?}", cpu.get_regs().get_S());
            assert!(u16::from(*cpu.get_regs().get_S()) == 0x1ff);
        }
        {
            // Native mode 8 Bit registers
            cpu.get_regs().set_X(&X);
            cpu.get_regs().get_P().set_x(1);
            cpu.set_emulation_mode(false);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXS,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

            decoder.execute_instruction(&mut i);

            // println!("{:?}", cpu.get_regs().get_S());
            assert!(u16::from(*cpu.get_regs().get_S()) == 0xff);
        }
        {
            // Native mode 16 Bit registers
            X.set_high(0xba).set_low(0xbe);
            cpu.get_regs().set_X(&X);
            // println!("{:?}", cpu.get_regs().get_X());
            cpu.get_regs().get_P().set_x(0);
            cpu.set_emulation_mode(false);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXS,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

            decoder.execute_instruction(&mut i);
            // println!("{:?}", cpu.get_regs().get_S());

            assert!(u16::from(*cpu.get_regs().get_S()) == 0xbabe);
        }
    }
}
