use crate::mem::Bus;

pub fn cld(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    regs.P.d = 0;
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{
            addressmodes::AddressModes,
            decoder::{Decoder, Opcodes},
            instructions::Instruction,
            StatusRegister,
        },
        mem::Bus,
    };

    #[test]
    fn cld_instruction() {
        let mut bus = Bus::new();
        // let mut bus = Bus { cartridge: None };
        let mut P = StatusRegister::default();

        P.set_d(1);
        bus.get_cpu().get_regs().set_P(&P);

        let mut decoder = Decoder::new(&mut bus, true);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::CLD,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };

        decoder.execute_instruction(&mut i);

        assert!(bus.get_cpu().get_regs().get_P().get_d() == 0);
    }
}
