use crate::{_AX, CpuidResult};

pub struct AddressSize {
    pub physical: u8,
    pub virtual_: u8,
}

impl From<&CpuidResult> for AddressSize {
    fn from(cpuid: &CpuidResult) -> Self {
        Self {
            physical: (cpuid.eax & 0xFF) as u8,
            virtual_: ((cpuid.eax >> 8) & 0xFF) as u8,
        }
    }
}

impl AddressSize {
    pub fn get() -> Self {
        Self::from(&cpuid!(_AX+0x8, 0x0))
    }
}
