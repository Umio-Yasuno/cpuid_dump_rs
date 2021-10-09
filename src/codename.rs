//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

#[path = "./_codename/codename_amd.rs"]
    mod codename_amd;   use codename_amd::*;
#[path = "./_codename/codename_intel.rs"]
    mod codename_intel; use codename_intel::*;

pub struct ProcInfo {
    pub codename:   String,
    pub archname:   String,
    pub process:    String,
}

#[macro_export]
macro_rules! info {
    ($codename: expr, $arch: expr, $process: expr) => {
        ProcInfo {
            codename:   $codename.to_string(),
            archname:   $arch.to_string(),
            process:    $process.to_string(),
        }
    }
}

//  f: Family, m: Model, s: Stepping
//  pub fn get_codename(f: u32, m: u32, s: u32) -> String {
pub fn get_codename(f: u32, m: u32, s: u32) -> ProcInfo {

    match f {
        _ => format!("Unknown"),
    };
    return match f {
        0x5 => info!("Quark X1000", "P5C", "32 nm"),
        0x6 => fam06h(m, s),

        0x17 => fam17h(m, s),
        0x19 => fam19h(m, s),
        _ => ProcInfo {
                codename: format!("F{}h_M{}h_S{}h", f, m, s),
                archname: "Unknown".to_string(),
                process:  "".to_string(),
            },
    };
}

