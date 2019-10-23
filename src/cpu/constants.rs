// mod constants;

pub const GI_MASK: u8 = 0b0001_1111;
pub const GII_MASK: u8 = 0b0001_1100;
pub const GII_MASK2: u8 = 0b0001_1000;
// pub const GII_MASK_4_ADDR_MODES: u8 = 0b0001_1000;

// 6502 Address Modes
pub const GI_ADDR_MODE_INTERMEDIATE: u8 = 0b0_1001;
pub const GI_ADDR_MODE_DIRECT_ZERO_PAGE: u8 = 0b0_0101;
pub const GI_ADDR_MODE_ABSOLUTE: u8 = 0b0_1101;
pub const GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X: u8 = 0b1_0101;
pub const GI_ADDR_MODE_ABSOLUTE_INDEXED_X: u8 = 0b1_1101;
pub const GI_ADDR_MODE_ABSOLUTE_INDEXED_Y: u8 = 0b1_1001;
pub const GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_X: u8 = 0b0_0001;
pub const GI_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_INDIRECT_Y: u8 = 0b1_0001;

// 65816 Address Modes
pub const GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG_INDEXED_Y: u8 = 0b1_0111;
pub const GI_ADDR_MODE_DIRECT_PAGE_INDIRECT_LONG: u8 = 0b0_0111;
pub const GI_ADDR_MODE_ABSOLUTE_LONG: u8 = 0b0_1111;
pub const GI_ADDR_MODE_ABSOLUTE_LONG_INDEXED_X: u8 = 0b1_1111;
pub const GI_ADDR_MODE_STACK_RELATIVE: u8 = 0b0_0011;
pub const GI_ADDR_MODE_STACK_RELATIVE_INDIRECT_INDEXED_Y: u8 = 0b1_0011;
pub const GI_ADDR_MODE_DIRECT_PAGE_INDIRECT: u8 = 0b1_0010;

// Group I Instructions
pub const G1_OP_ADC: u8 = 0b0110_0000; // Add with Carry to Acc
pub const G1_OP_AND: u8 = 0b0010_0000; // And the Accumulator
pub const G1_OP_CMP: u8 = 0b1100_0000; // Compare the Accumulator
pub const G1_OP_EOR: u8 = 0b0100_0000; // Exclusive Or the Accumulator
pub const G1_OP_LDA: u8 = 0b1010_0000; // Load the Accumulator (LDA)
pub const G1_OP_ORA: u8 = 0b0000_0000; // Or the Accumulator
pub const G1_OP_SBC: u8 = 0b1110_0000; // Substract with Borrow from the Accumulator
pub const G1_OP_STA: u8 = 0b1000_0000; // Store the Acccumulator

// Group II Instructions
pub const G2_OP_ASL: u8 = 0b0000_0010; // Arithmetic shift left
pub const G2_OP_DEC: u8 = 0b1100_0110; // Decrement
pub const G2_OP_INC: u8 = 0b1110_0110; // Increment
pub const G2_OP_LSR: u8 = 0b0100_0010; // Logical shift right
pub const G2_OP_ROL: u8 = 0b0010_0010; // Rotate right through carry
pub const G2_OP_ROR: u8 = 0b0110_0010; // Rotate right through carry
pub const G2_OP_STX: u8 = 0b1000_0110; // Store Index Register X
pub const G2_OP_STY: u8 = 0b1000_0100; // Store Index Register Y

// Group II Address Modes
pub const G2_ADDR_MODE_ACCUMULATOR: u8 = 0b000_1000;
pub const G2_ADDR_MODE_DIRECT_ZERO_PAGE: u8 = 0b0000_0100;
pub const G2_ADDR_MODE_ABSOLUTE: u8 = 0b0000_1100;
pub const G2_ADDR_MODE_DIRECT_ZERO_PAGE_INDEXED_X: u8 = 0b0001_0100;
pub const G2_ADDR_MODE_ABSOLUTE_INDEXED_X: u8 = 0b0001_1100;

// (Group II) Index register loading
pub const G2_OP_LDX: u8 = 0b1010_0010;
pub const G2_OP_LDY: u8 = 0b1010_0000;

pub const G2_REGLOAD_ADDR_MODE_IMMEDIATE: u8 = 0b000_0000;
pub const G2_REGLOAD_ADDR_MODE_DIRECT_PAGE: u8 = 0b0000_0100;
pub const G2_REGLOAD_ADDR_MODE_ABSOLUTE: u8 = 0b0000_1100;
pub const G2_REGLOAD_ADDR_MODE_DIRECT_PAGE_INDEXED: u8 = 0b0001_0100;
pub const G2_REGLOAD_ADDR_MODE_ABSOLUTE_INDEXED: u8 = 0b0001_1100;

// Group III Instructions
pub const G3_OP_BRK: u8 = 0x0;
pub const G3_OP_COP: u8 = 0x2;
pub const G3_OP_TSB: u8 = 0x4 | 0xc;
pub const G3_OP_PHP: u8 = 0x8;
pub const G3_OP_PHD: u8 = 0xb;
pub const G3_OP_BLP: u8 = 0x10;
pub const G3_OP_TRB: u8 = 0x14 | 0x1c; // TODO: Or with other modea
pub const G3_OP_CLC: u8 = 0x18;
pub const G3_OP_TCS: u8 = 0x1b;
pub const G3_OP_JSR: u8 = 0x20 | 0x22 | 0xfc;
pub const G3_OP_BIT: u8 = 0x24 | 0x2c | 0x34 | 0x3c | 0x89;
pub const G3_OP_PLP: u8 = 0x28;
pub const G3_OP_PLD: u8 = 0x2b;
pub const G3_OP_BMI: u8 = 0x30;
pub const G3_OP_SEC: u8 = 0x38;
pub const G3_OP_TSC: u8 = 0x3b;
pub const G3_OP_RTI: u8 = 0x40;
pub const G3_OP_WDM: u8 = 0x42;
pub const G3_OP_MVP: u8 = 0x44;
pub const G3_OP_PHA: u8 = 0x48;
pub const G3_OP_PHK: u8 = 0x4b;
pub const G3_OP_JMP: u8 = 0x4c | 0x5c | 0x6c | 0x7c | 0xdc;
pub const G3_OP_BVC: u8 = 0x50;
pub const G3_OP_MVN: u8 = 0x54;
pub const G3_OP_CLI: u8 = 0x58;
pub const G3_OP_PHY: u8 = 0x5a;
pub const G3_OP_TCD: u8 = 0x5b;
pub const G3_OP_RTS: u8 = 0x60;
pub const G3_OP_PER: u8 = 0x62;
pub const G3_OP_STZ: u8 = 0x64 | 0x74 | 0x9e | 0x9c;
pub const G3_OP_PLA: u8 = 0x68;
pub const G3_OP_RTL: u8 = 0x6b;
pub const G3_OP_BVS: u8 = 0x70;
pub const G3_OP_SEI: u8 = 0x78;
pub const G3_OP_PLY: u8 = 0x7a;
pub const G3_OP_TDC: u8 = 0x7b;
pub const G3_OP_BRA: u8 = 0x80;
pub const G3_OP_BRL: u8 = 0x82;
pub const G3_OP_DEY: u8 = 0x88;
pub const G3_OP_TXA: u8 = 0x8a;
pub const G3_OP_PHB: u8 = 0x8b;
pub const G3_OP_BCC: u8 = 0x90;
pub const G3_OP_TYA: u8 = 0x98;
pub const G3_OP_TXS: u8 = 0x9a;
pub const G3_OP_TXY: u8 = 0x9b;
pub const G3_OP_TAY: u8 = 0xa8;
pub const G3_OP_TAX: u8 = 0xaa;
pub const G3_OP_PLB: u8 = 0xab;
pub const G3_OP_BCS: u8 = 0xb0;
pub const G3_OP_CLV: u8 = 0xb8;
pub const G3_OP_TSX: u8 = 0xba;
pub const G3_OP_TYX: u8 = 0xbb;
pub const G3_OP_CPY: u8 = 0xc0 | 0xc4 | 0xcc;
pub const G3_OP_REP: u8 = 0xc2;
pub const G3_OP_INY: u8 = 0xc8;
pub const G3_OP_DEX: u8 = 0xca;
pub const G3_OP_WAI: u8 = 0xcb;
pub const G3_OP_BNE: u8 = 0xd0;
pub const G3_OP_PEI: u8 = 0xd4;
pub const G3_OP_CLD: u8 = 0xd8;
pub const G3_OP_PHX: u8 = 0xda;
pub const G3_OP_STP: u8 = 0xdb;
pub const G3_OP_CPX: u8 = 0xe0 | 0xe4 | 0xec;
pub const G3_OP_INX: u8 = 0xe8;
pub const G3_OP_NOP: u8 = 0xea;
pub const G3_OP_XBA: u8 = 0xeb;
pub const G3_OP_BEQ: u8 = 0xf0;
pub const G3_OP_PEA: u8 = 0xf4;
pub const G3_OP_SED: u8 = 0xf8;
pub const G3_OP_PLX: u8 = 0xfa;
pub const G3_OP_XCE: u8 = 0xfb;
