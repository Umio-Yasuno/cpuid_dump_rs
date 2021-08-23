use crate::{ProcInfo, info};

pub fn fam17h(m: u32, s: u32) -> ProcInfo {
    match m {
        // Zen
        0x01 => match s {
            0x01 => info!("Summit Ridge",   "Zen",  "14 nm"),
            0x02 => info!("Naples",         "Zen",  "14 nm"),
            _    => info!("Unknow [Zen]",   "Zen",  ""),
        },
        0x20 => info!("Raven2 (Dali/Pollock)",  "Zen",  "14 nm"),

        // Zen
        0x08 => info!("Pinnacle Ridge",         "Zen+", "12 nm"),
        0x18 => info!("Picasso",                "Zen+", "12 nm"),

        // Zen 2
        0x31 => info!("Rome",       "Zen 2",    "7 nm"),
        0x60 => info!("Renoir",     "Zen 2",    "7 nm"),
        0x68 => info!("Lucienne",   "Zen 2",    "7 nm"),
        0x71 => info!("Matisse",    "Zen 2",    "7 nm"),
        0x90 => info!("VanGogh",    "Zen 2",    "7 nm"),
        
        _    => info!("Unknow [Zen/+/2]", "Zen/+/2", ""),
    }
}

pub fn fam19h(m: u32, _s: u32) -> ProcInfo {
    match m {
        0x01 => info!("Milan",      "Zen 3",    "7 nm"),
        0x21 => info!("Vermeer",    "Zen 3",    "7 nm"),
        0x50 => info!("Cezanne",    "Zen 3",    "7 nm"),

        _    => info!("Unknow [Zen 3]", "Zen 3", ""),
    }
}
