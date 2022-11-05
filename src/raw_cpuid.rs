use crate::{CpuidResult, CpuVendor};
use super::*;

#[derive(Debug, PartialEq, Eq)]
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

    fn parse(&self, vendor: &CpuVendor) -> String {
        match self.leaf {
            0x0 => format!("[{vendor}]"),
            0x1 => [
                self.result.info_00_01h(vendor),
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
            0x8000_0001 => [
                if let CpuVendor::AuthenticAMD = vendor {
                    [self.result.pkgtype_amd_80_01h(), lnpad!()].concat()
                } else {
                    "".to_string()
                },
                self.result.feature_80_01h(),
            ].concat(),
            0x8000_0002..=0x8000_0004 => format!("[\"{}\"]", self.result.cpu_name()),
            0x8000_0008 => [
                self.result.addr_size_80_08h(),
                lnpad!(),
                self.result.ftr_ext_id_80_08h_ebx(),
                if let CpuVendor::AuthenticAMD = vendor {
                    [lnpad!(), self.result.size_id_amd_80_08h()].concat()
                } else {
                    "".to_string()
                },
            ].concat(),
            _ => match vendor {
                CpuVendor::AuthenticAMD => {
                    match self.leaf {
                        0x8000_0005 => self.result.l1_amd_80_05h(),
                        0x8000_0006 => self.result.l2_amd_80_06h(),
                        0x8000_0007 => self.result.apmi_amd_80_07h(),
                        0x8000_000A => [
                            self.result.svm_rev_amd_80_0ah_eax_ebx(),
                            lnpad!(),
                            self.result.svm_ftr_amd_80_0ah_edx()
                        ].concat(),
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
                },
                CpuVendor::GenuineIntel => {
                    match self.leaf {
                        0x4 => self.result.cache_prop(),
                        0x16 => self.result.clock_speed_intel_00_16h(),
                        0x1A => self.result.intel_hybrid_1ah(),
                        0x1F => self.result.v2_ext_topo_intel_1fh(),
                        _ => "".to_string(),
                    }
                },
                _ => "".to_string(),
            }
        }
    }

    fn result(&self, end_str: &str) -> String {
        format!(
            "  {:#010X} {:#3X}:  {:#010X} {:#010X} {:#010X} {:#010X}  {}\n",
            self.leaf,
            self.sub_leaf,
            self.result.eax,
            self.result.ebx,
            self.result.ecx,
            self.result.edx,
            end_str,
        )
    }

    pub fn raw_fmt(&self, _: &CpuVendor) -> String {
        self.result("")
    }

    pub fn parse_fmt(&self, vendor: &CpuVendor) -> String {
        let parsed = self.parse(vendor);

        self.result(&parsed)
    }

    pub fn bin_fmt(&self, _: &CpuVendor) -> String {
        let separate = |reg: u32| -> String {
            let tmp = format!("{reg:032b}");

            format!(
                "{}_{}_{}_{}",
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

        const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; 18]) };

        format!("  {leaf:#010X} {sub_leaf:#03X}:  {eax}  {ebx} \n{PAD} {ecx}  {edx} \n")
    }

    pub fn compat_fmt(&self, _: &CpuVendor) -> String {
        let [leaf, sub_leaf] = [self.leaf, self.sub_leaf];
        let CpuidResult { eax, ebx, ecx, edx } = self.result;

        format!("   {leaf:#010x} {sub_leaf:#04x}: eax={eax:#010x} ebx={ebx:#010x} ecx={ecx:#010x} edx={edx:#010x}\n")
    }
}
