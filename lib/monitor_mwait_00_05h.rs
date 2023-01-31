use crate::{cpuid, CpuidResult};

#[derive(Debug)]
pub struct MonitorMwait {
    pub min_monitor_line_size: u16,
    pub max_monitor_line_size: u16,
    /// Indicates enumeration MONITOR/MWAIT extensions
    pub emx_supported: bool,
    /// Interrupt break-event
    pub ibe_supported: bool,
    pub mwait_sub_states: [u8; 8],
}

impl From<&CpuidResult> for MonitorMwait {
    fn from(cpuid: &CpuidResult) -> Self {
        let min_monitor_line_size = (cpuid.eax & 0xFFFF) as u16;
        let max_monitor_line_size = (cpuid.ebx & 0xFFFF) as u16;

        let emx_supported = (cpuid.ecx & 0b1) == 0b1;
        let ibe_supported = (cpuid.ecx & 0b10) == 0b10;

        let mut mwait_sub_states = [0u8; 8];

        for (i, byte) in cpuid.edx.to_le_bytes().iter().enumerate() {
            let i = i * 2;
            mwait_sub_states[i] = byte & 0xF;
            mwait_sub_states[i+1] = (byte >> 4) & 0xF;
        }

        Self {
            min_monitor_line_size,
            max_monitor_line_size,
            emx_supported,
            ibe_supported,
            mwait_sub_states,
        }
    }
}

impl MonitorMwait {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x5, 0x0))
    }
}
