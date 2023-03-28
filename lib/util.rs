use crate::{cpuid, CacheProp, TopoId, TopoLevelType};

/// Pin thread to CPU
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

        let mut set = core::mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);
        CPU_SET(cpu, &mut set);

        let status = sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &set);
        if status == -1 {
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

    Ok(())
}

/// Get list of available CPUs
#[cfg(feature = "std")]
pub fn cpu_set_list() -> Result<Vec<usize>, i32> {
    let mut cpus: Vec<usize> = Vec::with_capacity(256);

    #[cfg(unix)]
    unsafe {
        use libc::{
            cpu_set_t,
            CPU_ISSET,
            CPU_ZERO,
            CPU_SETSIZE,
            sched_getaffinity,
        };

        let mut set = core::mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);

        let status = sched_getaffinity(0, core::mem::size_of::<cpu_set_t>(), &mut set);
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

    Ok(cpus)
}

pub fn get_total_logical_processor() -> Option<u32> {
    if let Some(topo_leaf) = TopoId::get_topology_leaf() {
        let thread_count = (cpuid!(topo_leaf, 0x1).ebx >> 16) & 0xFF;

        return Some(thread_count);
    } else {
        let leaf_01h = cpuid!(0x1, 0x0);
        let proc_count = ((leaf_01h.ebx >> 16) & 0xFF) + 1;

        if proc_count == 0 { return None; }

        return Some(proc_count);
    }
}

pub fn get_threads_per_core() -> Option<u32> {
    /* Extended Topology Enumeration */
    if let Some(topo_leaf) = TopoId::get_topology_leaf() {
        /* SMT Level */
        let cpuid = cpuid!(topo_leaf, 0x0);
        let level = (cpuid.ecx >> 8) & 0xFF;

        if level == (TopoLevelType::SMT as u32) {
            return Some(cpuid.ebx & 0xFFFF);
        }
    }

    /*
        AMD TopologyExtensions flag: CPUID[Leaf=0x8000_0001, SubLeaf=0x0].ECX[22]
    */
    let check_topoext = ((cpuid!(0x8000_0001, 0x0).ecx >> 22) & 0b1) != 0;
    if check_topoext {
        let cpuid = cpuid!(0x8000_001E, 0x0).ebx;
        let per_core = (cpuid >> 8) & 0xFF;

        return Some(per_core);
    }

    /* Cache Parameters/Properties */
    if let Some(cache_leaf) = CacheProp::get_cache_prop_leaf() {
        /* L1 Data Cache or L1 Instruction Cache */
        let cpuid = cpuid!(cache_leaf, 0x0);
        let cache_prop = CacheProp::from(&cpuid);

        if cache_prop.level != 1 {
            return None;
        }

        return Some(cache_prop.share_thread);
    }

    /* return 1; */
    None
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
