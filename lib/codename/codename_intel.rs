use crate::{CpuVendor, ProcInfo, CpuCodename, CpuMicroArch, CpuStepping, ProcessNode};
#[cfg(feature = "std")]
use std::fmt;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/* ref:
    https://www.intel.com/content/www/us/en/developer/topic-technology/software-security-guidance/processors-affected-consolidated-product-cpu-model.html
    https://github.com/coreboot/coreboot/blob/master/src/cpu/intel/model_206ax/model_206ax.h
    https://github.com/coreboot/coreboot/blob/master/src/cpu/intel/haswell/haswell.h
    https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/intel-family.h
    https://github.com/coreboot/coreboot/blob/master/src/include/cpu/intel/cpu_ids.h
    https://github.com/coreboot/coreboot/blob/master/src/soc/intel/skylake/Makefile.inc
    https://github.com/intel/Intel-Linux-Processor-Microcode-Data-Files */
impl ProcInfo {
    pub(super) fn intel_fam05h(m: u32, s: u32) -> Self {
        match m {
            0x09 => Self {
                codename: CpuCodename::Intel(IntelCodename::Quark_X1000),
                archname: CpuMicroArch::Intel(IntelMicroArch::P5C),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(32)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::GenuineIntel, 0x5, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn intel_fam06h(m: u32, s: u32) -> Self {
        match m {
        /* Big core, Core, P-Core */
            0xE => Self {
                codename: CpuCodename::Intel(IntelCodename::Yonah),
                archname: CpuMicroArch::Intel(IntelMicroArch::PentiumM),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(90)),
            },
            0xF | 0x16 => {
                let codename = match m {
                    0xF => IntelCodename::Merom,
                    0x16 => IntelCodename::Merom_L,
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Merom),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(65)),
                }
            },
            /* Penryn */
            0x17 | 0x1D => {
                let codename = match m {
                    0x17 => IntelCodename::Penryn,
                    0x1D => IntelCodename::Dunnington,
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Penryn),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(45)),
                }
            },
            /* Nehalem */
            0x1E | 0x1F | 0x1A | 0x2E => {
                let codename = match m {
                    0x1E => IntelCodename::Nehalem,
                    0x1F => IntelCodename::Nehalem_G,
                    0x1A => IntelCodename::Nehalem_EP,
                    0x2E => IntelCodename::Nehalem_EX,
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Nehalem),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(45)),
                }
            },
            /* Westmere */
            0x25 | 0x2C | 0x2F => {
                let codename = match m {
                    0x25 => IntelCodename::Westmere,
                    0x2C => IntelCodename::Westmere_EP,
                    0x2F => IntelCodename::Westmere_EX,
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Westmere),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(32)),
                }
            },
            /* Sandy Bridge */
            0x2A => Self {
                codename: CpuCodename::Intel(IntelCodename::SandyBridge),
                archname: CpuMicroArch::Intel(IntelMicroArch::SandyBridge),
                step_info: match s {
                    0x2 => CpuStepping::B2,
                    0x3 => CpuStepping::C0,
                    0x5 => CpuStepping::D0_J0,
                    0x6 => CpuStepping::D1,
                    0x7 => CpuStepping::D2_J1_Q0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(32)),
            },
            0x2D => Self {
                codename: CpuCodename::Intel(IntelCodename::SandyBridge_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::SandyBridge),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(32)),
            },
            /* Ivy Bridge */
            0x3A => Self {
                codename: CpuCodename::Intel(IntelCodename::IvyBridge),
                archname: CpuMicroArch::Intel(IntelMicroArch::IvyBridge),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x2 => CpuStepping::B0,
                    0x4 => CpuStepping::C0,
                    0x5 => CpuStepping::K0,
                    0x6 => CpuStepping::D0,
                    0x8 => CpuStepping::E0,
                    0x9 => CpuStepping::E1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(22)),
            },
            0x3E => Self {
                codename: CpuCodename::Intel(IntelCodename::IvyBridge_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::IvyBridge),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(22)),
            },
            /* Haswell */
            0x3C => Self {
                codename: CpuCodename::Intel(IntelCodename::Haswell_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::Haswell),
                step_info: match s {
                    0x1 => CpuStepping::A0,
                    0x2 => CpuStepping::B0,
                    0x3 => CpuStepping::C0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(22)),
            },
            0x3F => Self {
                codename: CpuCodename::Intel(IntelCodename::Haswell_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::Haswell),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(22)),
            },
            0x45 => Self {
                codename: CpuCodename::Intel(IntelCodename::Haswell_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::Haswell),
                step_info: match s {
                    0x0 => CpuStepping::B0,
                    0x1 => CpuStepping::C0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(22)),
            },
            /* Haswell-G, Crystalwell */
            0x46 => Self {
                codename: CpuCodename::Intel(IntelCodename::Haswell_G),
                archname: CpuMicroArch::Intel(IntelMicroArch::Haswell),
                step_info: match s {
                    0x0 => CpuStepping::B0,
                    0x1 => CpuStepping::C0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(22)),
            },
            /* Broadwell */
            0x3D => Self {
                codename: CpuCodename::Intel(IntelCodename::Broadwell),
                archname: CpuMicroArch::Intel(IntelMicroArch::Broadwell),
                step_info: match s {
                    0x2 => CpuStepping::C0,
                    0x3 => CpuStepping::D0,
                    0x4 => CpuStepping::E0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            0x47 => Self {
                codename: CpuCodename::Intel(IntelCodename::Broadwell_G),
                archname: CpuMicroArch::Intel(IntelMicroArch::Broadwell),
                step_info: match s {
                    0x1 => CpuStepping::C0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            0x4F => Self {
                codename: CpuCodename::Intel(IntelCodename::Broadwell_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::Broadwell),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(14)),
            },
            0x56 => Self {
                codename: CpuCodename::Intel(IntelCodename::Broadwell_D),
                archname: CpuMicroArch::Intel(IntelMicroArch::Broadwell),
                step_info: match s {
                    0x3 => CpuStepping::V2_V3,
                    0x4 => CpuStepping::Y0,
                    0x5 => CpuStepping::A1, // BDX-NS, Hewitt Lake
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            /* Skylake */
            0x4E => Self {
                codename: CpuCodename::Intel(IntelCodename::SkyLake_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::Skylake),
                step_info: match s {
                    0x2 => CpuStepping::C0,
                    0x3 => CpuStepping::D0, // U23e: K1
                    0x8 => CpuStepping::G0, // Kaby Lake
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            0x5E => Self {
                codename: CpuCodename::Intel(IntelCodename::SkyLake_S),
                archname: CpuMicroArch::Intel(IntelMicroArch::Skylake),
                step_info: match s {
                    0x1 => CpuStepping::HQ0,
                    0x3 => CpuStepping::HR0, // R0, N0
                    0x8 => CpuStepping::HA0, // Kaby Lake
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            /* Sky Lake-SP */
            0x55 => {
                let (codename, archname, step_info) = match s {
                    0x6 => (
                        IntelCodename::CascadeLake_X,
                        IntelMicroArch::Skylake_AVX512_VNNI,
                        CpuStepping::B0
                    ),
                    0x7 => (
                    /* https://www.intel.com/content/www/us/en/content-details/338848/2nd-gen-intel-xeon-scalable-processors-specification-update.html */
                        IntelCodename::CascadeLake_X,
                        IntelMicroArch::Skylake_AVX512_VNNI,
                        /* XCC: B1, HCC: L1, LCC: R1 */
                        CpuStepping::B1
                    ),
                    0xA => (
                        IntelCodename::CooperLake_X,
                        IntelMicroArch::Skylake_AVX512_VNNI_BF16,
                        CpuStepping::A0
                    ),
                    0xB => (
                        IntelCodename::CooperLake_X,
                        IntelMicroArch::Skylake_AVX512_VNNI_BF16,
                        CpuStepping::A1
                    ),
                    _ => (
                    /* https://www.intel.com/content/www/us/en/content-details/336065/intel-xeon-processor-scalable-family-specification-update.html */
                        IntelCodename::SkyLake_X,
                        IntelMicroArch::Skylake_AVX512,
                        /* Sky Lake-D Stepping 4: M1 */
                        /* Stepping 2, XCC: B0, HCC: L0 */
                        /* Stepping 3: B1 */
                        /* Stepping 4, XCC: H0, HCC: M0, LCC: U0 */
                        CpuStepping::Unknown(s)
                    ),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(archname),
                    step_info,
                    node: Some(ProcessNode::NM(14)),
                }
            },
            /* Kaby Lake */
            0x8E => {
                let (codename, step_info) = match s {
                    0x9 => (IntelCodename::KabyLake_L, CpuStepping::H0), // B0, S0, J0, J1
                    0xA => (IntelCodename::KabyLake_L, CpuStepping::Y0),
                    0xB => (IntelCodename::WhiskeyLake_L, CpuStepping::W0),
                    0xC => (IntelCodename::WhiskeyLake_L, CpuStepping::V0),
                    _ => (IntelCodename::KabyLake_L, CpuStepping::Unknown(s)),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Skylake),
                    step_info,
                    node: Some(ProcessNode::NM(14)),
                }
            },
            0x9E => {
                let (codename, step_info) = match s {
                    0x9 => (IntelCodename::KabyLake_S, CpuStepping::HB0),
                    0xA => (IntelCodename::CoffeeLake_S, CpuStepping::U0),
                    0xB => (IntelCodename::CoffeeLake_S, CpuStepping::B0),
                    0xC => (IntelCodename::CoffeeLake_S, CpuStepping::P0),
                    0xD => (IntelCodename::CoffeeLake_S, CpuStepping::R0),
                    _ => (IntelCodename::KabyLake_S, CpuStepping::Unknown(s)),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Skylake),
                    step_info,
                    node: Some(ProcessNode::NM(14)),
                }
            },
            /* Comet Lake */
            0xA5 => Self {
                codename: CpuCodename::Intel(IntelCodename::CometLake_S),
                archname: CpuMicroArch::Intel(IntelMicroArch::Skylake),
                step_info: match s {
                    0x0 => CpuStepping::G0,
                    0x1 => CpuStepping::P0,
                    0x2 => CpuStepping::R1,
                    0x3 => CpuStepping::G1,
                    0x4 => CpuStepping::P1,
                    0x5 => CpuStepping::Q0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            0xA6 => Self {
                codename: CpuCodename::Intel(IntelCodename::CometLake_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::Skylake),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x1 => CpuStepping::K1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            /* Cannon Lake */
            0x66 => Self {
                codename: CpuCodename::Intel(IntelCodename::CannonLake_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::PalmCove),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(10)),
            },
            /* Ice Lake */
            /* Ice Lake-X/SP Stepping 0x6, XCC: D0/D2, HCC: M1 */
            0x6A | 0x6C | 0x7D | 0x7E | 0x9D => Self {
                codename: match m {
                    0x6A => CpuCodename::Intel(IntelCodename::IceLake_X),
                    0x6C => CpuCodename::Intel(IntelCodename::IceLake_D),
                    0x7D => CpuCodename::Intel(IntelCodename::IceLake_S),
                    0x7E => CpuCodename::Intel(IntelCodename::IceLake_L),
                    0x9D => CpuCodename::Intel(IntelCodename::IceLake_NNPI),
                    _ => unreachable!(),
                },
                archname: CpuMicroArch::Intel(IntelMicroArch::SunnyCove),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(10)),
            },
            /* Rocket Lake */
            0xA7 => Self {
                codename: CpuCodename::Intel(IntelCodename::RocketLake_S),
                archname: CpuMicroArch::Intel(IntelMicroArch::CypressCove),
                step_info: match s {
                    0x1 => CpuStepping::B0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
                /* 06_A8h: Xeon W-1300? */
                /* https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/best-practices/data-operand-independent-timing-isa-guidance.html */
            0x8C => Self {
                codename: CpuCodename::Intel(IntelCodename::TigerLake_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::WillowCove),
                step_info: match s {
                    0x1 => CpuStepping::B0,
                    0x2 => CpuStepping::C0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(10)),
            },
            0x8D => Self {
                codename: CpuCodename::Intel(IntelCodename::TigerLake_H),
                archname: CpuMicroArch::Intel(IntelMicroArch::WillowCove),
                step_info: match s {
                    0x1 => CpuStepping::R0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(10)),
            },

            /* https://github.com/qizhangz/tdx-module/blob/main/src/common/x86_defs/x86_defs.h */
            /* https://review.coreboot.org/c/coreboot/+/71967 */
            0x8F => Self {
                codename: CpuCodename::Intel(IntelCodename::SapphireRapids_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::GoldenCove),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x1 => CpuStepping::B0,
                    0x2 => CpuStepping::C0,
                    0x3 => CpuStepping::D0,
                    0x4 => CpuStepping::E0,
                    0x5 => CpuStepping::E2,
                    0x6 => CpuStepping::E3,
                    0x7 => CpuStepping::E4,
                    // 0x8 => CpuStepping::Ex,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::Intel(7)),
            },
            /* https://github.com/liuwei142536/hamburger_Bios/blob/AMTCODE/ServerSecurityPkg/Pfr/PfrShellCommand/Common/App.h */
            /* 0x95 => Sapphire Rapids-G?, */
            0xCF => Self {
                codename: CpuCodename::Intel(IntelCodename::EmeraldRapids_X),
                archname: CpuMicroArch::Intel(IntelMicroArch::GoldenCove),
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
            0xAD => Self {
                codename: CpuCodename::Intel(IntelCodename::GraniteRapids_X),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::Intel(3)),
            },
            0xAE => Self {
                codename: CpuCodename::Intel(IntelCodename::GraniteRapids_D),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::Intel(3)),
            },
        /* Small Core, Atom, E-Core */
            /* Bonnell */
            0x1C | 0x26 => {
                let codename = match m {
                    0x16 => IntelCodename::Bonnell, // Diamondville, Pineview
                    0x26 => IntelCodename::Bonnell_MID, // Silverthorne, Lincroft
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Bonnell),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(45)),
                }
            },
            /* Saltwell */
            0x36 | 0x27 | 0x35 => {
                let codename = match m {
                    0x36 => IntelCodename::Saltwell, // Cedarview
                    0x27 => IntelCodename::Saltwell_MID, // Penwell
                    0x35 => IntelCodename::Saltwell_TABLET, // Cloverview
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Saltwell),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(32)),
                }
            },

            /* Sivermont */
            0x37 | 0x4D | 0x4A => {
                let codename = match m {
                    0x37 => IntelCodename::Silvermont, // Bay Trail, Valleyview
                    0x4D => IntelCodename::Silvermont_D, // Avaton, Rangely
                    0x4A => IntelCodename::Silvermont_MID, // Merriefield
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Silvermont),
                    step_info: CpuStepping::Unknown(s),
                    node: Some(ProcessNode::NM(22)),
                }
            },
            0x5D => Self {
                codename: CpuCodename::Intel(IntelCodename::SoFIA_3G),
                archname: CpuMicroArch::Intel(IntelMicroArch::Silvermont),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(28)),
            },
            /* Airmont */
            0x4C => Self {
                codename: CpuCodename::Intel(IntelCodename::Airmont), // Cherry Trail, Braswell
                archname: CpuMicroArch::Intel(IntelMicroArch::Airmont),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(14)),
            },
            0x5A => Self {
                codename: CpuCodename::Intel(IntelCodename::Airmont_MID), // Moorefield, Anniedale
                archname: CpuMicroArch::Intel(IntelMicroArch::Airmont),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(22)),
            },
            0x65 | 0x6E => {
                let codename = match m {
                    0x65 => IntelCodename::XMM7272,
                    0x6E => IntelCodename::CougarMountain,
                    _ => unreachable!(),
                };

                Self {
                    codename: CpuCodename::Intel(codename),
                    archname: CpuMicroArch::Intel(IntelMicroArch::Airmont),
                    step_info: CpuStepping::Unknown(s),
                    node: None,
                }
            },
            0x75 => Self {
                codename: CpuCodename::Intel(IntelCodename::Airmont_NP), // Lightning Mountain, Butter
                archname: CpuMicroArch::Intel(IntelMicroArch::Airmont),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(14)),
            },
            /* Goldmont */
            0x5C => Self {
                codename: CpuCodename::Intel(IntelCodename::ApolloLake),
                archname: CpuMicroArch::Intel(IntelMicroArch::Goldmont),
                step_info: match s {
                    0x9 => CpuStepping::D0,
                    0xA => CpuStepping::E0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            0x5F => Self {
                codename: CpuCodename::Intel(IntelCodename::Denverton),
                archname: CpuMicroArch::Intel(IntelMicroArch::Goldmont),
                step_info: match s {
                    0x1 => CpuStepping::B0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            /* Goldmont Plus */
            0x7A => Self {
                codename: CpuCodename::Intel(IntelCodename::GeminiLake),
                archname: CpuMicroArch::Intel(IntelMicroArch::GoldmontPlus),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x1 => CpuStepping::B0,
                    0x8 => CpuStepping::R0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(14)),
            },
            /* Tremont */
            0x86 => Self {
                codename: CpuCodename::Intel(IntelCodename::SnowRidge), // Jacobsville
                archname: CpuMicroArch::Intel(IntelMicroArch::Tremont),
                step_info: match s {
                    0x4 => CpuStepping::B0,
                    0x5 => CpuStepping::B1,
                    // 0x7 => Parker Ridge, Snow Ridge-NS/NX
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(10)),
            },
            0x96 => Self {
                codename: CpuCodename::Intel(IntelCodename::ElkhartLake),
                archname: CpuMicroArch::Intel(IntelMicroArch::Tremont),
                step_info: match s {
                    0x1 => CpuStepping::B1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(10)),
            },
            0x9C => Self {
                codename: CpuCodename::Intel(IntelCodename::JasperLake),
                archname: CpuMicroArch::Intel(IntelMicroArch::Tremont),
                step_info: match s {
                    0x0 => CpuStepping::A0_A1,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(10)),
            },
            /* Gracemont */
            0xBE => Self {
                codename: CpuCodename::Intel(IntelCodename::AlderLake_N),
                archname: CpuMicroArch::Intel(IntelMicroArch::Gracemont),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::Intel(7)),
            },
            /* Crestmont ? */
            0xAF => Self {
                codename: CpuCodename::Intel(IntelCodename::SierraForest_X),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::Intel(3)),
            },
            0xB6 => Self {
                codename: CpuCodename::Intel(IntelCodename::GrandRidge),
                archname: CpuMicroArch::Intel(IntelMicroArch::Crestmont),
                step_info: CpuStepping::Unknown(s),
                node: None,
            },

        /* Hybrid */
            0x8A => Self {
                codename: CpuCodename::Intel(IntelCodename::Lakefield),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::SunnyCove,
                    IntelMicroArch::Tremont
                )),
                step_info: match s {
                    0x1 => CpuStepping::B2_B3,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::NM(10)),
            },
            /* Alder Lake */
            /* https://github.com/coreboot/coreboot/blob/master/src/soc/intel/alderlake/Makefile.inc */
            0x97 => Self {
                codename: CpuCodename::Intel(IntelCodename::AlderLake_S),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::GoldenCove,
                    IntelMicroArch::Gracemont
                )),
                step_info: match s {
                    0x0 => CpuStepping::A0,
                    0x1 => CpuStepping::B0,
                    0x2 => CpuStepping::C0,
                    0x4 => CpuStepping::G0,
                    0x5 => CpuStepping::H0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::Intel(7)),
            },
            0x9A => Self {
                codename: CpuCodename::Intel(IntelCodename::AlderLake_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::GoldenCove,
                    IntelMicroArch::Gracemont,
                )),
                /* Alder Lake-M: 0x1, 0x4 */
                /* Alder Lake-P: 0x0, 0x2, 0x3 */
                step_info: match s {
                    0x0 => CpuStepping::J0,
                    0x1 => CpuStepping::Q0,
                    0x2 => CpuStepping::K0,
                    0x3 => CpuStepping::L0,
                    0x4 => CpuStepping::R0,
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::Intel(7)),
            },
            /* Raptor Lake */
            0xB7 => Self {
                codename: CpuCodename::Intel(IntelCodename::RaptorLake_S),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::GoldenCove,
                    IntelMicroArch::Gracemont,
                )),
                step_info: match s {
                    0x1 => CpuStepping::B0, // 8P+16E
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::Intel(7)),
            },
            0xBA => Self {
                codename: CpuCodename::Intel(IntelCodename::RaptorLake_P),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::GoldenCove,
                    IntelMicroArch::Gracemont,
                )),
                step_info: match s {
                    0x2 => CpuStepping::J0, // 6P+8E
                    0x3 => CpuStepping::Q0, // 2P+8E
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::Intel(7)),
            },
            0xBF => Self {
                codename: CpuCodename::Intel(IntelCodename::RaptorLake_S_BFH),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::GoldenCove,
                    IntelMicroArch::Gracemont,
                )),
                step_info: match s {
                    0x2 | 0x5 => CpuStepping::C0, // 0x2: 8P+8E, 0x5: 6P+0E
                    _ => CpuStepping::Unknown(s),
                },
                node: Some(ProcessNode::Intel(7)),
            },
            /* Meteor Lake */
            0xAA => Self {
                codename: CpuCodename::Intel(IntelCodename::MeteorLake_S),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::RedwoodCove,
                    IntelMicroArch::Crestmont,
                )),
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
            0xAC => Self {
                codename: CpuCodename::Intel(IntelCodename::MeteorLake_L),
                archname: CpuMicroArch::Intel(IntelMicroArch::hybrid(
                    IntelMicroArch::RedwoodCove,
                    IntelMicroArch::Crestmont,
                )),
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
            0xB5 => Self {
                codename: CpuCodename::Intel(IntelCodename::MeteorLake_B5H),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
            0xBD => Self {
                codename: CpuCodename::Intel(IntelCodename::LunarLake_M),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
            0xC5 => Self {
                codename: CpuCodename::Intel(IntelCodename::ArrowLake),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::IntelA(20)),
            },
            0xC6 => Self {
                codename: CpuCodename::Intel(IntelCodename::ArrowLake_S),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::IntelA(20)),
            },
            _ => Self {
                codename: CpuCodename::Unknown(CpuVendor::GenuineIntel, 0x6, m),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
}

/// List of Intel CPU (SoC) codenmaes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum IntelCodename {
/* Family 5 */
    Quark_X1000,
/* Family 6 */
    /* Big Core, Core, P-Core */
    Yonah, // Core
    Merom, // Core2
    Merom_L, // Core2
    Penryn, // Core2
    Dunnington, // Core2
    Nehalem,
    Nehalem_G,
    Nehalem_EP,
    Nehalem_EX,
    Westmere,
    Westmere_EP,
    Westmere_EX,
    SandyBridge,
    SandyBridge_X,
    IvyBridge,
    IvyBridge_X,
    Haswell,
    Haswell_X,
    Haswell_L,
    Haswell_G,
    Broadwell,
    Broadwell_G,
    Broadwell_X,
    Broadwell_D,
    SkyLake_L,
    SkyLake_S,
    SkyLake_X,
    CascadeLake_X,
    CooperLake_X,
    KabyLake_L,
    AmberLake_L,
    CoffeeLake_L,
    WhiskeyLake_L,
    KabyLake_S,
    CoffeeLake_S,
    CometLake_S,
    CometLake_L,
    CannonLake_L,
    IceLake_L,
    IceLake_X,
    IceLake_D,
    IceLake_S,
    IceLake_NNPI,
    RocketLake_S,
    TigerLake_L,
    TigerLake_H,
    SapphireRapids_X,
    EmeraldRapids_X,
    GraniteRapids_X,
    GraniteRapids_D,
    /* Small Core, Atom, E-Core */
    Bonnell, /* Diamondville, Pineview */
    Bonnell_MID, /* Silverthorne, Lincroft */
    Saltwell, /* Cedarview */
    Saltwell_MID, /* Penwell */
    Saltwell_TABLET, /* Cloverview */
    Silvermont, /* Bay Trail, Valleyview */
    Silvermont_D, /* Avaton, Rangely */
    Silvermont_MID, /* Merriefield */
    XMM7272,
    CougarMountain,
    SoFIA_3G,
    Airmont, /* Cherry Trail, Braswell */
    Airmont_MID, /* Moorefield */
    Airmont_NP, /* Lightning Mountain */
    // Goldmont,
    ApolloLake,
    // Goldmont_D,
    Denverton,
    // Goldmont_Plus,
    GeminiLake,
    // Tremont_D, Jacobsville
    SnowRidge,
    // Tremont,
    ElkhartLake,
    // Tremont_L,
    JasperLake,
    SierraForest_X,
    GrandRidge,
    /* Hybrid */
    Lakefield,
    AlderLake_S,
    AlderLake_L,
    AlderLake_N,
    RaptorLake_S,
    RaptorLake_P,
    RaptorLake_S_BFH,
    MeteorLake_S,
    MeteorLake_L,
    MeteorLake_B5H,
    LunarLake_M,
    ArrowLake,
    ArrowLake_S,
    /* Xeon Phi */
    KnightsLanding,
    KnightsMill,
}

#[cfg(feature = "std")]
impl fmt::Display for IntelCodename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// List of Intel micro-architectures
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum IntelMicroArch {
    P5C,
    PentiumM,
    /* Core */
    Merom,
    Penryn,
    Nehalem,
    Westmere,
    SandyBridge,
    IvyBridge,
    Haswell,
    Broadwell,
    Skylake,
    Skylake_AVX512,
    Skylake_AVX512_VNNI,
    Skylake_AVX512_VNNI_BF16,
    PalmCove,
    SunnyCove,
    CypressCove,
    WillowCove,
    GoldenCove,
    RedwoodCove,
    /* Atom */
    Bonnell,
    Saltwell,
    Silvermont,
    Airmont,
    Goldmont,
    GoldmontPlus,
    Tremont,
    Gracemont,
    Crestmont,
    /* Hybrid */
    Hybrid(Box<Self>, Box<Self>),
    // Hybrid_ { core: Box<Self>, atom: Box<Self> },
    /* Xeon Phi, Knights */
    KnightsLanding,
    KnightsMill,
    _Reserved,
}

impl IntelMicroArch {
    pub(crate) fn hybrid(core: Self, atom: Self) -> Self {
        Self::Hybrid(Box::new(core), Box::new(atom))
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Self::Bonnell |
            Self::Saltwell |
            Self::Silvermont |
            Self::Airmont |
            Self::Goldmont |
            Self::GoldmontPlus |
            Self::Tremont |
            Self::Gracemont |
            Self::Crestmont => true,
            _ => false,
        }
    }

    pub fn is_core(&self) -> bool {
        !self.is_atom() && *self != Self::_Reserved
    }

    pub fn is_hybrid(&self) -> bool {
        if let Self::Hybrid(_, _) = *self {
            true
        } else {
            false
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for IntelMicroArch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Hybrid(core, atom) => write!(f, "{core} + {atom}"),
            _ => write!(f, "{:?}", self),
        }
    }
}
