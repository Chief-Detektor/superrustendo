// Re-export all the ops instructions to avoid double namespace e.g. cpu::instructions::ops::and::and
mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod bra;
mod brk;
mod brl;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod mvn;
mod mvp;
mod ora;
mod pea;
mod pei;
mod per;
mod pha;
mod phb;
mod phd;
mod phk;
mod php;
mod phx;
mod phy;
mod pla;
mod plb;
mod pld;
mod plp;
mod plx;
mod ply;
mod rep;
mod rol;
mod ror;
mod rti;
mod rtl;
mod rts;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sep;
mod sta;
mod stp;
mod stx;
mod sty;
mod stz;
mod tax;
mod tay;
mod tcd;
mod tcs;
mod tdc;
mod trb;
mod tsb;
mod tsc;
mod tsx;
mod txa;
mod txs;
mod txy;
mod tya;
mod tyx;
mod xba;
mod xce;

pub mod utils {
    use crate::cpu::Registers;

    pub fn check_negative_u8(regs: &mut Registers, value: u8) {
        println!("check negative Value: {:b}", value);
        println!("check negative calc: {:b}", (value & 0x8));
        if (value & 0x80) == 0x80 {
            regs.get_P_mut().set_n(1);
        } else {
            regs.get_P_mut().set_n(0);
        }
    }

    pub fn check_negative_u16(regs: &mut Registers, value: u16) {
        if (value & 0x8000) == 0x8000 {
            regs.get_P_mut().set_n(1);
        } else {
            regs.get_P_mut().set_n(0);
        }
    }

    pub fn check_zero(regs: &mut Registers, value: u16) {
        if value == 0 {
            regs.get_P_mut().set_z(1);
        } else {
            regs.get_P_mut().set_z(0);
        }
    }

    pub fn check_signed_overflow_u16(regs: &mut Registers, value: u16) {
        if value > 0x7FFF {
            regs.get_P_mut().set_v(1);
        } else {
            regs.get_P_mut().set_v(0);
        }
    }

    pub fn check_signed_overflow_u8(regs: &mut Registers, value: u8) {
        if value > 0x7F {
            regs.get_P_mut().set_v(1);
        } else {
            regs.get_P_mut().set_v(0);
        }
    }

    pub trait Bcd<T> {
        fn to_bcd(self) -> T;
        fn from_bcd(self) -> T;
    }

    impl Bcd<u8> for u8 {
        fn to_bcd(self) -> Self {
            if self > 99 {
                panic!("Value must be between 0 and 99");
            }
            let low = self % 10;
            let high = self / 10;
            (high << 4) | low
        }
        fn from_bcd(self) -> Self {
            let low = self & 0x0F;
            let high = (self & 0xF0) >> 4;
            (high * 10) + low
        }
    }

    pub fn to_bcd(value: u8) -> u8 {
        if value > 99 {
            panic!("Value must be between 0 and 99");
        }
        let low = value % 10;
        println!("Low: {:b}", low);
        let high = value / 10;
        println!("High: {:b}", high);
        (high << 4) | low
    }

    pub fn from_bcd(value: u8) -> u8 {
        let low = value & 0x0F;
        let high = (value & 0xF0) >> 4;
        (high * 10) + low
    }
}

pub use adc::adc;
pub use and::and;
pub use asl::asl;
pub use bcc::bcc;
pub use bcs::bcs;
pub use beq::beq;
pub use bit::bit;
pub use bmi::bmi;
pub use bne::bne;
pub use bpl::bpl;
pub use bra::bra;
pub use brk::brk;
pub use brl::brl;
pub use bvc::bvc;
pub use bvs::bvs;
pub use clc::clc;
pub use cld::cld;
pub use cli::cli;
pub use clv::clv;
pub use cmp::cmp;
pub use cpx::cpx;
pub use cpy::cpy;
pub use dec::dec;
pub use dex::dex;
pub use dey::dey;
pub use eor::eor;
pub use inc::inc;
pub use inx::inx;
pub use iny::iny;
pub use jmp::jmp;
pub use jsr::jsr;
pub use lda::lda;
pub use ldx::ldx;
pub use ldy::ldy;
pub use lsr::lsr;
pub use mvn::mvn;
pub use mvp::mvp;
pub use ora::ora;
pub use pea::pea;
pub use pei::pei;
pub use per::per;
pub use pha::pha;
pub use phb::phb;
pub use phd::phd;
pub use phk::phk;
pub use php::php;
pub use phx::phx;
pub use phy::phy;
pub use pla::pla;
pub use plb::plb;
pub use pld::pld;
pub use plp::plp;
pub use plx::plx;
pub use ply::ply;
pub use rep::rep;
pub use rol::rol;
pub use ror::ror;
pub use rti::rti;
pub use rtl::rtl;
pub use rts::rts;
pub use sbc::sbc;
pub use sec::sec;
pub use sed::sed;
pub use sei::sei;
pub use sep::sep;
pub use sta::sta;
pub use stp::stp;
pub use stx::stx;
pub use sty::sty;
pub use stz::stz;
pub use tax::tax;
pub use tay::tay;
pub use tcd::tcd;
pub use tcs::tcs;
pub use tdc::tdc;
pub use trb::trb;
pub use tsb::tsb;
pub use tsc::tsc;
pub use tsx::tsx;
pub use txa::txa;
pub use txs::txs;
pub use txy::txy;
pub use tya::tya;
pub use tyx::tyx;
pub use xba::xba;
pub use xce::xce;
