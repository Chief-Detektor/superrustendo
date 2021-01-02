#[cfg(test)]
// #![feature(or_patterns)]

mod tests {
    extern crate superrustendo;

    use std::path::Path;

    use superrustendo::{cartridge::Cartridge, cpu::{decoder::Opcodes, Accumulator, IndexRegister, StatusRegister}};
    use superrustendo::cpu::*;
    use superrustendo::{cpu::instructions::Instruction, mem::WRAM};
    use superrustendo::{
        cpu::{addressmodes::AddressModes, decoder::Decoder, CPU},
        mem::Bus,
    };

    #[test]
    fn and_instruction() {
        // Well we need to have a dummy rom here.. Maybe one that has first byte 0x0 second 0x1 etc...
        // That way the rom would be deterministic: bus.read([80]12) => 13
        let mut cpu = CPU::new();

        // let rom = Cartridge::load_rom(Path::new("./snes-test-roms/PeterLemon/SNES-CPUTest-CPU/AND/CPUAND.sfc")).unwrap();
        // println!("{:?}", rom);
        let mut bus = Bus {
            cartridge: None,
            wram: WRAM::new(),
        };
        // Emulation mode
        {
            // Emulation mode
            cpu.set_emulation_mode(true); // Values anded are 8-Bit
            cpu.get_regs().set_C(&Accumulator::from(0b00000001 as u16));
            println!("{:?}", cpu.get_regs().get_C());

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::AND,
                // payload: vec![0b11110011],
                payload: vec![0x0f],
                address_mode: AddressModes::Immediate,
                length: 2,
                cycles: 2,
            };

            i.execute(&mut cpu, &mut bus, false);

            // println!("{:?}", cpu)
            // println!("{:?}", cpu.get_regs());
            println!("{:?}", cpu.get_regs().get_C().get_A());

            // 00110000 & 11110011 => 00110000 right?
            assert!(cpu.get_regs().get_C().get_A() as u8 == 1 as u8);
            assert!(cpu.get_regs().get_P().get_n() == 0);
            assert!(cpu.get_regs().get_P().get_z() == 0);
        }
        {
            // Emulation mode
            cpu.set_emulation_mode(true); // Values anded are 8-Bit
            cpu.get_regs().set_C(&Accumulator::from(0b00110000 as u16));

            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::AND,
                payload: vec![0b0],
                address_mode: AddressModes::Immediate,
                length: 2,
                cycles: 2,
            };

            i.execute(&mut cpu, &mut bus, false);

            assert!(cpu.get_regs().get_C().get_A() as u8 == 0b00000000 as u8);
            assert!(cpu.get_regs().get_P().get_n() == 0);
            assert!(cpu.get_regs().get_P().get_z() == 1);
        }
        {
            // Emulation mode
            cpu.set_emulation_mode(false); // Values anded are 16-Bit
            cpu.get_regs().set_C(&Accumulator::from(0xff00 as u16));
            cpu.get_regs().get_P().set_m(0);

            println!("{:?}", cpu.get_regs());
            let mut i = Instruction {
                address: 0x0,
                opcode: Opcodes::AND,
                payload: vec![0xff,0xff],
                address_mode: AddressModes::Immediate,
                length: 3,
                cycles: 2,
            };

            i.execute(&mut cpu, &mut bus, false);
            println!("{:?}", cpu.get_regs());

            println!("{:?}", cpu.get_regs().get_C());
            assert!(u16::from(*cpu.get_regs().get_C()) == 0xff00);
            assert!(cpu.get_regs().get_P().get_n() == 1);
            assert!(cpu.get_regs().get_P().get_z() == 0);
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
