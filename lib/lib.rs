#![cfg_attr(not(feature = "std"), no_std)]

use core::arch::x86_64::CpuidResult;

// pub const _AX: u32 = 0x8000_0000;

#[macro_export]
macro_rules! cpuid {
    ($leaf: expr) => {
        unsafe { core::arch::x86_64::__cpuid_count($leaf, 0x0) }
    };
    ($leaf: expr, $sub_leaf: expr) => {
        unsafe { core::arch::x86_64::__cpuid_count($leaf, $sub_leaf) }
    };
}

#[macro_use]
pub mod util;
// pub use util::*;

mod codename;
pub use codename::*;

mod vendor;
pub use vendor::*;

mod micro_arch_level;
pub use micro_arch_level::*;

#[cfg(feature = "std")]
mod proc_name;
#[cfg(feature = "std")]
pub use proc_name::*;

mod cache_prop;
pub use cache_prop::*;

mod info_01h_ebx;
pub use info_01h_ebx::*;

mod monitor_mwait_00_05h;
pub use monitor_mwait_00_05h::*;

mod intel_ext_topo_0bh_1fh;
pub use intel_ext_topo_0bh_1fh::*;

mod amd_tlb_info;
pub use amd_tlb_info::*;

mod hybrid_info_00_1ah;
pub use hybrid_info_00_1ah::*;

mod topo_info;
pub use topo_info::*;

#[cfg(feature = "std")]
mod hybrid_topology;
#[cfg(feature = "std")]
pub use hybrid_topology::*;

mod addr_size_80_08h_eax;
pub use addr_size_80_08h_eax::*;

mod amd_size_id_80_08h_ecx;
pub use amd_size_id_80_08h_ecx::*;

mod amd_pkg_type_80_01h;
pub use amd_pkg_type_80_01h::*;

mod amd_proc_topo_80_1eh;
pub use amd_proc_topo_80_1eh::*;

mod amd_ext_topo_80_26h;
pub use amd_ext_topo_80_26h::*;
