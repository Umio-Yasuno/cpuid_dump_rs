use crate::ProcInfo;

impl ProcInfo {
pub fn fam17h(m: u32, s: u32) -> Self {
    match m {
        // Zen
        0x01 => match s {
            0x01 => Self::info("Summit Ridge", "Zen", "14 nm"),
            0x02 => Self::info("Naples", "Zen", "14 nm"),
            _ => Self::info("{{Zen}}", "Zen", ""),
        },
        0x20 => Self::info("Raven2 (Dali/Pollock)", "Zen", "14 nm"),

        // Zen
        0x08 => Self::info("Pinnacle Ridge", "Zen+", "12 nm"),
        0x18 => Self::info("Picasso", "Zen+", "12 nm"),

        // Zen 2
        0x31 => Self::info("Rome", "Zen 2", "7 nm"),
        0x60 => Self::info("Renoir", "Zen 2", "7 nm"),
        0x68 => Self::info("Lucienne", "Zen 2", "7 nm"),
        0x71 => Self::info("Matisse", "Zen 2", "7 nm"),
        0x90 => Self::info("VanGogh", "Zen 2", "7 nm"),

        _ => Self::info("{{Zen/+/2}}", "Zen/+/2", ""),
    }
}

pub fn fam19h(m: u32, _s: u32) -> Self {
    match m {
        0x01 => Self::info("Milan", "Zen 3", "7 nm"),
        0x21 => Self::info("Vermeer", "Zen 3", "7 nm"),
        0x50 => Self::info("Cezanne", "Zen 3", "7 nm"),

        _ => Self::info("{{Zen 3}}", "Zen 3", ""),
    }
}
}
