use crate::{cpuid, CpuidResult};

pub struct AmdSizeId {
    pub perf_tsc_size: u8,
    pub apic_id_size: u8,
    pub num_thread: u8,
}

impl From<u32> for AmdSizeId {
    fn from(ecx: u32) -> Self {
        Self {
            perf_tsc_size: ((ecx >> 16) & 0b11) as u8,
            apic_id_size: ((ecx >> 12) & 0xF) as u8,
            num_thread: ((ecx & 0xFF) as u8).saturating_add(1),
        }
    }
}

impl From<&CpuidResult> for AmdSizeId {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(cpuid.ecx)
    }
}

impl AmdSizeId {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x8000_0008, 0x0))
    }
}
