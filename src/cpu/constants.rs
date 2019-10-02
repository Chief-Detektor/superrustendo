// mod constants;

pub const GI_MASK: u8 = 0b0001_1111;

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

pub const G1_OP_ADC: u8 = 0b0110_0000; // Add with Carry to Acc
pub const G1_OP_AND: u8 = 0b0010_0000; // And the Accumulator
pub const G1_OP_CMP: u8 = 0b1100_0000; // Compare the Accumulator
pub const G1_OP_EOR: u8 = 0b0100_0000; // Exclusive Or the Accumulator
pub const G1_OP_LDA: u8 = 0b1010_0000; // Load the Accumulator (LDA)
pub const G1_OP_ORA: u8 = 0b0000_0000; // Or the Accumulator
pub const G1_OP_SBC: u8 = 0b1110_0000; // Substract with Borrow from the Accumulator
pub const G1_OP_STA: u8 = 0b1000_0000; // Store the Acccumulator