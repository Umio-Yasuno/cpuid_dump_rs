//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use crate::*;

#[path = "./_codename/codename_amd.rs"]
mod codename_amd;
pub use codename_amd::*;
#[path = "./_codename/codename_intel.rs"]
mod codename_intel;
pub use codename_intel::*;

pub struct ProcInfo {
    pub codename: String,
    pub archname: String,
    pub process: String,
}

impl ProcInfo {
    pub fn info(code: &str, arch: &str, process: &str) -> ProcInfo {
        ProcInfo {
            codename: code.to_string(),
            archname: arch.to_string(),
            process: process.to_string(),
        }
    }
}

pub struct FamModStep {
    pub syn_fam: u32,
    pub syn_mod: u32,
    pub step: u32,
    pub raw_eax: u32,
}

impl FamModStep {
    pub fn get() -> FamModStep {
        FamModStep::from_cpuid(&cpuid!(0x1, 0).eax)
    }
    pub fn dec(eax: u32) -> FamModStep {
         FamModStep {
            syn_fam: ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF),
            syn_mod: ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0),
            step: eax & 0xF,
            raw_eax: eax,
        }
    }
    pub fn from_cpuid(eax: &u32) -> FamModStep {
         FamModStep {
            syn_fam: ((*eax >> 8) & 0xF) + ((*eax >> 20) & 0xFF),
            syn_mod: ((*eax >> 4) & 0xF) + ((*eax >> 12) & 0xF0),
            step: *eax & 0xF,
            raw_eax: *eax,
        }
    }
    pub fn proc_info(&self) -> ProcInfo {
        let [f, m, s] = [self.syn_fam, self.syn_mod, self.step];

        return match f {
            0x5 => ProcInfo::info("Quark X1000", "P5C", "32 nm"),
            0x6 => fam06h(m, s),

            0x17 => fam17h(m, s),
            0x19 => fam19h(m, s),
            _ => ProcInfo {
                codename: format!("F{}h_M{}h_S{}h", f, m, s),
                archname: "Unknown".to_string(),
                process: "".to_string(),
            },
        };
    }
    pub fn codename(&self) -> String {
        self.proc_info().codename
    }
    pub fn archname(&self) -> String {
        self.proc_info().archname
    }
    pub fn process(&self) -> String {
        self.proc_info().process
    }
}
