//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

//  f: Family, m: Model, s: Stepping
pub fn get_codename(f: u32, m: u32, s: u32) -> String {

    return match f {
        0x6 => match m {
        //  https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/intel-family.h
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
            0x3D => format!("Broadwell"),
            0x47 => format!("Broadwell (Iris Pro)"),
            0x4F => format!("Broadwell (Server)"),
            0x56 => format!("Broadwell (Micro Server)"),

            //  Skylake
            0x4E => format!("Skylake (Mobile)"),
            0x5E => format!("Skylake"),
            0x55 => match s {
                0x9 => format!("Cascade Lake (Server)"),
                0xB => format!("Cooper Lake (Server)"),
                _   => format!("Skylake (Server)"),
            },
            0x8E => match s {
                0x9 => format!("Amber Lake (Mobile)"),
                0xA => format!("Coffee Lake (Mobile)"),
                0xB|
                0xC => format!("Whiskey Lake (Mobile)"),
                _   => format!("Kaby Lake (Mobile)"),
            },
            0x9E => match s {
                0xA|
                0xB|
                0xC => format!("Coffee Lake"),
                _   => format!("Kaby Lake"),
            },
            0xA5 => format!("Comet Lake"),
            0xA6 => format!("Comet Lake (Mobile)"),

            //  Palm Cove
            0x66 => format!("Cannon Lake (Mobile)"),

            //  Sunny Cove
            0x6A => format!("Ice Lake (Server)"),
            0x6C => format!("Ice Lake (Micro Server)"),
            0x7D => format!("Ice Lake"),
            0x7E => format!("Ice Lake (Mobile)"),
            0x9D => format!("Ice Lake (NNPI)"),

            //  Cypress Cove
            0xA7 => format!("Rocket Lake"),

            //  Willow Cove
            0x8C => format!("Tiger Lake (Mobile)"),
            0x8D => format!("Tiger Lake"),

            0x8F => format!("Sapphire Rapids (Server)"),

            //  Hybrid
            0x8A => format!("Lakefield"),
            0x97 => format!("Alder Lake"),
            0x9A => match s {
                0x1  => format!("Alder Lake-M (Mobile)"),
                _    => format!("Alder Lake-P (Mobile)"),
            },

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

            _    => format!("Unknown Intel CPU"),
        },
        0x5 => format!("Quark X1000"),

        //  AMD
        0x17 => match m {
            //  Zen
            0x01 => match s {
                0x01 => format!("Summit Ridge"),
                0x02 => format!("Naples"),
                _    => format!("Zen"),
            },
            0x08 => format!("Pinnacle Ridge"),
            0x11 => format!("Raven Ridge"),
            0x18 => format!("Picasso"),
            0x20 => format!("Raven2 (Dali/Pollock)"),
            //  Zen 2
            0x31 => format!("Rome"),
            0x60 => format!("Renoir"),
            0x68 => format!("Lucienne"),
            0x71 => format!("Matisse"),
            0x90 => format!("VanGogh"),
            _    => format!("Zen"),
        },
        //  Zen 3
        0x19 => match m {
            0x01 => format!("Milan"),
            0x21 => format!("Vermeer"),
            0x50 => format!("Cezanne"),
            _    => format!("Zen 3"),
        }
        _ => format!("Unknown"),
    }
}
