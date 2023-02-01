#[cfg(feature = "std")]
use std::fmt;

#[derive(Debug, Clone)]
pub enum TlbType {
    L1d,
    L1i,
    L2d,
    L2i,
}

#[cfg(feature = "std")]
impl fmt::Display for TlbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum TlbAssoc {
    Disabled,
    Way(u8),
    WayRange(core::ops::Range<u8>),
    Full,
    Invalid,
}

#[cfg(feature = "std")]
impl fmt::Display for TlbAssoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disabled |
            Self::Invalid => f.pad("0"),
            Self::Way(way) => f.pad(&way.to_string()),
            Self::WayRange(range) => f.pad(&format!("{:>2}-{}", range.start, range.end - 1)),
            Self::Full => f.pad("full"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TlbInfo {
    pub size: u16,
    pub assoc: TlbAssoc,
}

#[cfg(feature = "std")]
impl fmt::Display for TlbInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>4}_entry, {:>6}_way", self.size, self.assoc)
    }
}

impl TlbInfo {
    pub fn half_size(&self) -> Self {
        Self {
            size: self.size / 2,
            assoc: self.assoc.clone(),
        }
    }

    pub fn from_reg_l1(reg: u16) -> Self {
        let (size, assoc) = (reg & 0xFF, (reg >> 8) as u8);

        Self {
            size,
            assoc: match assoc {
                0x0 => TlbAssoc::Invalid, // Reserved
                0xFF => TlbAssoc::Full,
                _ => TlbAssoc::Way(assoc),
            },
        }
    }

    pub fn from_reg_l2(reg: u16) -> Self {
        let (size, assoc) = (reg & 0xFFF, reg >> 12);

        Self {
            size,
            assoc: match assoc {
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
                0xF => TlbAssoc::Full,
                _ => TlbAssoc::Invalid,
            },
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
        let [page_4k, page_2m] = match type_ {
            TlbType::L1d |
            TlbType::L1i => [TlbInfo::from_reg_l1(reg_4k), TlbInfo::from_reg_l1(reg_2m4m)],
            TlbType::L2d |
            TlbType::L2i => [TlbInfo::from_reg_l2(reg_4k), TlbInfo::from_reg_l2(reg_2m4m)],
        };
        let page_4m = page_2m.half_size();

        Self {
            type_,
            page_4k,
            page_2m,
            page_4m,
        }
    }
}
