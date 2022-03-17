use crate::{cpuid, CpuidResult};

pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub name: String,
}

impl<'a> Vendor {
    const AMD_EBX: u32 = 0x6874_7541;
    const AMD_ECX: u32 = 0x444D_4163;
    const AMD_EDX: u32 = 0x6974_6E65;
    const AMD_NAME: &'a str = "AuthenticAMD";

    const INTEL_EBX: u32 = 0x756E_6547;
    const INTEL_ECX: u32 = 0x4965_6E69;
    const INTEL_EDX: u32 = 0x6C65_746E;
    const INTEL_NAME: &'a str = "GenuineIntel";

    fn name_from_ebx(ebx: u32) -> String {
        match ebx {
            Self::AMD_EBX => Self::AMD_NAME,
            Self::INTEL_EBX => Self::INTEL_NAME,
            _ => "Unknown",
        }.to_string()
    }
    pub fn from_cpuid(cpuid: &CpuidResult) -> Vendor {
        Vendor {
            ebx: cpuid.ebx,
            ecx: cpuid.ecx,
            edx: cpuid.edx,
            name: Self::name_from_ebx(cpuid.ebx),
        }
    }
    pub fn get() -> Vendor {
        Vendor::from_cpuid(&cpuid!(0, 0))
    }
    pub fn amd() -> Vendor {
        Vendor {
            ebx: Self::AMD_EBX,
            ecx: Self::AMD_ECX,
            edx: Self::AMD_EDX,
            name: Self::AMD_NAME.to_string(),
        }
    }
    pub fn intel() -> Vendor {
        Vendor {
            ebx: Self::INTEL_EBX,
            ecx: Self::INTEL_ECX,
            edx: Self::INTEL_EDX,
            name: Self::INTEL_NAME.to_string(),
        }
    }
    pub fn check_amd(&self) -> bool {
        self.ebx == Self::AMD_EBX
    }
    pub fn check_intel(&self) -> bool {
        self.ebx == Self::INTEL_EBX
    }
}

pub struct VendorFlag {
    pub amd: bool,
    pub intel: bool,
}

impl VendorFlag {
    pub fn check() -> VendorFlag {
        let vendor = Vendor::get();
        let amd = vendor.check_amd();
        let intel = vendor.check_intel() && !amd;

        VendorFlag {
            amd,
            intel,
        }
    }
    pub fn all_true() -> VendorFlag {
        VendorFlag {
            amd: true,
            intel: true,
        }
    }
}

pub fn get_vendor_name() -> String {
    Vendor::get().name
}
