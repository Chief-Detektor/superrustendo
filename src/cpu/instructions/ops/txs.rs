use crate::mem::Bus;

pub fn txs(bus: &mut Bus) /* -> u8  (for counting cycles) */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if bus.get_cpu().get_emulation_mode() {
        // TXS emu
        regs.S.high = 1; // High byte stack pointer is always 1
        if regs.P.x != 1 {
            // 16Bit index
            regs.S.low = regs.X.low;
        } else {
            regs.S.low = regs.X.low;
            // 8Bit index
        }
    } else {
        // TXS native
        if regs.P.x != 1 {
            // 16Bit index
            regs.S.high = regs.X.high;
            regs.S.low = regs.X.low;
        } else {
            // 8Bit index
            regs.S.high = 0;
            regs.S.low = regs.X.low;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{decoder::Opcodes, IndexRegister};
    use crate::cpu::instructions::Instruction;
    use crate::{
        cpu::{addressmodes::AddressModes, decoder::Decoder},
        mem::Bus,
    };

    #[test]
    fn txs_instruction() {
        let mut bus = Bus::new();

        let mut X = IndexRegister::from(0xffu16);
        {
            // Emulation mode
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X);

            bus.get_cpu().set_emulation_mode(true);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXS,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);

            decoder.execute_instruction(&mut i);

            // println!("{:?}", cpu.regs_ref().borrow_mut().get_S());
            assert!(u16::from(*bus.get_cpu().regs_ref().borrow_mut().get_S()) == 0x1ff);
        }
        {
            // Native mode 8 Bit registers
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X).get_P_mut().set_x(1);
            bus.get_cpu().set_emulation_mode(false);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXS,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);

            decoder.execute_instruction(&mut i);

            // println!("{:?}", cpu.regs_ref().borrow_mut().get_S());
            assert!(u16::from(*bus.get_cpu().regs_ref().borrow_mut().get_S()) == 0xff);
        }
        {
            // Native mode 16 Bit registers
            X.set_high(0xba).set_low(0xbe);
            bus.get_cpu().regs_ref().borrow_mut().set_X(&X);
            // println!("{:?}", cpu.regs_ref().borrow_mut().get_X());
            bus.get_cpu().regs_ref().borrow_mut().get_P_mut().set_x(0);
            bus.get_cpu().set_emulation_mode(false);

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::TXS,
                payload: vec![],
                address_mode: AddressModes::Implied,
                length: 1,
                cycles: 2,
            };

            let mut decoder = Decoder::new(&mut bus, true);

            decoder.execute_instruction(&mut i);
            // println!("{:?}", cpu.regs_ref().borrow_mut().get_S());

            assert!(u16::from(*bus.get_cpu().regs_ref().borrow_mut().get_S()) == 0xbabe);
        }
    }
}
