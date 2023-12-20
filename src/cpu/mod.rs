use byte_struct::{bitfields, ByteStruct, ByteStructLen, ByteStructUnspecifiedByteOrder};

use std::convert::From;
use std::fmt;

pub mod address;
pub mod addressmodes;
pub mod constants;
pub mod decoder;
pub mod instructions;

// in emulation mode $100 to $1FF
#[derive(Copy, Clone)]
pub struct Stack {
    content: [u8; 0x10000],
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack {{ too large to print }}")
    }
}

impl Default for Stack {
    fn default() -> Stack {
        Stack {
            content: [0; 0x10000],
        }
    }
}

bitfields!(
  #[derive(PartialEq, Copy, Clone)]
  pub StatusRegister: u8 {
      c: 1, // CarryBit / Emulation Mode
      z: 1, // Result Zero
      i: 1, // IRQ Disable
      d: 1, // Decimal Mde
      x: 1, // Index Register Select/Break Instruction
      m: 1, // Memory/Accumulator Select
      v: 1, // Overflow
      n: 1, // Negative
   }
);

// impl From<StatusRegister> for u8 {
//     fn from(val: StatusRegister) -> Self {
//         let P =
//              val.c & 0x1
//             |(val.z >> 1) & 0x1
//             |(val.i >> 2) & 0x1
//             |(val.d >> 3) & 0x1
//             |(val.x >> 4) & 0x1
//             |(val.m >> 5) & 0x1
//             |(val.v >> 6) & 0x1
//             |(val.n >> 7) & 0x1;
//         P
//     }
// }
// impl From<u8> for StatusRegister {
//     fn from(val: u8) -> Self {
//         let P = StatusRegister{
//             c: val & 0x1,
//             z:(val >> 1) & 0x1,
//             i:(val >> 2) & 0x1,
//             d:(val >> 3) & 0x1,
//             x:(val >> 4) & 0x1,
//             m:(val >> 5) & 0x1,
//             v:(val >> 6) & 0x1,
//             n:(val >> 7) & 0x1
//         };
//         P
//     }
// }

impl StatusRegister {
    pub fn get_c(&self) -> u8 {
        return self.c;
    }
    pub fn get_z(&self) -> u8 {
        return self.z;
    }
    pub fn get_i(&self) -> u8 {
        return self.i;
    }
    pub fn get_d(&self) -> u8 {
        return self.d;
    }
    pub fn get_x(&self) -> u8 {
        return self.x;
    }
    pub fn get_m(&self) -> u8 {
        return self.m;
    }
    pub fn get_v(&self) -> u8 {
        return self.v;
    }
    pub fn get_n(&self) -> u8 {
        return self.n;
    }
    pub fn get_b(&self) -> u8 {
        return self.get_x();
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }
    pub fn set_z(&mut self, value: u8) {
        self.z = value;
    }
    pub fn set_i(&mut self, value: u8) {
        self.i = value;
    }
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }
    pub fn set_x(&mut self, value: u8) {
        self.x = value;
    }
    pub fn set_m(&mut self, value: u8) {
        self.m = value;
    }
    pub fn set_v(&mut self, value: u8) {
        self.v = value;
    }
    pub fn set_n(&mut self, value: u8) {
        self.n = value;
    }
    pub fn set_b(&mut self, value: u8) {
        self.set_x(value);
    }
}

// NOTE: This is because default Debug prints the single bits in reverse order, which sucks to debug
impl fmt::Debug for StatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StatusRegister {{ n: {}, v: {}, m: {}, x: {}, d: {}, i: {}, z: {}, c: {} }}",
            self.n, self.v, self.m, self.x, self.d, self.i, self.z, self.c
        )
    }
}

// Conversion helper functions
impl From<StatusRegister> for u8 {
    fn from(p: StatusRegister) -> Self {
        let mut number = [0];
        p.write_bytes_default_le(&mut number);
        return number[0];
    }
}

impl From<u8> for StatusRegister {
    fn from(byte: u8) -> Self {
        byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[byte])
    }
}

impl From<u8> for IndexRegister {
    fn from(number: u8) -> Self {
        let high = 0x0;
        let low = number;
        byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
    }
}

impl From<IndexRegister> for u16 {
    fn from(register: IndexRegister) -> Self {
        let mut number = [0; 2];
        register.write_bytes_default_le(&mut number);
        return (number[1] as u16) << 8 | number[0] as u16;
    }
}

impl From<u16> for IndexRegister {
    fn from(number: u16) -> Self {
        let high = (number >> 8) as u8;
        let low = (number & 0xff) as u8;
        byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
    }
}

impl From<Accumulator> for u16 {
    fn from(register: Accumulator) -> Self {
        let mut number = [0; 2];
        register.write_bytes_default_le(&mut number);
        return (number[1] as u16) << 8 | number[0] as u16;
    }
}

impl From<u16> for Accumulator {
    fn from(number: u16) -> Self {
        let high = (number >> 8) as u8;
        let low = (number & 0xff) as u8;
        byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
    }
}

impl From<Accumulator> for usize {
    fn from(register: Accumulator) -> Self {
        let mut number = [0; 2];
        register.write_bytes_default_le(&mut number);
        return (number[1] as usize) << 8 | number[0] as usize;
    }
}

impl From<usize> for Accumulator {
    fn from(number: usize) -> Self {
        let high = (number >> 8) as u8;
        let low = (number & 0xff) as u8;
        byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[low, high])
    }
}

// NOTE: Verify if this is correct
impl Default for StatusRegister {
    fn default() -> StatusRegister {
        StatusRegister {
            n: 0,
            v: 0,
            m: 1,
            x: 1,
            d: 0,
            i: 1,
            z: 0,
            c: 0,
        }
    }
}

bitfields!(
  #[derive(PartialEq, Copy, Clone)]
  pub Accumulator: u16 {
    A: 8,
    B: 8,
  }
);

impl fmt::Debug for Accumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Accumulator {{ B: {:x}, A: {:x} }}", self.B, self.A)
    }
}

impl Default for Accumulator {
    fn default() -> Accumulator {
        Accumulator { A: 0, B: 0 }
    }
}

impl Accumulator {
    pub fn get_A(&self) -> u8 {
        return self.A as u8;
    }
    pub fn get_B(&self) -> u8 {
        return self.B as u8;
    }
    pub fn set_A(&mut self, A: u8) -> &mut Self {
        self.A = A.into();
        self
    }
    pub fn set_B(&mut self, B: u8) -> &mut Self {
        self.B = B.into();
        self
    }
}

bitfields!(
  #[derive(PartialEq, Debug, Copy, Clone)]
  pub IndexRegister: u16 {
    low: 8,
    high: 8
  }
);

impl Default for IndexRegister {
    fn default() -> IndexRegister {
        IndexRegister { low: 0, high: 0 }
    }
}

impl IndexRegister {
    pub fn new() -> IndexRegister {
        IndexRegister {
            low: 0xff,
            high: 0xff,
        }
    }
    pub fn get_low(&self) -> u8 {
        return self.low as u8;
    }
    pub fn get_high(&self) -> u8 {
        return self.high as u8;
    }
    pub fn set_low(&mut self, low: u8) -> &mut Self {
        self.low = low.into();
        self
    }
    pub fn set_high(&mut self, high: u8) -> &mut Self {
        self.high = high.into();
        self
    }
}

// TODO: Proper inital state
#[derive(ByteStruct, PartialEq, Debug, Clone, Copy)]
#[byte_struct_le]
pub struct Registers {
    P: StatusRegister,
    C: Accumulator,
    X: IndexRegister, // X Index Register,
    Y: IndexRegister, // Y Index Register,
    D: u16,           // Direct Page Register
    S: IndexRegister, // Stack Pointer (or 24 bits?)
    PBR: u8,          // Programm Bank Register
    DBR: u8,          // Data Bank Register
    PC: u16,          // Programm Counter
}

impl Registers {
    // Setteras sly as a fox as strong as an oxs
    pub fn set_P(&mut self, P: &StatusRegister) -> &mut Self {
        self.P = *P;
        self
    }
    pub fn set_C(&mut self, C: &Accumulator) -> &mut Self {
        self.C = *C;
        self
    }
    pub fn set_X(&mut self, X: &IndexRegister) -> &mut Self {
        self.X = *X;
        self
    }
    pub fn set_Y(&mut self, Y: &IndexRegister) -> &mut Self {
        self.Y = *Y;
        self
    }
    pub fn set_D(&mut self, D: &u16) -> &mut Self {
        self.D = *D;
        self
    }
    pub fn set_S(&mut self, S: &IndexRegister) -> &mut Self {
        self.S = *S;
        self
    }
    pub fn set_PBR(&mut self, PBR: &u8) -> &mut Self {
        self.PBR = *PBR;
        self
    }
    pub fn set_DBR(&mut self, DBR: &u8) -> &mut Self {
        self.DBR = *DBR;
        self
    }
    pub fn set_PC(&mut self, PC: &u16) -> &mut Self {
        self.PC = *PC;
        self
    }
    // Getters
    pub fn get_P(&mut self) -> &mut StatusRegister {
        return &mut self.P;
    }
    pub fn get_C(&mut self) -> &mut Accumulator {
        return &mut self.C;
    }
    pub fn get_X(&mut self) -> &mut IndexRegister {
        return &mut self.X;
    }
    pub fn get_Y(&mut self) -> &mut IndexRegister {
        return &mut self.Y;
    }
    pub fn get_D(&self) -> u16 {
        return self.D;
    }
    pub fn get_S(&mut self) -> &mut IndexRegister {
        return &mut self.S;
    }
    pub fn get_PBR(&self) -> u8 {
        return self.PBR;
    }
    pub fn get_DBR(&self) -> u8 {
        return self.DBR;
    }
    pub fn get_PC(&self) -> u16 {
        return self.PC;
    }
}

#[derive(Debug)]
pub struct CPU {
    regs: Registers,
    stack: Stack,
    e: bool, // Emulation mode
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            P: StatusRegister::default(),
            C: Accumulator::default(),
            X: IndexRegister::default(),
            Y: IndexRegister::default(),
            D: 0,
            S: byte_struct::ByteStructUnspecifiedByteOrder::read_bytes_default_le(&[0xff, 0xf2]),
            PBR: 0,
            DBR: 0,
            PC: 0,
        }
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::default(),
            stack: Stack::default(),
            e: true,
        }
    }

    pub fn get_regs(&mut self) -> &mut Registers {
        return &mut self.regs;
    }

    pub fn get_emulation_mode(&self) -> bool {
        return self.e;
    }
    pub fn set_regs(&mut self, regs: &Registers) {
        self.regs = *regs;
    }

    pub fn set_emulation_mode(&mut self, e: bool) {
        self.e = e;
    }

    pub fn stack_push(&mut self, payload: u8) {
        let index = <u16>::from(self.regs.S);
        println!("Pushing {:x} on stack address {:x}", payload, index);

        let mut new_index: i32 = index as i32 - 1;

        if new_index == -1 {
            new_index = 0xffff
        }
        self.stack.content[(new_index) as usize] = payload;
        self.regs.S = IndexRegister::from(new_index as u16);
    }

    pub fn stack_pull(&mut self) -> u8 {
        let index = <u16>::from(self.regs.S);
        let ret = self.stack.content[index as usize];

        let mut new_index: i32 = index as i32 + 1;
        if new_index == 0x10000 {
            new_index = 0;
        }
        self.regs.S = IndexRegister::from(new_index as u16);
        println!("Popping {:x} from Stack", ret);
        ret
    }
}
