use crate::{CpuVendor, ProcInfo, CpuCodename, CpuMicroArch, CpuStepping, ProcessNode};
#[cfg(feature = "std")]
use std::fmt;
/* ref: https://github.com/illumos/illumos-gate/blob/master/usr/src/uts/intel/os/cpuid_subr.c */
/* ref: https://en.wikipedia.org/wiki/List_of_AMD_CPU_microarchitectures */
/* ref: https://developer.amd.com/resources/developer-guides-manuals/ */

impl ProcInfo {
    pub(super) fn amd_fam10h(m: u32, s: u32) -> Self {
        /* https://www.amd.com/system/files/TechDocs/41322_10h_Rev_Gd.pdf */
        /* https://www.amd.com/system/files/TechDocs/43374.pdf */
        match m {
            0x2 => Self {
                codename: CpuCodename::Amd(AmdCodename::DR),
                archname: CpuMicroArch::Amd(AmdMicroArch::Barcelona),
                step_info: match s {
                    0x1 => CpuStepping::B1,
                    0x2 => CpuStepping::B2,
                    0x3 => CpuStepping::B3,
                    0xA => CpuStepping::BA,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(65)),
            },
            0x4 => Self {
                codename: CpuCodename::Amd(AmdCodename::RB),
                archname: CpuMicroArch::Amd(AmdMicroArch::Shanghai),
                step_info: match s {
                    0x2 => CpuStepping::C2,
                    0x3 => CpuStepping::C3,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(45)),
            },
            0x5 => Self {
                codename: CpuCodename::Amd(AmdCodename::BL),
                archname: CpuMicroArch::Amd(AmdMicroArch::K10),
                step_info: match s {
                    0x2 => CpuStepping::C2,
                    0x3 => CpuStepping::C3,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(45)),
            },
            0x6 => Self {
                codename: CpuCodename::Amd(AmdCodename::DA),
                archname: CpuMicroArch::Amd(AmdMicroArch::K10),
                step_info: match s {
                    0x2 => CpuStepping::C2,
                    0x3 => CpuStepping::C3,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(45)),
            },
            0x8 => Self {
                codename: CpuCodename::Amd(AmdCodename::HY),
                archname: CpuMicroArch::Amd(AmdMicroArch::Istanbul),
                step_info: match s {
                    0x0 => CpuStepping::D0,
                    0x1 => CpuStepping::D1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(45)),
            },
            0x9 => Self {
                codename: CpuCodename::Amd(AmdCodename::HY),
                archname: CpuMicroArch::Amd(AmdMicroArch::K10),
                step_info: match s {
                    0x1 => CpuStepping::D1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(45)),
            },
            0xA => Self {
                codename: CpuCodename::Amd(AmdCodename::PH),
                archname: CpuMicroArch::Amd(AmdMicroArch::K10),
                step_info: match s {
                    0x0 => CpuStepping::E0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(45)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x10, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn amd_fam11h(m: u32, s: u32) -> Self {
        match m {
            0x3 => Self {
                codename: CpuCodename::Amd(AmdCodename::Griffin),
                archname: CpuMicroArch::Amd(AmdMicroArch::Puma2008),
                step_info: CpuStepping::B1, // LG-B1
                node: Some(ProcessNode::NM(65)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x11, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn amd_fam12h(m: u32, s: u32) -> Self {
        match m {
            0x1 => Self {
                codename: CpuCodename::Amd(AmdCodename::Llano),
                archname: CpuMicroArch::Amd(AmdMicroArch::K10),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(32)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x12, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn amd_fam14h(m: u32, s: u32) -> Self {
        /* https://www.amd.com/system/files/TechDocs/47534_14h_Mod_00h-0Fh_Rev_Guide.pdf */
        match m {
            0x1 | 0x2 => Self {
                codename: CpuCodename::Amd(AmdCodename::Ontario_Zacate),
                archname: CpuMicroArch::Amd(AmdMicroArch::Bobcat),
                step_info: match m {
                    0x1 => CpuStepping::B0,
                    0x2 => CpuStepping::C0,
                    _ => unreachable!(),
                },
                node: Some(ProcessNode::NM(40)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x14, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn amd_fam15h(m: u32, s: u32) -> Self {
        match m {
            0x1 | 0x2 => Self {
                codename: CpuCodename::Amd(AmdCodename::Orochi),
                archname: CpuMicroArch::Amd(AmdMicroArch::Bulldozer),
                step_info: match (m, s) {
                    (0x1, 0x2) => CpuStepping::B2,
                    (0x2, 0x0) => CpuStepping::C0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(32)),
            },
            0x10 => Self {
                codename: CpuCodename::Amd(AmdCodename::Trinity),
                archname: CpuMicroArch::Amd(AmdMicroArch::Piledriver),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(32)),
            },
            0x13 => Self {
                codename: CpuCodename::Amd(AmdCodename::Richland),
                archname: CpuMicroArch::Amd(AmdMicroArch::Piledriver),
                step_info: match s {
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(32)),
            },
            0x30 => Self {
                codename: CpuCodename::Amd(AmdCodename::Kaveri),
                archname: CpuMicroArch::Amd(AmdMicroArch::Steamroller),
                step_info: match s {
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(28)),
            },
            0x38 => Self {
                codename: CpuCodename::Amd(AmdCodename::Godavari),
                archname: CpuMicroArch::Amd(AmdMicroArch::Steamroller),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(28)),
            },
            0x60 => Self {
                codename: CpuCodename::Amd(AmdCodename::Carrizo),
                archname: CpuMicroArch::Amd(AmdMicroArch::Excavator),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(28)),
            },
            0x65 => Self {
                codename: CpuCodename::Amd(AmdCodename::BristolRidge),
                archname: CpuMicroArch::Amd(AmdMicroArch::Excavator),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(28)),
            },
            0x70 => Self {
                codename: CpuCodename::Amd(AmdCodename::StoneyRidge),
                archname: CpuMicroArch::Amd(AmdMicroArch::Excavator),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(28)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x15, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn amd_fam16h(m: u32, s: u32) -> Self {
        match m {
            0x00 => Self {
                codename: CpuCodename::Amd(AmdCodename::Kabini_Temash),
                archname: CpuMicroArch::Amd(AmdMicroArch::Jaguar),
                step_info: match s {
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(28)),
            },
            /*
            0x13: DG1101SKF84HV
            https://linux-hardware.org/?probe=87f754ac29
            */
            /* A9-9820: https://linux-hardware.org/?probe=1053adf355 */
            0x26 => Self {
                codename: CpuCodename::Amd(AmdCodename::Cato),
                archname: CpuMicroArch::Amd(AmdMicroArch::Jaguar),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(28)),
            },
            0x30 => Self {
                codename: CpuCodename::Amd(AmdCodename::Beema_Mullins),
                archname: CpuMicroArch::Amd(AmdMicroArch::Puma2014),
                step_info: match s {
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(28)),
            },
            /*
            0x43: DG1501SML87LB
            https://linux-hardware.org/?probe=2fb1797d3d
            */
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x16, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn amd_fam17h(m: u32, s: u32) -> Self {
        match m {
            /* Zen */
            /* Naples, Zeppelin/ZP */
            0x00 | 0x01 => Self {
                codename: CpuCodename::Amd(AmdCodename::Naples),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen),
                step_info: match (m, s) {
                    (0x00, _) => CpuStepping::A0,
                    (0x01, 0x1) => CpuStepping::B1, // Ryzen, Summit Ridge
                    (0x01, 0x2) => CpuStepping::B2,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            0x11 => Self {
                codename: CpuCodename::Amd(AmdCodename::RavenRidge),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(14)),
            },
            0x20 => Self {
                codename: CpuCodename::Amd(AmdCodename::Raven2),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen),
                step_info: match m {
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },

            /* Zen+ */
            0x08 => Self {
                codename: CpuCodename::Amd(AmdCodename::PinnacleRidge),
                archname: CpuMicroArch::Amd(AmdMicroArch::ZenPlus),
                step_info: match s {
                    0x2 => CpuStepping::B2,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(12)),
            },
            0x18 => Self {
                codename: CpuCodename::Amd(AmdCodename::Picasso),
                archname: CpuMicroArch::Amd(AmdMicroArch::ZenPlus),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(12)),
            },

            /* Zen 2 */
            /* Rome, Starship/SSP */
            0x30 | 0x31 => Self {
                codename: CpuCodename::Amd(AmdCodename::Rome),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen2),
                step_info: match (m, s) {
                    (0x30, 0x0) => CpuStepping::A0,
                    (0x31, 0x0) => CpuStepping::B0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(7)),
            },
            0x60 => Self {
                codename: CpuCodename::Amd(AmdCodename::Renoir),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen2),
                step_info: match s {
                    0x1 => CpuStepping::A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(7)),
            },
            0x68 => Self {
                codename: CpuCodename::Amd(AmdCodename::Lucienne),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen2),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(7)),
            },
            0x71 => Self {
                codename: CpuCodename::Amd(AmdCodename::Matisse),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen2),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(7)),
            },
            0x90 => Self {
                codename: CpuCodename::Amd(AmdCodename::VanGogh),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen2),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(7)),
            },
            0xA0..=0xAF => Self {
                codename: CpuCodename::Amd(AmdCodename::Mendocino),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen2),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(6)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x17, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }

    pub(crate) fn amd_fam19h(m: u32, s: u32) -> Self {
        match m {
            /* Milan, Genesis/GN */
            /* Revision Guide for AMD Family 19h Models 00h-0Fh Processors: https://www.amd.com/system/files/TechDocs/56683-PUB-1.07.pdf */
            0x00 | 0x01 => Self {
                codename: CpuCodename::Amd(AmdCodename::Milan),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen3),
                step_info: match (m, s) {
                    (0x00, _) => CpuStepping::A0,
                    (0x01, 0x0) => CpuStepping::B0,
                    (0x01, 0x1) => CpuStepping::B1, // EPYC 7003
                    (0x01, 0x2) => CpuStepping::B2, // EPYC 7003 with 3D V-Cache
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(7)),
            },
            0x08 => Self {
                codename: CpuCodename::Amd(AmdCodename::Chagall),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen3),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(7)),
            },
            0x20 | 0x21 => Self {
                codename: CpuCodename::Amd(AmdCodename::Vermeer),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen3),
                step_info: match (m, s) {
                    (0x20, _) => CpuStepping::A0,
                    (0x21, 0x0) => CpuStepping::B0,
                    (0x21, 0x2) => CpuStepping::B2,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(7)),
            },
            /* https://www.openmp.org/wp-content/uploads/ecp_sollve_openmp_monthly.offload_perf_ana_craypat.marcus.hpe_.26aug2022.v2.pdf */
            0x30 => Self {
                codename: CpuCodename::Amd(AmdCodename::Trento),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen3),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(7)),
            },
            0x40..=0x4F => Self {
                codename: CpuCodename::Amd(AmdCodename::Rembrandt),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen3Plus),
                step_info: match (m, s) {
                    (0x40, _) => CpuStepping::A0,
                    (0x44, 0x0) => CpuStepping::B0,
                    (0x44, 0x1) => CpuStepping::B1, // product
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(6)),
            },
            0x50..=0x5F => Self {
                codename: CpuCodename::Amd(AmdCodename::Cezanne_Barcelo),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen3),
                step_info: match (m, s) {
                    (0x50, 0x0) => CpuStepping::A0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(7)),
            },
            /* Zen 4 */
            /* Genoa, Stones, RS */
            0x10..=0x1F => Self {
                codename: CpuCodename::Amd(AmdCodename::Genoa),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen4),
                step_info: match (m, s) {
                    (0x10, _) => CpuStepping::A0,
                    (0x11, 0x0) => CpuStepping::B0,
                    (0x11, 0x1) => CpuStepping::B1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(5)),
            },
            0x60..=0x6F => Self {
                codename: CpuCodename::Amd(AmdCodename::Raphael),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen4),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(5)),
            },
            0x70..=0x7F => Self {
                codename: CpuCodename::Amd(AmdCodename::Phoenix),
                archname: CpuMicroArch::Amd(AmdMicroArch::Zen4),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(4)),
            },
            /* https://review.coreboot.org/c/coreboot/+/71731/7/src/soc/amd/phoenix/include/soc/cpu.h */
            /* 0x78 => Phoenix A0  */
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::AuthenticAMD, 0x19, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AmdCodename {
    /* Fam10h */
    Fam10h,
    DR,
    RB,
    BL,
    DA,
    HY,
    PH,
    // Barcelona,
    // Shanghai,
    // Istanbul,
    /* Fam11h */
    Griffin,
    /* Fam12h */
    Llano,
    /* Fam14h */
    Ontario_Zacate,
    /* Fam15h */
    Orochi,
    Trinity,
    Richland,
    Kaveri,
    Carrizo,
    Godavari,
    BristolRidge,
    StoneyRidge,
    /* Fam16h */
    Kabini_Temash,
    Cato,
    Beema_Mullins,
    /* Fam17h */
    Naples,
    RavenRidge,
    Raven2, /* Dali, Pollock */
    PinnacleRidge,
    Picasso,
    Rome,
    Renoir,
    Lucienne,
    Matisse,
    VanGogh,
    Mendocino,
    /* Fam19h */
    Milan,
    Chagall,
    Trento,
    Vermeer,
    Rembrandt,
    Cezanne_Barcelo,
    Genoa,
    Raphael,
    Phoenix,
}

#[cfg(feature = "std")]
impl fmt::Display for AmdCodename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ontario_Zacate => write!(f, "Ontario/Zacate"),
            Self::Kabini_Temash => write!(f, "Kabini/Temash"),
            Self::Beema_Mullins => write!(f, "Beema/Mullins"),
            Self::Cezanne_Barcelo => write!(f, "Cezanne/Barcelo"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug)]
pub enum AmdMicroArch {
    Puma2008,
    K10,
    Barcelona,
    Shanghai,
    Istanbul,
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

#[cfg(feature = "std")]
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
