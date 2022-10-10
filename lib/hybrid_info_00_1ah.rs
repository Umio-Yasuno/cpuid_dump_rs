use crate::{cpuid, CpuidResult};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum HybridCoreType {
    _Reserved1,
    Atom,
    _Reserved2,
    Core,
    Invalid,
}

impl fmt::Display for HybridCoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid"),
            Self::_Reserved1 => write!(f, "_Reserved1"),
            Self::Atom => write!(f, "Atom"),
            Self::_Reserved2 => write!(f, "_Reserved2"),
            Self::Core => write!(f, "Core"),
        }
    }
}

pub struct HybridInfo;

impl HybridInfo {
    pub fn get_hybrid_info_from_cpuid(cpuid: CpuidResult) -> (Option<HybridCoreType>, u32) {
        (
            Self::get_core_type(cpuid),
            Self::get_native_model_id(cpuid),
        )
    }
    
    pub fn get_hybrid_info() -> (Option<HybridCoreType>, u32) {
        let cpuid = cpuid!(0x1A, 0x0);
        
        Self::get_hybrid_info_from_cpuid(cpuid)
    }

    pub fn get_core_type(cpuid: CpuidResult) -> Option<HybridCoreType> {
        let core_type = match cpuid.eax >> 24 {
            0x10 => HybridCoreType::_Reserved1,
            0x20 => HybridCoreType::Atom,
            0x30 => HybridCoreType::_Reserved2,
            0x40 => HybridCoreType::Core,
            _ => return None,
        };

        Some(core_type)
    }

    pub fn get_native_model_id(cpuid: CpuidResult) -> u32 {
        cpuid.eax & 0x00FFFFFF
    }
}
