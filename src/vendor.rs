use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

macro_rules! amd { () => {
    Vendor {
        ebx: 0x6874_7541,
        ecx: 0x444D_4163,
        edx: 0x6974_6E65,
    }
}}

macro_rules! intel { () => {
    Vendor {
        ebx: 0x756E_6547,
        ecx: 0x4965_6E69,
        edx: 0x6C65_746E,
    }
}}

impl Vendor {
    pub fn from_cpuid(cpuid: &CpuidResult) -> Vendor {
        Vendor {
            ebx: cpuid.ebx,
            ecx: cpuid.ecx,
            edx: cpuid.edx,
        }
    }
    pub fn get() -> Vendor {
        Vendor::from_cpuid(&cpuid!(0, 0))
    }
    pub fn amd() -> Vendor {
        amd!()
    }
    pub fn intel() -> Vendor {
        intel!()
    }
    pub fn check_amd(&self) -> bool {
        *self == amd!()
    }
    pub fn check_intel(&self) -> bool {
        *self == intel!()
    }
    pub fn name(&self) -> String {
        match *self {
            amd!() => "AuthenticAMD",
            intel!() => "GenuineIntel",
            _ => "Unknown",
        }.to_string()
    }
}

pub fn get_vendor_name() -> String {
    Vendor::get().name()
}

pub struct VendorFlag {
    pub amd: bool,
    pub intel: bool,
}

impl VendorFlag {
    pub fn check() -> VendorFlag {
        let vendor = Vendor::get();
        let amd = vendor.check_amd();
        let intel = vendor.check_intel();

        VendorFlag {
            amd,
            intel,
        }
    }
}
