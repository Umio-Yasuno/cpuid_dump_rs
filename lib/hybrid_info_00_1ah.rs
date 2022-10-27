use crate::{cpuid, CpuidResult};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum HybridCoreType {
    _Reserved1 = 0x10,
    Atom = 0x20,
    _Reserved2 = 0x30,
    Core = 0x40,
    Invalid,
}

impl fmt::Display for HybridCoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&CpuidResult> for HybridCoreType {
    fn from(cpuid: &CpuidResult) -> Self {
        match cpuid.eax >> 24 {
            0x10 => HybridCoreType::_Reserved1,
            0x20 => HybridCoreType::Atom,
            0x30 => HybridCoreType::_Reserved2,
            0x40 => HybridCoreType::Core,
            _ => Self::Invalid,
        }
    }
}

pub struct HybridInfo;

impl HybridInfo {
    pub fn get_hybrid_info_from_cpuid(cpuid: &CpuidResult) -> (Option<HybridCoreType>, u32) {
        (
            Self::get_core_type(&cpuid),
            Self::get_native_model_id(&cpuid),
        )
    }
    
    pub fn get_hybrid_info() -> (Option<HybridCoreType>, u32) {
        let cpuid = cpuid!(0x1A, 0x0);
        
        Self::get_hybrid_info_from_cpuid(&cpuid)
    }

    pub fn get_core_type(cpuid: &CpuidResult) -> Option<HybridCoreType> {
        let core_type = HybridCoreType::from(cpuid);

        if core_type == HybridCoreType::Invalid {
            return None;
        }

        Some(core_type)
    }

    pub fn get_native_model_id(cpuid: &CpuidResult) -> u32 {
        cpuid.eax & 0x00FFFFFF
    }
}

/* https://github.com/intel/perfmon/blob/main/mapfile.csv */
impl IntelNativeModelId {
    const fn gen_eax(core_type: HybridCoreType, nid: u32) -> u32 {
        ((core_type as u32) << 24) | (nid & 0x00FFFFFF)
    }

    const TNT: u32 = Self::gen_eax(HybridCoreType::Atom, 0x0);
    const GRT: u32 = Self::gen_eax(HybridCoreType::Atom, 0x1);
    const CMT: u32 = Self::gen_eax(HybridCoreType::Atom, 0x2);

    const SNC: u32 = Self::gen_eax(HybridCoreType::Core, 0x0);
    const GLC: u32 = Self::gen_eax(HybridCoreType::Core, 0x1);
    const RWC: u32 = Self::gen_eax(HybridCoreType::Core, 0x2);
}

#[derive(Debug)]
#[repr(u32)]
enum IntelNativeModelId {
    /* Atom */
    Tremont = Self::TNT,
    Gracemont = Self::GRT,
    Crestmont = Self::CMT,
    /* Core */
    SunnyCove = Self::SNC,
    GoldenCove = Self::GLC,
    RedwoodCove = Self::RWC,
    /* */
    _Reserved,
}

impl From<u32> for IntelNativeModelId {
    fn from(eax: u32) -> Self {
        match eax {
            Self::TNT => Self::Tremont,
            Self::GRT => Self::Gracemont,
            Self::CMT => Self::Crestmont,
            Self::SNC => Self::SunnyCove,
            Self::GLC => Self::GoldenCove,
            Self::RWC => Self::RedwoodCove,
            _ => Self::_Reserved,
        }
    }
}
