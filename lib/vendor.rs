use crate::{cpuid, CpuidResult};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl Vendor {
    const REG_AMD: Self = Self {
        ebx: 0x6874_7541,
        ecx: 0x444D_4163,
        edx: 0x6974_6E65,
    };

    const REG_INTEL: Self = Self {
        ebx: 0x756E_6547,
        ecx: 0x4965_6E69,
        edx: 0x6C65_746E,
    };

    const REG_CENTAUR: Self = Self {
        ebx: 0x746E_6543,
        ecx: 0x736C_7561,
        edx: 0x4872_7561,
    };

    const REG_SHANGHAI: Self = Self {
        ebx: 0x6853_2020,
        ecx: 0x2020_6961,
        edx: 0x6867_6E61,
    };

    pub fn get() -> Self {
        Self::from(&cpuid!(0x0, 0x0))
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let check = |reg: u32| -> Vec<u8> {
            reg.to_le_bytes().iter().map(|&byte|
                if char::from(byte).is_control() { 0x20 } else { byte }
            ).collect()
        };

        let bytes = [
            self.ebx,
            self.edx,
            self.ecx,
        ]
        .map(check)
        .concat();

        write!(f, "{}", String::from_utf8(bytes).unwrap())
    }
}

impl From<&CpuidResult> for Vendor {
    fn from(cpuid: &CpuidResult) -> Self {
        Self {
            ebx: cpuid.ebx,
            ecx: cpuid.ecx,
            edx: cpuid.edx,
        }
    }
}

#[derive(Debug)]
pub enum CpuVendor {
    AuthenticAMD,
    GenuineIntel,
    CentaurHauls,
    Shanghai,
    Unknown,
}

impl From<&Vendor> for CpuVendor {
    fn from(vendor: &Vendor) -> Self {
        match vendor {
            &Vendor::REG_AMD => Self::AuthenticAMD,
            &Vendor::REG_INTEL => Self::GenuineIntel,
            &Vendor::REG_CENTAUR => Self::CentaurHauls,
            &Vendor::REG_SHANGHAI => Self::Shanghai,
            _ => Self::Unknown,
        }
    }
}

impl From<&CpuidResult> for CpuVendor {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from(&Vendor::from(cpuid))
    }
}

impl fmt::Display for CpuVendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CpuVendor {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x0, 0x0))
    }
}
