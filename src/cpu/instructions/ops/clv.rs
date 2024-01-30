use crate::mem::Bus;


pub fn clv(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs_ref().borrow_mut();
    regs.P.v = 0;
}

#[cfg(test)]
mod tests{
    use crate::cpu::instructions::Instruction;
    use crate::cpu::decoder::Opcodes;
    use crate::{
        cpu::{addressmodes::AddressModes, decoder::Decoder},
        mem::Bus,
    };
    #[test]
    fn clv_instruction() {
        let mut bus = Bus::new();
        bus.get_cpu().regs_ref().borrow_mut().P.v = 1;
        let mut decoder = Decoder::new(&mut bus, true);
        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::CLV,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };
        decoder.execute_instruction(&mut i);
        assert!(bus.get_cpu().get_regs().get_P().get_v() == 0);
    }
}
