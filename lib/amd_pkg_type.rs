use crate::{_AX, cpuid, CpuidResult, FamModStep};

/* Leaf: 0x8000_0001, AMD CPU only */
/* ref: https://www.amd.com/en/support/tech-docs */
/* ref: https://github.com/illumos/illumos-gate/blob/master/usr/src/uts/intel/os/cpuid_subr.c */
/* ref: https://github.com/coreboot/coreboot/blob/master/src/soc/amd/picasso/include/soc/soc_util.h */

#[derive(Debug)]
pub enum AmdPkgType {
    FT3, // Family 16h Models 00h-0Fh
    FS1b, // Family 16h Models 00h-0Fh
    FT3b, // Family 16h Models 30h-3Fh
    FP4, // Family 15h Models 70h-7Fh, Family 16h Models 30h-3Fh
    FT4, // Family 15h Models 70h-7Fh
    SP3,
    SP3r2, // same as TR4
    STRX4,
    AM4,
    FP5,
    FT5,
    FP6,
    FF3,
    SP5,
    FP7,
    FP7r2,
    AM5,
    FT6,
    Unknown,
}

impl AmdPkgType {
    pub fn from_cpuid(cpuid: &CpuidResult) -> Self {
        let fms = FamModStep::from_cpuid(cpuid.eax);
        let pkg_type = cpuid.ebx >> 28;

        match fms {
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
                _ => Self::Unknown,
            },
            /* Renoir, Lucienne */
            FamModStep { syn_fam: 0x17, syn_mod: 0x60..=0x6F, .. } |
            /* Cezanne, Barcelo */
            FamModStep { syn_fam: 0x19, syn_mod: 0x50..=0x5F, .. } => match pkg_type {
                0x0 => Self::FP6,
                0x2 => Self::AM4,
                _ => Self::Unknown,
            },
            /* Milan, Chagall */
            FamModStep { syn_fam: 0x19, syn_mod: 0x00..=0x0F, .. } => match pkg_type {
                0x4 => Self::SP3,
                0x7 => Self::STRX4,
                _ => Self::Unknown,
            },
            /* Rembrandt */
            FamModStep { syn_fam: 0x19, syn_mod: 0x40..=0x4F, .. } => match pkg_type {
                0x0 => Self::AM5,
                0x1 => Self::FP7,
                0x2 => Self::FP7r2,
                _ => Self::Unknown,
            },
            /* Mendocino */
            FamModStep { syn_fam: 0x17, syn_mod: 0xA0..=0xAF, .. } => match pkg_type {
                0x1 => Self::FT6,
                _ => Self::Unknown,
            },
            /* VanGogh */
            FamModStep { syn_fam: 0x17, syn_mod: 0x90, .. } => match pkg_type {
                0x3 => Self::FF3,
                _ => Self::Unknown,
            },
            /* Genoa */
            FamModStep { syn_fam: 0x19, syn_mod: 0x10..=0x1F, .. } => match pkg_type {
                0x4 => Self::SP5,
                _ => Self::Unknown,
            },
            /* Raphael */
            FamModStep { syn_fam: 0x19, syn_mod: 0x60..=0x6F, .. } => match pkg_type {
                0x0 => Self::AM5,
                _ => Self::Unknown,
            },
            _ => Self::Unknown,
        }
    }

    pub fn get() -> Self {
        Self::from_cpuid(&cpuid!(_AX+0x1, 0x0))
    }
}
