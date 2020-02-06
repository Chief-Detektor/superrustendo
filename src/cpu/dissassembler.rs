use crate::cpu::addressmodes::AddressModes;
use crate::cpu::decoder::Opcodes;
use crate::cpu::instructions::Instruction;
use crate::cpu::CPU;
use std::collections::HashMap;
use std::convert::TryInto;

pub trait PrintToken {
  fn print(&self, label_map: &mut HashMap<usize, String>) -> String;
}

impl PrintToken for Instruction {
  fn print(&self, mut label_map: &mut HashMap<usize, String>) -> String {
    let mut op_string = get_opcode_string(&self.opcode);
    op_string.push_str(&get_operant_string(
      &self.address_mode,
      &self.payload,
      self.address,
      &mut label_map,
    ));
    op_string
  }
}

fn get_opcode_string(op: &Opcodes) -> String {
  match op {
    Opcodes::ADC => String::from("ADC "),
    Opcodes::AND => String::from("AND "),
    Opcodes::ASL => String::from("ASL "),
    Opcodes::BCC => String::from("BCC "),
    Opcodes::BEQ => String::from("BEQ "),
    Opcodes::BIT => String::from("BIT "),
    Opcodes::BMI => String::from("BMI "),
    Opcodes::BNE => String::from("BNE "),
    Opcodes::BPL => String::from("BPL "),
    Opcodes::BRA => String::from("BRA "),
    Opcodes::BRK => String::from("BRK "),
    Opcodes::BRL => String::from("BRL "),
    Opcodes::BVC => String::from("BVC "),
    Opcodes::BVS => String::from("BVS "),
    Opcodes::CLC => String::from("CLC "),
    Opcodes::CLD => String::from("CLD "),
    Opcodes::CLI => String::from("CLI "),
    Opcodes::CLV => String::from("CLV "),
    Opcodes::CMP => String::from("CMP "),
    Opcodes::COP => String::from("COP "),
    Opcodes::CPX => String::from("CPX "),
    Opcodes::CPY => String::from("CPY "),
    Opcodes::DEC => String::from("DEC "),
    Opcodes::DEX => String::from("DEX "),
    Opcodes::DEY => String::from("DEY "),
    Opcodes::EOR => String::from("EOR "),
    Opcodes::INC => String::from("INC "),
    Opcodes::INX => String::from("INX "),
    Opcodes::INY => String::from("INY "),
    Opcodes::JMP => String::from("JMP "),
    Opcodes::JSR => String::from("JSR "),
    Opcodes::LDA => String::from("LDA "),
    Opcodes::LDX => String::from("LDX "),
    Opcodes::LDY => String::from("LDY "),
    Opcodes::MVN => String::from("MVN "),
    Opcodes::MVP => String::from("MVP "),
    Opcodes::NOP => String::from("NOP "),
    Opcodes::ORA => String::from("ORA "),
    Opcodes::PEA => String::from("PEA "),
    Opcodes::PEI => String::from("PEI "),
    Opcodes::PER => String::from("PER "),
    Opcodes::PHA => String::from("PHA "),
    Opcodes::PHB => String::from("PHB "),
    Opcodes::PHD => String::from("PHD "),
    Opcodes::PHK => String::from("PHK "),
    Opcodes::PHP => String::from("PHP "),
    Opcodes::PHX => String::from("PHX "),
    Opcodes::PHY => String::from("PHY "),
    Opcodes::PLA => String::from("PLA "),
    Opcodes::PLB => String::from("PLB "),
    Opcodes::PLD => String::from("PLD "),
    Opcodes::PLP => String::from("PLP "),
    Opcodes::PLX => String::from("PLX "),
    Opcodes::PLY => String::from("PLY "),
    Opcodes::REP => String::from("REP "),
    Opcodes::ROL => String::from("ROL "),
    Opcodes::ROR => String::from("ROR "),
    Opcodes::RTI => String::from("RTI "),
    Opcodes::RTL => String::from("RTL "),
    Opcodes::RTS => String::from("RTS "),
    Opcodes::SBC => String::from("SBC "),
    Opcodes::SEC => String::from("SEC "),
    Opcodes::SED => String::from("SED "),
    Opcodes::SEI => String::from("SEI "),
    Opcodes::SEP => String::from("SEP "),
    Opcodes::STA => String::from("STA "),
    Opcodes::STP => String::from("STP "),
    Opcodes::STX => String::from("STX "),
    Opcodes::STY => String::from("STY "),
    Opcodes::STZ => String::from("STZ "),
    Opcodes::TAX => String::from("TAX "),
    Opcodes::TAY => String::from("TAY "),
    Opcodes::TCD => String::from("TCD "),
    Opcodes::TCS => String::from("TCS "),
    Opcodes::TDC => String::from("TDC "),
    Opcodes::TRB => String::from("TRB "),
    Opcodes::TSB => String::from("TSB "),
    Opcodes::TSC => String::from("TSC "),
    Opcodes::TSX => String::from("TSX "),
    Opcodes::TXA => String::from("TXA "),
    Opcodes::TXS => String::from("TXS "),
    Opcodes::TXY => String::from("TXY "),
    Opcodes::TYA => String::from("TYA "),
    Opcodes::TYX => String::from("TYX "),
    Opcodes::WAI => String::from("WAI "),
    Opcodes::WDM => String::from("WDM "),
    Opcodes::XBA => String::from("XBA "),
    Opcodes::XCE => String::from("XCE "),
    _ => String::from("op_unimplemented: "),
  }
}

fn get_operant_string(
  addressmode: &AddressModes,
  payload: &Vec<u8>,
  address: u32,
  label_map: &mut HashMap<usize, String>,
) -> String {
  match addressmode {
    AddressModes::Absolute => {
      let constant: u16;
      if payload.len() < 2 {
        constant = payload[0].into();
      } else {
        constant = payload[0] as u16 | (payload[1] as u16) << 8;
      }
      let mut ret = String::from("$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&number);
      ret
    }
    AddressModes::AbsoluteLong => {
      let constant: u32 = payload[0] as u32 | (payload[1] as u32) << 8 | (payload[2] as u32) << 16;
      let mut ret = String::from("$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&number);
      ret
    }
    AddressModes::AbsoluteLongIndexedX => {
      let constant: u32 = payload[0] as u32 | (payload[1] as u32) << 8 | (payload[2] as u32) << 16;
      let mut ret = String::from("$");
      let mut number = format!("{:x}, X", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&number);
      ret
    }
    AddressModes::DirectPage => {
      let constant: u8 = payload[0];
      let mut ret = String::from("$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&format!("{}", number));
      ret
    }
    AddressModes::DirectPageIndirectLong => {
      let constant: u8 = payload[0];
      let mut ret = String::from("$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&format!("[{}]", number));
      ret
    }
    AddressModes::DirectPageIndirect => {
      let constant: u8 = payload[0];
      let mut ret = String::from("$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&format!("({})", number));
      ret
    }
    AddressModes::DirectPageIndexedIndirectX => {
      let constant: u8 = payload[0];
      let mut ret = String::from("$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&format!("({}, X)", number));
      ret
    }
    AddressModes::Immediate => {
      let constant: u16;
      if payload.len() < 2 {
        constant = payload[0].into();
      } else {
        constant = payload[0] as u16 | (payload[1] as u16) << 8;
      }
      let mut ret = String::from("#$");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&number);
      ret
    }
    AddressModes::Implied => String::from(""),
    AddressModes::StackPush => String::from(""),
    AddressModes::StackRTS => String::from(""),
    AddressModes::StackRelative => {
      let constant: u8 = payload[0];
      let mut ret = String::from("");
      let mut number = format!("{:x}", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&format!("{}, S", number));
      ret
    }
    AddressModes::StackRelativeIndirectIndexedY => {
      let constant: u8 = payload[0];
      let mut ret = String::from("");
      let mut number = format!("({:x},S), Y", constant).to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      ret.push_str(&format!("{}", number));
      ret
    }
    AddressModes::ProgrammCounterRelative => {
      let constant: i8 = payload[0] as _;
      let mut ret = String::from("");
      println!("Address: {:x}", address);
      println!("Offset: {:x}", constant);
      let mut number = format!(
        "{:x}",
        // TODO: Ensure that op_length is properly set on decoding
        (address as i32 + /*op_length as i32*/ 2 + constant as i32) as u32
      )
      .to_string();
      if number.len() % 2 == 1 {
        number = "0".to_owned() + &number;
      }
      let label_number = label_map.len().to_string();
      &label_map.insert(
        ((address as i32 + /*op_length as i32*/ 2 + constant as i32) as u32)
          .try_into()
          .unwrap(),
        "Label_".to_owned() + &label_number,
      );
      // ret.push_str(&number);
      ret.push_str(&(("Label_".to_owned() + &label_number) + &" ; 0x".to_owned() + &number));
      ret
    }
    AddressModes::DirectPageIndirectLongIndexedY => {
      let constant = payload[0];
      let mut ret = String::from("$");
      let mut number = format!("[{:x}], Y", constant).to_string();
      // if number.len() % 2 == 1 {
      //   number = "0".to_owned() + &number;
      // }
      ret.push_str(&number);
      ret
    }
    AddressModes::StackPull => String::from(""),
    AddressModes::StackRTI => String::from(""),
    AddressModes::StackInterrupt => String::from(""),
    AddressModes::Accumulator => String::from("A"),
    AddressModes::Unknown => String::from("{:x}".to_owned() + &payload[0].to_string()),
    AddressModes::BlockMove => payload[0].to_string() + "," + &payload[1].to_string(),
    AddressModes::StackAbsolute => payload[0].to_string(), // _ => String::from("addmode_unimplemented"),
    _ => panic!("{:?}", addressmode),
  }
}
