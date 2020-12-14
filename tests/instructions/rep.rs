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
    fn rep_instruction() {
        let mut cpu = CPU::new();
        let mut bus = Bus {
            cartridge: None,
            wram: WRAM::new(),
        };
        // let mut bus = Bus { cartridge: None };
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

        let mut decoder = Decoder::new(&mut cpu, &mut bus, true);
        decoder.execute_instruction(&mut i);

        assert!(u8::from(*cpu.get_regs().get_P()) == 0);
    }
}
