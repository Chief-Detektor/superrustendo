mod tests {
    extern crate superrustendo;

    use superrustendo::cpu::{decoder::Opcodes, StatusRegister};
    use superrustendo::{cpu::instructions::Instruction, mem::WRAM};
    use superrustendo::{
        cpu::{addressmodes::AddressModes, decoder::Decoder, CPU},
        mem::Bus,
    };

    #[test]
    fn cld_instruction() {
        let mut cpu = CPU::new();
        let mut bus = Bus {
            cartridge: None,
            wram: WRAM::new(),
        };
        // let mut bus = Bus { cartridge: None };
        let mut P = StatusRegister::default();

        P.set_d(1);
        cpu.get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut cpu, &mut bus, true);

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
}
