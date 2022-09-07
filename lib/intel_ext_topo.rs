use crate::CpuidResult;

#[repr(u8)]
pub enum IntelLevelType {
    Invalid,
    SMT,
    Core,
    Module,
    Tile,
    Die,
}

impl IntelLevelType {
    fn from_reg(reg: u8) -> Self {
        match reg {
            0x1 => Self::SMT,
            0x2 => Self::Core,
            0x3 => Self::Module,
            0x4 => Self::Tile,
            0x5 => Self::Die,
            0x0 |
            _ => Self::Invalid,
        }
    }
}

use std::fmt;
impl fmt::Display for IntelLevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SMT => write!(f, "SMT"),
            Self::Core => write!(f, "Core"),
            Self::Module => write!(f, "Module"),
            Self::Tile => write!(f, "Tile"),
            Self::Die => write!(f, "Die"),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
}

pub struct IntelExtTopo {
    pub next_level: u32,
    pub x2apic_id: u32,
    pub num_proc: u32,
    pub level_type: IntelLevelType,
}

impl IntelExtTopo {
    pub fn from_cpuid(cpuid: &CpuidResult) -> Self {
        let next_level = cpuid.eax & 0xF;
        let x2apic_id = cpuid.edx;
        let level_type = {
            let reg = (cpuid.ecx >> 8) & 0xFF;

            IntelLevelType::from_reg(reg as u8)
        };
        let num_proc = cpuid.ebx & 0xFFFF;

        Self {
            next_level,
            x2apic_id,
            level_type,
            num_proc,
        }
    }
}
