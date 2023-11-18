use crate::{cpuid, CpuidResult};

/* ref: https://github.com/llvm/llvm-project/blob/main/clang/lib/Headers/cpuid.h */
/* ref: https://github.com/gcc-mirror/gcc/blob/master/gcc/config/i386/cpuid.h */

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[allow(dead_code)]
impl Vendor {
    const AMD_EBX: u32 = 0x6874_7541;
    const AMD_ECX: u32 = 0x444D_4163;
    const AMD_EDX: u32 = 0x6974_6E65;
    const REG_AMD: Self = Self {
        ebx: Self::AMD_EBX,
        ecx: Self::AMD_ECX,
        edx: Self::AMD_EDX,
    };

    const INTEL_EBX: u32 = 0x756E_6547;
    const INTEL_ECX: u32 = 0x6C65_746E;
    const INTEL_EDX: u32 = 0x4965_6E69;
    const REG_INTEL: Self = Self {
        ebx: Self::INTEL_EBX,
        ecx: Self::INTEL_ECX,
        edx: Self::INTEL_EDX,
    };

    const CENTAUR_EBX: u32 = 0x746E_6543;
    const CENTAUR_ECX: u32 = 0x736C_7561;
    const CENTAUR_EDX: u32 = 0x4872_7561;
    const REG_CENTAUR: Self = Self {
        ebx: Self::CENTAUR_EBX,
        ecx: Self::CENTAUR_ECX,
        edx: Self::CENTAUR_EDX,
    };

    const SHANGHAI_EBX: u32 = 0x6853_2020;
    const SHANGHAI_ECX: u32 = 0x2020_6961;
    const SHANGHAI_EDX: u32 = 0x6867_6E61;
    const REG_SHANGHAI: Self = Self {
        ebx: Self::SHANGHAI_EBX,
        ecx: Self::SHANGHAI_ECX,
        edx: Self::SHANGHAI_EDX,
    };

    pub fn get() -> Self {
        Self::from(&cpuid!(0x0, 0x0))
    }
}

/*
#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::ProcName;

        let mut total = [0u8; 12];

        /* ebx, edx, ecx */
        for (i, reg) in [self.ebx, self.edx, self.ecx].iter().enumerate() {
            total[(i*4)..(i*4+4)].copy_from_slice(&ProcName::check_reg(*reg))
        }

        write!(f, "{}", String::from_utf8(total.to_vec()).unwrap())
    }
}
*/

impl From<&CpuidResult> for Vendor {
    fn from(cpuid: &CpuidResult) -> Self {
        Self {
            ebx: cpuid.ebx,
            ecx: cpuid.ecx,
            edx: cpuid.edx,
        }
    }
}

/// List of x86_64 CPU vendors
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuVendor {
    AuthenticAMD,
    GenuineIntel,
    CentaurHauls,
    Shanghai,
    Unknown(Vendor),
}

impl From<&Vendor> for CpuVendor {
    fn from(vendor: &Vendor) -> Self {
        match vendor.ebx {
            Vendor::AMD_EBX => Self::AuthenticAMD,
            Vendor::INTEL_EBX => Self::GenuineIntel,
            Vendor::CENTAUR_EBX => Self::CentaurHauls,
            Vendor::SHANGHAI_EBX => Self::Shanghai,
            _ => Self::Unknown(vendor.clone()),
        }
    }
}

impl From<&CpuidResult> for CpuVendor {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(&Vendor::from(cpuid))
    }
}

impl CpuVendor {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x0, 0x0))
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for CpuVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
