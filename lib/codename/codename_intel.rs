use crate::{ProcInfo, ProcessNode};
/* ref:
    https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/intel-family.h
    https://github.com/coreboot/coreboot/blob/master/src/include/cpu/intel/cpu_ids.h */

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(self) enum IntelMicroArch {
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
    /* Xeon Phi, Knights */
    KnightsLanding,
    KnightsMill,
    _Reserved,
}

impl IntelMicroArch {
    fn hybrid(core: Self, atom: Self) -> Self {
        Self::Hybrid(Box::new(core), Box::new(atom))
    }
}

use std::fmt;
impl fmt::Display for IntelMicroArch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Hybrid(core, atom) => write!(f, "{core} + {atom}"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<IntelMicroArch> for String {
    fn from(s: IntelMicroArch) -> Self {
        s.to_string()
    }
}

impl ProcInfo {
    pub(super) fn intel_fam05h(m: u32, _s: u32) -> Option<Self> {
        use IntelMicroArch as uarch;

        Some(match m {
            0x09 => Self::info("Quark X1000", uarch::P5C, ProcessNode::NM(32)),
            _ => return None,
        })
    }

    pub(super) fn intel_fam06h(m: u32, s: u32) -> Option<Self> {
        use IntelMicroArch as uarch;

        Some(match m {
            /* Core */
            0x0E => Self::info("Yonah", uarch::PentiumM, ProcessNode::NM(90)),
            0x0F => Self::info("Merom", uarch::Merom, ProcessNode::NM(65)),
            0x16 => Self::info("Merom (Mobile)", uarch::Merom, ProcessNode::NM(65)),
            0x17 => Self::info("Penryn", uarch::Penryn, ProcessNode::NM(45)),
            0x1D => Self::info("Dunnington", uarch::Penryn, ProcessNode::NM(45)),
            /* Nehalem */
            0x1E => Self::info("Nehalem", uarch::Nehalem, ProcessNode::NM(45)),
            0x1F => Self::info("Auburndale/Havendale", uarch::Nehalem, ProcessNode::NM(45)),
            0x1A => Self::info("Nehalem-EP", uarch::Nehalem, ProcessNode::NM(45)),
            0x2E => Self::info("Nehalem-EX", uarch::Nehalem, ProcessNode::NM(45)),
            /* Westmere */
            0x25 => Self::info("Westmere", uarch::Westmere, ProcessNode::NM(32)),
            0x2C => Self::info("Westmere-EP", uarch::Westmere, ProcessNode::NM(32)),
            0x2F => Self::info("Westmere-EX", uarch::Westmere, ProcessNode::NM(32)),
            /* Sandy Bridge */
            0x2A => Self::info("Sandy Bridge", uarch::SandyBridge, ProcessNode::NM(32)),
            0x2D => Self::info("Sandy Bridge-E", uarch::SandyBridge, ProcessNode::NM(32)),
            /* Ivy Bridge */
            0x3A => Self::info("Ivy Bridge", uarch::IvyBridge, ProcessNode::NM(22)),
            0x3E => Self::info("Ivy Bridge-E", uarch::IvyBridge, ProcessNode::NM(22)),
            /* Haswell */
            0x3C => Self::info("Haswell", uarch::Haswell, ProcessNode::NM(22)),
            0x3F => Self::info("Haswell-E", uarch::Haswell, ProcessNode::NM(22)),
            0x45 => Self::info("Haswell-U", uarch::Haswell, ProcessNode::NM(22)),
            0x46 => Self::info("Haswell-G", uarch::Haswell, ProcessNode::NM(22)),
            /* Broadwell */
            0x3D => Self::info("Broadwell", uarch::Broadwell, ProcessNode::NM(14)),
            0x47 => Self::info("Broadwell-G", uarch::Broadwell, ProcessNode::NM(14)),
            0x4F => Self::info("Broadwell-EP", uarch::Broadwell, ProcessNode::NM(14)),
            0x56 => Self::info("Broadwell-DE", uarch::Broadwell, ProcessNode::NM(14)),
            /* Skylake */
            0x4E => Self::info("Skylake-U", uarch::Skylake, ProcessNode::NM(14)),
            0x5E => Self::info(
                match s {
                    0x8 => "Kaby Lake (HA0)",
                    _ => "Skylake-S",
                },
                uarch::Skylake,
                ProcessNode::NM(14)
            ),
            0x55 => match s {
                0x7 => Self::info(
                    "Cascade Lake-SP",
                    uarch::Skylake_AVX512_VNNI,
                    ProcessNode::NM(14)
                ),
                0xA |
                0xB => Self::info(
                    "Cooper Lake-SP",
                    uarch::Skylake_AVX512_VNNI_BF16,
                    ProcessNode::NM(14)
                ),
                _ => Self::info("Skylake-SP", uarch::Skylake_AVX512, ProcessNode::NM(14)),
            },
            /* 10th Generation Intel® Core™ Processors Datasheet, Volume 1 of 2 */
            /* https://www.intel.com/content/www/us/en/content-details/615211/10th-generation-intel-core-processors-datasheet-volume-1-of-2.html */
            /* https://github.com/coreboot/coreboot/blob/master/src/soc/intel/skylake/Makefile.inc */
            0x8E => match s {
                0x9 => Self::info("Amber Lake-Y", uarch::Skylake, ProcessNode::NM(14)),
                0xA => Self::info("Coffee Lake-U", uarch::Skylake, ProcessNode::NM(14)),
                0xB => Self::info("Whiskey Lake-U (W0)", uarch::Skylake, ProcessNode::NM(14)),
                0xC => Self::info(
                    "Comet/Whiskey Lake-U (V0)",
                    uarch::Skylake,
                    ProcessNode::NM(14)
                ),
                _ => Self::info("Kaby Lake-U", uarch::Skylake, ProcessNode::NM(14)),
            },
            /* Kaby Lake */
            0x9E => match s {
                0x9 => Self::info("Kaby Lake-S (HB0)", uarch::Skylake, ProcessNode::NM(14)),
                0xA |
                0xB |
                0xC => Self::info("Coffee Lake-S", uarch::Skylake, ProcessNode::NM(14)),
                0xD => Self::info(
                    "Comet/Coffee Lake (ES) (R0)",
                    uarch::Skylake,
                    ProcessNode::NM(14)
                ),
                _ => Self::info("Kaby Lake-S", uarch::Skylake, ProcessNode::NM(14)),
            },
            /* Comet Lake */
            0xA5 => Self::info(
                &["Comet Lake-S", match s {
                    0x0 => " (G0)",
                    0x1 => " (P0)",
                    0x2 => " (R1)",
                    0x3 => " (G1)",
                    0x4 => " (P1)",
                    0x5 => " (Q0)",
                    _ => "",
                }].concat(),
                uarch::Skylake,
                ProcessNode::NM(14)
            ),
            0xA6 => Self::info(
                &["Comet Lake-U", match s {
                    0x0 => " (A0)",
                    _ => "",
                }].concat(),
                uarch::Skylake,
                ProcessNode::NM(14)
            ),
            /* Cannon Lake */
            0x66 => Self::info("Cannon Lake-U", uarch::PalmCove, ProcessNode::NM(10)),
            /* Ice Lake */
            0x6A => Self::info("Ice Lake-SP", uarch::SunnyCove, ProcessNode::NM(10)),
            0x6C => Self::info("Ice Lake-D", uarch::SunnyCove, ProcessNode::NM(10)),
            0x7D => Self::info("Ice Lake-S", uarch::SunnyCove, ProcessNode::NM(10)),
            0x7E => Self::info("Ice Lake-U", uarch::SunnyCove, ProcessNode::NM(10)),
            0x9D => Self::info("Ice Lake-NNPI", uarch::SunnyCove, ProcessNode::NM(10)),
            /* Rocket Lake */
            0xA7 => Self::info(
                &["Rocket Lake-S", match s {
                    0x0 => " (A0)",
                    0x1 => " (B0)",
                    _   => "",
                }].concat(),
                uarch::CypressCove,
                ProcessNode::NM(14)
            ),
            /* Xeon W-1300 */
            /* 06_A8H: https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/best-practices/data-operand-independent-timing-isa-guidance.html */
            0xA8 => Self::info("Rocket Lake (WS)", uarch::CypressCove, ProcessNode::NM(14)),
            /* Tiger Lake */ 
            0x8C => Self::info(
                &["Tiger Lake-U", match s {
                    0x0 => " (A0)",
                    0x1 => " (B0)",
                    0x2 => " (C0)",
                    _   => "",
                }].concat(),
                uarch::WillowCove,
                ProcessNode::NM(10)
            ),
            0x8D => Self::info("Tiger Lake-H", uarch::WillowCove, ProcessNode::NM(10)),

            0x8F => Self::info("Sapphire Rapids-SP", uarch::GoldenCove, ProcessNode::NM(10)),
            // Stepping 8?: Self::info("Emerald Rapids-SP", uarch::GoldenCove, ProcessNode::NM(10)),
            0xAD |
            0xAE => Self::info("Granite Rapids-SP", "", ProcessNode::Intel(3)),

            /* Hybrid */
            0x8A => Self::info(
                "Lakefield (1+4)",
                uarch::hybrid(uarch::SunnyCove, uarch::Tremont),
                ProcessNode::NM(10)
            ),
            /* Alder Lake */
            /* https://edc.intel.com/content/www/us/en/design/ipla/software-development-platforms/client/platforms/alder-lake-desktop/12th-generation-intel-core-processors-datasheet-volume-1-of-2/005/cpuid/ */
            /* https://github.com/coreboot/coreboot/blob/master/src/soc/intel/alderlake/Makefile.inc */
            0x97 => Self::info(
                &["Alder Lake-S", match s {
                    0x0 => " (A0, ES)",
                    0x1 => " (B0, ES)",
                    0x2 => " (C0, 8+8)",
                    0x4 => " (G0, ES)",
                    0x5 => " (H0, 6+0)",
                    _   => "",
                }].concat(),
                uarch::hybrid(uarch::GoldenCove, uarch::Gracemont),
                ProcessNode::Intel(7)
            ),
            /* https://review.coreboot.org/c/coreboot/+/63299 */
            0x9A => {
                let [variant, stepping] = match s {
                    /* Alder Lake-M */
                    0x1 => ["M", " (Q0, 2+8)"],
                    0x4 => ["M", " (R0, 2+8)"],
                    /* Alder Lake-P */
                    _ => ["P", match s {
                        0x0 => " (J0, 6+8)",
                        0x2 => " (K0, 6+8)",
                        0x3 => " (L0, 6+8)",
                        // 0x4 => " (R0?)",
                        _ => " (6+8)",
                    }],
                };

                Self::info(
                    &format!("Alder Lake-{variant} {stepping}"),
                    uarch::hybrid(uarch::GoldenCove, uarch::Gracemont),
                    ProcessNode::Intel(7),
                )
            },
            /* Raptor Lake */
            0xB7 => Self::info(
                "Raptor Lake-S",
                uarch::hybrid(uarch::GoldenCove, uarch::Gracemont),
                ProcessNode::Intel(7)
            ),
            0xBA => Self::info(
                "Raptor Lake-P",
                uarch::hybrid(uarch::GoldenCove, uarch::Gracemont),
                ProcessNode::Intel(7)
            ),
            0xBF => Self::info(
                "Raptor Lake-S (0xBF)",
                uarch::hybrid(uarch::GoldenCove, uarch::Gracemont),
                ProcessNode::Intel(7)
            ),
            /* Meteor Lake */
            0xAA => Self::info(
                "Meteor Lake-M/P",
                uarch::hybrid(uarch::RedwoodCove, uarch::Crestmont),
                ""
            ),
            0xAC => Self::info(
                "Meteor Lake-S",
                uarch::hybrid(uarch::RedwoodCove, uarch::Crestmont),
                ""
            ),
            0xB5 => Self::info(
                "Meteor Lake (0xB5)",
                "",
                "",
            ),

            /* Atom */
            /* Bonnell */
            0x1C => Self::info("Diamondville/Pineview", uarch::Bonnell, ProcessNode::NM(45)),
            0x26 => Self::info("Silverthorne/Lincroft", uarch::Bonnell, ProcessNode::NM(45)),
            /* Saltwell */
            0x36 => Self::info("Cedarview", uarch::Saltwell, ProcessNode::NM(32)),
            0x27 => Self::info("Penwell", uarch::Saltwell, ProcessNode::NM(32)),
            0x35 => Self::info("Cloverview", uarch::Saltwell, ProcessNode::NM(32)),
            /* Silvermont */
            0x37 => Self::info("Bay Trail/Valleyview", uarch::Silvermont, ProcessNode::NM(22)),
                /* Atom C2000 */
            0x4D => Self::info("Avoton/Rangely", uarch::Silvermont, ProcessNode::NM(22)),
            0x4A => Self::info("Merriefield/Tangier", uarch::Silvermont, ProcessNode::NM(22)),
            0x5D => Self::info("SoFIA 3G", uarch::Silvermont, ProcessNode::NM(28)), // ?
            /* Airmont */
            0x4C => Self::info("Cherry Trail/Braswell", uarch::Airmont, ProcessNode::NM(14)),
            0x5A => Self::info("Moorefield/Anniedale", uarch::Airmont, ProcessNode::NM(22)),
            0x65 => Self::info("XMM7272", uarch::Airmont, ""),
            0x6E => Self::info("Cougar Mountain", uarch::Airmont, ""),
            0x75 => Self::info("Lightning Mountain/Butter", uarch::Airmont, ProcessNode::NM(14)),
            /* Goldmont */
            0x5C => Self::info(
                &["Apollo Lake", match s {
                    0x8 => " (A0)",
                    0x9 => " (B0)",
                    0xA => " (E0)",
                    _ => "",
                }].concat(),
                uarch::Goldmont,
                ProcessNode::NM(14)
            ),
                /* Atom C3000 */
            0x5F => Self::info("Denverton", uarch::Goldmont, ProcessNode::NM(14)),
            /* Goldmont Plus */
            0x7A => Self::info(
                &["Gemini Lake", match s {
                    0x0 => " (A0)",
                    0x1 => " (B0)",
                    0x8 => " (R0)",
                    _ => "",
                }].concat(),
                uarch::GoldmontPlus,
                ProcessNode::NM(14)
            ),
            /* Tremont */
            0x86 => Self::info("Jacobsville", uarch::Tremont, ProcessNode::NM(10)),
            0x96 => Self::info("Elkhart Lake", uarch::Tremont, ProcessNode::NM(10)),
            0x9C => Self::info("Jasper Lake", uarch::Tremont, ProcessNode::NM(10)),
            /* Gracemont */
            0xBE => Self::info("Alder Lake-N", uarch::Gracemont, ProcessNode::Intel(7)),
            /* Crestmont ? */
            0xAF => Self::info("Sierra Forest", "", ProcessNode::Intel(3)),
            0xB6 => Self::info("Grand Ridge", "", ""),
            /* Xeon Phi */
            0x57 => Self::info("Knights Landing", uarch::KnightsLanding, ProcessNode::NM(14)),
            0x85 => Self::info("Knights Mill", uarch::KnightsMill, ProcessNode::NM(14)),
            _ => return None,
        })
    }
}
