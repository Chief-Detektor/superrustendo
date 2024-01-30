use crate::mem::Bus;

pub fn sec(bus: &mut Bus) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    regs.P.c = 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sec_instruction() {
        let mut bus = Bus::new();
        {
            let mut regs = bus.get_cpu().regs.borrow_mut();
            regs.P.c = 0;
        }
        sec(&mut bus);
        let regs = bus.get_cpu().regs.borrow();
        assert_eq!(regs.P.c, 1);
    }
}
