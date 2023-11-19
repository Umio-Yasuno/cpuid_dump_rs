use crate::CpuidResult;

/// Used for [IntelExtTopo]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TopoLevelType {
    Invalid,
    SMT,
    Core,
    Module,
    Tile,
    Die,
}

impl From<u8> for TopoLevelType {
    fn from(reg: u8) -> Self {
        match reg {
            0x1 => Self::SMT,
            0x2 => Self::Core,
            0x3 => Self::Module,
            0x4 => Self::Tile,
            0x5 => Self::Die,
            /* 0x0 | */
            _ => Self::Invalid,
        }
    }
}

impl From<u32> for TopoLevelType {
    fn from(ecx: u32) -> Self {
        Self::from(((ecx >> 8) & 0xFF) as u8)
    }
}

impl From<&CpuidResult> for TopoLevelType {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(cpuid.ecx)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for TopoLevelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Intel Extended Topology, available from `CPUID.(EAX=0Bh, ECX=n)` or `CPUID.(EAX=1Fh, ECX=n)`
#[derive(Debug, Clone)]
pub struct IntelExtTopo {
    pub next_level: u32,
    pub x2apic_id: u32,
    pub num_proc: u32,
    pub level_type: TopoLevelType,
}

impl From<&CpuidResult> for IntelExtTopo {
    fn from(cpuid: &CpuidResult) -> Self {
        let next_level = cpuid.eax & 0xF;
        let num_proc = cpuid.ebx & 0xFFFF;
        let x2apic_id = cpuid.edx;
        let level_type = TopoLevelType::from(cpuid);

        Self {
            next_level,
            x2apic_id,
            level_type,
            num_proc,
        }
    }
}
