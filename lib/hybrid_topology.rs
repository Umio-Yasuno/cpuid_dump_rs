use crate::*;

use std::sync::{Arc, Mutex};
use std::{thread};

#[derive(Debug)]
pub struct CachePropCount {
    pub prop: Option<CacheProp>,
    pub count: u32,
    pub shared_between_topology: bool, // shared_all_threads?
}

#[derive(Debug)]
pub struct TopoCacheInfo {
    pub l1d: Option<CachePropCount>,
    pub l1i: Option<CachePropCount>,
    pub l2: Option<CachePropCount>,
    pub l3: Option<CachePropCount>,
    pub l4: Option<CachePropCount>,
}

impl TopoCacheInfo {
    fn shared_all_threads(prop: &CacheProp, max_apic_id: u32) -> bool {
        prop.share_thread == max_apic_id
    }

    pub fn get_topology_cache_info(type_only_list: &[usize]) -> Option<Self> {
        let cache_leaf = Arc::new(CacheProp::get_cache_prop_leaf()?);

        if *cache_leaf == 0x8000_001D {
            return Self::from_amd_80_1dh(*cache_leaf);
        }

        let [mut l1d, mut l1i, mut l2, mut l3, mut l4]: [Option<CachePropCount>; 5]
            = [None, None, None, None, None];
        /*
        let [mut l1d_prop, mut l1i_prop, mut l2_prop, mut l3_prop, mut l4_prop]:
            [Option<CacheProp>; 5] = [None, None, None, None, None];
        */

        let [mut l1d_ids, mut l1i_ids, mut l2_ids, mut l3_ids, mut l4_ids] = [
            Vec::<u32>::with_capacity(64),
            Vec::<u32>::with_capacity(64),
            Vec::<u32>::with_capacity(64),
            Vec::<u32>::with_capacity(32),
            Vec::<u32>::with_capacity(32),
        ];

        /* fill cache prop */
        /* TODO: thread */
        {
            pin_thread(type_only_list[0]).unwrap();
            let eax = cpuid!(0x1, 0x0).eax;
            let apicid = initial_apic_id!(eax);
            let max_apic_id = max_apic_id!(eax);
            
            /* 0x2..=0x4 (L2 Cache .. L4 Cache) ? */
            for sub_leaf in 0x0..=0x4 {
                let cpuid = cpuid!(*cache_leaf, sub_leaf);
                let prop = CacheProp::from_cpuid(&cpuid);
                let cache_id = Self::get_cache_id(apicid, prop.share_thread);
                let shared_between_topology = Self::shared_all_threads(&prop, max_apic_id);

                match prop {
                    CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                        l1d = Some(CachePropCount {
                            prop: Some(prop),
                            count: 1,
                            shared_between_topology,
                        });
                        l1d_ids.push(cache_id);
                    },
                    CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                        l1i = Some(CachePropCount {
                            prop: Some(prop),
                            count: 1,
                            shared_between_topology,
                        });
                        l1i_ids.push(cache_id);
                    },
                    CacheProp { level: 2, .. } => {
                        l2 = Some(CachePropCount {
                            prop: Some(prop),
                            count: 1,
                            shared_between_topology,
                        });
                        l2_ids.push(cache_id);
                    },
                    CacheProp { level: 3, .. } => {
                        l3 = Some(CachePropCount {
                            prop: Some(prop),
                            count: 1,
                            shared_between_topology,
                        });
                        l3_ids.push(cache_id);
                    },
                    CacheProp { level: 4, .. } => {
                        l4 = Some(CachePropCount {
                            prop: Some(prop),
                            count: 1,
                            shared_between_topology,
                        });
                        l4_ids.push(cache_id);
                    },
                    _ => {},
                }
            }
        }

        let [l1d_ids, l1i_ids, l2_ids, l3_ids, l4_ids] = [
            Arc::new(Mutex::new(l1d_ids)),
            Arc::new(Mutex::new(l1i_ids)),
            Arc::new(Mutex::new(l2_ids)),
            Arc::new(Mutex::new(l3_ids)),
            Arc::new(Mutex::new(l4_ids)),
        ];

        let update_cache_ids = |ids: &Arc<Mutex<Vec<u32>>>, cache_id: u32| {
            let mut ids = ids.lock().unwrap();
            if !ids.contains(&cache_id) {
                ids.push(cache_id);
            }
        };

        for cpu in &type_only_list[1..] {
            let cpu = *cpu;
            let cache_leaf = Arc::clone(&cache_leaf);

            let [l1d_ids, l1i_ids, l2_ids, l3_ids, l4_ids] = [
                Arc::clone(&l1d_ids),
                Arc::clone(&l1i_ids),
                Arc::clone(&l2_ids),
                Arc::clone(&l3_ids),
                Arc::clone(&l4_ids),
            ];

            thread::spawn(move || {
                pin_thread(cpu).unwrap();
                let apicid = initial_apic_id!();

                for sub_leaf in 0x0..=0x4 {
                    let cpuid = cpuid!(*cache_leaf, sub_leaf);
                    let prop = CacheProp::from_cpuid(&cpuid);

                    if prop.cache_type == CacheType::Unknown {
                        continue;
                    }
                    
                    let cache_id = Self::get_cache_id(apicid, prop.share_thread);

                    match prop {
                        CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                            update_cache_ids(&l1d_ids, cache_id);
                        },
                        CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                            update_cache_ids(&l1i_ids, cache_id);
                        },
                        CacheProp { level: 2, .. } => {
                            update_cache_ids(&l2_ids, cache_id);
                        },
                        CacheProp { level: 3, .. } => {
                            update_cache_ids(&l3_ids, cache_id);
                        },
                        CacheProp { level: 4, .. } => {
                            update_cache_ids(&l4_ids, cache_id);
                        },
                        _ => {},
                    }
                }
            }).join().unwrap();
        }

        let [l1d_ids, l1i_ids, l2_ids, l3_ids, l4_ids] =
            [l1d_ids, l1i_ids, l2_ids, l3_ids, l4_ids]
            .map(|ids| Arc::try_unwrap(ids).unwrap().into_inner().unwrap() );

        let ids = [l1d_ids, l1i_ids, l2_ids, l3_ids, l4_ids];

        /* Done????? */
        for (cache, ids) in [&mut l1d, &mut l1i, &mut l2, &mut l3, &mut l4].iter_mut().zip(ids) {
            if let Some(cache) = cache {
                cache.count = ids.len() as u32;
            }
        }

        Some(Self {
            l1d,
            l1i,
            l2,
            l3,
            l4,
        })
    }

    fn from_amd_80_1dh(cache_leaf: u32) -> Option<Self> {
        let [mut l1d, mut l1i, mut l2, mut l3, mut l4]: [Option<CachePropCount>; 5]
            = [None, None, None, None, None];
        let total_logical_proc = get_total_logical_processor()?;
        let max_apic_id = max_apic_id!();

        for sub_leaf in 0x0..=0x4 {
            let cpuid = cpuid!(cache_leaf, sub_leaf);
            let prop = CacheProp::from_cpuid(&cpuid);
            let count = total_logical_proc / prop.share_thread;
            let shared_between_topology = Self::shared_all_threads(&prop, max_apic_id);

            match prop {
                CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                    l1d = Some(CachePropCount {
                        prop: Some(prop),
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                    l1i = Some(CachePropCount {
                        prop: Some(prop),
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { level: 2, .. } => {
                    l2 = Some(CachePropCount {
                        prop: Some(prop),
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { level: 3, .. } => {
                    l3 = Some(CachePropCount {
                        prop: Some(prop),
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { level: 4, .. } => {
                    l4 = Some(CachePropCount {
                        prop: Some(prop),
                        count,
                        shared_between_topology,
                    })
                },
                _ => {},
            }
        }

        Some(Self {
            l1d,
            l1i,
            l2,
            l3,
            l4,
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

        cpuid == 0b1
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
        let logi_proc = Arc::new(cpu_list.len() as u32);

        let phy_proc = Arc::new(Mutex::new(0u32));
        let topo_cache: Arc<Mutex<Option<TopoCacheInfo>>> = Arc::new(Mutex::new(None));

        /* To confine the effects of pin_thread */
        {
                let logi_proc = Arc::clone(&logi_proc);
                let phy_proc = Arc::clone(&phy_proc);
                let topo_cache = Arc::clone(&topo_cache);

                thread::spawn(move || {
                    pin_thread(cpu_list[0]).unwrap();

                    let threads_per_core = get_threads_per_core().unwrap_or(1);
                    let mut phy_proc = phy_proc.lock().unwrap();
                    let mut topo_cache = topo_cache.lock().unwrap();

                    *phy_proc = *logi_proc / threads_per_core;
                    *topo_cache = TopoCacheInfo::get_topology_cache_info(&cpu_list);
                }).join().unwrap();
        }

        let num_physical_proc = *phy_proc.lock().unwrap();
        let num_logical_proc = *logi_proc;
        let cache = Arc::try_unwrap(topo_cache).unwrap().into_inner().unwrap();

        Self {
            core_type,
            num_logical_proc,
            num_physical_proc,
            cache,
        }
    }
}
