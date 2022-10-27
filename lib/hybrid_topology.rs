use crate::*;

use std::sync::Arc;
use std::thread;

#[derive(Debug)]
pub struct CachePropCount {
    pub prop: CacheProp,
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

        let len = type_only_list.len();
        let [mut l1d, mut l1i, mut l2, mut l3, mut l4]: [Option<CachePropCount>; 5]
            = [None, None, None, None, None];
        let [mut l1d_ids, mut l1i_ids, mut l2_ids, mut l3_ids, mut l4_ids] = [
            Vec::<u32>::with_capacity(len),
            Vec::<u32>::with_capacity(len),
            Vec::<u32>::with_capacity(len),
            Vec::<u32>::with_capacity(len),
            Vec::<u32>::with_capacity(len),
        ];

        /* fill cache prop */
        thread::scope(|s| s.spawn(|| {
            self::pin_thread(type_only_list[0]).unwrap();
            let eax = cpuid!(0x1, 0x0).eax;
            let apicid = initial_apic_id!(eax);
            let max_apic_id = max_apic_id!(eax);
            
            /* 0x2..=0x4 (L2 Cache .. L4 Cache) ? */
            for sub_leaf in 0x0..=0x4 {
                let cpuid = cpuid!(*cache_leaf, sub_leaf);
                let prop = match CacheProp::option_from_cpuid(&cpuid) {
                    Some(prop) => prop,
                    None => continue,
                };
                let cache_id = Self::get_cache_id(apicid, prop.share_thread);
                let shared_between_topology = Self::shared_all_threads(&prop, max_apic_id);

                match prop {
                    CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                        l1d = Some(CachePropCount {
                            prop,
                            count: 1,
                            shared_between_topology,
                        });
                        l1d_ids.push(cache_id);
                    },
                    CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                        l1i = Some(CachePropCount {
                            prop,
                            count: 1,
                            shared_between_topology,
                        });
                        l1i_ids.push(cache_id);
                    },
                    CacheProp { level: 2, .. } => {
                        l2 = Some(CachePropCount {
                            prop,
                            count: 1,
                            shared_between_topology,
                        });
                        l2_ids.push(cache_id);
                    },
                    CacheProp { level: 3, .. } => {
                        l3 = Some(CachePropCount {
                            prop,
                            count: 1,
                            shared_between_topology,
                        });
                        l3_ids.push(cache_id);
                    },
                    CacheProp { level: 4, .. } => {
                        l4 = Some(CachePropCount {
                            prop,
                            count: 1,
                            shared_between_topology,
                        });
                        l4_ids.push(cache_id);
                    },
                    _ => {},
                }
            }
        }).join().unwrap());

        let update_cache_ids = |ids: &mut Vec<u32>, cache_id: u32| {
            if !ids.contains(&cache_id) {
                ids.push(cache_id);
            }
        };

        let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(type_only_list.len());

        for cpu in &type_only_list[1..] {
            let cpu = *cpu;
            let cache_leaf = Arc::clone(&cache_leaf);

            handles.push(thread::spawn(move || -> Vec<Option<(CacheProp, u32)>> {
                self::pin_thread(cpu).unwrap();
                let apicid = initial_apic_id!();
                let mut props: Vec<Option<(CacheProp, u32)>> = Vec::with_capacity(6);

                for sub_leaf in 0x0..=0x4 {
                    let cpuid = cpuid!(*cache_leaf, sub_leaf);
                    let prop = match CacheProp::option_from_cpuid(&cpuid) {
                        Some(prop) => prop,
                        None => {
                            props.push(None);
                            continue;
                        },
                    };

                    let cache_id = Self::get_cache_id(apicid, prop.share_thread);

                    props.push(Some((prop, cache_id)));
                }

                props
            }));
        }

        for h in handles {
            for (prop, cache_id) in h.join().unwrap().into_iter().flatten() {
                match prop {
                    CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                        update_cache_ids(&mut l1d_ids, cache_id);
                    },
                    CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                        update_cache_ids(&mut l1i_ids, cache_id);
                    },
                    CacheProp { level: 2, .. } => {
                        update_cache_ids(&mut l2_ids, cache_id);
                    },
                    CacheProp { level: 3, .. } => {
                        update_cache_ids(&mut l3_ids, cache_id);
                    },
                    CacheProp { level: 4, .. } => {
                        update_cache_ids(&mut l4_ids, cache_id);
                    },
                    _ => {},
                }
            }
        }

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
            let prop = match CacheProp::option_from_cpuid(&cpuid) {
                Some(prop) => prop,
                None => continue,
            };
            let count = total_logical_proc / prop.share_thread;
            let shared_between_topology = Self::shared_all_threads(&prop, max_apic_id);

            match prop {
                CacheProp { cache_type: CacheType::Data, level: 1, .. } => {
                    l1d = Some(CachePropCount {
                        prop,
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { cache_type: CacheType::Instruction, level: 1, .. } => {
                    l1i = Some(CachePropCount {
                        prop,
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { level: 2, .. } => {
                    l2 = Some(CachePropCount {
                        prop,
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { level: 3, .. } => {
                    l3 = Some(CachePropCount {
                        prop,
                        count,
                        shared_between_topology,
                    })
                },
                CacheProp { level: 4, .. } => {
                    l4 = Some(CachePropCount {
                        prop,
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

    fn get_core_type_only_list(core_type: HybridCoreType) -> Vec<usize> {
        let core_type = Arc::new(core_type);
        let cpu_list = cpu_set_list().unwrap();
        let mut type_only_list: Vec<usize> = Vec::with_capacity(cpu_list.len());
        let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(cpu_list.len());

        for cpu in cpu_list {
            let core_type = Arc::clone(&core_type);

            handles.push(thread::spawn(move || -> Option<usize> {
                self::pin_thread(cpu).unwrap();
                let leaf_1ah = cpuid!(0x1A, 0x0);

                if let Some(cur_core_type) = HybridInfo::get_core_type(&leaf_1ah) {
                    if cur_core_type == *core_type {
                        return Some(cpu);
                    }
                };

                None
            }));
        }

        for h in handles {
            if let Some(cpu) = h.join().unwrap() {
                type_only_list.push(cpu)
            }
        }

        type_only_list
    }

    pub fn get(core_type: HybridCoreType) -> Self {
        let cpu_list = Self::get_core_type_only_list(core_type.clone());
        /* core type only */
        let num_logical_proc = cpu_list.len() as u32;

        /* To confine the effects of pin_thread */
        let (num_physical_proc, cache) = thread::scope(|s| s.spawn(move || {
            self::pin_thread(cpu_list[0]).unwrap();

            let threads_per_core = get_threads_per_core().unwrap_or(1);

            (
                num_logical_proc / threads_per_core,
                TopoCacheInfo::get_topology_cache_info(&cpu_list),
            )
        }).join().unwrap());

        Self {
            core_type,
            num_logical_proc,
            num_physical_proc,
            cache,
        }
    }
}
