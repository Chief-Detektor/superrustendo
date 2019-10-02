
mod cpu;

// use crate::cpu::constants::GI_MASK;
use crate::cpu::*;

// #[macro_use]
// extern crate bitpat;



fn main() {
  // ADC opcode test
  Decoder::decode(0x69);
  Decoder::decode(0x6D);
  Decoder::decode(0x6F);
  Decoder::decode(0x65);
  Decoder::decode(0x72);
  Decoder::decode(0x67);
  Decoder::decode(0x7D);
  Decoder::decode(0x7F);
  Decoder::decode(0x79);
  Decoder::decode(0x75);
  Decoder::decode(0x61);
  Decoder::decode(0x71);
  Decoder::decode(0x77);
  Decoder::decode(0x63);
  Decoder::decode(0x73);

  // ASL
  Decoder::decode(0x0A);
  Decoder::decode(0x0E);
  // Decoder::decode(0x6F);
  // Decoder::decode(0x6F);
}
