use crate::{cpuid, CpuidResult, CpuVendor};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Unit {
    Byte,
    KiB,
    MiB,
    GiB,
}

impl Unit {
    pub(self) const KIB_BYTE: u32 = 1 << 10;
    pub(self) const MIB_BYTE: u32 = 1 << 20;
    pub(self) const GIB_BYTE: u32 = 1 << 30;

    pub fn to_byte(&self) -> u32 {
        match self {
            Self::Byte => 1,
            Self::KiB => Self::KIB_BYTE,
            Self::MiB => Self::MIB_BYTE,
            Self::GiB => Self::GIB_BYTE,
        }
    }
}

impl From<u32> for Unit {
    fn from(byte: u32) -> Self {
        match byte {
            Self::GIB_BYTE.. => Self::GiB,
            Self::MIB_BYTE.. => Self::MiB,
            Self::KIB_BYTE.. => Self::KiB,
            _ => Self::Byte,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CacheType {
    Data,
    Instruction,
    Unified,
    Unknown,
}

impl CacheType {
    pub fn from_reg(reg: u32) -> Self {
        match reg {
            0x1 => Self::Data,
            0x2 => Self::Instruction,
            0x3 => Self::Unified,
            // 0x0 |
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for CacheType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CacheProp {
    pub cache_type: CacheType,
    pub level: u32,
    pub line_size: u32,
    pub way: u32,
    pub set: u32,
    pub size: u32,
    pub size_unit: Unit,
    pub share_thread: u32,
    pub inclusive: bool,
}

impl From<&CpuidResult> for CacheProp {
    fn from(cpuid: &CpuidResult) -> Self {
        let CpuidResult { eax, ebx, ecx, edx } = cpuid;

        let cache_type = CacheType::from_reg(eax & 0x1F);

        let level = (eax >> 5) & 0b111;
        let line_size = (ebx & 0xFFF) + 1;
        let way = (ebx >> 22) + 1;
        let set = ecx + 1;
        let size = line_size * way * set;

        let share_thread = ((eax >> 14) & 0xFFF) + 1;

        let size_unit = Unit::from(size);

        let inclusive = (edx & 0b10) != 0;

        Self {
            cache_type,
            level,
            line_size,
            way,
            set,
            size,
            size_unit,
            share_thread,
            inclusive,
        }
    }
}

impl CacheProp {
    pub fn get_cache_prop_leaf() -> Option<u32> {
        match CpuVendor::get() {
            CpuVendor::AuthenticAMD => {
                /* AMD TopologyExtensions: CPUID[Leaf=0x8000_0001, SubLeaf=0x0].ECX[22] */
                let amd_topo_ext = ((cpuid!(0x8000_0001, 0x0).ecx >> 22) & 0b1) != 0;

                if amd_topo_ext {
                    Some(0x8000_001D)
                } else {
                    None
                }
            },
            CpuVendor::GenuineIntel => Some(0x4),
            _ => None,
        }
    }

    pub fn option_from_cpuid(cpuid: &CpuidResult) -> Option<Self> {
        let prop = Self::from(cpuid);

        if prop.level != 0 {
            Some(prop)
        } else {
            None
        }
    }

    pub fn size_in_the_unit(&self) -> f32 {
        let value = match self.size_unit {
            Unit::GiB => self.size / Unit::MIB_BYTE,
            Unit::MiB => self.size / Unit::KIB_BYTE,
            Unit::KiB => self.size,
            Unit::Byte => return self.size as f32,
        } as f32;

        value / 1024f32
    }
}

#[test]
fn test_cache_prop() {
    /* CPUID Ryzen 5 5600G, 0x8000001D_x3 */
    let cpuid = CpuidResult {
        eax: 0x0002C163,
        ebx: 0x03C0003F,
        ecx: 0x00003FFF,
        edx: 0x00000001,
    };

    let cache = CacheProp::from(&cpuid);

    let test = {
        let cache_type = CacheType::Unified;
        let unit = Unit::MiB;

        CacheProp {
            cache_type,
            level: 3,
            line_size: 64,
            way: 16,
            set: 16384,
            size: 16 * unit.to_byte(),
            size_unit: unit,
            share_thread: 12,
            inclusive: false,
        }
    };

    assert_eq!(cache, test);

    println!("CacheProp: [L{} {}, {:>3}-way, {:>4}-{}]",
        cache.level, cache.cache_type, cache.way,
        cache.size / cache.size_unit.to_byte(), cache.size_unit);
    println!("CacheProp: [Shared {}T] [Inclusive: {}]",
        cache.share_thread, cache.inclusive);
}
