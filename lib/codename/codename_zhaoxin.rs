use crate::{ProcInfo, CpuCodename, CpuMicroArch, CpuStepping, ProcessNode};
use std::fmt;
/* ref: https://github.com/google/cpu_features/pull/218/ */

impl ProcInfo {
    pub(super) fn zhaoxin_fam06h(m: u32, s: u32) -> Self {
        match m {
            0x0F | 0x19 => Self {
                codename: CpuCodename::Zhaoxin(ZhaoxinCodename::ZX_C_4000),
                archname: CpuMicroArch::Zhaoxin(ZhaoxinMicroArch::Zhangjiang),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(28)),
            },
            _ => Self {
                codename: CpuCodename::Zhaoxin(ZhaoxinCodename::Unknown(0x6, m)),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
    pub(super) fn zhaoxin_fam07h(m: u32, s: u32) -> Self {
        match m {
            0x1B => Self {
                codename: CpuCodename::Zhaoxin(ZhaoxinCodename::KX5000_KH20000),
                archname: CpuMicroArch::Zhaoxin(ZhaoxinMicroArch::Wudaokou),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(28)),
            },
            0x3B => Self {
                codename: CpuCodename::Zhaoxin(ZhaoxinCodename::KX6000_KH30000),
                archname: CpuMicroArch::Zhaoxin(ZhaoxinMicroArch::Lujiazui),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(16)),
            },
            0x5B => Self {
                codename: CpuCodename::Zhaoxin(ZhaoxinCodename::KH40000),
                archname: CpuMicroArch::Zhaoxin(ZhaoxinMicroArch::Yongfeng),
                step_info: CpuStepping::Unknown(s),
                node: Some(ProcessNode::NM(16)),
            },
            _ => Self {
                codename: CpuCodename::Zhaoxin(ZhaoxinCodename::Unknown(0x7, m)),
                archname: CpuMicroArch::Unknown,
                step_info: CpuStepping::Unknown(s),
                node: None,
            },
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ZhaoxinCodename {
    ZX_C_4000,
    // ZX_C_Plus_4000,
    KX5000_KH20000,
    KX6000_KH30000,
    KH40000,
    Unknown(u32, u32),
}

impl fmt::Display for ZhaoxinCodename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ZX_C_4000 => write!(f, "ZX-C 4000"),
            Self::KX5000_KH20000 => write!(f, "KX5000/KH20000"),
            Self::KX6000_KH30000 => write!(f, "KX6000/KH30000"),
            Self::KH40000 => write!(f, "KH40000"),
            Self::Unknown(fam, model) => write!(f, "Family{fam:X}h Model{model:X}h"),
        }
    }
}

#[derive(Debug)]
pub enum ZhaoxinMicroArch {
    Zhangjiang, // 张江
    Wudaokou, // 五道口
    Lujiazui, // 陆家嘴
    Yongfeng, // 永丰
}

impl fmt::Display for ZhaoxinMicroArch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ZhaoxinMicroArch> for String {
    fn from(s: ZhaoxinMicroArch) -> Self {
        s.to_string()
    }
}
