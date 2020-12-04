use std::fmt;

#[derive(Clone, Copy)]
pub struct Address {
    pub bank: u8,
    pub address: u16,
}

impl Address {
    pub fn add(&self, offset: usize) -> Address {
        Address {
            bank: self.bank,
            // TODO: wrapping add?
            address: self.address.wrapping_add(offset as u16),
        }
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address {{ bank: {:x}, address: {:x} }}",
            self.bank, self.address
        )
    }
}
