use crate::CpuidResult;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
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

impl fmt::Display for TopoLevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
        let level_type = {
            let reg = (cpuid.ecx >> 8) & 0xFF;

            TopoLevelType::from(reg as u8)
        };

        Self {
            next_level,
            x2apic_id,
            level_type,
            num_proc,
        }
    }
}
