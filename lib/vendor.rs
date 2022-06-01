use crate::{cpuid, CpuidResult};

pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub name: String,
}

impl Vendor {
    const AMD_EBX: u32 = 0x6874_7541;
    const AMD_ECX: u32 = 0x444D_4163;
    const AMD_EDX: u32 = 0x6974_6E65;
    const AMD_NAME_BYTE: [u8; 12] = *b"AuthenticAMD";

    const INTEL_EBX: u32 = 0x756E_6547;
    const INTEL_ECX: u32 = 0x4965_6E69;
    const INTEL_EDX: u32 = 0x6C65_746E;
    const INTEL_NAME_BYTE: [u8; 12] = *b"GenuineIntel";

    fn name_to_string(byte: &[u8]) -> String {
        String::from_utf8(byte.to_vec()).unwrap()
    }
    fn name_from_ebx(ebx: u32) -> String {
        Self::name_to_string(
            match ebx {
                Self::AMD_EBX => &Self::AMD_NAME_BYTE,
                Self::INTEL_EBX => &Self::INTEL_NAME_BYTE,
                _ => b"Unknown",
            }
        )
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
        Self::from_cpuid(&cpuid!(0x0, 0x0))
    }
    pub fn get_name() -> String {
        Self::get().name
    }
    pub fn amd() -> Vendor {
        Vendor {
            ebx: Self::AMD_EBX,
            ecx: Self::AMD_ECX,
            edx: Self::AMD_EDX,
            name: Self::name_to_string(&Self::AMD_NAME_BYTE),
        }
    }
    pub fn intel() -> Vendor {
        Vendor {
            ebx: Self::INTEL_EBX,
            ecx: Self::INTEL_ECX,
            edx: Self::INTEL_EDX,
            name: Self::name_to_string(&Self::INTEL_NAME_BYTE),
        }
    }
    pub fn check_amd(&self) -> bool {
        self.ebx == Self::AMD_EBX
    }
    pub fn check_intel(&self) -> bool {
        self.ebx == Self::INTEL_EBX
    }
    pub fn reg_to_name(ebx: u32, edx: u32, ecx: u32) -> String {
        let dec = |reg: u32| -> String {
            let tmp = reg.to_le_bytes().iter().map(|&byte|
                if char::from(byte).is_control() { 0x20 } else { byte }
            ).collect::<Vec<u8>>();

            String::from_utf8(tmp).unwrap()
        };
        let [ebx, edx, ecx] = [
            ebx,
            edx,
            ecx,
        ].map(|reg| dec(reg) );

        format!("{ebx}{edx}{ecx}")
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

/*
#[test]
fn test_vendor_name() {
    println!("Vendor Name: [{}]", Vendor::get_name());
}
*/
