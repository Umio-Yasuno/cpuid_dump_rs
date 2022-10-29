use crate::{ProcInfo, ProcessNode};

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
        match self {
            Self::ZenPlus => write!(f, "Zen+"),
            Self::Zen2 => write!(f, "Zen 2"),
            Self::Zen3 => write!(f, "Zen 3"),
            Self::Zen3Plus => write!(f, "Zen 3+"),
            Self::Zen4 => write!(f, "Zen 4"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<AmdMicroArch> for String {
    fn from(s: AmdMicroArch) -> Self {
        s.to_string()
    }
}

impl ProcInfo {
pub(super) fn fam17h(m: u32, s: u32) -> Option<Self> {
    use AmdMicroArch as uarch;

    Some(match m {
        /* Zen */
        0x01 => match s {
            0x01 => Self::info("Summit Ridge (B1)", uarch::Zen, ProcessNode::NM(14)),
            0x02 => Self::info("Naples (B2)", uarch::Zen, ProcessNode::NM(14)),
            _ => Self::info("Zen", "Zen", ""),
        },
        0x11 => Self::info("Raven Ridge", uarch::Zen, ProcessNode::NM(14)),
        0x20 => Self::info(&["Raven2 [Dali/Pollock]", match s {
            0x1 => " (A1)",
            _ => "",
        }].concat(), uarch::Zen, ProcessNode::NM(14)),

        /* Zen+ */
        0x08 => Self::info(&["Pinnacle Ridge", match s {
            0x2 => " (B2)",
            _ => "",
        }].concat(), uarch::ZenPlus, ProcessNode::NM(12)),
        0x18 => Self::info(&["Picasso", match s {
            0x1 => " (B1)",
            _ => "",
        }].concat(), uarch::ZenPlus, ProcessNode::NM(12)),

        /* Zen 2 */
        0x31 => Self::info("Rome", "Zen 2", ProcessNode::NM(7)),
        0x60 => Self::info(&["Renoir", match s {
            0x1 => " (A1)",
            _ => "",
        }].concat(), uarch::Zen2, ProcessNode::NM(7)),
        0x68 => Self::info("Lucienne", uarch::Zen2, ProcessNode::NM(7)),
        0x71 => Self::info("Matisse", uarch::Zen2, ProcessNode::NM(7)),
        0x90 => Self::info("VanGogh", uarch::Zen2, ProcessNode::NM(7)),
        0xA0..=0xAF => Self::info("Mendocino", uarch::Zen2, ProcessNode::NM(6)),

        _ => return None,
    })
}

pub(super) fn fam19h(m: u32, s: u32) -> Option<Self> {
    use AmdMicroArch as uarch;

    Some(match m {
        /* Zen 3 */
        /* Revision Guide for AMD Family 19h Models 00h-0Fh Processors: https://www.amd.com/system/files/TechDocs/56683-PUB-1.07.pdf */
        0x01 => Self::info(&["Milan", match s {
            0x1 => " (B1)", // EPYC 7003
            0x2 => " (B2)", // EPYC 7003 3D V-Cache
            _ => "",
        }].concat(), uarch::Zen3, ProcessNode::NM(7)),
        0x08 => Self::info("Chagall", uarch::Zen3, ProcessNode::NM(7)),
        0x21 => Self::info("Vermeer", uarch::Zen3, ProcessNode::NM(7)),
        /* https://www.openmp.org/wp-content/uploads/ecp_sollve_openmp_monthly.offload_perf_ana_craypat.marcus.hpe_.26aug2022.v2.pdf */
        0x30 => Self::info("Trento", uarch::Zen3, ProcessNode::NM(7)),
        /* 0x44: Rembrandt */
        0x40..=0x4F => Self::info("Rembrandt", uarch::Zen3Plus, ProcessNode::NM(6)),
        0x50..=0x5F => Self::info("Cezanne/Barcelo", uarch::Zen3, ProcessNode::NM(7)),

        /* Zen 4 */
        0x60..=0x6F => Self::info("Phoenix", uarch::Zen4, ""),
        0x70..=0x7F => Self::info("Raphael", uarch::Zen4, ProcessNode::NM(5)),

        _ => return None,
    })
}
}
