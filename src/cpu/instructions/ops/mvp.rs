use crate::{
    cpu::{address::Address, Accumulator, IndexRegister},
    mem::Bus,
};


// TODO: Cover case x = 1 or e = 1
// In that case everything is happening in the zero page

pub fn mvp(bus: &mut Bus, low: u8, high: u8) {
    {
        let (src_bnk, dest_bnk) = (high, low);
        let mut regs = bus.get_cpu().regs.borrow_mut();
        loop {
            if regs.C == Accumulator::from(0xffffu16) {
                break;
            }
            let source = u16::from(regs.X);
            let dest = u16::from(regs.Y);
            let _length = u16::from(regs.C);

            let src_address = Address {
                bank: src_bnk,
                address: source,
            };
            let val = bus.read(src_address);
            let dest_address = Address {
                bank: dest_bnk,
                address: dest,
            };
            bus.write(dest_address, val);

            // print!("{:x} : {:?}|", val, address);
            regs.X = IndexRegister::from(u16::from(regs.X).wrapping_sub(1));
            regs.Y = IndexRegister::from(u16::from(regs.Y).wrapping_sub(1));
            regs.C = Accumulator::from(u16::from(regs.C).wrapping_sub(1));
        }

        // panic!("src: {} : {} dest: {} : {} count: {}", src_bnk, source, dest_bnk, dest, length);/
    }
}
