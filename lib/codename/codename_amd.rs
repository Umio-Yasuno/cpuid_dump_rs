use crate::ProcInfo;

impl ProcInfo {
pub fn fam17h(m: u32, s: u32) -> Self {
    match m {
        // Zen
        0x01 => match s {
            0x01 => Self::info("Summit Ridge (B1)", "Zen", "14 nm"),
            0x02 => Self::info("Naples (B2)", "Zen", "14 nm"),
            _ => Self::info("{{Zen}}", "Zen", ""),
        },
        0x20 => Self::info(&format!("Raven2 [Dali/Pollock]{}", match s {
            0x1 => " (A1)",
            _ => "",
        }), "Zen", "14 nm"),

        // Zen+
        0x08 => Self::info(&format!("Pinnacle Ridge{}", match s {
            0x2 => " (B2)",
            _ => "",
        }), "Zen+", "12 nm"),
        0x18 => Self::info(&format!("Picasso{}", match s {
            0x1 => " (B1)",
            _ => "",
        }), "Zen+", "12 nm"),

        // Zen 2
        0x31 => Self::info("Rome", "Zen 2", "7 nm"),
        0x60 => Self::info(&format!("Renoir{}", match s {
            0x1 => " (A1)",
            _ => "",
        }), "Zen 2", "7 nm"),
        0x68 => Self::info("Lucienne", "Zen 2", "7 nm"),
        0x71 => Self::info("Matisse", "Zen 2", "7 nm"),
        0x90 => Self::info("VanGogh", "Zen 2", "7 nm"),

        _ => Self::info("{{Zen/+/2}}", "Zen/+/2", ""),
    }
}

pub fn fam19h(m: u32, s: u32) -> Self {
    match m {
        0x01 => Self::info(&format!("Milan{}", match s {
            0x1 => " (B0)", // EPYC 7003
            0x2 => " (B1)", // EPYC 7003 3D V-Cache
            _ => "",
        }), "Zen 3", "7 nm"),
        0x21 => Self::info("Vermeer", "Zen 3", "7 nm"),
        0x50 => Self::info("Cezanne (A1)", "Zen 3", "7 nm"),

        _ => Self::info("{{Zen 3}}", "Zen 3", ""),
    }
}
}
