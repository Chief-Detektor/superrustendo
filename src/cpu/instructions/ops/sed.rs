use crate::mem::Bus;


pub fn sed(bus: &mut Bus) /* -> u8  (for counting cycles) */
{
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.P.d = 1;
}

#[cfg(test)]

mod tests{
    use super::*;
    use crate::mem::Bus;
    #[test]
    fn sed_instruction() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.d = 0;
        }
        sed(&mut bus);
        println!("cpu: {:?}", bus.get_cpu());
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.P.d, 1);
    }
}
