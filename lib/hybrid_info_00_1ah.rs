use crate::{cpuid, CpuidResult};

/* https://github.com/slimbootloader/slimbootloader/blob/master/Platform/AlderlakeBoardPkg/Library/Stage2BoardInitLib/CpuInfoLib.c */

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum HybridCoreType {
    _Reserved1 = 0x10, // Quark?
    Atom = 0x20,
    _Reserved2 = 0x30, // Knights?
    Core = 0x40,
    Invalid,
}

#[cfg(feature = "std")]
impl std::fmt::Display for HybridCoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
pub enum IntelNativeModelId {
    /* Atom */
    Tremont = Self::TNT,
    Gracemont = Self::GRT,
    Crestmont = Self::CMT,
    /* Core */
    SunnyCove = Self::SNC,
    GoldenCove = Self::GLC,
    RedwoodCove = Self::RWC,
    /* */
    Unknown(u32),
}

impl From<&CpuidResult> for IntelNativeModelId {
    fn from(cpuid: &CpuidResult) -> Self {
        match cpuid.eax {
            Self::TNT => Self::Tremont,
            Self::GRT => Self::Gracemont,
            Self::CMT => Self::Crestmont,
            Self::SNC => Self::SunnyCove,
            Self::GLC => Self::GoldenCove,
            Self::RWC => Self::RedwoodCove,
            _ => Self::Unknown(cpuid.eax),
        }
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for IntelNativeModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown(eax) => write!(f, "Unknown({eax:#010X})"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct HybridInfo;

impl HybridInfo {
    pub fn get_hybrid_info() -> (Option<HybridCoreType>, IntelNativeModelId) {
        Self::get_hybrid_info_from_cpuid(&cpuid!(0x1A, 0x0))
    }

    pub fn get_hybrid_info_from_cpuid(cpuid: &CpuidResult) -> (
        Option<HybridCoreType>,
        IntelNativeModelId
    ) {
        (
            Self::get_core_type(cpuid),
            Self::get_native_model_id(cpuid),
        )
    }

    pub fn get_core_type(cpuid: &CpuidResult) -> Option<HybridCoreType> {
        let core_type = HybridCoreType::from(cpuid);

        if core_type == HybridCoreType::Invalid {
            None
        } else {
            Some(core_type)
        }
    }

    pub fn get_native_model_id(cpuid: &CpuidResult) -> IntelNativeModelId {
        IntelNativeModelId::from(cpuid)
    }
}
