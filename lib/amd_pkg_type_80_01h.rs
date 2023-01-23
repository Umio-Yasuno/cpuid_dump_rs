use crate::{cpuid, CpuidResult, FamModStep};

/* Leaf: 0x8000_0001, AMD CPU only */
/* ref: https://en.wikipedia.org/wiki/List_of_AMD_CPU_microarchitectures */
/* ref: https://www.amd.com/en/support/tech-docs */
/* ref: https://developer.amd.com/resources/developer-guides-manuals/ */
/* ref: https://github.com/illumos/illumos-gate/blob/master/usr/src/uts/intel/os/cpuid_subr.c */
/* ref: https://github.com/coreboot/coreboot/blob/master/src/soc/amd/picasso/include/soc/soc_util.h */

#[derive(Debug)]
pub enum AmdPkgType {
    F1207,
    AM2r2,
    S1g3,
    ASB2,
    S1g2,
    FS1,
    FM1,
    FT1,
    AM3r2,
    G34,
    C32,
    FP2,
    FS1r2,
    FM2,
    FP3,
    FT3,
    FS1b, // AM1
    FT3b,
    FP4,
    FT4,
    FM2r2,
    SP3,
    SP3r2, // same as TR4
    STRX4, // same as SP3r2/TR4?
    AM4,
    FP5,
    FT5,
    FP6,
    FF3,
    SP5,
    FP7,
    FP7r2,
    // FP7r7, // ?, AMD Ryzen 9 6900HS, Ryzen 7 6800HS
    AM5,
    FT6,
    Unknown(u32),
}

use std::fmt;
impl fmt::Display for AmdPkgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl From<&CpuidResult> for AmdPkgType {
    fn from(cpuid: &CpuidResult) -> Self {
        let fms = FamModStep::from(cpuid.eax);
        let pkg_type = cpuid.ebx >> 28;

        match fms {
            FamModStep { syn_fam: 0x10, .. } => match pkg_type {
                0x0 => Self::F1207,
                0x1 => Self::AM2r2,
                0x2 => Self::S1g3,
                0x3 => Self::G34,
                0x4 => Self::ASB2,
                0x5 => Self::C32,
                _ => Self::Unknown(pkg_type),
            },
            /* Griffin, Turion */
            FamModStep { syn_fam: 0x11, .. } => match pkg_type {
                0x2 => Self::S1g2,
                _ => Self::Unknown(pkg_type),
            },
            /* Llano */
            FamModStep { syn_fam: 0x12, .. } => match pkg_type {
                0x1 => Self::FS1,
                0x2 => Self::FM1,
                _ => Self::Unknown(pkg_type),
            },
            /* Bobcat */
            FamModStep { syn_fam: 0x14, .. } => match pkg_type {
                0x0 => Self::FT1,
                _ => Self::Unknown(pkg_type),
            },
            /* Bulldozer, Interlagos, Valencia, Zambezi */
            FamModStep { syn_fam: 0x15, syn_mod: 0x00..=0x0F, .. } => match pkg_type {
                0x1 => Self::AM3r2,
                0x3 => Self::G34,
                0x5 => Self::C32,
                _ => Self::Unknown(pkg_type),
            },
            /* Piledrive, Richland */
            FamModStep { syn_fam: 0x15, syn_mod: 0x10..=0x1F, .. } => match pkg_type {
                0x0 => Self::FP2,
                0x1 => Self::FS1r2,
                0x2 => Self::FM2,
                _ => Self::Unknown(pkg_type),
            },
            /* Kaveri, Godavari */
            FamModStep { syn_fam: 0x15, syn_mod: 0x30..=0x3F, .. } => match pkg_type {
                0x0 => Self::FP3,
                0x1 => Self::FM2r2,
                _ => Self::Unknown(pkg_type),
            },
            /* Carrizo, Bristol Ridge */
            FamModStep { syn_fam: 0x15, syn_mod: 0x60..=0x6F, .. } => match pkg_type {
                0x0 => Self::FP4,
                0x2 => Self::AM4,
                0x3 => Self::FM2r2,
                _ => Self::Unknown(pkg_type),
            },
            /* Stoney Ridge */
            FamModStep { syn_fam: 0x15, syn_mod: 0x70..=0x7F, .. } => match pkg_type {
                0x0 => Self::FP4,
                0x2 => Self::AM4,
                0x4 => Self::FT4,
                _ => Self::Unknown(pkg_type),
            },
            /* Jaguar, Kabini, Termash */
            FamModStep { syn_fam: 0x16, syn_mod: 0x00..=0x0F, .. } => match pkg_type {
                0x0 => Self::FT3,
                0x1 => Self::FS1b,
                _ => Self::Unknown(pkg_type),
            },
            /* Puma, Beema, Mullins */
            FamModStep { syn_fam: 0x16, syn_mod: 0x30..=0x3F, .. } => match pkg_type {
                0x0 => Self::FT3b,
                0x3 => Self::FP4,
                _ => Self::Unknown(pkg_type),
            },
            /* Summit Ridge, Naples */
            FamModStep { syn_fam: 0x17, syn_mod: 0x00..=0x0F, .. } |
            /* Raven, Picasso */
            FamModStep { syn_fam: 0x17, syn_mod: 0x10..=0x1F, .. } |
            /* Raven2 (Dali, Pollock) */
            FamModStep { syn_fam: 0x17, syn_mod: 0x20, .. } |
            /* Rome */
            FamModStep { syn_fam: 0x17, syn_mod: 0x30..=0x3F, .. } |
            /* Matisse */
            FamModStep { syn_fam: 0x17, syn_mod: 0x71, .. } => match pkg_type {
                0x0 => Self::FP5,
                0x2 => Self::AM4,
                0x3 => Self::FT5,
                0x4 => Self::SP3,
                0x7 => Self::SP3r2,
                _ => Self::Unknown(pkg_type),
            },
            /* Renoir, Lucienne */
            FamModStep { syn_fam: 0x17, syn_mod: 0x60..=0x6F, .. } |
            /* Cezanne, Barcelo */
            FamModStep { syn_fam: 0x19, syn_mod: 0x50..=0x5F, .. } => match pkg_type {
                0x0 => Self::FP6,
                0x2 => Self::AM4,
                _ => Self::Unknown(pkg_type),
            },
            /* Milan, Chagall */
            FamModStep { syn_fam: 0x19, syn_mod: 0x00..=0x0F, .. } => match pkg_type {
                0x4 => Self::SP3,
                0x7 => Self::STRX4,
                _ => Self::Unknown(pkg_type),
            },
            /* Rembrandt */
            FamModStep { syn_fam: 0x19, syn_mod: 0x40..=0x4F, .. } => match pkg_type {
                0x0 => Self::AM5,
                0x1 => Self::FP7,
                0x2 => Self::FP7r2,
                /* AMD Ryzne 7 6800H: https://linux-hardware.org/?probe=147556bf0a&log=cpuid */
                0x4 => Self::FP7r2,
                /* AMD Ryzen 5 9 PRO 6950HS: https://linux-hardware.org/?probe=b4a34edd03&log=cpuid */
                0x5 => Self::FP7r2,
                _ => Self::Unknown(pkg_type),
            },
            /* Mendocino */
            FamModStep { syn_fam: 0x17, syn_mod: 0xA0..=0xAF, .. } => match pkg_type {
                0x1 => Self::FT6,
                _ => Self::Unknown(pkg_type),
            },
            /* VanGogh */
            FamModStep { syn_fam: 0x17, syn_mod: 0x90, .. } => match pkg_type {
                0x3 => Self::FF3,
                _ => Self::Unknown(pkg_type),
            },
            /* Genoa */
            FamModStep { syn_fam: 0x19, syn_mod: 0x10..=0x1F, .. } => match pkg_type {
                0x4 => Self::SP5,
                _ => Self::Unknown(pkg_type),
            },
            /* Raphael */
            FamModStep { syn_fam: 0x19, syn_mod: 0x60..=0x6F, .. } => match pkg_type {
                0x0 => Self::AM5,
                _ => Self::Unknown(pkg_type),
            },
            _ => Self::Unknown(pkg_type),
        }
    }
}

impl AmdPkgType {
    pub fn get() -> Self {
        Self::from(&cpuid!(0x8000_0001, 0x0))
    }
}
