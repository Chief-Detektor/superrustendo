use crate::cpu::address::Address;
use crate::cpu::addressmodes::{
    get_gi_addr_mode, get_gii_addr_mode, get_gii_reg_load_addr_mode, AddressModes,
};
use crate::cpu::constants::*;
use crate::cpu::instructions::Instruction;
use crate::cpu::CPU;
use crate::mem::Bus;
use std::convert::TryInto;
use std::num::Wrapping;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcodes {
    Unknown,
    // Group I Opcodes
    ADC,
    AND,
    CMP,
    EOR,
    LDA,
    ORA,
    SBC,
    STA,
    // Group II opcodes
    ASL,
    DEC,
    INC,
    LSR,
    ROL,
    ROR,
    STX,
    STY,
    // Group III opcodes
    BRK,
    COP,
    TSB,
    PHP,
    PHD,
    BPL,
    TRB,
    CLC,
    TCS,
    JSR,
    BIT,
    PLP,
    PLD,
    BMI,
    SEC,
    TSC,
    RTI,
    WDM,
    MVP,
    PHA,
    PHK,
    JMP,
    BVC,
    MVN,
    CLI,
    PHY,
    TCD,
    RTS,
    PER,
    STZ,
    PLA,
    RTL,
    BVS,
    SEI,
    PLY,
    TDC,
    BRA,
    BRL,
    DEY,
    TXA,
    PHB,
    BCC,
    TYA,
    TXY,
    TXS,
    LDX,
    LDY,
    TAY,
    TAX,
    PLB,
    BCS,
    CLV,
    TSX,
    TYX,
    CPY,
    REP,
    INY,
    DEX,
    WAI,
    BNE,
    PEI,
    CLD,
    PHX,
    STP,
    CPX,
    INX,
    NOP,
    XBA,
    BEQ,
    PEA,
    SED,
    PLX,
    XCE,
    SEP,
}

impl Default for Opcodes {
    fn default() -> Opcodes {
        Opcodes::Unknown
    }
}

fn decode_group_III(opcode: u8) -> Option<(Opcodes, AddressModes)> {
    match opcode | G3_OP_TSB {
        G3_OP_TSB => match opcode {
            0xc => return Some((Opcodes::TSB, AddressModes::Absolute)),
            0x4 => return Some((Opcodes::TSB, AddressModes::DirectPage)),
            _ => {}
        },
        _ => {}
    }
    match opcode | G3_OP_TRB {
        G3_OP_TRB => match opcode {
            0x1c => return Some((Opcodes::TRB, AddressModes::Absolute)),
            0x14 => return Some((Opcodes::TRB, AddressModes::DirectPage)),
            _ => {}
        },
        _ => {}
    }
    match opcode | G3_OP_JSR {
        G3_OP_JSR => match opcode {
            0x20 => return Some((Opcodes::JSR, AddressModes::Absolute)),
            0xfc => return Some((Opcodes::JSR, AddressModes::AbsoluteIndexedIndirect)),
            0x22 => return Some((Opcodes::JSR, AddressModes::AbsoluteLong)),
            _ => {}
        },
        _ => {}
    }
    match opcode | G3_OP_BIT {
        G3_OP_BIT => match opcode {
            0x24 => return Some((Opcodes::BIT, AddressModes::DirectPage)),
            0x2c => return Some((Opcodes::BIT, AddressModes::Absolute)),
            0x34 => return Some((Opcodes::BIT, AddressModes::DirectPageIndexedX)),
            0x3c => return Some((Opcodes::BIT, AddressModes::AbsoluteIndexedX)),
            0x89 => return Some((Opcodes::BIT, AddressModes::Immediate)), // bytes length can be 3 if m = 0 (16 bit accumulator)
            _ => {}
        },
        _ => {}
    }
    match opcode | G3_OP_JMP {
        G3_OP_JMP => match opcode {
            0x4c => return Some((Opcodes::JMP, AddressModes::Absolute)),
            0x5c => return Some((Opcodes::JMP, AddressModes::AbsoluteLong)),
            0x6c => return Some((Opcodes::JMP, AddressModes::AbsoluteIndirect)),
            0x7c => return Some((Opcodes::JMP, AddressModes::AbsoluteIndexedIndirect)),
            0xdc => return Some((Opcodes::JMP, AddressModes::AbsoluteIndirectLong)), // bytes length can be 3 if m = 0 (16 bit accumulator)
            _ => {}
        },
        _ => {}
    }
    match opcode | G3_OP_STZ {
        G3_OP_STZ => match opcode {
            0x9c => return Some((Opcodes::STZ, AddressModes::Absolute)),
            0x64 => return Some((Opcodes::STZ, AddressModes::DirectPage)),
            0x9e => return Some((Opcodes::STZ, AddressModes::AbsoluteIndexedX)),
            0x74 => return Some((Opcodes::STZ, AddressModes::DirectPageIndexedX)),
            _ => {}
        },
        _ => {}
    }

    match opcode | G3_OP_CPY {
        G3_OP_CPY => match opcode {
            0xc0 => return Some((Opcodes::CPY, AddressModes::Immediate)),
            0xc4 => return Some((Opcodes::CPY, AddressModes::DirectPage)),
            0xcc => return Some((Opcodes::CPY, AddressModes::Absolute)),
            _ => {}
        },
        _ => {}
    }

    match opcode {
        G3_OP_SEP => return Some((Opcodes::SEP, AddressModes::Immediate)), // This one was missing >_<
        _ => {}
    }

    match opcode | G3_OP_CPX {
        G3_OP_CPX => match opcode {
            0xe0 => return Some((Opcodes::CPX, AddressModes::Immediate)),
            0xe4 => return Some((Opcodes::CPX, AddressModes::DirectPage)),
            0xec => return Some((Opcodes::CPX, AddressModes::Absolute)),
            _ => {}
        },
        _ => {}
    }
    match opcode {
        G3_OP_BRK => Some((Opcodes::BRK, AddressModes::StackInterrupt)),
        G3_OP_COP => Some((Opcodes::COP, AddressModes::StackInterrupt)),
        G3_OP_PHP => Some((Opcodes::PHP, AddressModes::StackPush)),
        G3_OP_PHD => Some((Opcodes::PHD, AddressModes::StackPush)),
        G3_OP_BPL => Some((Opcodes::BPL, AddressModes::ProgrammCounterRelative)),
        G3_OP_CLC => Some((Opcodes::CLC, AddressModes::Implied)),
        G3_OP_TCS => Some((Opcodes::TCS, AddressModes::Implied)),
        G3_OP_PLP => Some((Opcodes::PLP, AddressModes::StackPull)),
        G3_OP_PLD => Some((Opcodes::PLD, AddressModes::StackPull)),
        G3_OP_BMI => Some((Opcodes::BMI, AddressModes::ProgrammCounterRelative)),
        G3_OP_SEC => Some((Opcodes::SEC, AddressModes::Implied)),
        G3_OP_TSC => Some((Opcodes::TSC, AddressModes::Implied)),
        G3_OP_RTI => Some((Opcodes::RTI, AddressModes::StackRTI)),
        G3_OP_WDM => Some((Opcodes::WDM, AddressModes::Unknown)),
        G3_OP_MVP => Some((Opcodes::MVP, AddressModes::BlockMove)),
        G3_OP_PHA => Some((Opcodes::PHA, AddressModes::StackPush)),
        G3_OP_PHK => Some((Opcodes::PHK, AddressModes::StackPush)),
        G3_OP_BVC => Some((Opcodes::BVC, AddressModes::ProgrammCounterRelative)),
        G3_OP_MVN => Some((Opcodes::MVN, AddressModes::BlockMove)),
        G3_OP_CLI => Some((Opcodes::CLI, AddressModes::Implied)),
        G3_OP_PHY => Some((Opcodes::PHY, AddressModes::StackPush)),
        G3_OP_TCD => Some((Opcodes::TCD, AddressModes::Implied)),
        G3_OP_RTS => Some((Opcodes::RTS, AddressModes::StackRTS)),
        G3_OP_PER => Some((Opcodes::PER, AddressModes::StackPCRelativeLong)),
        G3_OP_PLA => Some((Opcodes::PLA, AddressModes::StackPull)),
        G3_OP_RTL => Some((Opcodes::RTL, AddressModes::StackRTL)),
        G3_OP_BVS => Some((Opcodes::BVS, AddressModes::ProgrammCounterRelative)),
        G3_OP_SEI => Some((Opcodes::SEI, AddressModes::Implied)),
        G3_OP_PLY => Some((Opcodes::PLY, AddressModes::StackPull)),
        G3_OP_TDC => Some((Opcodes::TDC, AddressModes::Implied)),
        G3_OP_BRA => Some((Opcodes::BRA, AddressModes::ProgrammCounterRelative)),
        G3_OP_BRL => Some((Opcodes::BRL, AddressModes::ProgrammCounterRelativeLong)),
        G3_OP_DEY => Some((Opcodes::DEY, AddressModes::Implied)),
        G3_OP_TXA => Some((Opcodes::TXA, AddressModes::Implied)),
        G3_OP_PHB => Some((Opcodes::PHB, AddressModes::StackPush)),
        G3_OP_BCC => Some((Opcodes::BCC, AddressModes::ProgrammCounterRelative)),
        G3_OP_TYA => Some((Opcodes::TYA, AddressModes::Implied)),
        G3_OP_TXS => Some((Opcodes::TXS, AddressModes::Implied)),
        G3_OP_TXY => Some((Opcodes::TXY, AddressModes::Implied)),
        G3_OP_TAY => Some((Opcodes::TAY, AddressModes::Implied)),
        G3_OP_TAX => Some((Opcodes::TAX, AddressModes::Implied)),
        G3_OP_PLB => Some((Opcodes::PLB, AddressModes::StackPull)),
        G3_OP_BCS => Some((Opcodes::BCS, AddressModes::ProgrammCounterRelative)),
        G3_OP_CLV => Some((Opcodes::CLV, AddressModes::Implied)),
        G3_OP_TSX => Some((Opcodes::TSX, AddressModes::Implied)),
        G3_OP_TYX => Some((Opcodes::TYX, AddressModes::Implied)),
        G3_OP_REP => Some((Opcodes::REP, AddressModes::Immediate)),
        G3_OP_INY => Some((Opcodes::INY, AddressModes::Implied)),
        G3_OP_DEX => Some((Opcodes::DEX, AddressModes::Implied)),
        G3_OP_WAI => Some((Opcodes::WAI, AddressModes::Implied)),
        G3_OP_BNE => Some((Opcodes::BNE, AddressModes::ProgrammCounterRelative)),
        G3_OP_PEI => Some((Opcodes::PEI, AddressModes::StackDirectPageIndirect)),
        G3_OP_CLD => Some((Opcodes::CLD, AddressModes::Implied)),
        G3_OP_PHX => Some((Opcodes::PHX, AddressModes::StackPush)),
        G3_OP_STP => Some((Opcodes::STP, AddressModes::Implied)),
        G3_OP_INX => Some((Opcodes::INX, AddressModes::Implied)),
        G3_OP_NOP => Some((Opcodes::NOP, AddressModes::Implied)),
        G3_OP_XBA => Some((Opcodes::XBA, AddressModes::Implied)),
        G3_OP_BEQ => Some((Opcodes::BEQ, AddressModes::ProgrammCounterRelative)),
        G3_OP_PEA => Some((Opcodes::PEA, AddressModes::StackAbsolute)),
        G3_OP_SED => Some((Opcodes::SED, AddressModes::Implied)),
        G3_OP_PLX => Some((Opcodes::PLX, AddressModes::StackPull)),
        G3_OP_XCE => Some((Opcodes::XCE, AddressModes::Implied)),
        _ => None,
    }
}

fn decode_group_II(opcode: u8) -> Option<(Opcodes, AddressModes)> {
    let g2_mask = opcode & !GII_MASK;
    let g2_mask2 = opcode & !GII_MASK2;

    // LDX LDY

    // Edge Cases...
    match opcode {
        0x1a => {
            return Some((Opcodes::INC, AddressModes::Accumulator));
        }
        0x3a => {
            return Some((Opcodes::DEC, AddressModes::Accumulator));
        }
        0xe2 => {
            return Some((Opcodes::CPX, AddressModes::Immediate)); // Only on 65816...
        }
        _ => {}
    }

    match g2_mask2 {
        G2_OP_DEC => {
            if let Some(address_mode) = get_gii_addr_mode(opcode) {
                return Some((Opcodes::DEC, address_mode));
            } else {
                return None;
            }
        }
        G2_OP_INC => {
            if let Some(addr_mode) = get_gii_addr_mode(opcode) {
                return Some((Opcodes::INC, addr_mode));
            } else {
                return None;
            }
        }
        G2_OP_STX => {
            if let Some(address_mode) = get_gii_addr_mode(opcode) {
                return Some((Opcodes::STX, address_mode));
            } else {
                return None;
            }
        }
        G2_OP_STY => {
            if let Some(address_mode) = get_gii_addr_mode(opcode) {
                return Some((Opcodes::STY, address_mode));
            } else {
                return None;
            }
        }
        _ => {}
    }

    match g2_mask {
        G2_OP_ASL => {
            if let Some(addr_mode) = get_gii_addr_mode(opcode) {
                Some((Opcodes::ASL, addr_mode))
            } else {
                None
            }
        }
        G2_OP_LSR => {
            if let Some(addr_mode) = get_gii_addr_mode(opcode) {
                Some((Opcodes::LSR, addr_mode))
            } else {
                None
            }
        }
        G2_OP_ROL => {
            if let Some(addr_mode) = get_gii_addr_mode(opcode) {
                Some((Opcodes::ROL, addr_mode))
            } else {
                None
            }
        }
        G2_OP_ROR => {
            if let Some(address_mode) = get_gii_addr_mode(opcode) {
                Some((Opcodes::ROR, address_mode))
            } else {
                None
            }
        }
        // Diffrend addressing here
        G2_OP_LDX => {
            if let Some(address_mode) = get_gii_reg_load_addr_mode(opcode) {
                Some((Opcodes::LDX, address_mode))
            } else {
                None
            }
        }
        G2_OP_LDY => {
            if let Some(address_mode) = get_gii_reg_load_addr_mode(opcode) {
                Some((Opcodes::LDY, address_mode))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn decode_group_I(opcode: u8) -> Option<(Opcodes, AddressModes)> {
    let group_1_mask: u8 = !GI_MASK;
    let g1_mask = opcode & group_1_mask;

    match g1_mask {
        G1_OP_ADC => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::ADC, addr_mode))
            } else {
                None
            }
        }
        G1_OP_AND => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::AND, addr_mode))
            } else {
                None
            }
        }
        G1_OP_CMP => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::CMP, addr_mode))
            } else {
                None
            }
        }
        G1_OP_EOR => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::EOR, addr_mode))
            } else {
                None
            }
        }
        G1_OP_LDA => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::LDA, addr_mode))
            } else {
                None
            }
        }
        G1_OP_ORA => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::ORA, addr_mode))
            } else {
                None
            }
        }
        G1_OP_SBC => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::SBC, addr_mode))
            } else {
                None
            }
        }
        G1_OP_STA => {
            if let Some(addr_mode) = get_gi_addr_mode(opcode) {
                Some((Opcodes::STA, addr_mode))
            } else {
                None
            }
        }
        _ => None,
    }
}

// #[derive(Debug)]
pub struct Decoder<'t> {
    instructions: Vec<Instruction>,
    pub cpu: &'t mut CPU,
    bus: &'t mut Bus,
    follow_jumps: bool,
}

impl<'t> Decoder<'t> {
    pub fn new(cpu: &'t mut super::CPU, bus: &'t mut Bus, follow_jumps: bool) -> Decoder<'t> {
        let mut decoder = Decoder {
            instructions: Vec::new(),
            cpu: cpu,
            bus: bus,
            follow_jumps: follow_jumps,
        };

        match decoder.bus.cartridge {
            Some(_) => {
                decoder.cpu.regs.PC = decoder
                    .bus
                    .cartridge
                    .as_ref()
                    .unwrap()
                    .get_emu_reset_vector();
            }
            None => {}
        }

        decoder
    }

    pub fn execute_instruction(&mut self, instruction: &mut Instruction) {
        println!("Executing: {:?}", instruction);
        instruction.execute(self.cpu, self.bus, self.follow_jumps);
    }

    /// # Examples
    ///
    /// ```
    /// use superrustendo::cpu::addressmodes::AddressModes;
    ///
    /// use superrustendo::cpu::decoder::{ Decoder, Opcodes};
    /// use superrustendo::mem::{Bus, WRAM};
    /// use superrustendo::cpu::{Accumulator, CPU, IndexRegister, Registers, StatusRegister};
    /// use std::convert::TryInto;
    ///
    /// let mut c = CPU::new();
    /// let mut m = Bus { cartridge: None, wram: WRAM::new()};
    /// let d = Decoder::new(&mut c, &mut m, /* follow_jumps */ false);
    /// let result = d.decode(0x3d);
    /// let res = result.unwrap();
    /// let addr = res.1;
    /// let op = res.0;
    /// assert_eq!(op, Opcodes::AND);
    /// assert_eq!(addr, AddressModes::AbsoluteIndexedX);
    /// ```
    pub fn decode(&self, opcode: u8) -> Result<(Opcodes, AddressModes), &'static str> {
        // Group I decode
        if let Some(instr) = decode_group_III(opcode) {
            // println!("Group III: {:?}", instr);
            return Ok(instr);
        } else if let Some(instr) = decode_group_II(opcode) {
            // println!("Group II: {:?}", instr);
            return Ok(instr);
        } else if let Some(instr) = decode_group_I(opcode) {
            // println!("Group I: {:?}", instr);
            return Ok(instr);
        }
        // This should never happen because everyting between 0x00..=0xff is interpreted
        unreachable!();
    }
}

// This needs to be on ROM?
impl Iterator for Decoder<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {
        // TODO: Evaluate this
        let address = Address {
            bank: self.cpu.regs.PBR,
            address: self.cpu.regs.PC,
        };
        let inst = self
            .decode(self.bus.read(address).try_into().unwrap())
            .unwrap();

        let payload = self.bus.cartridge.as_ref().unwrap().read_bytes(
            (self.cpu.regs.PC as u32 + 1) as usize, // The payload starts 1 after opcode
            inst.1.len(&self.cpu.regs, &inst.0) - 1, // substract the opcode from length
        );

        let mut instr = Instruction::new();
        instr.address = self.cpu.regs.PC as _;

        // TODO: Handle Overflow

        let mut new_pc = Wrapping(self.cpu.regs.PC);
        // increase Programm Counter
        new_pc.0 += inst.1.len(&self.cpu.regs, &inst.0) as u16;
        self.cpu.regs.PC = new_pc.0;

        instr.opcode = inst.0;
        instr.address_mode = inst.1;
        instr.payload = payload;

        instr.execute(&mut self.cpu, &mut self.bus, self.follow_jumps);
        Some(instr)
    }
}
