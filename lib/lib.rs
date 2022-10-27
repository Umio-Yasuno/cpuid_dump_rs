use core::arch::x86_64::CpuidResult;

pub const _AX: u32 = 0x8000_0000;

#[macro_export]
macro_rules! cpuid {
    ($leaf: expr) => {
        unsafe { std::arch::x86_64::__cpuid_count($leaf, 0x0) }
    };
    ($leaf: expr, $sub_leaf: expr) => {
        unsafe { std::arch::x86_64::__cpuid_count($leaf, $sub_leaf) }
    };
}

mod util;
pub use util::*;

mod codename;
pub use codename::*;

mod vendor;
pub use vendor::*;

mod micro_arch_level;
pub use micro_arch_level::*;

mod proc_name;
pub use proc_name::*;

mod cache_prop;
pub use cache_prop::*;

mod intel_ext_topo_0bh_1fh;
pub use intel_ext_topo_0bh_1fh::*;

mod amd_tlb_info;
pub use amd_tlb_info::*;

mod hybrid_info_00_1ah;
pub use hybrid_info_00_1ah::*;

mod topo_info;
pub use topo_info::*;

mod hybrid_topology;
pub use hybrid_topology::*;

mod addr_size_80_08;
pub use addr_size_80_08::*;

mod amd_pkg_type_80_01;
pub use amd_pkg_type_80_01::*;

// pub mod cpuid_macro;
