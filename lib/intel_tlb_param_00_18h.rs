use crate::CpuidResult;

#[derive(Debug, Clone)]
pub struct IntelTlbParam {
    pub cache_type: IntelTlbType,
    pub cache_level: u8,
    pub set: u32,
    pub way: u16,
    pub partitioning: u8,
    pub support_4k: bool,
    pub support_2m: bool,
    pub support_4m: bool,
    pub support_1g: bool,
    pub fully_assoc: bool,
    pub max_shared_thread: u16,
}

impl From<&CpuidResult> for IntelTlbParam {
    fn from(cpuid: &CpuidResult) -> Self {
        let cache_type = IntelTlbType::from(cpuid);
        let cache_level = ((cpuid.edx >> 5) & 0b111) as u8;
        let set = cpuid.ecx;
        let way = (cpuid.ebx >> 16) as u16;
        let partitioning = ((cpuid.ebx >> 8) & 0b111) as u8;
        let support_4k = (cpuid.ebx & 0b1) == 0b1;
        let support_2m = (cpuid.ebx & 0b10) == 0b10;
        let support_4m = (cpuid.ebx & 0b100) == 0b100;
        let support_1g = (cpuid.ebx & 0b1000) == 0b1000;
        let fully_assoc = (cpuid.edx & 0b1000_0000) == 0b1000_0000;
        let max_shared_thread = ((cpuid.edx >> 14) & 0xFFF) as u16;

        Self {
            cache_type,
            cache_level,
            set,
            way,
            partitioning,
            support_4k,
            support_2m,
            support_4m,
            support_1g,
            fully_assoc,
            max_shared_thread,
        }
    }
}

impl IntelTlbParam {
    #[cfg(feature = "std")]
    pub fn get() -> Vec<Self> {
        let max_sub_leaf = cpuid!(0x18, 0).eax;
        let mut params: Vec<Self> = Vec::new();

        for sub_leaf in 0..=max_sub_leaf {
            let cpuid = cpuid!(0x18, sub_leaf);
            params.push(Self::from(&cpuid));
        }

        params
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntelTlbType {
    Null,
    Data,
    Instruction,
    Unified,
    LoadOnly,
    StoreOnly,
    Reserved,
}

impl From<u8> for IntelTlbType {
    fn from(reg: u8) -> Self {
        match reg {
            0b0 => Self::Null,
            0b1 => Self::Data,
            0b10 => Self::Instruction,
            0b11 => Self::Unified,
            0b100 => Self::LoadOnly,
            0b101 => Self::StoreOnly,
            _ => Self::Reserved,
        }
    }
}

impl From<&CpuidResult> for IntelTlbType {
    fn from(cpuid: &CpuidResult) -> Self {
        Self::from((cpuid.edx & 0xF) as u8)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for IntelTlbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
