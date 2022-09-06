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

mod intel_ext_topo;
pub use intel_ext_topo::*;

mod amd_tlb_info;
pub use amd_tlb_info::*;

mod hybrid_info_00_1ah;
pub use hybrid_info_00_1ah::*;

mod topo_info;
pub use topo_info::*;

mod hybrid_topology;
pub use hybrid_topology::*;

pub mod cpuid_macro;

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
    let mut cpus: Vec<usize> = Vec::with_capacity(256);
    
    #[cfg(unix)]
    unsafe {
        use std::mem;
        use libc::{
            cpu_set_t,
            CPU_ISSET,
            CPU_ZERO,
            CPU_SETSIZE,
            sched_getaffinity,
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

        /* TODO: error check */
        for i in 0..GetCurrentProcessorNumber() as usize {
            cpus.push(i);
        }
    }

    return Ok(cpus);
}

pub fn get_total_logical_processor() -> Option<u32> {
    let topo_leaf = match TopoId::get_topology_leaf() {
        Some(v) => v,
        None => {
            let leaf_01h = cpuid!(0x1, 0x0);
            let proc_count = ((leaf_01h.ebx >> 16) & 0xFF) + 1;

            if proc_count == 0 { return None; }

            return Some(proc_count);
        },
    };
    
    let thread_count = (cpuid!(topo_leaf, 0x1).ebx >> 16) & 0xFF;
    
    return Some(thread_count);
}

pub fn get_threads_per_core() -> Option<u32> {
    /* Extended Topology Enumeration */
    if let Some(topo_leaf) = TopoId::get_topology_leaf() {
        let cpuid = cpuid!(topo_leaf, 0x0);
        let level = (cpuid.ecx >> 8) & 0xFF;

        if level == (TopoLevelType::SMT as u32) {
            return Some(cpuid.ebx & 0xFFFF);
        }
    }

    /* Cache Parameters/Properties */
    if let Some(cache_leaf) = CacheProp::get_cache_prop_leaf() {
        /* L1 Data Cache? */
        let cpuid = cpuid!(cache_leaf, 0x0);
        let cache_prop = CacheProp::from_cpuid(&cpuid);

        if cache_prop.level != 1 {
            return None;
        }

        return Some(cache_prop.share_thread);
    }

    return None;
}

#[macro_export]
macro_rules! initial_apic_id {
    () => {
        initial_apic_id!(cpuid!(0x1, 0x0).eax)
    };
    ($eax: expr) => {
        $eax >> 24
    };
}

#[macro_export]
macro_rules! max_apic_id {
    () => {
        max_apic_id!(cpuid!(0x1, 0x0).ebx)
    };
    ($ebx: expr) => {
        ($ebx >> 16) & 0xFF
    };
}
