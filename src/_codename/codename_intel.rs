use crate::{info, ProcInfo};

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
        0x3D => info!("Broadwell (Desktop)", "Broadwell", "14 nm"),
        0x47 => info!("Broadwell (Iris Pro)", "Broadwell", "14 nm"),
        0x4F => info!("Broadwell (Server)", "Broadwell", "14 nm"),
        0x56 => info!("Broadwell (Micro Server)", "Broadwell", "14 nm"),

        0x4E => info!("Skylake (Mobile)", "Skylake", "14 nm"),
        0x5E => info!("Skylake (Desktop)", "Skylake", "14 nm"),
        0x55 => match s {
            0x9 => info!("Cascade Lake (Server)", "Skylake AVX512/VNNI", "14 nm"),
            0xB => info!("Cooper Lake (Server)", "Skylake AVX512/VNNI/BF16", "14 nm"),
            _ => info!("Skylake (Server)", "Skylake AVX512", "14 nm"),
        },
        0x8E => match s {
            0x9 => info!("Amber Lake (Mobile)", "Skylake", "14 nm"),
            0xA => info!("Coffee Lake (Mobile)", "Skylake", "14 nm"),
            0xB | 0xC => info!("Whiskey Lake (Mobile)", "Skylake", "14 nm"),
            _ => info!("Kaby Lake (Mobile)", "Skylake", "14 nm"),
        },
        0x9E => match s {
            0xA | 0xB | 0xC => info!("Coffee Lake (Desktop)", "Skylake", "14 nm"),
            _ => info!("Kaby Lake (Desktop)", "Skylake", "14 nm"),
        },

        0xA5 => info!("Comet Lake (Desktop)", "Skylake", "14 nm"),
        0xA6 => info!("Comet Lake (Mobile)", "Skylake", "14 nm"),

        0x66 => info!("Cannon Lake (Mobile)", "Palm Cove", "10 nm"),

        0x6A => info!("Ice Lake (Server)", "Sunny Cove", "10nm"),
        0x6C => info!("Ice Lake (Micro Server)", "Sunny Cove", "10nm"),
        0x7D => info!("Ice Lake (Desktop)", "Sunny Cove", "10nm"),
        0x7E => info!("Ice Lake (Mobile)", "Sunny Cove", "10nm"),
        0x9D => info!("Ice Lake (NNPI)", "Sunny Cove", "10nm"),

        0xA7 => info!("Rocket Lake (Desktop)", "Cypress Cove", "14 nm"),

        0x8C => info!("Tiger Lake (Mobile)", "Willow Cove", "10 nm SF"),
        0x8D => info!("Tiger Lake (Desktop)", "Willow Cove", "10 nm SF"),

        //  Hybrid
        0x8A => info!("Lakefield", "Sunny Cove/Tremont", "10 nm"),

        0x97 => info!("Alder Lake", "Golden Cove/Gracemont", "Intel 7 /10 nm eSF"),
        0x9A => match s {
            0x1 => info!(
                "Alder Lake-M (Mobile)",
                "Golden Cove/Gracemont", "Intel 7 /10 nm eSF"
            ),
            _ => info!(
                "Alder Lake-P (Mobile)",
                "Golden Cove/Gracemont", "Intel 7 /10 nm eSF"
            ),
        },
        0xBE => info!("Alder Lake-N", "Gracemont", "Intel 7 /10 nm eSF"),

        0x8F => info!("Sapphire Rapids (Server)", "Golden Cove", "10nm eSF"),
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
