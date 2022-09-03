use crate::*;

#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum TopoLevelType {
    Invalid = 0,
    SMT = 1, // Thread
    Core = 2,
    Module = 3,
    Tile = 4,
    Die = 5,
}

pub struct TopoId {
    pub smt_id: u32,
    pub core_id: u32,
    pub pkg_id: u32,
    pub x2apic_id: u32,
}

impl TopoId {
    fn check_topology_leaf(leaf: u32) -> bool {
        /* Sub-Leaf = 0 (SMT Level) */
        let cpuid = cpuid!(leaf, 0);
        let level = (cpuid.ecx >> 8) & 0xFF;

        if cpuid.ebx == 0 || level != (TopoLevelType::SMT as u32) {
            return false;
        }

        return true;
    }

    pub(crate) fn get_topology_leaf() -> Option<u32> {
        let topo_leaf = if Self::check_topology_leaf(0x1F) {
            0x1F
        } else if Self::check_topology_leaf(0xB) {
            0xB
        } else {
            return None;
        };

        return Some(topo_leaf);
    }

    /* Page 9: [Detecting Hyper-Threading Technology - kuo-cputopology-rc1-rh1-final-256920.pdf](https://www.intel.com/content/dam/develop/external/us/en/documents/kuo-cputopology-rc1-rh1-final-256920.pdf) */
    pub fn get_topo_info() -> Option<Self> {
        let topo_leaf = Self::get_topology_leaf()?;

        let smt_cpuid = cpuid!(topo_leaf, 0x0);
        let core_cpuid = cpuid!(topo_leaf, 0x1);

        let x2apic_id = smt_cpuid.edx;

        let smt_mask_width = smt_cpuid.eax & 0x1F;
        let smt_select_mask = !(u32::MAX << smt_mask_width);

        let coreplus_mask_width = core_cpuid.eax & 0x1F;
        let coreonly_select_mask = (!(u32::MAX << coreplus_mask_width)) ^ smt_select_mask;

        let pkg_select_mask = u32::MAX << coreplus_mask_width;

        let smt_id = x2apic_id & smt_select_mask;
        let core_id = (x2apic_id & coreonly_select_mask) >> smt_mask_width;
        let pkg_id = (x2apic_id & pkg_select_mask) >> coreplus_mask_width;

        Some(Self {
            smt_id,
            core_id,
            pkg_id,
            x2apic_id,
        })
    }
}
