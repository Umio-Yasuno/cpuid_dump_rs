use crate::*;

const VENDOR_AMD_EBX: u32 = 0x6874_7541;
const VENDOR_AMD_ECX: u32 = 0x444D_4163;
const VENDOR_AMD_EDX: u32 = 0x6974_6E65;
const VENDOR_AMD_NAME: &str = "AuthenticAMD";

const VENDOR_INTEL_EBX: u32 = 0x756E_6547;
const VENDOR_INTEL_ECX: u32 = 0x4965_6E69;
const VENDOR_INTEL_EDX: u32 = 0x6C65_746E;
const VENDOR_INTEL_NAME: &str = "GenuineIntel";

pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub name: String,
}

impl Vendor {
    fn name_from_ebx(ebx: u32) -> String {
        match ebx {
            VENDOR_AMD_EBX => VENDOR_AMD_NAME,
            VENDOR_INTEL_EBX => VENDOR_INTEL_NAME,
            _ => "Unknown",
        }.to_string()
    }
    pub fn from_cpuid(cpuid: &CpuidResult) -> Vendor {
        Vendor {
            ebx: cpuid.ebx,
            ecx: cpuid.ecx,
            edx: cpuid.edx,
            name: Vendor::name_from_ebx(cpuid.ebx),
        }
    }
    pub fn get() -> Vendor {
        Vendor::from_cpuid(&cpuid!(0, 0))
    }
    pub fn amd() -> Vendor {
        Vendor {
            ebx: VENDOR_AMD_EBX,
            ecx: VENDOR_AMD_ECX,
            edx: VENDOR_AMD_EDX,
            name: VENDOR_AMD_NAME.to_string(),
        }
    }
    pub fn intel() -> Vendor {
        Vendor {
            ebx: VENDOR_INTEL_EBX,
            ecx: VENDOR_INTEL_ECX,
            edx: VENDOR_INTEL_EDX,
            name: VENDOR_INTEL_NAME.to_string(),
        }
    }
    pub fn check_amd(&self) -> bool {
        self.ebx == VENDOR_AMD_EBX
    }
    pub fn check_intel(&self) -> bool {
        self.ebx == VENDOR_INTEL_EBX
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
}

pub fn get_vendor_name() -> String {
    Vendor::get().name
}
