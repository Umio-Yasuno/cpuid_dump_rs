use crate::CpuidResult;

/// Information available from `CPUID.(EAX=8000_001Eh)`, AMD CPU only
pub struct AmdProcTopo {
    pub ext_apic_id: u32,
    pub threads_per_core: u8,
    pub core_id: u8,
    pub nodes_per_processor: u8,
    pub node_id: u8,
}

impl From<&CpuidResult> for AmdProcTopo {
    fn from(cpuid: &CpuidResult) -> Self {
        let ext_apic_id = cpuid.eax;
        let threads_per_core = (((cpuid.ebx >> 8) & 0xFF) as u8).saturating_add(1);
        let core_id = (cpuid.ebx & 0xFF) as u8;
        let nodes_per_processor = (cpuid.ecx & 0b111) as u8;
        let node_id = (cpuid.ecx & 0xFF) as u8;

        Self {
            ext_apic_id,
            threads_per_core,
            core_id,
            nodes_per_processor,
            node_id,
        }
    }
}

impl AmdProcTopo {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x8000_001E, 0x0))
    }
}
