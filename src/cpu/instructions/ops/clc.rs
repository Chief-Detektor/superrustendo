use crate::mem::Bus;

pub fn clc(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    regs.P.c = 0;
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
    fn clc_instruction() {
        let mut bus = Bus::new();
        let mut P = StatusRegister::default();
        P.set_c(1);
        bus.get_cpu().get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut bus, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::CLC,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };

        decoder.execute_instruction(&mut i);

        assert!(bus.get_cpu().get_regs().get_P().get_c() == 0);
    }
}
