use crate::{CpuidResult};

enum Unit {
    Byte,
    KiB,
    MiB,
    GiB,
}
impl Unit {
    const KIB_BYTE: u64 = 1 << 10;
    const MIB_BYTE: u64 = 1 << 20;
    const GIB_BYTE: u64 = 1 << 30;

    fn from_byte(size: u64) -> Unit {
        if Self::GIB_BYTE < size {
            Self::GiB
        } else if Self::MIB_BYTE < size {
            Self::MiB
        } else if Self::KIB_BYTE < size {
            Self::KiB
        } else {
            Self::Byte
        }
    }
    fn to_byte(&self) -> u64 {
        match self {
            Self::Byte => 1,
            Self::KiB => Self::KIB_BYTE,
            Self::MiB => Self::MIB_BYTE,
            Self::GiB => Self::GIB_BYTE,
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Byte => "Byte",
            Self::KiB => "KiB",
            Self::MiB => "MiB",
            Self::GiB => "GiB",
        }.to_string()
    }
}

#[derive(Debug, PartialEq)]
pub enum CacheType {
    Data,
    Instruction,
    Unified,
    Unknown,
}

impl CacheType {
    pub fn from_reg(reg: u32) -> CacheType {
        match reg {
            0x1 => Self::Data,
            0x2 => Self::Instruction,
            0x3 => Self::Unified,
            0x0 | _ => Self::Unknown,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Data => "Data",
            Self::Instruction => "Instruction",
            Self::Unified => "Unified",
            Self::Unknown => "Unknown",
        }.to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct CacheProp {
    pub cache_type: CacheType,
    pub cache_type_string: String,
    pub level: u32,
    pub line_size: u32,
    pub way: u32,
    pub set: u32,
    pub size: u64,
    pub share_thread: u32,
    pub size_unit_byte: u64,
    pub size_unit_string: String,
    pub inclusive: bool,
}

impl CacheProp {
    pub fn from_cpuid(cpuid: &CpuidResult) -> CacheProp {
        let [eax, ebx, ecx, edx] = [cpuid.eax, cpuid.ebx, cpuid.ecx, cpuid.edx];

        let cache_type = CacheType::from_reg(eax & 0x1F);
        let cache_type_string = cache_type.to_string();

        let level = (eax >> 5) & 0b111;
        let line_size = (ebx & 0xFFF) + 1;
        let way = (ebx >> 22) + 1;
        let set = ecx + 1;
        let size = line_size as u64 * way as u64 * set as u64;

        let share_thread = ((eax >> 14) & 0xFFF) + 1;

        let (size_unit_byte, size_unit_string) = {
            let unit = Unit::from_byte(size);

            (unit.to_byte(), unit.to_string())
        };

        let inclusive = (edx & 0b10) != 0;

        CacheProp {
            cache_type,
            cache_type_string,
            level,
            line_size,
            way,
            set,
            size,
            share_thread,
            size_unit_byte,
            size_unit_string,
            inclusive,
        }
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

    let cache = CacheProp::from_cpuid(&cpuid);

    let test = {
        let cache_type = CacheType::Unified;
        let cache_type_string = cache_type.to_string();
        let unit = Unit::MiB;

        CacheProp {
            cache_type,
            cache_type_string,
            level: 3,
            line_size: 64,
            way: 16,
            set: 16384,
            size: 16 * unit.to_byte(),
            share_thread: 12,
            size_unit_byte: unit.to_byte(),
            size_unit_string: unit.to_string(),
            inclusive: false,
        }
    };

    assert_eq!(cache, test);

    println!("CacheProp: [L{} {}, {:>3}-way, {:>4}-{}]",
        cache.level, cache.cache_type_string, cache.way,
        cache.size / cache.size_unit_byte, cache.size_unit_string);
    println!("CacheProp: [Shared {}T] [Inclusive: {}]",
        cache.share_thread, cache.inclusive);
}
