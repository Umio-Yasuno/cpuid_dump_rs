//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::CpuidResult;

#[path = "./codename_mod.rs"]
mod codename_mod;
pub use codename_mod::*;

#[path = "./vendor.rs"]
mod vendor;
pub use vendor::*;

#[path = "./micro_arch_level.rs"]
mod micro_arch_level;
pub use micro_arch_level::*;

#[path = "./proc_name.rs"]
mod proc_name;
pub use proc_name::*;

#[path = "./cache_prop.rs"]
mod cache_prop;
pub use cache_prop::*;

#[path = "./intel_ext_topo.rs"]
mod intel_ext_topo;
pub use intel_ext_topo::*;

pub mod cpuid_macro;

pub const _AX: u32 = 0x8000_0000;

#[macro_export]
macro_rules! cpuid {
    ($leaf: expr, $sub_leaf: expr) => {
        unsafe { std::arch::x86_64::__cpuid_count($leaf, $sub_leaf) }
    };
}

pub fn pin_thread(cpu: usize) -> Result<(), i32> {
    #[cfg(unix)]
    unsafe {
        use libc::{
            cpu_set_t,
            // sched_getaffinity,
            sched_setaffinity,
            // CPU_ALLOC_SIZE,
            CPU_SET,
            CPU_ZERO
        };

        let mut set = std::mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);
        CPU_SET(cpu, &mut set);

        let status = sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &set);
        if status == -1 {
            eprintln!("sched_setaffinity failed.");
            return Err(status);
        }
    }

    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Threading::{
            GetCurrentThread,
            SetThreadAffinityMask,
        };
        SetThreadAffinityMask(GetCurrentThread(), 1 << cpu);
    }

    return Ok(());
}

pub fn cpu_set_list() -> Result<Vec<usize>, i32> {
    let mut cpus: Vec<usize> = Vec::new();
    
    #[cfg(unix)]
    unsafe {
        use std::mem;
        use libc::{
            cpu_set_t, CPU_ISSET, CPU_ZERO,
            CPU_SETSIZE, sched_getaffinity
        };

        let mut set = mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);

        let status = sched_getaffinity(0, mem::size_of::<cpu_set_t>(), &mut set);
        if status == -1 {
            eprintln!("sched_getaffinity failed");
            return Err(status);
        }

        for i in 0..CPU_SETSIZE as usize {
            if CPU_ISSET(i, &set) {
                cpus.push(i);
            }
        }
    }

    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Threading::{
            GetCurrentProcessorNumber,
        };

        for i in 0..GetCurrentProcessorNumber() as usize {
            cpus.push(i);
        }
    }

    return Ok(cpus);
}

/*
enum CoreType {
    _Reserved1 = 0x10,
    Core = 0x20, // big
    _Reserved2 = 0x30,
    Atom = 0x40, // small
}

pub struct CpuInfo {
    pub is_hybrid: bool,
    pub total_phy_core: u32,
    pub total_thread: u32,
    pub big_topo: CpuTopoInfo,
    pub small_topo: CpuTopoInfo,
}

impl CpuInfo {
    pub fn get() -> CpuInfo {
        let lf_01h = cpuid!(0x1, 0);
        let is_hybrid = ((cpuid!(0x7, 0).edx >> 15) & 0b1) == 1;

        let big_topo = CpuTopoInfo::zero();
        let small_topo = CpuTopoInfo::zero();

        /* LogicalProcessorCount */
        let total_thread = (lf_01h.ebx >> 16) & 0xFF;
        let total_phy_core = total_thread;

        CpuInfo {
            is_hybrid,
            total_phy_core,
            total_thread,
            big_topo,
            small_topo,
        }
    }
}

pub struct CpuTopoInfo {
    pub core_count: u32,
    pub thread_count: u32,
    pub thread_per_core: u32,
    pub cache_info: CacheInfo,

}

impl CpuTopoInfo {
    fn zero() -> CpuTopoInfo {
        CpuTopoInfo {
            topo_core_count: 0,
            topo_thread_count: 0,
            thread_per_core: 0,
            topo_cache_info: CacheInfo::zero(),
        }
    }
}
*/

pub struct CpuCoreCount {
    pub has_htt: bool,
    pub total_thread: u32,
    pub thread_per_core: u32,
    pub phy_core: u32,
    pub apic_id: u32,
    pub core_id: u32,
}

impl CpuCoreCount {
    pub fn get() -> CpuCoreCount {
        //  let lf_04h = cpuid!(0x4, 0x0);
        //  let lf_0bh = cpuid!(0xB, 0);
        let lf_80_1eh = cpuid!(_AX + 0x1E, 0x0);
        //  let lf_08h = cpuid!(0x4, 0x1);

        let vendor = VendorFlag::check();

        let lf_01h = cpuid!(0x1, 0x0);
        let has_htt = ((lf_01h.edx >> 28) & 0b1) == 1;

        // TODO: Use Leaf: 0xB, Sub-leaf: 0x2 if Intel CPU
        //  Topology type: Core, for Alder Lake
        let total_thread = if vendor.intel {
            cpuid!(0xB, 0x1).ebx & 0xFFFF
        } else {
            (lf_01h.ebx >> 16) & 0xFF
        };

        let thread_per_core = if vendor.intel {
            (lf_80_1eh.eax & 0xFFF) + 1
        } else if vendor.amd {
            (cpuid!(_AX + 0x1E, 0x0).ebx & 0xFF) + 1
        } else if has_htt {
            2
        } else {
            1
        };

        let phy_core = total_thread / thread_per_core;
        let apic_id = (lf_01h.ebx >> 24) & 0xFF;
        // TODO: CoreID for Intel CPU
        // arch/x86/kernel/cpu/topology.c
        let core_id = lf_80_1eh.ebx & 0xFF;

        CpuCoreCount {
            has_htt,
            total_thread,
            thread_per_core,
            phy_core,
            apic_id,
            core_id,
        }
    }
}
