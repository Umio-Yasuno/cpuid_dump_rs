use crate::ProcInfo;

impl ProcInfo {
pub(super) fn fam17h(m: u32, s: u32) -> Option<Self> {
    Some(match m {
        /* Zen */
        0x01 => match s {
            0x01 => Self::info("Summit Ridge (B1)", "Zen", "14 nm"),
            0x02 => Self::info("Naples (B2)", "Zen", "14 nm"),
            _ => Self::info("Zen", "Zen", ""),
        },
        0x20 => Self::info(&["Raven2 [Dali/Pollock]", match s {
            0x1 => " (A1)",
            _ => "",
        }].concat(), "Zen", "14 nm"),

        /* Zen+ */
        0x08 => Self::info(&["Pinnacle Ridge", match s {
            0x2 => " (B2)",
            _ => "",
        }].concat(), "Zen+", "12 nm"),
        0x18 => Self::info(&["Picasso", match s {
            0x1 => " (B1)",
            _ => "",
        }].concat(), "Zen+", "12 nm"),

        /* Zen 2 */
        0x31 => Self::info("Rome", "Zen 2", "7 nm"),
        0x60 => Self::info(&["Renoir", match s {
            0x1 => " (A1)",
            _ => "",
        }].concat(), "Zen 2", "7 nm"),
        0x68 => Self::info("Lucienne", "Zen 2", "7 nm"),
        0x71 => Self::info("Matisse", "Zen 2", "7 nm"),
        0x90 => Self::info("VanGogh", "Zen 2", "7 nm"),
        0xA0..=0xAF => Self::info("Mendocino", "Zen 2", "6 nm"),

        _ => return None,
    })
}

pub(super) fn fam19h(m: u32, s: u32) -> Option<Self> {
    Some(match m {
        /* Zen 3 */
        0x01 => Self::info(&["Milan", match s {
            0x1 => " (B0)", // EPYC 7003
            0x2 => " (B1)", // EPYC 7003 3D V-Cache
            _ => "",
        }].concat(), "Zen 3", "7 nm"),
        0x08 => Self::info("Chagall", "Zen 3", "7 nm"),
        0x21 => Self::info("Vermeer", "Zen 3", "7 nm"),
        /* https://www.openmp.org/wp-content/uploads/ecp_sollve_openmp_monthly.offload_perf_ana_craypat.marcus.hpe_.26aug2022.v2.pdf */
        0x30 => Self::info("Trento", "Zen 3", "7 nm"),
        /* 0x44: Rembrandt */
        0x40..=0x4F => Self::info("Rembrandt", "Zen 3+", "6 nm"),
        0x50..=0x5F => Self::info("Cezanne", "Zen 3", "7 nm"),

        /* Zen 4 */
        0x60..=0x6F => Self::info("Phoenix", "Zen 4", ""),
        0x70..=0x7F => Self::info("Raphael", "Zen 4", "5 nm"),

        _ => return None,
    })
}
}
