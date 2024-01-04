use crate::mem::Bus;

pub fn bcc(bus: &mut Bus, value: u8) /* -> u8  (for counting cycles) */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    if regs.P.c == 0 {
        let mut addr = (value as i8) as i16;
        addr += regs.PC as i16;
        regs.PC = addr as u16;
    }
    //else {
    //    regs.PC += 2;
    //}
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn bcc_instruction_0() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.c = 0;
            regs.PC = 0x0000;
        }
        bcc(&mut bus, 0x70);
        println!("cpu: {:?}", bus.get_cpu());
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.PC, 0x0070);
    }

    #[test]
    fn bcc_instruction_1() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.c = 1;
            regs.PC = 0x0000;
        }
        bcc(&mut bus, 0x70);
        println!("cpu: {:?}", bus.get_cpu());
        let regs = bus.get_cpu().regs.borrow();
        // NOTE: PC is incremented by 2 after the instruction is executed. Not by the instruction
        // itself. This is done by the iterator.
        assert_eq!(regs.PC, 0x0000);
    }
}
