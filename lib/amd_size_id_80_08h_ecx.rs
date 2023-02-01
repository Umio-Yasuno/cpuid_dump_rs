use crate::{cpuid, CpuidResult};

pub struct AmdSizeId {
    pub perf_tsc_size: u8,
    pub apic_id_size: u8,
    pub num_thread: u8,
    pub rdpru_max_input: u16,
    pub invlpgb_max_page: u16,
}


impl From<&CpuidResult> for AmdSizeId {
    fn from(cpuid: &CpuidResult) -> Self {
        let perf_tsc_size = 40 + ((cpuid.ecx >> 16) & 0b11) as u8 * 8;
        let apic_id_size = ((cpuid.ecx >> 12) & 0xF) as u8;
        let num_thread = ((cpuid.ecx & 0xFF) as u8).saturating_add(1);
        let rdpru_max_input = (cpuid.edx >> 16) as u16;
        let invlpgb_max_page = (cpuid.edx & 0xFFFF) as u16;

        Self {
            perf_tsc_size,
            apic_id_size,
            num_thread,
            rdpru_max_input,
            invlpgb_max_page,
        }
    }
}

impl AmdSizeId {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x8000_0008, 0x0))
    }
}
