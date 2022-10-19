use crate::cpuid;

pub struct ProcInfo {
    pub codename: String,
    pub archname: String,
    pub process: String,
}

impl ProcInfo {
    pub fn from_fms(fms: &FamModStep) -> Option<Self> {
        let [f, m, s] = [fms.syn_fam, fms.syn_mod, fms.step];

        match f {
            /* Intel */
            0x5 => Some(ProcInfo::info("Quark X1000", "P5C", "32 nm")),
            0x6 => ProcInfo::fam06h(m, s),

            /* AMD */
            0x17 => ProcInfo::fam17h(m, s),
            0x19 => ProcInfo::fam19h(m, s),
            _ => None,
        }
    }
    pub fn info(code: &str, arch: &str, process: &str) -> Self {
        Self {
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
    pub fn from_cpuid(eax: u32) -> Self {
         Self {
            syn_fam: ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF),
            syn_mod: ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0),
            step: eax & 0xF,
            raw_eax: eax,
        }
    }
    
    pub fn get() -> Self {
        Self::from_cpuid(cpuid!(0x1).eax)
    }
}
