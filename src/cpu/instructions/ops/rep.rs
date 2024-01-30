use crate::{mem::Bus, cpu::StatusRegister};

pub fn rep(bus: &mut Bus, value: u8) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // Reset Status Bits
    let tmp = <u8>::from(regs.P);
    let next = tmp & !value; // Clear bits
    regs.P = StatusRegister::from(next);
}


#[cfg(test)]
mod tests {
    use crate::cpu::{decoder::Opcodes, StatusRegister};
    use crate::cpu::instructions::Instruction;
    use crate::{
        cpu::{addressmodes::AddressModes, decoder::Decoder},
        mem::Bus,
    };

    #[test]
    fn rep_instruction() {
        let mut bus = Bus::new();
        // let mut bus = Bus { cartridge: None };
        let P = StatusRegister::from(0b11111111);

        bus.get_cpu().get_regs().set_P(&P);

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::REP,
            payload: vec![0b11111111], // Reset all flags
            address_mode: AddressModes::Implied,
            length: 2,
            cycles: 3,
        };

        let mut decoder = Decoder::new(&mut bus, true);
        decoder.execute_instruction(&mut i);

        assert!(u8::from(*bus.get_cpu().get_regs().get_P()) == 0);
    }
}
