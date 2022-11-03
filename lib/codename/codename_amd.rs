use crate::{ProcInfo, ProcessNode};
/* ref: https://github.com/illumos/illumos-gate/blob/master/usr/src/uts/intel/os/cpuid_subr.c */
/* ref: https://en.wikipedia.org/wiki/List_of_AMD_CPU_microarchitectures */
/* ref: https://developer.amd.com/resources/developer-guides-manuals/ */

#[derive(Debug)]
pub(self) enum AmdMicroArch {
    Puma2008,
    K10,
    Bobcat,
    Bulldozer,
    Piledriver,
    Steamroller,
    Excavator,
    Jaguar,
    Puma2014,
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
    pub(super) fn amd_fam10h(m: u32, s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        /* https://www.amd.com/system/files/TechDocs/41322_10h_Rev_Gd.pdf */
        /* https://www.amd.com/system/files/TechDocs/43374.pdf */
        Some(match m {
            0x02 => Self::info(
                match s {
                    0x1 => "DR-B1",
                    0x2 => "DR-B2",
                    0x3 => "DR-B3",
                    0xA => "DR-BA",
                    _=> "",
                },
                uarch::K10,
                ProcessNode::NM(65),
            ),
            0x04 => Self::info(
                match s {
                    0x2 => "RB-C2",
                    0x3 => "RB-C3",
                    _ => "",
                },
                uarch::K10,
                ProcessNode::NM(45),
            ),
            0x05 => Self::info(
                match s {
                    0x2 => "BL-C2",
                    0x3 => "BL-C3",
                    _ => "",
                },
                uarch::K10,
                ProcessNode::NM(45),
            ),
            0x06 => Self::info(
                match s {
                    0x2 => "DA-C2",
                    0x3 => "DA-C3",
                    _ => "",
                },
                uarch::K10,
                ProcessNode::NM(45),
            ),
            0x08 => Self::info(
                match s {
                    0x0 => "HY-D0",
                    0x1 => "HY-D1",
                    _ => "",
                },
                uarch::K10,
                ProcessNode::NM(45),
            ),
            0x09 => Self::info(
                match s {
                    0x1 => "HY-D1",
                    _ => "",
                },
                uarch::K10,
                ProcessNode::NM(45),
            ),
            0x0A => Self::info(
                match s {
                    0x0 => "PH-E0",
                    _ => "",
                },
                uarch::K10,
                ProcessNode::NM(45),
            ),
            _ => return None,
        })
    }

    pub(super) fn amd_fam11h(m: u32, _s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            0x03 => Self::info("LG-B1", uarch::Puma2008, ProcessNode::NM(65)),
            _ => return None,
        })
    }

    pub(super) fn amd_fam12h(m: u32, _s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            0x01 => Self::info("Llano (B0)", uarch::K10, ProcessNode::NM(32)),
            _ => return None,
        })
    }

    pub(super) fn amd_fam14h(m: u32, _s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            /* https://www.amd.com/system/files/TechDocs/47534_14h_Mod_00h-0Fh_Rev_Guide.pdf */
            0x01 => Self::info("Ontario/Zacate (B0)", uarch::Bobcat, ProcessNode::NM(40)),
            0x02 => Self::info("Ontario/Zacate (C0)", uarch::Bobcat, ProcessNode::NM(40)),
            _ => return None,
        })
    }

    pub(super) fn amd_fam15h(m: u32, s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            0x01 => Self::info(
                &["Orochi", match s {
                    0x2 => " (B2)",
                    _ => "",
                }].concat(),
                uarch::Bulldozer,
                ProcessNode::NM(32)
            ),
            0x02 => Self::info("Orochi (C0)", uarch::Bulldozer, ProcessNode::NM(32)),
            0x10 => Self::info(
                &["Trinity", match s {
                    0x0 => " (A0)",
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Piledriver,
                ProcessNode::NM(32),
            ),
            0x13 => Self::info(
                &["Richland", match s {
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Piledriver,
                ProcessNode::NM(32)
            ),
            0x30 => Self::info(
                &["Kaveri", match s {
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Steamroller,
                ProcessNode::NM(28),
            ),
            0x38 => Self::info("Godavari", uarch::Steamroller, ProcessNode::NM(28)),
            0x60 => Self::info(
                &["Carrizo", match s {
                    0x0 => " (A0)",
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Excavator,
                ProcessNode::NM(28)
            ),
            0x65 => Self::info("Bristol Ridge", uarch::Excavator, ProcessNode::NM(28)),
            0x70 => Self::info(
                &["Stoney Ridge", match s {
                    0x0 => " (A0)",
                    _ => "",
                }].concat(),
                uarch::Excavator,
                ProcessNode::NM(28),
            ),
            _ => return None,
        })
    }

    pub(super) fn amd_fam16h(m: u32, s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            0x00 => Self::info(
                &["Kabini/Temash", match s {
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Jaguar,
                ProcessNode::NM(28)
            ),
            /* A9-9820: https://linux-hardware.org/?probe=1053adf355 */
            0x26 => Self::info("Cato", uarch::Jaguar, ProcessNode::NM(28)),
            0x30 => Self::info(
                &["Beema/Mullins",  match s {
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Puma2014,
                ProcessNode::NM(28),
            ),
            _ => return None,
        })
    }

    pub(super) fn amd_fam17h(m: u32, s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            /* Zen */
            /* Naples, Zeppelin/ZP */
            0x00 |
            0x01 => Self::info(
                &["Naples/Zeppelin", match (m, s) {
                    (0x00, _) => " (A0)",
                    (0x01, 0x1) => " (B1)", // Ryzen, Summit Ridge
                    (0x01, 0x2) => " (B2)",
                    _ => "",
                }].concat(),
                uarch::Zen,
                ProcessNode::NM(14),
            ),
            0x11 => Self::info("Raven Ridge", uarch::Zen, ProcessNode::NM(14)),
            0x20 => Self::info(
                &["Raven2 (Dali/Pollock)", match s {
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Zen,
                ProcessNode::NM(14)
            ),

            /* Zen+ */
            0x08 => Self::info(
                &["Pinnacle Ridge", match s {
                    0x2 => " (B2)",
                    _ => "",
                }].concat(),
                uarch::ZenPlus,
                ProcessNode::NM(12)
            ),
            0x18 => Self::info(
                &["Picasso", match s {
                    0x1 => " (B1)",
                    _ => "",
                }].concat(),
                uarch::ZenPlus,
                ProcessNode::NM(12)
            ),

            /* Zen 2 */
            /* Rome, Starship/SSP */
            0x30 |
            0x31 => Self::info(
                &["Rome/Starship", match (m, s) {
                    (0x30, 0x0) => " (A0)",
                    (0x31, 0x0) => " (B0)",
                    _ => "",
                }].concat(),
                uarch::Zen2,
                ProcessNode::NM(7)
            ),
            0x60 => Self::info(
                &["Renoir", match s {
                    0x1 => " (A1)",
                    _ => "",
                }].concat(),
                uarch::Zen2,
                ProcessNode::NM(7)
            ),
            0x68 => Self::info("Lucienne", uarch::Zen2, ProcessNode::NM(7)),
            0x71 => Self::info("Matisse", uarch::Zen2, ProcessNode::NM(7)),
            0x90 => Self::info("VanGogh", uarch::Zen2, ProcessNode::NM(7)),
            0xA0..=0xAF => Self::info("Mendocino", uarch::Zen2, ProcessNode::NM(6)),

            _ => return None,
        })
    }

    pub(super) fn amd_fam19h(m: u32, s: u32) -> Option<Self> {
        use AmdMicroArch as uarch;

        Some(match m {
            /* Zen 3 */
            /* Milan, Genesis/GN */
            /* Revision Guide for AMD Family 19h Models 00h-0Fh Processors: https://www.amd.com/system/files/TechDocs/56683-PUB-1.07.pdf */
            0x00 |
            0x01 => Self::info(
                &["Milan/Genesis", match (m, s) {
                    (0x00, _) => " (A0)",
                    (0x01, 0x0) => " (B0)",
                    (0x01, 0x1) => " (B1)", // EPYC 7003
                    (0x01, 0x2) => " (B2)", // EPYC 7003 3D V-Cache
                    _ => "",
                }].concat(),
                uarch::Zen3,
                ProcessNode::NM(7)
            ),
            0x08 => Self::info("Chagall", uarch::Zen3, ProcessNode::NM(7)),
            0x20 |
            0x21 => Self::info(
                &["Vermeer", match (m, s) {
                    (0x20, _) => " (A0)",
                    (0x21, 0x0) => " (B0)",
                    (0x21, 0x2) => " (B2)",
                    _ => "",
                }].concat(),
                uarch::Zen3,
                ProcessNode::NM(7)
            ),
            /* https://www.openmp.org/wp-content/uploads/ecp_sollve_openmp_monthly.offload_perf_ana_craypat.marcus.hpe_.26aug2022.v2.pdf */
            0x30 => Self::info("Trento", uarch::Zen3, ProcessNode::NM(7)),
            0x40..=0x4F => Self::info(
                &["Rembrandt", match (m, s) {
                    (0x40, _) => " (A0)",
                    (0x44, 0x0) => " (B0)",
                    (0x44, 0x1) => " (B1)", // product
                    _ => "",
                }].concat(),
                uarch::Zen3Plus,
                ProcessNode::NM(6)
            ),
            0x50..=0x5F => Self::info(
                &["Cezanne/Barcelo", match (m, s) {
                    (0x50, 0x0) => " (A0)",
                    _ => "",
                }].concat(),
                uarch::Zen3,
                ProcessNode::NM(7)
            ),

            /* Zen 4 */
            /* Genoa, Stones, RS */
            /*
                0x11 => "Genoa/Stones (B0)"
                https://github.com/redhat-performance/autohpl-wrapper/issues/22
            */
            0x10..=0x1F => Self::info(
                &["Genoa/Stones", match (m, s) {
                    (0x10, _) => " (A0)",
                    _ => "",
                }].concat(),
                uarch::Zen4,
                ProcessNode::NM(5)
            ),
            0x60..=0x6F => Self::info("Phoenix", uarch::Zen4, ""),
            0x70..=0x7F => Self::info("Raphael", uarch::Zen4, ProcessNode::NM(5)),

            _ => return None,
        })
    }
}
