use crate::{cpuid, CpuidResult};

/// Information available from `CPUID.(EAX=01h):EBX`
pub struct Info01h {
    pub local_apic_id: u8,
    pub max_apic_id: u8,
    pub clflush_size: u8,
    pub brand_id: u8,
}

impl From<u32> for Info01h {
    fn from(ebx: u32) -> Self {
        Self {
            local_apic_id: ((ebx >> 24) & 0xFF) as u8,
            max_apic_id: ((ebx >> 16) & 0xFF) as u8,
            clflush_size: (((ebx >> 8) & 0xFF) as u8).saturating_mul(8),
            brand_id: (ebx & 0xFF) as u8,
        }
    }
}

impl From<&CpuidResult> for Info01h {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(cpuid.ebx)
    }
}

impl Info01h {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x1, 0x0))
    }
}
