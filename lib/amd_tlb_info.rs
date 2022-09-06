use std::fmt;

#[derive(Debug, Clone)]
pub enum TlbType {
    L1d,
    L1i,
    L2d,
    L2i,
}

impl TlbType {
    pub fn get_offset(&self) -> u16 {
        match self {
            Self::L1d |
            Self::L1i => 0xFF,
            Self::L2d |
            Self::L2i => 0xFFF,
        }
    }
}

impl fmt::Display for TlbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L1d => write!(f, "L1d"),
            Self::L1i => write!(f, "L1i"),
            Self::L2d => write!(f, "L2d"),
            Self::L2i => write!(f, "L2i"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TlbAssoc {
    Way(u8),
    Full,
}

impl fmt::Display for TlbAssoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Way(way) => write!(f, "{way}"),
            Self::Full => write!(f, "full"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TlbInfo {
    pub size: u16,
    pub assoc: TlbAssoc,
}

impl TlbInfo {
    pub fn from_reg(reg: u16, offset: u16) -> Self {
        let shift = offset.trailing_ones();
        let assoc = reg >> shift;

        let assoc = if assoc == (u16::MAX >> shift) {
            TlbAssoc::Full
        } else if offset == 0xFF {
            TlbAssoc::Way(assoc as u8)
        } else {
            match assoc {
                0x0 => TlbAssoc::Way(0), // Disabled
                0x1 => TlbAssoc::Way(1), // Direct Mapped
                0x2 => TlbAssoc::Way(2),
                // 0x3 => Reserved,
                0x4 => TlbAssoc::Way(4),
                // 0x5 => Reserved,
                0x6 => TlbAssoc::Way(8),
                // 0x7 => Reserved,
                0x8 => TlbAssoc::Way(16),
                // 0x9 => Reserved,
                0xA => TlbAssoc::Way(32),
                0xB => TlbAssoc::Way(48),
                0xC => TlbAssoc::Way(64),
                0xD => TlbAssoc::Way(96),
                0xE => TlbAssoc::Way(128),
                _ => TlbAssoc::Way(assoc as u8),
            }
        };


        Self {
            size: reg & offset,
            assoc,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tlb {
    pub type_: TlbType,
    pub page_4k: TlbInfo,
    pub page_2m: TlbInfo,
    pub page_4m: TlbInfo,
    // page_1g
}

impl Tlb {
    pub fn reg(type_: TlbType, reg_4k: u16, reg_2m4m: u16) -> Self {
        let offset = type_.get_offset();
        let page_4k = TlbInfo::from_reg(reg_4k, offset);
        let page_2m = TlbInfo::from_reg(reg_2m4m, offset);
        let page_4m = TlbInfo {
            size: page_2m.size / 2,
            assoc: page_2m.assoc,
        };

        Self {
            type_,
            page_4k,
            page_2m,
            page_4m,
        }
    }
}
