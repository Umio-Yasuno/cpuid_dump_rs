use crate::cpuid;

pub struct ProcInfo {
    pub codename: String,
    pub archname: String,
    pub process: String,
}

impl ProcInfo {
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
    pub fn get() -> Self {
        Self::from_cpuid(cpuid!(0x1, 0).eax)
    }
    pub fn from_cpuid(eax: u32) -> Self {
         Self {
            syn_fam: ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF),
            syn_mod: ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0),
            step: eax & 0xF,
            raw_eax: eax,
        }
    }
    pub fn rev_dec(f: u32, m: u32, e_f: u32, e_m: u32, step: u32) -> u32 {
        let mut cpuid = 0u32;
        cpuid |= step;
        cpuid |= f << 8;
        cpuid |= m << 4;
        cpuid |= e_f << 20;
        cpuid |= e_m << 16;

        return cpuid;
    }
    pub fn proc_info(&self) -> ProcInfo {
        let [f, m, s] = [self.syn_fam, self.syn_mod, self.step];

        match f {
            0x5 => ProcInfo::info("Quark X1000", "P5C", "32 nm"),
            0x6 => ProcInfo::fam06h(m, s),

            0x17 => ProcInfo::fam17h(m, s),
            0x19 => ProcInfo::fam19h(m, s),
            _ => ProcInfo {
                codename: format!("F{}h_M{}h_S{}h", f, m, s),
                archname: "NoIndex".to_string(),
                process: "".to_string(),
            },
        }
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

#[test]
fn test_rev() {
    // FamModStep::rev_dec(0x7, 0xB, 0x0, 0x3, 0x0);
    let cpuid = FamModStep::rev_dec(0xF, 0x0, 0xA, 0x5, 0x0);
    assert_eq!(0x00A50F00, cpuid);
}
