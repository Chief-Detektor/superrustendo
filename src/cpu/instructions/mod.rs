use crate::cpu::addressmodes::AddressModes;
use crate::cpu::decoder::Opcodes;
use crate::mem::Bus;
use std::borrow::BorrowMut;
use std::num::Wrapping;

use super::address::Address;
use super::decoder::Decoder;

mod ops;

#[derive(Debug, Default, Clone)]
pub struct Instruction {
    pub address: u32,
    pub opcode: Opcodes,
    pub address_mode: AddressModes,
    pub length: usize,
    pub payload: Vec<u8>,
    pub cycles: usize,
}

#[derive(Debug)]
enum Operant_Value {
    short(u8),
    long(u32),
}

impl Operant_Value {
    fn lower_to_number(&self) -> usize {
        match *self {
            Operant_Value::short(a) => {
                return a as usize;
            }
            Operant_Value::long(a) => return a as usize,
        }
    }
}

impl Instruction {
    pub fn new(decoder: &mut Decoder) -> Instruction {
        let address = Address {
            bank: decoder.get_bus().get_cpu().regs_ref().borrow().PBR,
            address: decoder.get_bus().get_cpu().regs_ref().borrow().PC,
        };
        let inst = decoder
            .decode(decoder.get_bus().read(address).try_into().unwrap())
            .unwrap();

        let payload = decoder.get_bus().read_bytes(
            address.add(1),
            inst.1
                .len(&decoder.get_bus().get_cpu().regs_ref().borrow(), &inst.0)
                - 1,
        );

        let new_pc = Wrapping(decoder.get_bus().get_cpu().regs_ref().borrow().PC)
            + Wrapping(
                (inst
                    .1
                    .len(&decoder.get_bus().get_cpu().regs_ref().borrow(), &inst.0))
                    as u16,
            );
        // increase Program Counter
        decoder.get_bus().get_cpu().regs_ref().borrow_mut().PC = new_pc.0;

        Instruction {
            address: address.address as u32,
            opcode: inst.0.clone(),
            address_mode: inst.1.clone(),
            length: inst
                .1
                .len(&decoder.get_bus().get_cpu().regs_ref().borrow(), &inst.0)
                + 1,
            payload,
            // TODO: Properly calculate cycles
            cycles: 0,
        }
    }

    fn load_value(&self, bus: &Bus) -> Option<Operant_Value> {
        // get address
        let address = self
            .address_mode
            .get_effective_address(&self.payload, &self.opcode, bus);
        if address.is_some() {
            if !&bus.get_cpu().is_16_bit_mem_and_accu() {
                let val = bus.read(address.unwrap());
                return Some(Operant_Value::short(val));
            } else {
                let val =
                    bus.read(address.unwrap()) as u32 | (bus.read(address.unwrap()) as u32) << 8; //| (regs.PBR as u32) << 16;
                return Some(Operant_Value::long(val));
            }
        } else {
            return None;
        }
    }

    // TODO: Make this clean.
    // 1. Introduce variable value
    // 2. Is addressing mode immediate?
    // - if yes then payload is value
    // - if not then load value from address and save it to value
    pub fn execute(&self, bus: &mut Bus, follow_jumps: bool) {
        // if this is None it's implied addressing
        let value;
        if self.address_mode != AddressModes::Immediate {
            value = self.load_value(bus);
        } else {
            let address = self
                .address_mode
                .get_effective_address(&self.payload, &self.opcode, bus);

            if let Some(a) = address {
                print!("{:?}", <u16>::from(a));
                value = Some(Operant_Value::long(<u16>::from(a) as u32));
            } else {
                value = None;
            }
        }
        //println!("{:?}", value);

        let effective_address =
            self.address_mode
                .get_effective_address(&self.payload, &self.opcode, bus);

        match &self.opcode {
            // TODO: handle immediate addressing early in such a way that the following patterns use it transparently/agnostic
            Opcodes::ADC => ops::adc(bus, value.expect("Value is None")),
            Opcodes::AND => ops::and(bus, value),
            Opcodes::ASL => ops::asl(bus, effective_address, self.address_mode.clone()),
            Opcodes::BCC => ops::bcc(bus, self.payload[0]),
            Opcodes::BCS => ops::bcs(bus, self.payload[0]),
            Opcodes::BEQ => ops::beq(bus, self.payload[0]),
            Opcodes::BIT => ops::bit(bus, value, self.address_mode.clone()),
            Opcodes::BMI => ops::bmi(bus, self.payload[0]),
            Opcodes::BNE => ops::bne(bus, self.payload[0]),
            Opcodes::BPL => ops::bpl(bus, self.payload[0]),
            Opcodes::BRA => ops::bra(bus, self.payload[0]),
            Opcodes::BRK => ops::brk(bus),
            Opcodes::BRL => ops::brl(bus, self.payload[0], self.payload[1]),
            Opcodes::BVC => ops::bvc(bus, self.payload[0]),
            Opcodes::BVS => ops::bvs(bus, self.payload[0]),
            Opcodes::CLC => ops::clc(bus),
            Opcodes::CLD => ops::cld(bus),
            Opcodes::CLI => ops::cli(bus),
            Opcodes::CLV => ops::clv(bus),
            Opcodes::CMP => ops::cmp(bus, value),
            Opcodes::COP => todo!(),
            Opcodes::CPX => ops::cpx(bus, value),
            Opcodes::CPY => ops::cpy(bus, value),
            Opcodes::DEC => ops::dec(bus, effective_address, self.address_mode.clone()),
            Opcodes::DEX => ops::dex(bus),
            Opcodes::DEY => ops::dey(bus),
            Opcodes::EOR => ops::eor(bus, effective_address),
            Opcodes::INC => ops::inc(bus, effective_address, self.address_mode.clone()),
            Opcodes::INX => ops::inx(bus),
            Opcodes::INY => ops::iny(bus),
            Opcodes::JMP => {
                if follow_jumps {
                    ops::jmp(bus, effective_address, self.address_mode.clone())
                }
            }
            Opcodes::JSR => {
                if follow_jumps {
                    ops::jsr(bus, effective_address, self.address_mode.clone())
                }
            }
            Opcodes::LDA => ops::lda(
                bus,
                effective_address,
                self.address_mode.clone(),
                &self.payload,
                value,
            ),
            Opcodes::LDX => ops::ldx(bus, value),
            Opcodes::LDY => ops::ldy(bus, value),
            Opcodes::LSR => ops::lsr(bus, effective_address, self.address_mode.clone()),
            Opcodes::MVN => ops::mvn(bus, self.payload[0], self.payload[1]),
            Opcodes::MVP => ops::mvp(bus, self.payload[0], self.payload[1]),
            Opcodes::NOP => { /* // No need to increment.. this is done by the iterator                //regs.PC += 1;         */
            }
            Opcodes::ORA => ops::ora(
                bus,
                effective_address,
                self.address_mode.clone(),
                self.payload[0],
                self.payload[1],
            ),
            Opcodes::PEA => ops::pea(bus, self.payload[0], self.payload[1]),
            Opcodes::PEI => ops::pei(bus, self.payload[0], effective_address),
            Opcodes::PER => ops::per(bus, self.payload[0], self.payload[1]),
            Opcodes::PHA => ops::pha(bus),
            Opcodes::PHB => ops::phb(bus),
            Opcodes::PHD => ops::phd(bus),
            Opcodes::PHK => ops::phk(bus),
            Opcodes::PHP => ops::php(bus),
            Opcodes::PHX => ops::phx(bus),
            Opcodes::PHY => ops::phy(bus),
            Opcodes::PLA => ops::pla(bus),
            Opcodes::PLB => ops::plb(bus),
            Opcodes::PLD => ops::pld(bus),
            Opcodes::PLP => ops::plp(bus),
            Opcodes::PLX => ops::plx(bus),
            Opcodes::PLY => ops::ply(bus),
            Opcodes::REP => ops::rep(bus, self.payload[0]),
            Opcodes::ROL => ops::rol(bus, effective_address, self.address_mode.clone()),
            Opcodes::ROR => ops::ror(bus, effective_address, self.address_mode.clone()),
            Opcodes::RTI => ops::rti(bus),
            Opcodes::RTL => ops::rtl(bus),
            Opcodes::RTS => ops::rts(bus),
            Opcodes::SBC => ops::sbc(bus, effective_address),
            Opcodes::SEC => ops::sec(bus),
            Opcodes::SED => ops::sed(bus),
            Opcodes::SEI => ops::sei(bus),
            Opcodes::SEP => ops::sep(bus, self.payload[0]),
            Opcodes::STA => ops::sta(bus, effective_address),
            Opcodes::STP => ops::stp(bus),
            Opcodes::STX => ops::stx(bus, effective_address),
            Opcodes::STY => ops::sty(bus, effective_address),
            Opcodes::STZ => ops::stz(bus, effective_address),
            Opcodes::TAX => ops::tax(bus),
            Opcodes::TAY => ops::tay(bus),
            Opcodes::TCD => ops::tcd(bus),
            Opcodes::TCS => ops::tcs(bus),
            Opcodes::TDC => ops::tdc(bus),
            Opcodes::TRB => ops::trb(bus, effective_address.unwrap(), self.address_mode.clone()),
            Opcodes::TSB => ops::tsb(bus, effective_address.unwrap(), self.address_mode.clone()),
            Opcodes::TSC => ops::tsc(bus),
            Opcodes::TSX => ops::tsx(bus),
            Opcodes::TXA => ops::txa(bus),
            Opcodes::TXS => ops::txs(bus),
            Opcodes::TXY => ops::txy(bus),
            Opcodes::TYA => ops::tya(bus),
            Opcodes::TYX => ops::tyx(bus),
            Opcodes::Unknown => todo!(),
            Opcodes::WAI => { /* NOTE: Wait for Interrupt */ },
            Opcodes::WDM => { /* WDM is a like a 2 byte NOP */ bus.get_cpu().regs_ref().borrow_mut().PC += 1;  }
            Opcodes::XBA => ops::xba(bus),
            Opcodes::XCE => ops::xce(bus),
//            _ => {
//                unimplemented!(
//                    "{:?} {:?} payload: {:?}",
//                    &self.opcode,
//                    &self.address_mode,
//                    &self.payload,
//                );
//            }
        }
    }

//    pub fn increment_pc(&self, bus: &mut Bus) {
//        let new_pc =
//            Wrapping(bus.get_cpu().regs_ref().borrow().PC) + Wrapping((self.length - 1) as u16);
//        // increase Program Counter
//        bus.get_cpu().regs_ref().borrow_mut().PC = new_pc.0;
//    }

    pub fn print_info(&self) {
        println!("Payload: ");
        for i in 0..self.payload.len() {
            print!("0x{:x} ", self.payload[i]);
        }
        println!(
            "0x{:x}: {:?} {:?}",
            self.address, self.opcode, self.address_mode
        );
    }
}
