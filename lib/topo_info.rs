use crate::{cpuid, CpuidResult, TopoLevelType};

/// Topology ID (SMT, Core, Pkg, X2APIC)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TopoId {
    pub smt_id: u32,
    pub core_id: u32,
    pub pkg_id: u32,
    pub x2apic_id: u32,
}

impl TopoId {
    fn check_topology_leaf(leaf: u32) -> bool {
        const SUB_LEAF: u32 = 0x1;
        let cpuid = cpuid!(leaf, SUB_LEAF);

        /* ECX[07-00]: Level number. Same value in ECX input (Sub_Leaf) */
        (cpuid.ecx & 0xFF) == SUB_LEAF
    }

    pub(crate) fn get_topology_leaf() -> Option<u32> {
        let topo_leaf = if Self::check_topology_leaf(0x1F) {
            0x1F
        } else if Self::check_topology_leaf(0xB) {
            0xB
        } else {
            return None;
        };

        Some(topo_leaf)
    }

    pub(crate) fn get_cpuid_by_level_type(
        topo_leaf: u32,
        target_level_type: TopoLevelType
    ) -> Option<CpuidResult> {
        for sub_leaf in 0..(TopoLevelType::Die as u32) {
            let cpuid = cpuid!(topo_leaf, sub_leaf);
            let level_type = TopoLevelType::from(&cpuid);
            
            if level_type == target_level_type {
                return Some(cpuid);
            }
        }

        None
    }

    pub fn get_topo_info_with_smt_core_cpuid(
        smt_cpuid: &CpuidResult,
        core_cpuid: &CpuidResult,
    ) -> Self {
        let x2apic_id = smt_cpuid.edx;

        let smt_mask_width = smt_cpuid.eax & 0x1F;
        let smt_select_mask = !(u32::MAX << smt_mask_width);

        let coreplus_mask_width = core_cpuid.eax & 0x1F;
        let coreonly_select_mask = (!(u32::MAX << coreplus_mask_width)) ^ smt_select_mask;

        let pkg_select_mask = u32::MAX << coreplus_mask_width;

        let smt_id = x2apic_id & smt_select_mask;
        let core_id = (x2apic_id & coreonly_select_mask) >> smt_mask_width;
        let pkg_id = (x2apic_id & pkg_select_mask) >> coreplus_mask_width;

        Self {
            smt_id,
            core_id,
            pkg_id,
            x2apic_id,
        }
    }

    /*
        Page 9: Detecting Hyper-Threading Technology - kuo-cputopology-rc1-rh1-final-256920.pdf
        https://www.intel.com/content/dam/develop/external/us/en/documents/kuo-cputopology-rc1-rh1-final-256920.pdf
    */
    pub fn get_topo_info() -> Option<Self> {
        let topo_leaf = Self::get_topology_leaf()?;

        let smt_cpuid = Self::get_cpuid_by_level_type(topo_leaf, TopoLevelType::SMT)?;
        let core_cpuid = Self::get_cpuid_by_level_type(topo_leaf, TopoLevelType::Core)?;

        Some(Self::get_topo_info_with_smt_core_cpuid(&smt_cpuid, &core_cpuid))
    }
}
