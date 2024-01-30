use crate::{cpu::Accumulator, mem::Bus};

use crate::cpu::instructions::Operant_Value;

pub fn and(bus: &mut Bus, value: Option<Operant_Value>) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    // TODO: This needs to become a function returning an enum having either a 8 or 16 bit value
    match value.unwrap() {
        Operant_Value::short(val) => {
            regs.C = Accumulator::from((regs.clone().C.A as u8 & val) as u16);
            if u16::from(regs.C.clone()) >> 8 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        }
        Operant_Value::long(val) => {
            regs.C = Accumulator::from(u16::from(regs.C.clone()) & val as u16);
            if u16::from(regs.C.clone()) >> 15 == 1 {
                regs.P.n = 1;
            } else {
                regs.P.n = 0;
            }
        }
    }
    if u16::from(regs.C.clone()) == 0 {
        regs.P.z = 1;
    } else {
        regs.P.z = 0;
    }
}

#[cfg(test)]
mod tests {
//    extern crate superrustendo;

    use crate::cpu::{decoder::Opcodes, Accumulator};

    use crate::cpu::instructions::Instruction;
    use crate::{
        cpu::{addressmodes::AddressModes},
        mem::Bus,
    };

    #[test]
    fn and_instruction() {
        // Well we need to have a dummy rom here.. Maybe one that has first byte 0x0 second 0x1 etc...
        // That way the rom would be deterministic: bus.read([80]12) => 13

        // let rom = Cartridge::load_rom(Path::new("./snes-test-roms/PeterLemon/SNES-CPUTest-CPU/AND/CPUAND.sfc")).unwrap();
        // println!("{:?}", rom);
        let mut bus = Bus::new();
        // Emulation mode
        {
            // Emulation mode
            bus.get_cpu().set_emulation_mode(true); // Values anded are 8-Bit
            // TODO: Fix setting values in the accumulator
            bus.get_cpu().regs_ref().borrow_mut().set_C(&Accumulator::from(0b00000001 as u16));
            //regs.get_mut().set_C(&Accumulator::from(0b00110000 as u16));
            //bus.get_cpu().regs_ref().borrow().set_C(&Accumulator::from(0b11111111 as u16));
//            *bus.get_cpu().get_regs().get_C_mut() = Accumulator::from(0b00000001 as u16);
            println!("Before:");
            println!("C: {:?}", bus.get_cpu().get_regs().get_C());
            println!("B: {:x}", bus.get_cpu().get_regs().get_C().get_B());
            println!("A: {:x}", bus.get_cpu().get_regs().get_C().get_A());

            let i = Instruction {
                address: 0x0,
                opcode: Opcodes::AND,
                // payload: vec![0b11110011],
                payload: vec![0b00000001],
                address_mode: AddressModes::Immediate,
                length: 2,
                cycles: 2,
            };

            i.execute(&mut bus, false);

            // println!("{:?}", cpu)
            println!("After:");            // println!("{:?}", cpu.get_regs());
            println!("C: {:?}", bus.get_cpu().get_regs().get_C());
            println!("B: {:x}", bus.get_cpu().get_regs().get_C().get_B());
            println!("A: {:x}", bus.get_cpu().get_regs().get_C().get_A());

            // 00110000 & 11110011 => 00110000 right?
            assert!(bus.get_cpu().get_regs().get_C().get_A() == 0b00000001);
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 0);
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
        }
        {
            // Emulation mode
            bus.get_cpu().set_emulation_mode(true); // Values anded are 8-Bit
            bus.get_cpu().regs_ref().borrow_mut().set_C(&Accumulator::from(0b00110000 as u16));

            let i = Instruction {
                address: 0x0,
                opcode: Opcodes::AND,
                payload: vec![0b0],
                address_mode: AddressModes::Immediate,
                length: 2,
                cycles: 2,
            };

            i.execute(&mut bus, false);

            assert!(bus.get_cpu().get_regs().get_C().get_A() as u8 == 0b00000000 as u8);
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 0);
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 1);
        }
        {
            // Emulation mode
            bus.get_cpu().set_emulation_mode(false); // Values anded are 16-Bit
            bus.get_cpu().regs_ref().borrow_mut().set_C(&Accumulator::from(0xff00 as u16));
            bus.get_cpu().regs_ref().borrow_mut().get_P_mut().set_m(0);

            println!("{:?}", bus.get_cpu().get_regs());
            let i = Instruction {
                address: 0x0,
                opcode: Opcodes::AND,
                payload: vec![0xff, 0xff],
                address_mode: AddressModes::Immediate,
                length: 3,
                cycles: 2,
            };

            i.execute(&mut bus, false);
            println!("{:?}", bus.get_cpu().get_regs());

            println!("{:?}", bus.get_cpu().get_regs().get_C());
            assert!(u16::from(*bus.get_cpu().get_regs().get_C()) == 0xff00);
            assert!(bus.get_cpu().get_regs().get_P().get_n() == 1);
            assert!(bus.get_cpu().get_regs().get_P().get_z() == 0);
        }
        // {
        //     // Emulation mode
        //     cpu.set_emulation_mode(false); // Values anded are 8-Bit
        //     cpu.get_regs().set_C(&Accumulator::from(0b1111000000000000 as u16));

        //     let mut i = Instruction {
        //         address: 0x0,
        //         opcode: Opcodes::AND,
        //         payload: vec![0b10000000,0b00000000],
        //         address_mode: AddressModes::Immediate,
        //         length: 3,
        //         cycles: 2,
        //     };

        //     i.execute(&mut cpu, &mut bus, false);

        //     assert!(cpu.get_regs().get_C().A as u8 == 0b10000000 as u8);
        //     assert!(cpu.get_regs().get_P().get_n() == 1);
        //     assert!(cpu.get_regs().get_P().get_z() == 0);
        // }
    }
}
