use crate::*;

pub struct RawCpuid {
    pub leaf: u32,      // in_eax
    pub sub_leaf: u32,  // in_ecx
    pub result: CpuidResult,
}

impl RawCpuid {
    pub fn exe(leaf: u32, sub_leaf: u32) -> RawCpuid {
        RawCpuid {
            leaf,
            sub_leaf,
            result: cpuid!(leaf, sub_leaf),
        }
    }
    pub fn check_result_zero(&self) -> bool {
        let cpuid = &self.result;
        (cpuid.eax == 0) && (cpuid.ebx == 0) && (cpuid.ecx == 0) && (cpuid.edx == 0)
    }
    pub fn parse(&self, vendor: &VendorFlag) -> String {
        let parse_result: String = match self.leaf {
            0x0 => format!(" [{}]", Vendor::from_cpuid(&self.result).name),
            0x1 => concat_string_from_slice(&[
                info_00_01h(&self.result),
                padln!().to_string(),
                feature_00_01h(&self.result),
            ]),
            0x7 => match self.sub_leaf {
                0x0 => feature_00_07h_x0(&self.result),
                0x1 => feature_00_07h_x1(&self.result.eax),
                _ => unreachable!(),
            },
            0xD => enum_amd_0dh(&self),
            0x1F => if vendor.intel {
                v2_ext_topo_intel_1fh(&self.result)
            } else {
                "".to_string()
            },
            0x8000_0001 => {
                let mut v: Vec<String> = Vec::new();
                if vendor.amd {
                    v.push(pkgtype_amd_80_01h(&self.result.ebx));
                    v.push(padln!().to_string());
                }
                v.push(feature_80_01h(&self.result));
                concat_string_from_slice(&v)
            },
            0x8000_0002..=0x8000_0004 => format!(" [{}]", cpu_name(&self.result)),
            _ => if vendor.amd {
                match self.leaf {
                    0x8000_0005 => l1_amd_80_05h(&self.result),
                    0x8000_0006 => l2_amd_80_06h(&self.result),
                    0x8000_0007 => apmi_amd_80_07h(&self.result.edx),
                    0x8000_0008 => concat_string_from_slice(&[
                        spec_amd_80_08h(&self.result.ebx),
                        padln!().to_string(),
                        size_amd_80_08h(&self.result.ecx),
                    ]),
                    0x8000_000A => rev_id_amd_80_0ah(&self.result),
                    0x8000_0019 => l1l2tlb_1g_amd_80_19h(&self.result),
                    0x8000_001A => fpu_width_amd_80_1ah(&self.result.eax),
                    0x8000_001B => ibs_amd_80_1bh(&self.result.eax),
                    0x8000_001D => cache_prop(&self.result),
                    0x8000_001E => cpu_topo_amd_80_1eh(&self.result),
                    0x8000_001F => secure_amd_80_1fh(&self.result.eax),
                    _ => "".to_string(),
                }
            } else if vendor.intel {
                match self.leaf {
                    0x4 => cache_prop(&self.result),
                    0x16 => clock_speed_intel_00_16h(&self.result),
                    0x1A => intel_hybrid_1ah(&self.result.eax),
                    _ => "".to_string(),
                }
            } else {
                "".to_string()
            },
        };

        return parse_result + "\n";
    }
    pub fn result(&self, end_str: &str) -> String {
        format!("  0x{:08X}_x{:1X}:  0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X} {}",
            self.leaf, self.sub_leaf,
            self.result.eax, self.result.ebx, self.result.ecx, self.result.edx,
            end_str,
        )
    }
    pub fn raw_fmt(&self) -> String {
        self.result("\n")
    }
    pub fn parse_fmt(&self, vendor: &VendorFlag) -> String {
        self.result(&self.parse(&vendor))
    }
}
