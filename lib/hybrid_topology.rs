use crate::*;

use std::sync::{Arc, Mutex};
use std::{thread};

/*
struct CachePropCount {
    pub prop: Option<CacheProp>,
    pub count: u32,
}
*/

#[derive(Debug)]
pub struct LastLevelCache {
    pub level: u32,
    pub share_thread: u32,
    pub shared_all_thread: bool,
}

impl LastLevelCache {
    fn from_cache_prop(prop: &CacheProp, max_apic_id: u32) -> Self {
        let shared_all_thread = prop.share_thread == max_apic_id;
        
        Self {
            level: prop.level,
            share_thread: prop.share_thread,
            shared_all_thread,
        }
    }
}

#[derive(Debug)]
pub struct TopoCacheInfo {
    pub l1d_cache: Option<CacheProp>,
    // l1d_count: u32,
    pub l1i_cache: Option<CacheProp>,
    // l1i_count: u32,
    pub l2_cache: Option<CacheProp>,
    pub l2_count: u32,
    pub l3_cache: Option<CacheProp>,
    pub l3_count: u32,
    pub l4_cache: Option<CacheProp>,
    pub l4_count: u32,
    pub last_level: LastLevelCache,
}

impl TopoCacheInfo {
    pub fn get_topology_cache_info(type_only_list: &[usize]) -> Option<Self> {
        let cache_leaf = Arc::new(CacheProp::get_cache_prop_leaf()?);
        let mut last_level = LastLevelCache {
            level: 0,
            share_thread: 0,
            shared_all_thread: false,
        };
        
        if *cache_leaf == 0x8000_001D {
            return Self::from_amd_80_1dh(*cache_leaf);
        }

        let [mut l2_ids, mut l3_ids, mut l4_ids] = [
            Vec::<u32>::with_capacity(32),
            Vec::<u32>::with_capacity(32),
            Vec::<u32>::with_capacity(32),
        ];

        let [mut l1d, mut l1i, mut l2, mut l3, mut l4]: [Option<CacheProp>; 5]
            = [None, None, None, None, None];

        {
            pin_thread(type_only_list[0]).unwrap();
            let eax = cpuid!(0x1, 0x0).eax;
            let apicid = initial_apic_id!(eax);
            let max_apic_id = max_apic_id!(eax);
            
            for sub_leaf in 0x0..=0x4 {
                let cpuid = cpuid!(*cache_leaf, sub_leaf);
                let prop = CacheProp::from_cpuid(&cpuid);
                let cache_id = Self::get_cache_id(apicid, prop.share_thread);

                match prop {
                    CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                        l1d = Some(prop)
                    },
                    CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                        l1i = Some(prop)
                    },
                    CacheProp { level: 2, .. } => {
                        last_level = LastLevelCache::from_cache_prop(&prop, max_apic_id);
                        l2 = Some(prop);
                        l2_ids.push(cache_id);
                    },
                    CacheProp { level: 3, .. } => {
                        last_level = LastLevelCache::from_cache_prop(&prop, max_apic_id);
                        l3 = Some(prop);
                        l3_ids.push(cache_id);
                    },
                    CacheProp { level: 4, .. } => {
                        last_level = LastLevelCache::from_cache_prop(&prop, max_apic_id);
                        l4 = Some(prop);
                        l4_ids.push(cache_id);
                    },
                    _ => {},
                }
            }
        }

        let [l2_ids, l3_ids, l4_ids] = [
            Arc::new(Mutex::new(l2_ids)),
            Arc::new(Mutex::new(l3_ids)),
            Arc::new(Mutex::new(l4_ids)),
        ];

        for cpu in &type_only_list[1..] {
            let cpu = *cpu;
            let cache_leaf = Arc::clone(&cache_leaf);

            let [l2_ids, l3_ids, l4_ids] = [
                Arc::clone(&l2_ids),
                Arc::clone(&l3_ids),
                Arc::clone(&l4_ids),
            ];

            thread::spawn(move || {
                pin_thread(cpu).unwrap();
                let apicid = initial_apic_id!();

                /* 0x2..=0x4 (L2 Cache .. L4 Cache) ? */
                for sub_leaf in 0x0..=0x4 {
                    let cpuid = cpuid!(*cache_leaf, sub_leaf);
                    let prop = CacheProp::from_cpuid(&cpuid);

                    if prop.cache_type == CacheType::Unknown {
                        continue;
                    }
                    
                    let cache_id = Self::get_cache_id(apicid, prop.share_thread);

                    match prop {
                        CacheProp { level: 2, .. } => {
                            let mut l2_ids = l2_ids.lock().unwrap();
                            if !l2_ids.contains(&cache_id) {
                                l2_ids.push(cache_id);
                            }
                        },
                        CacheProp { level: 3, .. } => {
                            let mut l3_ids = l3_ids.lock().unwrap();
                            if !l3_ids.contains(&cache_id) {
                                l3_ids.push(cache_id);
                            }
                        },
                        CacheProp { level: 4, .. } => {
                            let mut l4_ids = l4_ids.lock().unwrap();
                            if !l4_ids.contains(&cache_id) {
                                l4_ids.push(cache_id);
                            }
                        },
                        _ => {},
                    }
                }
            }).join().unwrap();
        }

        let [l2_ids, l3_ids, l4_ids] = [l2_ids, l3_ids, l4_ids]
            .map(|ids| Arc::try_unwrap(ids).unwrap().into_inner().unwrap() );

        Some(Self {
            l1d_cache: l1d,
            l1i_cache: l1i,
            l2_cache: l2,
            l2_count: l2_ids.len() as u32,
            l3_cache: l3,
            l3_count: l3_ids.len() as u32,
            l4_cache: l4,
            l4_count: l4_ids.len() as u32,
            last_level,
        })
    }

    fn from_amd_80_1dh(cache_leaf: u32) -> Option<Self> {
        let [mut l1d, mut l1i, mut l2, mut l3, mut l4]: [Option<CacheProp>; 5]
            = [None, None, None, None, None];
        let [mut l2_count, mut l3_count, mut l4_count]: [u32; 3] = [0, 0, 0];
        let mut last_level = LastLevelCache {
            level: 0,
            share_thread: 0,
            shared_all_thread: false,
        };
        let total_logical_proc = get_total_logical_processor()?;
        let max_apic_id = max_apic_id!();

        for sub_leaf in 0x0..=0x4 {
            let cpuid = cpuid!(cache_leaf, sub_leaf);
            let prop = CacheProp::from_cpuid(&cpuid);
            // let cache_id = Self::get_cache_id(apicid, prop.share_thread);

            match prop {
                CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                    l1d = Some(prop);
                },
                CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                    l1i = Some(prop);
                },
                CacheProp { level: 2, .. } => {
                    last_level = LastLevelCache::from_cache_prop(&prop, max_apic_id);
                    l2_count = total_logical_proc / prop.share_thread;
                    l2 = Some(prop);
                },
                CacheProp { level: 3, .. } => {
                    last_level = LastLevelCache::from_cache_prop(&prop, max_apic_id);
                    l3_count = total_logical_proc / prop.share_thread;
                    l3 = Some(prop);
                },
                CacheProp { level: 4, .. } => {
                    last_level = LastLevelCache::from_cache_prop(&prop, max_apic_id);
                    l4_count = total_logical_proc / prop.share_thread;
                    l4 = Some(prop);
                },
                _ => {},
            }
        }

        Some(Self {
            l1d_cache: l1d,
            l1i_cache: l1i,
            l2_cache: l2,
            l2_count,
            l3_cache: l3,
            l3_count,
            l4_cache: l4,
            l4_count,
            last_level,
        })
    }

    /* ref:
        Intel® 64 Architecture Processor Topology Enumeration - intel-64-architecture-processor-topology-enumeration.pdf
        https://www.intel.com/content/dam/develop/external/us/en/documents/intel-64-architecture-processor-topology-enumeration.pdf)
    */
    /* Linux Kernel: arch/x86/kernel/cpu/cacheinfo.c */
    fn get_cache_id(apicid: u32, num_sharing_thread: u32) -> u32 {
        /* find last set bit */
        let index_msb = u32::BITS - num_sharing_thread.leading_zeros();

        apicid & !((1 << index_msb) - 1)
    }
}

pub struct TopoPartInfo {
   pub core_type: HybridCoreType,
   pub num_logical_proc: u32,
   pub num_physical_proc: u32,
   pub cache: Option<TopoCacheInfo>,
}

impl TopoPartInfo {
    pub fn check_hybrid_flag() -> bool {
        let cpuid = (cpuid!(0x7, 0x0).edx >> 15) & 0b1;

        cpuid == 1
    }

    fn get_core_type() -> HybridCoreType {
        let leaf_1ah = cpuid!(0x1A, 0x0);

        match HybridInfo::get_core_type(leaf_1ah) {
            Some(t) => t,
            None => HybridCoreType::Invalid,
        }
    }
    
    fn get_core_type_only_list(core_type: HybridCoreType) -> Vec<usize> {
        let cpu_list = cpu_set_list().unwrap();
        let type_only_list = Arc::new(Mutex::new(Vec::<usize>::with_capacity(64)));
        let core_type = Arc::new(core_type);

        for cpu in cpu_list {
            let type_only_list = Arc::clone(&type_only_list);
            let core_type = Arc::clone(&core_type);

            thread::spawn(move || {
                pin_thread(cpu).unwrap();
                // let core_type = Arc::try_unwrap(core_type).unwrap();

                if Self::get_core_type() == *core_type {
                    let mut list = type_only_list.lock().unwrap();
                    list.push(cpu);
                }
            }).join().unwrap();
        }

        Arc::try_unwrap(type_only_list).unwrap().into_inner().unwrap()
    }

    pub fn get(core_type: HybridCoreType) -> Self {
        let core_type_ = core_type.clone();
        let cpu_list = Self::get_core_type_only_list(core_type_);
        /* core type only */
        let num_logical_proc = Arc::new(cpu_list.len() as u32);

        let num_physical_proc = Arc::new(Mutex::new(0u32));
        let cache: Arc<Mutex<Option<TopoCacheInfo>>> = Arc::new(Mutex::new(None));

        /* To confine the effects of pin_thread */
        {
                let logi_proc = Arc::clone(&num_logical_proc);
                let phy_proc = Arc::clone(&num_physical_proc);
                let topo_cache = Arc::clone(&cache);

                thread::spawn(move || {
                    pin_thread(cpu_list[0]).unwrap();

                    let threads_per_core = get_threads_per_core().unwrap_or(1);
                    let mut phy_proc = phy_proc.lock().unwrap();
                    let mut topo_cache = topo_cache.lock().unwrap();

                    *phy_proc = *logi_proc / threads_per_core;
                    *topo_cache = TopoCacheInfo::get_topology_cache_info(&cpu_list);
                }).join().unwrap();
        }

        let num_physical_proc = *num_physical_proc.lock().unwrap();
        let num_logical_proc = *num_logical_proc;
        let cache = Arc::try_unwrap(cache).unwrap().into_inner().unwrap();

        Self {
            core_type,
            num_logical_proc,
            num_physical_proc,
            cache,
        }
    }
}
