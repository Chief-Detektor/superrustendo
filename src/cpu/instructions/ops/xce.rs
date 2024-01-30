use crate::{cpu::IndexRegister, mem::Bus};

pub fn xce(bus: &mut Bus) {
   //  let mut regs = bus.get_cpu().regs.borrow_mut();
   //  // Exchange carry with phantom emulation flag
   //  // TODO: Reset program bank register
   //  if bus.get_cpu().get_emulation_mode() {
   //      regs.P.m = 0;
   //      regs.P.x = 0;
   //  } else {
   //      // Back from native mode to emulation mode

   //      // TODO: Find out if m and p are set to 0
   //      // The docs say that x becomes the b flag and m is inaccessible in emulation mode
   //      // Set stack pointer high byte to 1
   //      regs.S.set_high(1);
   //  }
   //  let temp = bus.get_cpu().get_emulation_mode();
   //  bus.get_cpu().set_emulation_mode(regs.P.c != 0);
   //  regs.P.c = temp as _;

                let mut regs = bus.get_cpu().regs.borrow_mut();
                // Exchange carry with phantom emulation flag
                // TODO: Reset programm bank register
                let temp = bus.get_cpu().get_emulation_mode();
                bus.get_cpu().set_emulation_mode(regs.P.c != 0);
                regs.P.c = temp as _;
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
    fn xce_instruction() {
        let mut bus = Bus::new();
        // let mut bus = Bus { cartridge: None };
        let mut P = StatusRegister::default();

        let mut i = Instruction {
            address: 0x0,
            opcode: Opcodes::XCE,
            payload: vec![],
            address_mode: AddressModes::Implied,
            length: 1,
            cycles: 2,
        };
        // Test emulation mode -> native mode
        {
            P.set_c(0);
            println!("{:?}", bus.get_cpu());
            bus.get_cpu().get_regs().set_P(&P);
            println!("{:?}", bus.get_cpu());

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            println!("{:?}", bus.get_cpu());

            assert!(bus.get_cpu().get_emulation_mode() == false);
            assert!(bus.get_cpu().get_regs().get_P().get_m() == 1);
            assert!(bus.get_cpu().get_regs().get_P().get_c() == 1);
            assert!(bus.get_cpu().get_regs().get_P().get_x() == 1);
        }
        // Test emulation mode <- native mode
        {
            P.set_c(1);
            bus.get_cpu().get_regs().set_P(&P);

            let mut decoder = Decoder::new(&mut bus, true);
            decoder.execute_instruction(&mut i);

            assert!(bus.get_cpu().get_emulation_mode() == true);
            // TODO: In emulation mode m should not be accessible
            //assert!(bus.get_cpu().get_regs().get_P().get_m() == 0);
            assert!(bus.get_cpu().get_regs().get_P().get_c() == 0);
            //assert!(bus.get_cpu().get_regs().get_P().get_x() == 1);
        }
    }
}
