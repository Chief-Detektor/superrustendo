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
    fn txs_instruction() {
        let mut cpu = CPU::new();
        let mut bus = Bus {
            cartridge: None,
            wram: WRAM::new(),
        };

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

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);

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

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);

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

            let mut decoder = Decoder::new(&mut cpu, &mut bus, true);

            decoder.execute_instruction(&mut i);
            // println!("{:?}", cpu.get_regs().get_S());

            assert!(u16::from(*cpu.get_regs().get_S()) == 0xbabe);
        }
    }
}
