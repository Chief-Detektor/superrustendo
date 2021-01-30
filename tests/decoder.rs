#[allow(unused_macros)]
macro_rules! opcode_test {
    ($raw_byte:expr, $opcode:expr, $addressmode:expr) => {{
        let mut c = CPU::new();
        let mut m = Bus {
            cartridge: None,
            wram: WRAM::new(),
        };
        let d = Decoder::new(&mut c, &mut m, false);
        let result = d.decode($raw_byte);
        let res = result.unwrap();
        let addr = res.1;
        let op = res.0;
        assert_eq!(op, $opcode);
        assert_eq!(addr, $addressmode);
    }};
}

#[cfg(test)]
mod tests {

    // extern crate snes_sim;
    extern crate superrustendo;
    use superrustendo::cpu::addressmodes::AddressModes;
    use superrustendo::cpu::decoder::Decoder;
    use superrustendo::cpu::decoder::Opcodes;
    use superrustendo::cpu::CPU;
    use superrustendo::mem::Bus;
    use superrustendo::mem::WRAM;
    #[test]
    fn brk() {
        opcode_test!(0x0, Opcodes::BRK, AddressModes::StackInterrupt);
    }
    // ADC
    #[test]
    fn adc_1() {
        opcode_test!(0x69, Opcodes::ADC, AddressModes::Immediate);
    }
    #[test]
    fn adc_2() {
        opcode_test!(0x6d, Opcodes::ADC, AddressModes::Absolute);
    }
    #[test]
    fn adc_3() {
        opcode_test!(0x6f, Opcodes::ADC, AddressModes::AbsoluteLong);
    }
    #[test]
    fn adc_4() {
        opcode_test!(0x65, Opcodes::ADC, AddressModes::DirectPage);
    }
    #[test]
    fn adc_5() {
        opcode_test!(0x72, Opcodes::ADC, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn adc_6() {
        opcode_test!(0x67, Opcodes::ADC, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn adc_7() {
        opcode_test!(0x7d, Opcodes::ADC, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn adc_8() {
        opcode_test!(0x7f, Opcodes::ADC, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn adc_9() {
        opcode_test!(0x79, Opcodes::ADC, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn adc_10() {
        opcode_test!(0x75, Opcodes::ADC, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn adc_11() {
        opcode_test!(0x61, Opcodes::ADC, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn adc_12() {
        opcode_test!(0x71, Opcodes::ADC, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn adc_13() {
        opcode_test!(
            0x77,
            Opcodes::ADC,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn adc_14() {
        opcode_test!(0x63, Opcodes::ADC, AddressModes::StackRelative);
    }
    #[test]
    fn adc_15() {
        opcode_test!(
            0x73,
            Opcodes::ADC,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // AND
    #[test]
    fn and_1() {
        opcode_test!(0x29, Opcodes::AND, AddressModes::Immediate);
    }
    #[test]
    fn and_2() {
        opcode_test!(0x2d, Opcodes::AND, AddressModes::Absolute);
    }
    #[test]
    fn and_3() {
        opcode_test!(0x2f, Opcodes::AND, AddressModes::AbsoluteLong);
    }
    #[test]
    fn and_4() {
        opcode_test!(0x25, Opcodes::AND, AddressModes::DirectPage);
    }
    #[test]
    fn and_5() {
        opcode_test!(0x32, Opcodes::AND, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn and_6() {
        opcode_test!(0x27, Opcodes::AND, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn and_7() {
        opcode_test!(0x3d, Opcodes::AND, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn and_8() {
        opcode_test!(0x3f, Opcodes::AND, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn and_9() {
        opcode_test!(0x39, Opcodes::AND, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn and_10() {
        opcode_test!(0x35, Opcodes::AND, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn and_11() {
        opcode_test!(0x21, Opcodes::AND, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn and_12() {
        opcode_test!(0x31, Opcodes::AND, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn and_13() {
        opcode_test!(
            0x37,
            Opcodes::AND,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn and_14() {
        opcode_test!(0x23, Opcodes::AND, AddressModes::StackRelative);
    }
    #[test]
    fn and_15() {
        opcode_test!(
            0x33,
            Opcodes::AND,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // CMP
    #[test]
    fn cmp_1() {
        opcode_test!(0xC9, Opcodes::CMP, AddressModes::Immediate);
    }
    #[test]
    fn cmp_2() {
        opcode_test!(0xcd, Opcodes::CMP, AddressModes::Absolute);
    }
    #[test]
    fn cmp_3() {
        opcode_test!(0xcf, Opcodes::CMP, AddressModes::AbsoluteLong);
    }
    #[test]
    fn cmp_4() {
        opcode_test!(0xc5, Opcodes::CMP, AddressModes::DirectPage);
    }
    #[test]
    fn cmp_5() {
        opcode_test!(0xd2, Opcodes::CMP, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn cmp_6() {
        opcode_test!(0xc7, Opcodes::CMP, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn cmp_7() {
        opcode_test!(0xdd, Opcodes::CMP, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn cmp_8() {
        opcode_test!(0xdf, Opcodes::CMP, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn cmp_9() {
        opcode_test!(0xd9, Opcodes::CMP, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn cmp_10() {
        opcode_test!(0xd5, Opcodes::CMP, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn cmp_11() {
        opcode_test!(0xc1, Opcodes::CMP, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn cmp_12() {
        opcode_test!(0xd1, Opcodes::CMP, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn cmp_13() {
        opcode_test!(
            0xd7,
            Opcodes::CMP,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn cmp_14() {
        opcode_test!(0xc3, Opcodes::CMP, AddressModes::StackRelative);
    }
    #[test]
    fn cmp_15() {
        opcode_test!(
            0xd3,
            Opcodes::CMP,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // EOR
    #[test]
    fn eor_1() {
        opcode_test!(0x49, Opcodes::EOR, AddressModes::Immediate);
    }
    #[test]
    fn eor_2() {
        opcode_test!(0x4d, Opcodes::EOR, AddressModes::Absolute);
    }
    #[test]
    fn eor_3() {
        opcode_test!(0x4f, Opcodes::EOR, AddressModes::AbsoluteLong);
    }
    #[test]
    fn eor_4() {
        opcode_test!(0x45, Opcodes::EOR, AddressModes::DirectPage);
    }
    #[test]
    fn eor_5() {
        opcode_test!(0x52, Opcodes::EOR, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn eor_6() {
        opcode_test!(0x47, Opcodes::EOR, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn eor_7() {
        opcode_test!(0x5d, Opcodes::EOR, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn eor_8() {
        opcode_test!(0x5f, Opcodes::EOR, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn eor_9() {
        opcode_test!(0x59, Opcodes::EOR, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn eor_10() {
        opcode_test!(0x55, Opcodes::EOR, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn eor_11() {
        opcode_test!(0x41, Opcodes::EOR, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn eor_12() {
        opcode_test!(0x51, Opcodes::EOR, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn eor_13() {
        opcode_test!(
            0x57,
            Opcodes::EOR,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn eor_14() {
        opcode_test!(0x43, Opcodes::EOR, AddressModes::StackRelative);
    }
    #[test]
    fn eor_15() {
        opcode_test!(
            0x53,
            Opcodes::EOR,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // LDA
    #[test]
    fn lda_1() {
        opcode_test!(0xa9, Opcodes::LDA, AddressModes::Immediate);
    }
    #[test]
    fn lda_2() {
        opcode_test!(0xad, Opcodes::LDA, AddressModes::Absolute);
    }
    #[test]
    fn lda_3() {
        opcode_test!(0xaf, Opcodes::LDA, AddressModes::AbsoluteLong);
    }
    #[test]
    fn lda_4() {
        opcode_test!(0xa5, Opcodes::LDA, AddressModes::DirectPage);
    }
    #[test]
    fn lda_5() {
        opcode_test!(0xb2, Opcodes::LDA, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn lda_6() {
        opcode_test!(0xa7, Opcodes::LDA, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn lda_7() {
        opcode_test!(0xbd, Opcodes::LDA, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn lda_8() {
        opcode_test!(0xbf, Opcodes::LDA, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn lda_9() {
        opcode_test!(0xb9, Opcodes::LDA, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn lda_10() {
        opcode_test!(0xb5, Opcodes::LDA, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn lda_11() {
        opcode_test!(0xa1, Opcodes::LDA, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn lda_12() {
        opcode_test!(0xb1, Opcodes::LDA, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn lda_13() {
        opcode_test!(
            0xb7,
            Opcodes::LDA,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn lda_14() {
        opcode_test!(0xa3, Opcodes::LDA, AddressModes::StackRelative);
    }
    #[test]
    fn lda_15() {
        opcode_test!(
            0xb3,
            Opcodes::LDA,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // ORA
    #[test]
    fn ora_1() {
        opcode_test!(0x9, Opcodes::ORA, AddressModes::Immediate);
    }
    #[test]
    fn ora_2() {
        opcode_test!(0xd, Opcodes::ORA, AddressModes::Absolute);
    }
    #[test]
    fn ora_3() {
        opcode_test!(0xf, Opcodes::ORA, AddressModes::AbsoluteLong);
    }
    #[test]
    fn ora_4() {
        opcode_test!(0x5, Opcodes::ORA, AddressModes::DirectPage);
    }
    #[test]
    fn ora_5() {
        opcode_test!(0x12, Opcodes::ORA, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn ora_6() {
        opcode_test!(0x7, Opcodes::ORA, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn ora_7() {
        opcode_test!(0x1d, Opcodes::ORA, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn ora_8() {
        opcode_test!(0x1f, Opcodes::ORA, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn ora_9() {
        opcode_test!(0x19, Opcodes::ORA, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn ora_10() {
        opcode_test!(0x15, Opcodes::ORA, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn ora_11() {
        opcode_test!(0x1, Opcodes::ORA, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn ora_12() {
        opcode_test!(0x11, Opcodes::ORA, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn ora_13() {
        opcode_test!(
            0x17,
            Opcodes::ORA,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn ora_14() {
        opcode_test!(0x03, Opcodes::ORA, AddressModes::StackRelative);
    }
    #[test]
    fn ora_15() {
        opcode_test!(
            0x13,
            Opcodes::ORA,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // SBC
    #[test]
    fn sbc_1() {
        opcode_test!(0xe9, Opcodes::SBC, AddressModes::Immediate);
    }
    #[test]
    fn sbc_2() {
        opcode_test!(0xed, Opcodes::SBC, AddressModes::Absolute);
    }
    #[test]
    fn sbc_3() {
        opcode_test!(0xef, Opcodes::SBC, AddressModes::AbsoluteLong);
    }
    #[test]
    fn sbc_4() {
        opcode_test!(0xe5, Opcodes::SBC, AddressModes::DirectPage);
    }
    #[test]
    fn sbc_5() {
        opcode_test!(0xf2, Opcodes::SBC, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn sbc_6() {
        opcode_test!(0xe7, Opcodes::SBC, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn sbc_7() {
        opcode_test!(0xfd, Opcodes::SBC, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn sbc_8() {
        opcode_test!(0xff, Opcodes::SBC, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn sbc_9() {
        opcode_test!(0xf9, Opcodes::SBC, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn sbc_10() {
        opcode_test!(0xf5, Opcodes::SBC, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn sbc_11() {
        opcode_test!(0xe1, Opcodes::SBC, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn sbc_12() {
        opcode_test!(0xf1, Opcodes::SBC, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn sbc_13() {
        opcode_test!(
            0xf7,
            Opcodes::SBC,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn sbc_14() {
        opcode_test!(0xe3, Opcodes::SBC, AddressModes::StackRelative);
    }
    #[test]
    fn sbc_15() {
        opcode_test!(
            0xf3,
            Opcodes::SBC,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // STA
    // #[test]
    // fn sta_1() {
    //   // opcode_test!(0x8d, Opcodes::STA, AddressModes::Immediate);
    //   // No immediate mode for STA
    //   assert!(true);
    // }
    #[test]
    fn sta_2() {
        opcode_test!(0x8d, Opcodes::STA, AddressModes::Absolute);
    }
    #[test]
    fn sta_3() {
        opcode_test!(0x8f, Opcodes::STA, AddressModes::AbsoluteLong);
    }
    #[test]
    fn sta_4() {
        opcode_test!(0x85, Opcodes::STA, AddressModes::DirectPage);
    }
    #[test]
    fn sta_5() {
        opcode_test!(0x92, Opcodes::STA, AddressModes::DirectPageIndirect);
    }
    #[test]
    fn sta_6() {
        opcode_test!(0x87, Opcodes::STA, AddressModes::DirectPageIndirectLong);
    }
    #[test]
    fn sta_7() {
        opcode_test!(0x9d, Opcodes::STA, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn sta_8() {
        opcode_test!(0x9f, Opcodes::STA, AddressModes::AbsoluteLongIndexedX);
    }
    #[test]
    fn sta_9() {
        opcode_test!(0x99, Opcodes::STA, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn sta_10() {
        opcode_test!(0x95, Opcodes::STA, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn sta_11() {
        opcode_test!(0x81, Opcodes::STA, AddressModes::DirectPageIndexedIndirectX);
    }
    #[test]
    fn sta_12() {
        opcode_test!(0x91, Opcodes::STA, AddressModes::DirectPageIndirectIndexedY);
    }
    #[test]
    fn sta_13() {
        opcode_test!(
            0x97,
            Opcodes::STA,
            AddressModes::DirectPageIndirectLongIndexedY
        );
    }
    #[test]
    fn sta_14() {
        opcode_test!(0x83, Opcodes::STA, AddressModes::StackRelative);
    }
    #[test]
    fn sta_15() {
        opcode_test!(
            0x93,
            Opcodes::STA,
            AddressModes::StackRelativeIndirectIndexedY
        );
    }
    // ASL
    #[test]
    fn asl_1() {
        opcode_test!(0xa, Opcodes::ASL, AddressModes::Accumulator);
    }
    #[test]
    fn asl_2() {
        opcode_test!(0xe, Opcodes::ASL, AddressModes::Absolute);
    }
    #[test]
    fn asl_3() {
        opcode_test!(0x6, Opcodes::ASL, AddressModes::DirectPage);
    }
    #[test]
    fn asl_4() {
        opcode_test!(0x1e, Opcodes::ASL, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn asl_5() {
        opcode_test!(0x16, Opcodes::ASL, AddressModes::DirectPageIndexedX);
    }
    // DEC
    #[test]
    fn dec_1() {
        opcode_test!(0x3a, Opcodes::DEC, AddressModes::Accumulator);
    }
    #[test]
    fn dec_2() {
        opcode_test!(0xce, Opcodes::DEC, AddressModes::Absolute);
    }
    #[test]
    fn dec_3() {
        opcode_test!(0xc6, Opcodes::DEC, AddressModes::DirectPage);
    }
    #[test]
    fn dec_4() {
        opcode_test!(0xde, Opcodes::DEC, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn dec_5() {
        opcode_test!(0xd6, Opcodes::DEC, AddressModes::DirectPageIndexedX);
    }
    // INC
    #[test]
    fn inc_1() {
        opcode_test!(0x1a, Opcodes::INC, AddressModes::Accumulator);
    }
    #[test]
    fn inc_2() {
        opcode_test!(0xee, Opcodes::INC, AddressModes::Absolute);
    }
    #[test]
    fn inc_3() {
        opcode_test!(0xe6, Opcodes::INC, AddressModes::DirectPage);
    }
    #[test]
    fn inc_4() {
        opcode_test!(0xfe, Opcodes::INC, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn inc_5() {
        opcode_test!(0xf6, Opcodes::INC, AddressModes::DirectPageIndexedX);
    }
    // LSR
    #[test]
    fn lsr_1() {
        opcode_test!(0x4a, Opcodes::LSR, AddressModes::Accumulator);
    }
    #[test]
    fn lsr_2() {
        opcode_test!(0x4e, Opcodes::LSR, AddressModes::Absolute);
    }
    #[test]
    fn lsr_3() {
        opcode_test!(0x46, Opcodes::LSR, AddressModes::DirectPage);
    }
    #[test]
    fn lsr_4() {
        opcode_test!(0x5e, Opcodes::LSR, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn lsr_5() {
        opcode_test!(0x56, Opcodes::LSR, AddressModes::DirectPageIndexedX);
    }
    // ROL
    #[test]
    fn rol_1() {
        opcode_test!(0x2a, Opcodes::ROL, AddressModes::Accumulator);
    }
    #[test]
    fn rol_2() {
        opcode_test!(0x2e, Opcodes::ROL, AddressModes::Absolute);
    }
    #[test]
    fn rol_3() {
        opcode_test!(0x26, Opcodes::ROL, AddressModes::DirectPage);
    }
    #[test]
    fn rol_4() {
        opcode_test!(0x3e, Opcodes::ROL, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn rol_5() {
        opcode_test!(0x36, Opcodes::ROL, AddressModes::DirectPageIndexedX);
    }
    // ROR
    #[test]
    fn ror_1() {
        opcode_test!(0x6a, Opcodes::ROR, AddressModes::Accumulator);
    }
    #[test]
    fn ror_2() {
        opcode_test!(0x6e, Opcodes::ROR, AddressModes::Absolute);
    }
    #[test]
    fn ror_3() {
        opcode_test!(0x66, Opcodes::ROR, AddressModes::DirectPage);
    }
    #[test]
    fn ror_4() {
        opcode_test!(0x7e, Opcodes::ROR, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn ror_5() {
        opcode_test!(0x76, Opcodes::ROR, AddressModes::DirectPageIndexedX);
    }
    // STX
    #[test]
    fn stx_1() {
        opcode_test!(0x8e, Opcodes::STX, AddressModes::Absolute);
    }
    #[test]
    fn stx_2() {
        opcode_test!(0x86, Opcodes::STX, AddressModes::DirectPage);
    }
    #[test]
    fn stx_3() {
        opcode_test!(0x96, Opcodes::STX, AddressModes::DirectPageIndexedX);
    }
    // STY
    #[test]
    fn sty_1() {
        opcode_test!(0x8c, Opcodes::STY, AddressModes::Absolute);
    }
    #[test]
    fn sty_2() {
        opcode_test!(0x84, Opcodes::STY, AddressModes::DirectPage);
    }
    #[test]
    fn sty_3() {
        opcode_test!(0x94, Opcodes::STY, AddressModes::DirectPageIndexedX);
    }
    // CPX
    #[test]
    fn cpx_1() {
        opcode_test!(0xe0, Opcodes::CPX, AddressModes::Immediate);
    }
    #[test]
    fn cpx_2() {
        opcode_test!(0xec, Opcodes::CPX, AddressModes::Absolute);
    }
    #[test]
    fn cpx_3() {
        opcode_test!(0xe4, Opcodes::CPX, AddressModes::DirectPage);
    }
    // CPY
    #[test]
    fn cpy_1() {
        opcode_test!(0xc0, Opcodes::CPY, AddressModes::Immediate);
    }
    #[test]
    fn cpy_2() {
        opcode_test!(0xcc, Opcodes::CPY, AddressModes::Absolute);
    }
    #[test]
    fn cpy_3() {
        opcode_test!(0xc4, Opcodes::CPY, AddressModes::DirectPage);
    }
    // TRB
    #[test]
    fn trb_1() {
        opcode_test!(0x1c, Opcodes::TRB, AddressModes::Absolute);
    }
    #[test]
    fn trb_2() {
        opcode_test!(0x14, Opcodes::TRB, AddressModes::DirectPage);
    }
    // TSB
    #[test]
    fn tsb_1() {
        opcode_test!(0xc, Opcodes::TSB, AddressModes::Absolute);
    }
    #[test]
    fn tsb_2() {
        opcode_test!(0x4, Opcodes::TSB, AddressModes::DirectPage);
    }
    #[test]
    fn bcc() {
        opcode_test!(0x90, Opcodes::BCC, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn bcs() {
        opcode_test!(0xb0, Opcodes::BCS, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn beq() {
        opcode_test!(0xf0, Opcodes::BEQ, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn bit_1() {
        opcode_test!(0x89, Opcodes::BIT, AddressModes::Immediate);
    }
    #[test]
    fn bit_2() {
        opcode_test!(0x2c, Opcodes::BIT, AddressModes::Absolute);
    }
    #[test]
    fn bit_3() {
        opcode_test!(0x24, Opcodes::BIT, AddressModes::DirectPage);
    }
    #[test]
    fn bit_4() {
        opcode_test!(0x3c, Opcodes::BIT, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn bit_5() {
        opcode_test!(0x34, Opcodes::BIT, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn bmi() {
        opcode_test!(0x30, Opcodes::BMI, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn bne() {
        opcode_test!(0xd0, Opcodes::BNE, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn bpl() {
        opcode_test!(0x10, Opcodes::BPL, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn bra() {
        opcode_test!(0x80, Opcodes::BRA, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn brl() {
        opcode_test!(
            0x82,
            Opcodes::BRL,
            AddressModes::ProgrammCounterRelativeLong
        );
    }
    #[test]
    fn bvc() {
        opcode_test!(0x50, Opcodes::BVC, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn bvs() {
        opcode_test!(0x70, Opcodes::BVS, AddressModes::ProgrammCounterRelative);
    }
    #[test]
    fn clc() {
        opcode_test!(0x18, Opcodes::CLC, AddressModes::Implied);
    }
    #[test]
    fn cld() {
        opcode_test!(0xd8, Opcodes::CLD, AddressModes::Implied);
    }
    #[test]
    fn cli() {
        opcode_test!(0x58, Opcodes::CLI, AddressModes::Implied);
    }
    #[test]
    fn clv() {
        opcode_test!(0xb8, Opcodes::CLV, AddressModes::Implied);
    }
    #[test]
    fn cop() {
        opcode_test!(0x2, Opcodes::COP, AddressModes::StackInterrupt);
    }
    #[test]
    fn dex() {
        opcode_test!(0xca, Opcodes::DEX, AddressModes::Implied);
    }
    #[test]
    fn dey() {
        opcode_test!(0x88, Opcodes::DEY, AddressModes::Implied);
    }
    #[test]
    fn inx() {
        opcode_test!(0xe8, Opcodes::INX, AddressModes::Implied);
    }
    #[test]
    fn iny() {
        opcode_test!(0xc8, Opcodes::INY, AddressModes::Implied);
    }

    #[test]
    fn jmp_1() {
        opcode_test!(0x4c, Opcodes::JMP, AddressModes::Absolute);
    }
    #[test]
    fn jmp_2() {
        opcode_test!(0x6c, Opcodes::JMP, AddressModes::AbsoluteIndirect);
    }
    #[test]
    fn jmp_3() {
        opcode_test!(0x7c, Opcodes::JMP, AddressModes::AbsoluteIndexedIndirect);
    }
    #[test]
    fn jmp_4() {
        opcode_test!(0x5c, Opcodes::JMP, AddressModes::AbsoluteLong);
    }
    #[test]
    fn jmp_5() {
        opcode_test!(0xdc, Opcodes::JMP, AddressModes::AbsoluteIndirectLong);
    }
    #[test]
    fn jsr_1() {
        opcode_test!(0x20, Opcodes::JSR, AddressModes::Absolute);
    }
    #[test]
    fn jsr_2() {
        opcode_test!(0xFC, Opcodes::JSR, AddressModes::AbsoluteIndexedIndirect);
    }
    #[test]
    fn jsr_3() {
        opcode_test!(0x22, Opcodes::JSR, AddressModes::AbsoluteLong);
    }
    #[test]
    fn ldx_1() {
        opcode_test!(0xa2, Opcodes::LDX, AddressModes::Immediate);
    }
    #[test]
    fn ldx_2() {
        opcode_test!(0xae, Opcodes::LDX, AddressModes::Absolute);
    }
    #[test]
    fn ldx_3() {
        opcode_test!(0xa6, Opcodes::LDX, AddressModes::DirectPage);
    }
    #[test]
    fn ldx_4() {
        opcode_test!(0xbe, Opcodes::LDX, AddressModes::AbsoluteIndexedY);
    }
    #[test]
    fn ldx_5() {
        opcode_test!(0xb6, Opcodes::LDX, AddressModes::DirectPageIndexedY);
    }
    #[test]
    fn ldy_1() {
        opcode_test!(0xa0, Opcodes::LDY, AddressModes::Immediate);
    }
    #[test]
    fn ldy_2() {
        opcode_test!(0xac, Opcodes::LDY, AddressModes::Absolute);
    }
    #[test]
    fn ldy_3() {
        opcode_test!(0xa4, Opcodes::LDY, AddressModes::DirectPage);
    }
    #[test]
    fn ldy_4() {
        opcode_test!(0xbc, Opcodes::LDY, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn ldy_5() {
        opcode_test!(0xb4, Opcodes::LDY, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn mvn() {
        opcode_test!(0x54, Opcodes::MVN, AddressModes::BlockMove);
    }
    #[test]
    fn mvp() {
        opcode_test!(0x44, Opcodes::MVP, AddressModes::BlockMove);
    }
    #[test]
    fn nop() {
        opcode_test!(0xea, Opcodes::NOP, AddressModes::Implied);
    }
    #[test]
    fn pea() {
        opcode_test!(0xf4, Opcodes::PEA, AddressModes::StackAbsolute);
    }
    #[test]
    fn pei() {
        opcode_test!(0xd4, Opcodes::PEI, AddressModes::StackDirectPageIndirect);
    }
    #[test]
    fn per() {
        opcode_test!(0x62, Opcodes::PER, AddressModes::StackPCRelativeLong);
    }
    #[test]
    fn pha() {
        opcode_test!(0x48, Opcodes::PHA, AddressModes::StackPush);
    }
    #[test]
    fn phb() {
        opcode_test!(0x8b, Opcodes::PHB, AddressModes::StackPush);
    }
    #[test]
    fn phd() {
        opcode_test!(0x0b, Opcodes::PHD, AddressModes::StackPush);
    }
    #[test]
    fn phk() {
        opcode_test!(0x4b, Opcodes::PHK, AddressModes::StackPush);
    }
    #[test]
    fn php() {
        opcode_test!(0x08, Opcodes::PHP, AddressModes::StackPush);
    }
    #[test]
    fn phx() {
        opcode_test!(0xda, Opcodes::PHX, AddressModes::StackPush);
    }
    #[test]
    fn phy() {
        opcode_test!(0x5a, Opcodes::PHY, AddressModes::StackPush);
    }
    #[test]
    fn pla() {
        opcode_test!(0x68, Opcodes::PLA, AddressModes::StackPull);
    }
    #[test]
    fn plb() {
        opcode_test!(0xab, Opcodes::PLB, AddressModes::StackPull);
    }
    #[test]
    fn pld() {
        opcode_test!(0x2b, Opcodes::PLD, AddressModes::StackPull);
    }
    #[test]
    fn plp() {
        opcode_test!(0x28, Opcodes::PLP, AddressModes::StackPull);
    }
    #[test]
    fn plx() {
        opcode_test!(0xfa, Opcodes::PLX, AddressModes::StackPull);
    }
    #[test]
    fn ply() {
        opcode_test!(0x7a, Opcodes::PLY, AddressModes::StackPull);
    }
    #[test]
    fn rep() {
        opcode_test!(0xc2, Opcodes::REP, AddressModes::Immediate);
    }
    #[test]
    fn rti() {
        opcode_test!(0x40, Opcodes::RTI, AddressModes::StackRTI);
    }
    #[test]
    fn rtl() {
        opcode_test!(0x6b, Opcodes::RTL, AddressModes::StackRTL);
    }
    #[test]
    fn rts() {
        opcode_test!(0x60, Opcodes::RTS, AddressModes::StackRTS);
    }
    #[test]
    fn sec() {
        opcode_test!(0x38, Opcodes::SEC, AddressModes::Implied);
    }
    #[test]
    fn sed() {
        opcode_test!(0xf8, Opcodes::SED, AddressModes::Implied);
    }
    #[test]
    fn sei() {
        opcode_test!(0x78, Opcodes::SEI, AddressModes::Implied);
    }
    #[test]
    fn sep() {
        opcode_test!(0xe2, Opcodes::SEP, AddressModes::Immediate);
    }
    #[test]
    fn stp() {
        opcode_test!(0xdb, Opcodes::STP, AddressModes::Implied);
    }
    #[test]
    fn stz_1() {
        opcode_test!(0x9c, Opcodes::STZ, AddressModes::Absolute);
    }
    #[test]
    fn stz_2() {
        opcode_test!(0x64, Opcodes::STZ, AddressModes::DirectPage);
    }
    #[test]
    fn stz_3() {
        opcode_test!(0x9e, Opcodes::STZ, AddressModes::AbsoluteIndexedX);
    }
    #[test]
    fn stz_4() {
        opcode_test!(0x74, Opcodes::STZ, AddressModes::DirectPageIndexedX);
    }
    #[test]
    fn tax() {
        opcode_test!(0xaa, Opcodes::TAX, AddressModes::Implied);
    }
    #[test]
    fn tay() {
        opcode_test!(0xa8, Opcodes::TAY, AddressModes::Implied);
    }
    #[test]
    fn tcd() {
        opcode_test!(0x5b, Opcodes::TCD, AddressModes::Implied);
    }
    #[test]
    fn tcs() {
        opcode_test!(0x1b, Opcodes::TCS, AddressModes::Implied);
    }
    #[test]
    fn tdc() {
        opcode_test!(0x7b, Opcodes::TDC, AddressModes::Implied);
    }
    #[test]
    fn tsc() {
        opcode_test!(0x3b, Opcodes::TSC, AddressModes::Implied);
    }
    #[test]
    fn tsx() {
        opcode_test!(0xba, Opcodes::TSX, AddressModes::Implied);
    }
    #[test]
    fn txa() {
        opcode_test!(0x8a, Opcodes::TXA, AddressModes::Implied);
    }
    #[test]
    fn txs() {
        opcode_test!(0x9a, Opcodes::TXS, AddressModes::Implied);
    }
    #[test]
    fn txy() {
        opcode_test!(0x9b, Opcodes::TXY, AddressModes::Implied);
    }
    #[test]
    fn tya() {
        opcode_test!(0x98, Opcodes::TYA, AddressModes::Implied);
    }
    #[test]
    fn tyx() {
        opcode_test!(0xbb, Opcodes::TYX, AddressModes::Implied);
    }
    #[test]
    fn wai() {
        opcode_test!(0xcb, Opcodes::WAI, AddressModes::Implied);
    }
    #[test]
    fn wdm() {
        opcode_test!(0x42, Opcodes::WDM, AddressModes::Unknown);
    }
    #[test]
    fn xba() {
        opcode_test!(0xeb, Opcodes::XBA, AddressModes::Implied);
    }
    #[test]
    fn xce() {
        opcode_test!(0xfb, Opcodes::XCE, AddressModes::Implied);
    }
}
