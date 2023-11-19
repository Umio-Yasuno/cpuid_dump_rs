use crate::{CpuidResult};

/// Physical/Virtual Addresses size (bit) available from `CPUID.(EAX=8000_0008h):EAX`
#[derive(Debug, Clone)]
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
        Self::from(&cpuid!(0x8000_0008, 0x0))
    }
}
