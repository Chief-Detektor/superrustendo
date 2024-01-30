use crate::{cpu::{address::Address, addressmodes::AddressModes}, mem::Bus};

pub fn jmp(bus: &mut Bus, effective_address: Option<Address>, address_mode: AddressModes) /* -> u8  (for counting cycles) */
{
    println!("====> Jump instruction: JMP");
    let mut regs = bus.get_cpu().regs.borrow_mut();
        regs.PC = effective_address.unwrap().address;
        if address_mode == AddressModes::AbsoluteLong {
            regs.PBR = effective_address.unwrap().bank;
        }
        //bus.get_cpu().
        // | (regs.DBR as u16) << 16;
        //regs.PC = value.unwrap().lower_to_number() as u16;
}
