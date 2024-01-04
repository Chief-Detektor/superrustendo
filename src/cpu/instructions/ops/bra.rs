use crate::mem::Bus;

pub fn bra(bus: &mut Bus, value: u8) {
    let mut regs = bus.get_cpu().regs.borrow_mut();
    let val = (value as i8) as i16;
    println!("BRA: {:X}", val);
    let foo = (regs.PC as i32 + val as i32) as i32;
    println!("BRA new pc: {:X}", foo);
    regs.PC = foo as u16;
}
