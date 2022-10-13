use crate::ProcInfo;
/* ref:
    https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/intel-family.h
    https://github.com/coreboot/coreboot/blob/master/src/include/cpu/intel/cpu_ids.h */

// TODO: use enum and fmt::Display ?

impl ProcInfo {
pub(super) fn fam06h(m: u32, s: u32) -> Self {
    match m {
        /*
            /* Core */
            0x0E => format!("Yonah"),
            0x0F => format!("Merom"),
            0x16 => format!("Merom (Mobile)"),
            0x17 => format!("Penryn"),
            0x1D => format!("Dunnington"),

            //  Nehalem
            0x1E => format!("Nehalem"),
            0x1F => format!("Auburndale/Havendale"),
            0x1A => format!("Nehalem-EP"),
            0x2E => format!("Nehalem-EX"),

            //  Westmere
            0x25 => format!("Westmere"),
            0x2C => format!("Westmere-EP"),
            0x2F => format!("Westmere-EX"),

            //  Sandy Bridge
            0x2A => format!("Sandy Bridge"),
            0x2D => format!("Sandy Bridge (Server)"),

            //  Ivy Bridge
            0x3A => format!("Ivy Bridge"),
            0x3E => format!("Ivy Bridge (Server)"),

            //  Haswell
            0x3C => format!("Haswell"),
            0x3F => format!("Haswell (Server)"),
            0x45 => format!("Haswell (Mobile)"),
            0x46 => format!("Haswell (Iris Pro)"),
        */
        0x3D => Self::info("Broadwell (Desktop)", "Broadwell", "14 nm"),
        0x47 => Self::info("Broadwell (Iris Pro)", "Broadwell", "14 nm"),
        0x4F => Self::info("Broadwell (Server)", "Broadwell", "14 nm"),
        0x56 => Self::info("Broadwell (Micro Server)", "Broadwell", "14 nm"),

        0x4E => Self::info("Skylake-U", "Skylake", "14 nm"),
        0x5E => Self::info(match s {
            0x8 => "Kaby Lake (HA0)",
            _ => "Skylake-S",
        }, "Skylake", "14 nm"),
        0x55 => match s {
            0x7 => Self::info("Cascade Lake-SP", "Skylake AVX512/VNNI", "14 nm"),
            0xA |
            0xB => Self::info("Cooper Lake-SP", "Skylake AVX512/VNNI/BF16", "14 nm"),
            _ => Self::info("Skylake-SP", "Skylake AVX512", "14 nm"),
        },

        /* 10th Generation Intel® Core™ Processors Datasheet, Volume 1 of 2 */
        /* https://www.intel.com/content/www/us/en/content-details/615211/10th-generation-intel-core-processors-datasheet-volume-1-of-2.html */
        /* https://github.com/coreboot/coreboot/blob/master/src/soc/intel/skylake/Makefile.inc */
        0x8E => match s {
            0x9 => Self::info("Amber Lake-Y", "Skylake", "14 nm"),
            0xA => Self::info("Coffee Lake-U", "Skylake", "14 nm"),
            0xB => Self::info("Whiskey Lake-U (W0)", "Skylake", "14 nm"),
            0xC => Self::info("Comet/Whiskey Lake-U (V0)", "Skylake", "14 nm"),
            _ => Self::info("Kaby Lake-U", "Skylake", "14 nm"),
        },

        0x9E => match s {
            0x9 => Self::info("Kaby Lake-S (HB0)", "Skylake", "14 nm"),
            0xA |
            0xB |
            0xC => Self::info("Coffee Lake-S", "Skylake", "14 nm"),
            0xD => Self::info("Comet/Coffee Lake (ES) (R0)", "Skylake", "14 nm"),
            _ => Self::info("Kaby Lake-S", "Skylake", "14 nm"),
        },

        0xA5 => Self::info(&[
            "Comet Lake-S",
            match s {
                0x0 => " (G0)",
                0x1 => " (P0)",
                0x2 => " (R1)",
                0x3 => " (G1)",
                0x4 => " (P1)",
                0x5 => " (Q0)",
                _ => "",
            },
        ].concat(), "Skylake", "14 nm"),

        0xA6 => Self::info(&[
            "Comet Lake-U",
            match s {
                0x0 => " (A0)",
                _ => "",
            }
        ].concat(), "Skylake", "14 nm"),

        0x66 => Self::info("Cannon Lake-U", "Palm Cove", "10 nm"),

        0x6A => Self::info("Ice Lake-SP", "Sunny Cove", "10nm"),
        0x6C => Self::info("Ice Lake-D", "Sunny Cove", "10nm"),
        0x7D => Self::info("Ice Lake-S", "Sunny Cove", "10nm"),
        0x7E => Self::info("Ice Lake-U", "Sunny Cove", "10nm"),
        0x9D => Self::info("Ice Lake-NNPI", "Sunny Cove", "10nm"),

        0xA7 => Self::info(&[
            "Rocket Lake-S",
            match s {
                0x0 => " (A0)",
                0x1 => " (B0)",
                _   => "",
            }
        ].concat(), "Cypress Cove", "14 nm"),
        /* Xeon W-1300 */
        /* 06_A8H: https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/best-practices/data-operand-independent-timing-isa-guidance.html */
        0xA8 => Self::info("Rocket Lake (WS)", "Cypress Cove", "14 nm"),
        
        0x8C => Self::info(&[
            "Tiger Lake-U",
            match s {
                0x0 => " (A0)",
                0x1 => " (B0)",
                0x2 => " (C0)",
                _   => "",
            }
        ].concat(), "Willow Cove", "10 nm SF"),

        0x8D => Self::info("Tiger Lake-H", "Willow Cove", "10 nm SF"),
        0x8F => Self::info("Sapphire Rapids-SP", "Golden Cove", "10nm eSF"),

        /* Hybrid */
        0x8A => Self::info("Lakefield (1+4)", "Sunny Cove + Tremont", "10 nm"),

        /* https://edc.intel.com/content/www/us/en/design/ipla/software-development-platforms/client/platforms/alder-lake-desktop/12th-generation-intel-core-processors-datasheet-volume-1-of-2/005/cpuid/ */
        /* https://github.com/coreboot/coreboot/blob/master/src/soc/intel/alderlake/Makefile.inc */
        0x97 => Self::info(&[
            "Alder Lake-S",
            match s {
                0x0 => " (A0, ES)",
                0x1 => " (B0, ES)",
                0x2 => " (C0, 8+8)",
                0x4 => " (G0, ES)",
                0x5 => " (H0, 6+0)",
                _   => "",
            }
        ].concat(), "Golden Cove + Gracemont", "Intel 7"),
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
                "Golden Cove + Gracemont",
                "Intel 7",
            )
        },
        0xB7 => Self::info("Raptor Lake-S", "Golden Cove + Gracemont", "Intel 7"),
        0xBA => Self::info("Raptor Lake-P", "Golden Cove + Gracemont", "Intel 7"),
        0xBF => Self::info("Raptor Lake-S (0xBF)", "Golden Cove + Gracemont", "Intel 7"),
        0xAA => Self::info("Meteor Lake-M/P", "Redwood Cove + Crestmont", ""),
        0xAC => Self::info("Meteor Lake-S", "Redwood Cove + Crestmont", ""),

        /* Atom */
        /* Goldmont */
        0x5C => Self::info(&["Apollo Lake", match s {
            0x8 => " (A0)",
            0x9 => " (B0)",
            0xA => " (E0)",
            _ => "",
        }].concat(), "Goldmont", "14 nm"),
        0x5F => Self::info("Denverton", "Goldmont", "14 nm"),
        /* Goldmont Plus */
        0x7A => Self::info(&["Gemini Lake", match s {
            0x0 => " (A0)",
            0x1 => " (B0)",
            0x8 => " (R0)",
            _ => "",
        }].concat(), "Goldmont Plus", "14 nm"),

        /* Tremont */
        0x86 => Self::info("Jacobsville", "Tremont", "10 nm"),
        0x96 => Self::info("Elkhart Lake", "Tremont", "10 nm"),
        0x9C => Self::info("Jasper Lake", "Tremont", "10 nm"),
        /* Gracemont */
        0xBE => Self::info("Alder Lake-N", "Gracemont", "Intel 7"),
        0xAF => Self::info("Sierra Forest", "", "Intel 3"),
        0xB6 => Self::info("Grand Ridge", "", ""),
        /*
                //  Bonnell
                0x1C => format!("Diamondville/Pineview"),
                0x26 => format!("Silverthorne/Lincroft"),
                //  Saltwell
                0x36 => format!("Cedarview"),
                0x27 => format!("Penwell"),
                0x35 => format!("Cloverview"),
                //  Silvermont
                0x37 => format!("Bay Trail/Valleyview"),
                0x4D => format!("Avaton/Rangely"),
                0x4A => format!("Marriefield"),
                //  Airmont
                0x4C => format!("Cherry Trail/Braswell"),
                0x5A => format!("Moorefield"),
                0x75 => format!("Lightning Mountain"),
                //  Xeon Phi
                0x57 => format!("Knights Landing"),
                0x85 => format!("Knights Mill"),
        */
        _ => Self::info(&format!("F06h_M{m}_S{s}h"), "", ""),
    }
}
}
