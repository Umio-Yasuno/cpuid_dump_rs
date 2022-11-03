use crate::{ProcInfo, ProcessNode};
/* https://github.com/google/cpu_features/pull/218/ */

#[derive(Debug)]
pub(self) enum ZhaoxinMicroArch {
    Zhangjiang, // 张江
    Wudaokou, // 五道口
    Lujiazui, // 陆家嘴
    Yongfeng, // 永丰
}

use std::fmt;
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

impl ProcInfo {
    pub(super) fn zhaoxin_fam06h(m: u32, _s: u32) -> Option<Self> {
        use ZhaoxinMicroArch as uarch;

        Some(match m {
            0x0F => Self::info("", uarch::Zhangjiang, ProcessNode::NM(28)),
            0x19 => Self::info("", uarch::Zhangjiang, ProcessNode::NM(28)),
            _ => return None,
        })
    }
    pub(super) fn zhaoxin_fam07h(m: u32, _s: u32) -> Option<Self> {
        use ZhaoxinMicroArch as uarch;

        Some(match m {
            /* KX-5000, KH-20000 */
            0x1B => Self::info("", uarch::Wudaokou, ProcessNode::NM(28)),
            /* KX-6000, KH-30000 */
            0x3B => Self::info("", uarch::Lujiazui, ProcessNode::NM(16)),
                /*  Stepping 0: KX-U66xx ?
                    https://linux-hardware.org/?probe=a68e653aa9&log=lscpu */
                /*  Stepping 1: KX-U6780A
                    https://linux-hardware.org/?probe=a68e653aa9&log=lscpu */
            /* KH-40000 */
            0x5B => Self::info("", uarch::Yongfeng, ProcessNode::NM(16)),
            _ => return None,
        })
    }
}
