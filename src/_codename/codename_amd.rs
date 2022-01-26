use crate::*;

pub fn fam17h(m: u32, s: u32) -> ProcInfo {
    match m {
        // Zen
        0x01 => match s {
            0x01 => ProcInfo::info("Summit Ridge", "Zen", "14 nm"),
            0x02 => ProcInfo::info("Naples", "Zen", "14 nm"),
            _ => ProcInfo::info("Unknow [Zen]", "Zen", ""),
        },
        0x20 => ProcInfo::info("Raven2 (Dali/Pollock)", "Zen", "14 nm"),

        // Zen
        0x08 => ProcInfo::info("Pinnacle Ridge", "Zen+", "12 nm"),
        0x18 => ProcInfo::info("Picasso", "Zen+", "12 nm"),

        // Zen 2
        0x31 => ProcInfo::info("Rome", "Zen 2", "7 nm"),
        0x60 => ProcInfo::info("Renoir", "Zen 2", "7 nm"),
        0x68 => ProcInfo::info("Lucienne", "Zen 2", "7 nm"),
        0x71 => ProcInfo::info("Matisse", "Zen 2", "7 nm"),
        0x90 => ProcInfo::info("VanGogh", "Zen 2", "7 nm"),

        _ => ProcInfo::info("Unknow [Zen/+/2]", "Zen/+/2", ""),
    }
}

pub fn fam19h(m: u32, _s: u32) -> ProcInfo {
    match m {
        0x01 => ProcInfo::info("Milan", "Zen 3", "7 nm"),
        0x21 => ProcInfo::info("Vermeer", "Zen 3", "7 nm"),
        0x50 => ProcInfo::info("Cezanne", "Zen 3", "7 nm"),

        _ => ProcInfo::info("Unknow [Zen 3]", "Zen 3", ""),
    }
}
