use crate::{cpuid, CpuidResult};
#[cfg(feature = "std")]
use std::fmt;

const LEAF: u32 = 0x8000_0026;

/* from AMD Zen 4 */
/* ref: #55901, Preliminary Processor Programming Reference (PPR) for AMD Family 19h Model 11h, Revision B1 Processors Volume 1 of 6 */
/// AMD Extended Topology, available from `CPUID.(EAX=8000_0026h, ECX=n)`
#[derive(Debug, Clone)]
pub struct AmdExtTopo {
    pub asymmetric_cores: bool,
    pub hetero_cores: bool,
    pub eff_rank_available: bool,
    pub next_level: u8,
    pub core_type: Option<AmdCoreType>,
    pub native_model_id: Option<AmdNativeModelId>,
    pub eff_rank: Option<u8>,
    pub num_proc: u16,
    pub level_type: AmdTopoLevelType,
    _input_ecx: u8,
    pub ext_apic_id: u32,
}

/// Used for [AmdExtTopo]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AmdTopoLevelType {
    Core = 1,
    Complex = 2,
    Die = 3, // CCD
    Socket = 4,
    Reserved,
}

impl From<u8> for AmdTopoLevelType {
    fn from(reg: u8) -> Self {
        match reg {
            0x1 => Self::Core,
            0x2 => Self::Complex,
            0x3 => Self::Die,
            0x4 => Self::Socket,
            _ => Self::Reserved,
        }
    }
}

impl From<&CpuidResult> for AmdTopoLevelType {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(((cpuid.ecx >> 8) & 0xFF) as u8)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for AmdTopoLevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AmdTopoLevelType {
    fn is_core(&self) -> bool {
        *self == Self::Core
    }
}

/// Used for [AmdExtTopo]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmdCoreType {
    Performance,
    Efficiency,
    Reserved,
}

impl From<u8> for AmdCoreType {
    fn from(reg: u8) -> Self {
        match reg {
            0x0 => Self::Performance,
            0x1 => Self::Efficiency,
            _ => Self::Reserved,
        }
    }
}

impl From<&CpuidResult> for AmdCoreType {
    fn from(cpuid: &CpuidResult) -> Self {
        AmdCoreType::from(((cpuid.ebx >> 28) & 0xF) as u8)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for AmdCoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Used for [AmdExtTopo]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AmdNativeModelId {
    Zen_4,
    Reserved,
}

impl From<u8> for AmdNativeModelId {
    fn from(reg: u8) -> Self {
        // TODO: check CPU Family, Model
        match reg {
            0x0 => Self::Zen_4,
            _ => Self::Reserved,
        }
    }
}

impl From<&CpuidResult> for AmdNativeModelId {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(((cpuid.ebx >> 24) & 0xF) as u8)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for AmdNativeModelId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Zen_4 => write!(f, "Zen 4"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<&CpuidResult> for AmdExtTopo {
    fn from(cpuid: &CpuidResult) -> Self {
        // TODO: check CPU Family, Model
        let level_type = AmdTopoLevelType::from(cpuid);
        let next_level = (cpuid.eax & 0xF) as u8;

        let asymmetric_cores = ((cpuid.eax >> 31) & 0b1) == 1;
        let hetero_cores = ((cpuid.eax >> 30) & 0b1) == 1;

        let eff_rank_available = level_type.is_core() && (((cpuid.eax >> 29) & 0b1) == 1);
        let core_type = level_type.is_core().then_some(AmdCoreType::from(cpuid));
        // TODO: need AmdCoreType?
        let native_model_id = level_type.is_core().then_some(AmdNativeModelId::from(cpuid));
        let eff_rank = level_type.is_core().then_some(((cpuid.ebx >> 16) & 0xFF) as u8);

        let num_proc = (cpuid.ebx & 0xFFFF) as u16;
        let _input_ecx = (cpuid.ecx & 0xFF) as u8;
        let ext_apic_id = cpuid.edx;

        Self {
            asymmetric_cores,
            hetero_cores,
            eff_rank_available,
            next_level,
            core_type,
            native_model_id,
            eff_rank,
            num_proc,
            level_type,
            _input_ecx,
            ext_apic_id,
        }
    }
}

impl AmdExtTopo {
    pub fn get(sub_leaf: u32) -> Self {
        Self::from(&cpuid!(LEAF, sub_leaf))
    }

    pub fn is_supported() -> bool {
        const INPUT_ECX: u32 = 0x1;
        let cpuid = cpuid!(LEAF, INPUT_ECX);

        (cpuid.ecx & 0xFF) == INPUT_ECX
    }
}
