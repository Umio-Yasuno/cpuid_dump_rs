use crate::ProcInfo;

#[derive(Debug)]
pub(self) enum AmdMicroArch {
    Zen,
    ZenPlus,
    Zen2,
    Zen3,
    Zen3Plus,
    Zen4,
    _Reserved,
}

use std::fmt;
impl fmt::Display for AmdMicroArch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<AmdMicroArch> for String {
    fn from(s: AmdMicroArch) -> Self {
        s.to_string()
    }
}

impl ProcInfo {
pub(super) fn fam17h(m: u32, s: u32) -> Option<Self> {
    Some(match m {
        /* Zen */
        0x01 => match s {
            0x01 => Self::info("Summit Ridge (B1)", AmdMicroArch::Zen, "14 nm"),
            0x02 => Self::info("Naples (B2)", AmdMicroArch::Zen, "14 nm"),
            _ => Self::info("Zen", "Zen", ""),
        },
        0x11 => Self::info("Raven Ridge", AmdMicroArch::Zen, "14 nm"),
        0x20 => Self::info(&["Raven2 [Dali/Pollock]", match s {
            0x1 => " (A1)",
            _ => "",
        }].concat(), AmdMicroArch::Zen, "14 nm"),

        /* Zen+ */
        0x08 => Self::info(&["Pinnacle Ridge", match s {
            0x2 => " (B2)",
            _ => "",
        }].concat(), AmdMicroArch::ZenPlus, "12 nm"),
        0x18 => Self::info(&["Picasso", match s {
            0x1 => " (B1)",
            _ => "",
        }].concat(), AmdMicroArch::ZenPlus, "12 nm"),

        /* Zen 2 */
        0x31 => Self::info("Rome", "Zen 2", "7 nm"),
        0x60 => Self::info(&["Renoir", match s {
            0x1 => " (A1)",
            _ => "",
        }].concat(), AmdMicroArch::Zen2, "7 nm"),
        0x68 => Self::info("Lucienne", AmdMicroArch::Zen2, "7 nm"),
        0x71 => Self::info("Matisse", AmdMicroArch::Zen2, "7 nm"),
        0x90 => Self::info("VanGogh", AmdMicroArch::Zen2, "7 nm"),
        0xA0..=0xAF => Self::info("Mendocino", AmdMicroArch::Zen2, "6 nm"),

        _ => return None,
    })
}

pub(super) fn fam19h(m: u32, s: u32) -> Option<Self> {
    Some(match m {
        /* Zen 3 */
        /* Revision Guide for AMD Family 19h Models 00h-0Fh Processors: https://www.amd.com/system/files/TechDocs/56683-PUB-1.07.pdf */
        0x01 => Self::info(&["Milan", match s {
            0x1 => " (B1)", // EPYC 7003
            0x2 => " (B2)", // EPYC 7003 3D V-Cache
            _ => "",
        }].concat(), AmdMicroArch::Zen3, "7 nm"),
        0x08 => Self::info("Chagall", AmdMicroArch::Zen3, "7 nm"),
        0x21 => Self::info("Vermeer", AmdMicroArch::Zen3, "7 nm"),
        /* https://www.openmp.org/wp-content/uploads/ecp_sollve_openmp_monthly.offload_perf_ana_craypat.marcus.hpe_.26aug2022.v2.pdf */
        0x30 => Self::info("Trento", AmdMicroArch::Zen3, "7 nm"),
        /* 0x44: Rembrandt */
        0x40..=0x4F => Self::info("Rembrandt", AmdMicroArch::Zen3Plus, "6 nm"),
        0x50..=0x5F => Self::info("Cezanne", AmdMicroArch::Zen3, "7 nm"),

        /* Zen 4 */
        0x60..=0x6F => Self::info("Phoenix", AmdMicroArch::Zen4, ""),
        0x70..=0x7F => Self::info("Raphael", AmdMicroArch::Zen4, "5 nm"),

        _ => return None,
    })
}
}
