use crate::{
    cpu::{address::Address, addressmodes::AddressModes},
    mem::Bus,
};

pub fn jsr(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) {
    println!("====> Jump instruction JSR");
    let regs = bus.get_cpu().regs.borrow().clone();
    let pc_low = (regs.PC & 0x00ff) as u8;
    let pc_high = (regs.PC >> 8) as u8;

    if address_mode == AddressModes::AbsoluteLong {
        println!("====> Absolute Long");
        let bank = regs.PBR.clone();
        bus.get_cpu().stack_push(bank);
    }

    bus.get_cpu().stack_push(pc_high);
    bus.get_cpu().stack_push(pc_low);
    let mut regs_mut = bus.get_cpu().regs.borrow_mut();

    // let address = effective_address
    if address_mode == AddressModes::AbsoluteLong {
        regs_mut.PBR = effective_address.unwrap().bank;
    }
    regs_mut.PC = effective_address.unwrap().address;
    //regs.PC = value.unwrap().lower_to_number() as u16;
}
