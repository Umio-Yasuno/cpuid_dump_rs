use crate::*;

pub fn fam06h(m: u32, s: u32) -> ProcInfo {
    match m {
        //  https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/intel-family.h
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

            //  Broadwell
        */
        0x3D => ProcInfo::info("Broadwell (Desktop)", "Broadwell", "14 nm"),
        0x47 => ProcInfo::info("Broadwell (Iris Pro)", "Broadwell", "14 nm"),
        0x4F => ProcInfo::info("Broadwell (Server)", "Broadwell", "14 nm"),
        0x56 => ProcInfo::info("Broadwell (Micro Server)", "Broadwell", "14 nm"),

        0x4E => ProcInfo::info("Skylake (Mobile)", "Skylake", "14 nm"),
        0x5E => ProcInfo::info("Skylake (Desktop)", "Skylake", "14 nm"),
        0x55 => match s {
            0x7 => ProcInfo::info("Cascade Lake (Server)", "Skylake AVX512/VNNI", "14 nm"),
            0xB => ProcInfo::info("Cooper Lake (Server)", "Skylake AVX512/VNNI/BF16", "14 nm"),
            _ => ProcInfo::info("Skylake (Server)", "Skylake AVX512", "14 nm"),
        },

        // 10th Generation Intel® Core™ Processors Datasheet, Volume 1 of 2
        0x8E => match s {
            0x9 => ProcInfo::info("Amber Lake (Mobile)", "Skylake", "14 nm"),
            0xA => ProcInfo::info("Coffee Lake (Mobile)", "Skylake", "14 nm"),
            0xB => ProcInfo::info("Whiskey Lake (Mobile)", "Skylake", "14 nm"),
            0xC => ProcInfo::info("Comet/Whiskey Lake (Mobile) (V0)", "Skylake", "14 nm"),
            _ => ProcInfo::info("Kaby Lake (Mobile)", "Skylake", "14 nm"),
        },

        0x9E => match s {
            0xA | 0xB | 0xC => ProcInfo::info("Coffee Lake (Desktop)", "Skylake", "14 nm"),
            0xD => ProcInfo::info("Comet/Coffee Lake (ES) (Desktop) (R0)", "Skylake", "14 nm"),
            _ => ProcInfo::info("Kaby Lake (Desktop)", "Skylake", "14 nm"),
        },

        0xA5 => ProcInfo::info( &format!("Comet Lake (Desktop){}", match s {
            0x0 => " (G0)",
            0x3 => " (G1)",
            0x1 => " (P0)",
            0x4 => " (P1)",
            0x5 => " (Q0)",
            0x2 => " (R1)",
            _ => "",
        }), "Skylake", "14 nm"),

        0xA6 => ProcInfo::info( match s {
            0x0 => "Comet Lake (Mobile) (A0)",
            _   => "Comet Lake (Mobile)",
        }, "Skylake", "14 nm"),

        0x66 => ProcInfo::info("Cannon Lake (Mobile)", "Palm Cove", "10 nm"),

        0x6A => ProcInfo::info("Ice Lake (Server)", "Sunny Cove", "10nm"),
        0x6C => ProcInfo::info("Ice Lake (Micro Server)", "Sunny Cove", "10nm"),
        0x7D => ProcInfo::info("Ice Lake (Desktop)", "Sunny Cove", "10nm"),
        0x7E => ProcInfo::info("Ice Lake (Mobile)", "Sunny Cove", "10nm"),
        0x9D => ProcInfo::info("Ice Lake (NNPI)", "Sunny Cove", "10nm"),

        0xA7 => ProcInfo::info( &format!("Rocket Lake (Desktop){}", match s {
            0x1 => " (B0)",
            _   => "",
        }), "Cypress Cove", "14 nm"),

        0x8C => ProcInfo::info( &format!("Tiger Lake (Mobile){}", match s {
            0x1 => " (B0)",
            0x2 => " (C0)",
            _   => "",
        }), "Willow Cove", "10 nm SF"),
        0x8D => ProcInfo::info("Tiger Lake (Desktop)", "Willow Cove", "10 nm SF"),

        //  Hybrid
        0x8A => ProcInfo::info("Lakefield", "Sunny Cove + Tremont", "10 nm"),

        0x97 => ProcInfo::info( &format!("Alder Lake-S (Desktop){}", match s {
            0x2 => " (C0, 8+8)",
            0x5 => " (H0, 6+0)",
            0x3 => " (L0, 6+8)",
            _   => "",
        }), "Golden Cove + Gracemont", "Intel 7 /10 nm eSF"),

        0x9A => match s {
            0x1 => ProcInfo::info(
                "Alder Lake-M (Mobile)",
                "Golden Cove + Gracemont", "Intel 7 /10 nm eSF"
            ),
            _ => ProcInfo::info(
                "Alder Lake-P (Mobile)",
                "Golden Cove + Gracemont", "Intel 7 /10 nm eSF"
            ),
        },
        0xBE => ProcInfo::info("Alder Lake-N", "Gracemont ?", "Intel 7 /10 nm eSF"),

        0x8F => ProcInfo::info("Sapphire Rapids (Server)", "Golden Cove", "10nm eSF"),
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
        _ => ProcInfo {
            codename: format!("F06h_M{}h_S{}h", m, s),
            archname: "Unknown".to_string(),
            process: "".to_string(),
        },
    }
}
