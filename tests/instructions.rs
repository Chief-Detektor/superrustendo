#[cfg(test)]
mod tests {
  extern crate superrustendo;

  use superrustendo::cpu::addressmodes::AddressModes;
  use superrustendo::cpu::decoder::Opcodes;
  use superrustendo::cpu::instructions::Instruction;

  #[test]
  fn sei_instruction() {
    let i = Instruction::new(false);
    // let i = Instruction {
    //   address: 0x0,
    //   opcode: Opcodes::SEI,
    //   payload: vec![0b11011010],
    //   address_mode: AddressModes::Immediate,
    //   length: 2,
    //   cycles: 4,
    // };
  }
}
