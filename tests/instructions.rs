#[cfg(test)]
// #![feature(or_patterns)]

mod tests {
    extern crate superrustendo;

    use superrustendo::cpu::addressmodes::AddressModes;
    use superrustendo::cpu::decoder::Opcodes;
    use superrustendo::cpu::instructions::Instruction;



    #[test]
    fn sei_instruction() {
        let mut cpu = superrustendo::cpu::CPU::new();
        let mut mapper = superrustendo::mem::Mapper {
            cartridge: None
        };
        cpu.regs.P.i = 0;
        // print!("CPU: {:?}", cpu.regs);

        let mut decoder = superrustendo::cpu::decoder::Decoder::new(&mut cpu, &mut mapper, true);
        
        //let i = Instruction::new(false);
        let mut i = Instruction {
          follow_jumps: false,
          address: 0x0,
          opcode: Opcodes::SEI,
          payload: vec![0b11011010],
          address_mode: AddressModes::Immediate,
          length: 2,
          cycles: 4,
        };

        decoder.execute_instruction(&mut i);

        // print!("CPU: {:?}", cpu.regs.P);

        assert!(cpu.regs.P.i == 1);
    }
}
