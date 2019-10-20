// mod constants;

pub const GI_MASK: u8 = 0b0001_1111;
pub const GII_MASK: u8 = 0b0001_1100;
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

// Group III Instructions
pub const G3_OP_BRK: u8 = 0x0;
pub const G3_OP_COP: u8 = 0x2;
pub const G3_OP_TSB: u8 = 0x4;
// pub const G3_OP_ASL_ACCU: u8 = 0x6;
pub const G3_OP_TSB_DIRECT: u8 = 0xc;

// pub const G3_ADDR_MODE_STACK_INTERRUPT =
