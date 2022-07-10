use crate::*;

pub trait ParseIntel {
    fn clock_speed_intel_00_16h(&self) -> String;
    fn intel_hybrid_1ah(&self) -> String;
    fn v2_ext_topo_intel_1fh(&self) -> String;
}

impl ParseIntel for CpuidResult {
    fn clock_speed_intel_00_16h(&self) -> String {
        format!(" [{}/{}/{} MHz]",
            self.eax & 0xFFFF,
            self.ebx & 0xFFFF,
            self.ecx & 0xFFFF
        )
    }

    fn intel_hybrid_1ah(&self) -> String {
        let eax = self.eax;

        let core_type = match eax >> 24 {
            0x10 => "Reserved_1",
            0x20 => "Atom",
            0x30 => "Reserved_2",
            0x40 => "Core",
            _    => return "".to_string(),
        };

        return format!(" [{}]", core_type);
    }

    fn v2_ext_topo_intel_1fh(&self) -> String {
        return IntelExtTopo::dec(self).disp();
    }
}

#[repr(u8)]
enum IntelLevelType {
    Invalid,
    SMT,
    Core,
    Module,
    Tile,
    Die,
}

impl IntelLevelType {
    fn from_reg(reg: u8) -> Self {
        match reg {
            0x1 => Self::SMT,
            0x2 => Self::Core,
            0x3 => Self::Module,
            0x4 => Self::Tile,
            0x5 => Self::Die,
            0x0 |
            _ => Self::Invalid,
        }
    }
}

use std::fmt;
impl fmt::Display for IntelLevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SMT => write!(f, "SMT"),
            Self::Core => write!(f, "Core"),
            Self::Module => write!(f, "Module"),
            Self::Tile => write!(f, "Tile"),
            Self::Die => write!(f, "Die"),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
}

#[allow(dead_code)]
struct IntelExtTopo {
    next_level: u32,
    x2apic_id: u32,
    level_type: IntelLevelType,
}

impl IntelExtTopo {
    fn dec(cpuid: &CpuidResult) -> IntelExtTopo {
        let next_level = cpuid.eax & 0xF;
        let x2apic_id = cpuid.edx;
        let level_type = {
            let reg = (cpuid.ecx >> 8) & 0xFF;
            IntelLevelType::from_reg(reg as u8)
        };

        IntelExtTopo {
            next_level,
            x2apic_id,
            level_type,
        }
    }

    fn disp(&self) -> String {
        return [
            format!(" [{}]", self.level_type.to_string()),
            padln!(),
            format!(" [x2APIC ID: {}]", self.x2apic_id),
        ].concat();
    }
}
