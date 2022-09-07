use crate::*;

#[derive(Debug, PartialEq)]
pub struct RawCpuid {
    pub leaf: u32,
    pub sub_leaf: u32,
    pub result: CpuidResult,
}

impl RawCpuid {
    pub fn exe(leaf: u32, sub_leaf: u32) -> Self {
        Self {
            leaf,
            sub_leaf,
            result: cpuid!(leaf, sub_leaf),
        }
    }

    pub fn check_result_zero(&self) -> bool {
        self.result == CpuidResult { eax: 0x0, ebx: 0x0, ecx: 0x0, edx: 0x0 }
    }

    fn parse(&self, vendor: &VendorFlag) -> String {
        let parse_result = match self.leaf {
            0x0 => self.result.vendor_00_00h(),
            0x1 => [
                self.result.info_00_01h(),
                lnpad!(),
                self.result.feature_00_01h(),
            ].concat(),
            0x5 => self.result.monitor_mwait_00_05h(),
            0x6 => self.result.thermal_power_00_06h(),
            0x7 => match self.sub_leaf {
                0x0 => self.result.feature_00_07h_x0(),
                0x1 => self.result.feature_00_07h_x1(),
                _ => "".to_string(),
            },
            0xB => self.result.topo_ext_00_0bh(),
            0xD => self.result.xstate_00_0dh(self.sub_leaf),
            0x1F => if vendor.intel {
                self.result.v2_ext_topo_intel_1fh()
            } else {
                "".to_string()
            },
            0x8000_0001 => [
                if vendor.amd {
                    [
                        self.result.pkgtype_amd_80_01h(),
                        lnpad!(),
                    ].concat()
                } else {
                    "".to_string()
                },
                self.result.feature_80_01h(),
            ].concat(),
            0x8000_0002..=0x8000_0004 => format!(" [\"{}\"]", self.result.cpu_name()),
            0x8000_0008 => [
                self.result.addr_size_80_08h(),
                if vendor.amd {
                    [
                        lnpad!(),
                        self.result.spec_amd_80_08h(),
                        lnpad!(),
                        self.result.size_amd_80_08h(),
                    ].concat()
                } else {
                    "".to_string()
                },
            ].concat(),
            _ => if vendor.amd {
                match self.leaf {
                    0x8000_0005 => self.result.l1_amd_80_05h(),
                    0x8000_0006 => self.result.l2_amd_80_06h(),
                    0x8000_0007 => self.result.apmi_amd_80_07h(),
                    0x8000_000A => self.result.rev_id_amd_80_0ah(),
                    0x8000_0019 => self.result.l1l2tlb_1g_amd_80_19h(),
                    0x8000_001A => self.result.fpu_width_amd_80_1ah(),
                    0x8000_001B => self.result.ibs_amd_80_1bh(),
                    0x8000_001D => self.result.cache_prop(),
                    0x8000_001E => self.result.cpu_topo_amd_80_1eh(),
                    0x8000_001F => [
                        self.result.encrypt_ftr_amd_80_1fh(),
                        self.result.reduction_phys_addr_amd_80_1fh(),
                    ].concat(),
                    0x8000_0021 => self.result.ext_amd_80_21h(),
                    _ => "".to_string(),
                }
            } else if vendor.intel {
                match self.leaf {
                    0x4 => self.result.cache_prop(),
                    0x16 => self.result.clock_speed_intel_00_16h(),
                    0x1A => self.result.intel_hybrid_1ah(),
                    _ => "".to_string(),
                }
            } else {
                "".to_string()
            },
        };

        parse_result + "\n"
    }

    fn result(&self, end_str: &str) -> String {
        format!("  0x{:08X}_x{:1X}:  0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X} {}",
            self.leaf,
            self.sub_leaf,
            self.result.eax,
            self.result.ebx,
            self.result.ecx,
            self.result.edx,
            end_str,
        )
    }

    pub fn raw_fmt(&self) -> String {
        self.result("\n")
    }

    pub fn parse_fmt(&self, vendor: &VendorFlag) -> String {
        let parsed = self.parse(vendor);

        if parsed.is_empty() {
            return parsed;
        }

        self.result(&parsed)
    }

    pub fn bin_fmt(&self) -> String {
        let separate = |reg: u32| -> String {
            let tmp = format!("{:032b}", reg);

            format!("{}_{}_{}_{}",
                &tmp[..8],
                &tmp[8..16],
                &tmp[16..24],
                &tmp[24..32],
            )
        };

        let [leaf, sub_leaf] = [self.leaf, self.sub_leaf];
        let [eax, ebx, ecx, edx] = [
            self.result.eax,
            self.result.ebx,
            self.result.ecx,
            self.result.edx,
        ].map(separate);

        const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; 17]) };

        format!("  0x{leaf:08X}_x{sub_leaf:1X}:  {eax}  {ebx} \n{PAD} {ecx}  {edx} \n")
    }
}
