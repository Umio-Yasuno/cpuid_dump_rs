use crate::{_AX, CpuidResult};

pub struct AddressSize {
    pub physical: u8,
    pub virtual_: u8,
}

impl AddressSize {
    pub fn from_cpuid(cpuid: &CpuidResult) -> Self {
        Self {
            physical: (cpuid.eax & 0xFF) as u8,
            virtual_: ((cpuid.eax >> 8) & 0xFF) as u8,
        }
    }
    pub fn get() -> Self {
        let cpuid = cpuid!(_AX+0x8, 0x0);
        
        Self::from_cpuid(&cpuid)
    }
}
