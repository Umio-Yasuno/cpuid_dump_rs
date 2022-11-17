use std::fmt;

#[derive(Debug, Clone)]
pub enum TlbType {
    L1d,
    L1i,
    L2d,
    L2i,
}

impl TlbType {
    pub fn get_offset(&self) -> (u16, u16) {
        match self {
            Self::L1d |
            Self::L1i => (0xFF, 8),
            Self::L2d |
            Self::L2i => (0xFFF, 12),
        }
    }
}

impl fmt::Display for TlbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum TlbAssoc {
    Disabled,
    Way(u8),
    WayRange(std::ops::Range<u8>),
    Full,
    Invalid,
}

impl fmt::Display for TlbAssoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disabled |
            Self::Invalid => f.pad("0"),
            Self::Way(way) => f.pad(&way.to_string()),
            Self::WayRange(range) => f.pad(&format!("{:>2}-{}", range.start, range.end)),
            Self::Full => f.pad("full"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TlbInfo {
    pub size: u16,
    pub assoc: TlbAssoc,
}

impl TlbInfo {
    pub fn half_size(&self) -> Self {
        Self {
            size: self.size / 2,
            assoc: self.assoc.clone(),
        }
    }

    pub fn from_reg(reg: u16, offset: u16, shift: u16) -> Self {
        let tmp = reg >> shift;

        let assoc = if tmp == (u16::MAX >> shift) {
            TlbAssoc::Full
        } else if offset == 0xFF {
            /* L1d, L1i */
            TlbAssoc::Way(tmp as u8)
        } else if offset == 0xFFF {
            /* L2d, L2i */
            match tmp {
                0x0 => TlbAssoc::Disabled,
                0x1 => TlbAssoc::Way(1), // Direct Mapped
                0x2 => TlbAssoc::Way(2),
                0x3 => TlbAssoc::Way(3),
                0x4 => TlbAssoc::WayRange(4..6),
                0x5 => TlbAssoc::WayRange(6..8),
                0x6 => TlbAssoc::WayRange(8..16),
                // 0x7 => Permanently reserved,
                0x8 => TlbAssoc::WayRange(16..32),
                0x9 => TlbAssoc::Invalid,
                0xA => TlbAssoc::WayRange(32..48),
                0xB => TlbAssoc::WayRange(48..64),
                0xC => TlbAssoc::WayRange(64..96),
                0xD => TlbAssoc::WayRange(96..128),
                0xE => TlbAssoc::WayRange(128..255), // 128..Full
                _ => TlbAssoc::Invalid,
            }
        } else {
            TlbAssoc::Invalid
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
}

impl Tlb {
    pub fn reg(type_: TlbType, reg_4k: u16, reg_2m4m: u16) -> Self {
        let (offset, shift) = type_.get_offset();
        let page_4k = TlbInfo::from_reg(reg_4k, offset, shift);
        let page_2m = TlbInfo::from_reg(reg_2m4m, offset, shift);
        let page_4m = page_2m.half_size();

        Self {
            type_,
            page_4k,
            page_2m,
            page_4m,
        }
    }
}
