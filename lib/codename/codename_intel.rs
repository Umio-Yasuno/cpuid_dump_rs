use crate::ProcInfo;

impl ProcInfo {
pub fn fam06h(m: u32, s: u32) -> Self {
    match m {
    /* https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/intel-family.h */
        /*
            //  Core
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

        0x4E => Self::info("Skylake (Mobile)", "Skylake", "14 nm"),
        0x5E => Self::info("Skylake (Desktop)", "Skylake", "14 nm"),
        0x55 => match s {
            0x7 => Self::info("Cascade Lake (Server)", "Skylake AVX512/VNNI", "14 nm"),
            0xB => Self::info("Cooper Lake (Server)", "Skylake AVX512/VNNI/BF16", "14 nm"),
            _ => Self::info("Skylake (Server)", "Skylake AVX512", "14 nm"),
        },

        /* 10th Generation Intel® Core™ Processors Datasheet, Volume 1 of 2 */
        0x8E => match s {
            0x9 => Self::info("Amber Lake (Mobile)", "Skylake", "14 nm"),
            0xA => Self::info("Coffee Lake (Mobile)", "Skylake", "14 nm"),
            0xB => Self::info("Whiskey Lake (Mobile)", "Skylake", "14 nm"),
            0xC => Self::info("Comet/Whiskey Lake (Mobile) (V0)", "Skylake", "14 nm"),
            _ => Self::info("Kaby Lake (Mobile)", "Skylake", "14 nm"),
        },

        0x9E => match s {
            0xA | 0xB | 0xC => Self::info("Coffee Lake (Desktop)", "Skylake", "14 nm"),
            0xD => Self::info("Comet/Coffee Lake (ES) (Desktop) (R0)", "Skylake", "14 nm"),
            _ => Self::info("Kaby Lake (Desktop)", "Skylake", "14 nm"),
        },

        0xA5 => Self::info( &format!("Comet Lake (Desktop){}", match s {
            0x0 => " (G0)",
            0x1 => " (P0)",
            0x2 => " (R1)",
            0x3 => " (G1)",
            0x4 => " (P1)",
            0x5 => " (Q0)",
            _ => "",
        }), "Skylake", "14 nm"),

        0xA6 => Self::info( &format!("Comet Lake (Mobile){}", match s {
            0x0 => " (A0)",
            _ => "",
        }), "Skylake", "14 nm"),

        0x66 => Self::info("Cannon Lake (Mobile)", "Palm Cove", "10 nm"),

        0x6A => Self::info("Ice Lake (Server)", "Sunny Cove", "10nm"),
        0x6C => Self::info("Ice Lake (Micro Server)", "Sunny Cove", "10nm"),
        0x7D => Self::info("Ice Lake (Desktop)", "Sunny Cove", "10nm"),
        0x7E => Self::info("Ice Lake (Mobile)", "Sunny Cove", "10nm"),
        0x9D => Self::info("Ice Lake (NNPI)", "Sunny Cove", "10nm"),

        0xA7 => Self::info( &format!("Rocket Lake (Desktop){}", match s {
            0x1 => " (B0)",
            _   => "",
        }), "Cypress Cove", "14 nm"),

        0x8C => Self::info( &format!("Tiger Lake (Mobile){}", match s {
            0x1 => " (B0)",
            0x2 => " (C0)",
            _   => "",
        }), "Willow Cove", "10 nm SF"),
        0x8D => Self::info("Tiger Lake (Desktop)", "Willow Cove", "10 nm SF"),

        /*  Hybrid */
        0x8A => Self::info("Lakefield (1+4)", "Sunny Cove + Tremont", "10 nm"),

        /* https://edc.intel.com/content/www/us/en/design/ipla/software-development-platforms/client/platforms/alder-lake-desktop/12th-generation-intel-core-processors-datasheet-volume-1-of-2/005/cpuid/ */
        0x97 => Self::info( &format!("Alder Lake-S (Desktop){}", match s {
            0x2 => " (C0, ES/QS, 8+8)",
            0x5 => " (H0, 6+0)",
            0x0 |
            0x1 |
            0x4 => " (ES)",
            _   => "",
        }), "Golden Cove + Gracemont", "Intel 7 /10 nm eSF"),

        /* https://review.coreboot.org/c/coreboot/+/63299 */
        0x9A => {
            let (variant, stepping) = match s {
                /* Alder Lake-M */
                0x1 | 0x4 => ("M", match s {
                    0x1 => " (Q0, 2+8)",
                    0x4 => " (R0, 2+8)",
                    _ => " (2+8)",
                }),
                /* Alder Lake-P */
                _ => ("P", match s {
                    0x0 => " (J0, 6+8)",
                    0x2 => " (K0, 6+8)",
                    0x3 => " (L0, 6+8)",
                    0x4 => " (R0, 6+8)",
                    _ => " (6+8)",
                }),
            };

            Self::info(
                &format!("Alder Lake-{variant} (Mobile)({stepping}"),
                "Golden Cove + Gracemont",
                "Intel 7 /10 nm eSF",
            )
        },
        0xBE => Self::info("Alder Lake-N", "Gracemont ?", "Intel 7 /10 nm eSF"),
        /*
        0xBA => Self::info(
            &format("Raptor Lake-P (Mobile){}",
                match s {
                    0x2 => "(J0)",
                    _ => "",
                },
            ),
            "", ""),
        */

        0x8F => Self::info("Sapphire Rapids (Server)", "Golden Cove", "10nm eSF"),
        /*
                //  Atom
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
                //  Goldmont
                0x5C => format!("Apollo Lake"),
                0x5F => format!("Denverton"),
                //  Goldmont Plus
                0x7A => format!("Gemini Lake"),
                //  Tremont
                0x86 => format!("Jacobsville"),
                0x96 => format!("Elkhart Lake"),
                0x9C => format!("Jasper Lake"),

                //  Xeon Phi
                0x57 => format!("Knights Landing"),
                0x85 => format!("Knights Mill"),
        */
        _ => Self::info(&format!("F06h_M{m}_S{s}h"), "{{No Data}}", "{{No Data}}"),
    }
}
}
