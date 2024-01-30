use crate::mem::Bus;

pub fn sei(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    regs.P.i = 1;
}

#[cfg(test)]

mod tests {
    use crate::cpu::instructions::Instruction;
    use crate::cpu::{decoder::Opcodes, StatusRegister};
    use crate::{
        cpu::{addressmodes::AddressModes, decoder::Decoder},
        mem::Bus,
    };

    #[test]
    fn sei_instruction() {
        let mut bus = Bus::new();
        // let mut bus = Bus { cartridge: None };
        let mut P = StatusRegister::default();
        P.set_i(0);
        bus.get_cpu().get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut bus, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::SEI,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };

        decoder.execute_instruction(&mut i);

        assert!(bus.get_cpu().get_regs().get_P().get_i() == 1);
    }
}
