#[cfg(test)]
// #![feature(or_patterns)]

mod tests {
    extern crate superrustendo;

    use superrustendo::cpu::{decoder::Opcodes, StatusRegister};
    use superrustendo::cpu::instructions::Instruction;
    use superrustendo::{
        cpu::{addressmodes::AddressModes, decoder::Decoder, CPU},
        mem::Mapper,
    };

    #[test]
    fn sei_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper { cartridge: None };
        let mut P = StatusRegister::default();
        P.set_i(0);
        cpu.get_regs().set_P(P);

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
        let mut mapper = Mapper{ cartridge: None};
        let mut P = StatusRegister::default();
        P.set_c(1);
        cpu.get_regs().set_P(P);

        let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::CLC,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2
        };

        decoder.execute_instruction(&mut i);

        assert!(cpu.get_regs().get_P().get_c() == 0);
    }
    #[test]
    fn xce_instruction() {
        let mut cpu = CPU::new();
        let mut mapper = Mapper{ cartridge: None};
        let mut P = StatusRegister::default();

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::XCE,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2
        };
        // Test emulation mode -> native mode   
        {
            P.set_c(0);
            println!("{:?}", cpu);
            cpu.get_regs().set_P(P);       
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
            cpu.get_regs().set_P(P);

            let mut decoder = Decoder::new(&mut cpu, &mut mapper, true);
            decoder.execute_instruction(&mut i);

            assert!(cpu.get_emulation_mode() == true);
            // TODO: In emulation mode m should not be accessible
            assert!(cpu.get_regs().get_P().get_m() == 1);
            assert!(cpu.get_regs().get_P().get_c() == 0);
        }
    }
}
