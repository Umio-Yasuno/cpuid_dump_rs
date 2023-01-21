use crate::{cpuid, CpuidResult, CpuVendor};
use crate::codename::{AmdCodename, IntelCodename, ZhaoxinCodename};
use crate::codename::{AmdMicroArch, IntelMicroArch, ZhaoxinMicroArch};
use std::fmt;

impl ProcInfo {
    pub fn from_fms(fms: &FamModStep, vendor: &CpuVendor) -> Self {
        let [f, m, s] = [fms.syn_fam, fms.syn_mod, fms.step];
        let unknown = |raw_eax: u32, step: u32| -> Self {

            Self {
                codename: CpuCodename::UnknownVendor(raw_eax),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(step),
                node: None,
            }
        };

        match *vendor {
            CpuVendor::AuthenticAMD => match f {
                0x10 => Self::amd_fam10h(m, s),
                0x11 => Self::amd_fam11h(m, s),
                0x12 => Self::amd_fam12h(m, s),
                0x14 => Self::amd_fam14h(m, s),
                0x15 => Self::amd_fam15h(m, s),
                0x16 => Self::amd_fam16h(m, s),
                0x17 => Self::amd_fam17h(m, s),
                0x19 => Self::amd_fam19h(m, s),
                _ => unknown(fms.raw_eax, s),
            },
            CpuVendor::GenuineIntel => match f {
                0x5 => Self::intel_fam05h(m, s),
                0x6 => Self::intel_fam06h(m, s),
                _ => unknown(fms.raw_eax, s),
            },
            CpuVendor::CentaurHauls |
            CpuVendor::Shanghai => match f {
                0x6 => Self::zhaoxin_fam06h(m, s),
                0x7 => Self::zhaoxin_fam07h(m, s),
                _ => unknown(fms.raw_eax, s),
            },
            _ => unknown(fms.raw_eax, s),
        }
    }
}

pub struct ProcInfo {
    pub codename: CpuCodename,
    pub archname: CpuMicroArch,
    pub step_info: CpuStepping,
    pub node: Option<ProcessNode>,
}

pub enum CpuCodename {
    Amd(AmdCodename),
    Intel(IntelCodename),
    Zhaoxin(ZhaoxinCodename),
    UnknownVendor(u32),
}

impl fmt::Display for CpuCodename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Amd(arch) => write!(f, "AMD {arch}"),
            Self::Intel(arch) => write!(f, "Intel {arch}"),
            Self::Zhaoxin(arch) => write!(f, "Zhaoxin {arch}"),
            Self::UnknownVendor(raw_eax) => write!(f, "UnknownVendor(EAX={raw_eax:X})"),
        }
    }
}

pub enum CpuMicroArch {
    Amd(AmdMicroArch),
    Intel(IntelMicroArch),
    Zhaoxin(ZhaoxinMicroArch),
    Unknown,
}

impl fmt::Display for CpuMicroArch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Amd(arch) => write!(f, "AMD {arch}"),
            Self::Intel(arch) => write!(f, "Intel {arch}"),
            Self::Zhaoxin(arch) => write!(f, "Zhaoxin {arch}"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CpuStepping {
    A0,
    A1,
    A0_A1,
    B0,
    B1,
    B2,
    B2_B3,
    B3,
    BA,
    C0,
    C2,
    C3,
    D0,
    D0_J0, // Sandy Bridge
    D1,
    D2_J1_Q0, // Sandy Bridge
    E0,
    E1,
    G0,
    G1,
    H0,
    J0,
    K0,
    K1,
    L0,
    P0,
    P1,
    Q0,
    R0,
    R1,
    U0,
    V0,
    V2_V3, // Broadwell-D
    W0,
    Y0,
    HA0,
    HB0,
    HQ0,
    HR0,
    Unknown(u32),
}

impl fmt::Display for CpuStepping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(val) => write!(f, "{val:X}"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub enum ProcessNode {
    _UM(u8),
    NM(u8),
    Intel(u8),
}

impl fmt::Display for ProcessNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_UM(size) => write!(f, "{size} um"),
            Self::NM(size) => write!(f, "{size} nm"),
            Self::Intel(size) => write!(f, "Intel {size}"),
        }
    }
}

pub struct FamModStep {
    pub syn_fam: u32,
    pub syn_mod: u32,
    pub step: u32,
    pub raw_eax: u32,
}

impl From<u32> for FamModStep {
    fn from(eax: u32) -> Self {
        Self {
            syn_fam: ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF),
            syn_mod: ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0),
            step: eax & 0xF,
            raw_eax: eax,
        }
    }
}

impl From<&CpuidResult> for FamModStep {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(cpuid.eax)
    }
}

impl FamModStep {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x1))
    }
}
