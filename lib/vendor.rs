use crate::{cpuid, CpuidResult};

#[derive(PartialEq, Eq)]
pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl Vendor {
    const AMD_EBX: u32 = 0x6874_7541;
    const AMD_ECX: u32 = 0x444D_4163;
    const AMD_EDX: u32 = 0x6974_6E65;
    const REG_AMD: Vendor = Vendor {
        ebx: Self::AMD_EBX,
        ecx: Self::AMD_ECX,
        edx: Self::AMD_EDX,
    };
    // const AMD_NAME_BYTE: [u8; 12] = *b"AuthenticAMD";

    const INTEL_EBX: u32 = 0x756E_6547;
    const INTEL_ECX: u32 = 0x4965_6E69;
    const INTEL_EDX: u32 = 0x6C65_746E;
    const REG_INTEL: Vendor = Vendor {
        ebx: Self::INTEL_EBX,
        ecx: Self::INTEL_ECX,
        edx: Self::INTEL_EDX,
    };
    // const INTEL_NAME_BYTE: [u8; 12] = *b"GenuineIntel";

    pub fn from_cpuid(cpuid: &CpuidResult) -> Vendor {
        Vendor {
            ebx: cpuid.ebx,
            ecx: cpuid.ecx,
            edx: cpuid.edx,
        }
    }
    pub fn get() -> Vendor {
        Self::from_cpuid(&cpuid!(0x0, 0x0))
    }
    pub fn get_name(&self) -> String {
        let dec = |reg: u32| -> String {
            let tmp = reg.to_le_bytes().iter().map(|&byte|
                if char::from(byte).is_control() { 0x20 } else { byte }
            ).collect::<Vec<u8>>();

            String::from_utf8(tmp).unwrap()
        };

        [
            self.ebx,
            self.edx,
            self.ecx,
        ]
        .map(dec)
        .concat()
    }
    fn check_amd(&self) -> bool {
        self == &Self::REG_AMD
    }
    fn check_intel(&self) -> bool {
        self == &Self::REG_INTEL
    }
}

pub struct VendorFlag {
    pub amd: bool,
    pub intel: bool,
}

impl VendorFlag {
    pub fn check() -> Self {
        let vendor = Vendor::get();
        let amd = vendor.check_amd();
        let intel = vendor.check_intel();

        Self {
            amd,
            intel,
        }
    }

    pub fn all_true() -> Self {
        Self {
            amd: true,
            intel: true,
        }
    }
}
