use crate::*;

struct CachePropCount {
    prop: CacheProp,
    count: u32,
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread};

#[derive(Debug)]
pub struct TopoCacheInfo {
    l1d_cache: Option<CacheProp>,
    // l1d_count: u32,
    l1i_cache: Option<CacheProp>,
    // l1i_count: u32,
    l2_cache: Option<CacheProp>,
    l2_count: u32,
    l3_cache: Option<CacheProp>,
    l3_count: u32,
    l4_cache: Option<CacheProp>,
    l4_count: u32,
}

impl TopoCacheInfo {
    pub fn get_topology_cache_info(type_only_list: &[usize]) -> Option<Self> {
        let cache_leaf = Arc::new(CacheProp::get_cache_prop_leaf()?);
        
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
            let apicid = cpuid!(0x1, 0x0).ebx >> 24;
            
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
                        l2 = Some(prop);
                        l2_ids.push(cache_id);
                    },
                    CacheProp { level: 3, .. } => {
                        l3 = Some(prop);
                        l3_ids.push(cache_id);
                    },
                    CacheProp { level: 4, .. } => {
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
            let cache_leaf = cache_leaf.clone();

            let [l2_ids, l3_ids, l4_ids] = [
                Arc::clone(&l2_ids),
                Arc::clone(&l3_ids),
                Arc::clone(&l4_ids),
            ];

            thread::spawn(move || {
                pin_thread(cpu).unwrap();
                let apicid = cpuid!(0x1, 0x0).ebx >> 24;

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

        return Some(Self {
            l1d_cache: l1d,
            l1i_cache: l1i,
            l2_cache: l2,
            l2_count: Arc::try_unwrap(l2_ids).unwrap().into_inner().unwrap().len() as u32,
            l3_cache: l3,
            l3_count: Arc::try_unwrap(l3_ids).unwrap().into_inner().unwrap().len() as u32,
            l4_cache: l4,
            l4_count: Arc::try_unwrap(l4_ids).unwrap().into_inner().unwrap().len() as u32,
        });
    }

    fn from_amd_80_1dh(cache_leaf: u32) -> Option<Self> {
        let [mut l1d, mut l1i, mut l2, mut l3, mut l4]: [Option<CacheProp>; 5]
            = [None, None, None, None, None];
        let total_logical_proc = get_total_logical_processor()?;
        let [mut l2_count, mut l3_count, mut l4_count]: [u32; 3] = [0, 0, 0];

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
                    l2_count = total_logical_proc / prop.share_thread;
                    l2 = Some(prop);
                },
                CacheProp { level: 3, .. } => {
                    l3_count = total_logical_proc / prop.share_thread;
                    l3 = Some(prop);
                },
                CacheProp { level: 4, .. } => {
                    l4_count = total_logical_proc / prop.share_thread;
                    l4 = Some(prop);
                },
                _ => {},
            }
        }

        return Some(Self {
            l1d_cache: l1d,
            l1i_cache: l1i,
            l2_cache: l2,
            l2_count,
            l3_cache: l3,
            l3_count,
            l4_cache: l4,
            l4_count,
        });
    }

    /* ref: [Intel® 64 Architecture Processor Topology Enumeration - intel-64-architecture-processor-topology-enumeration.pdf](https://www.intel.com/content/dam/develop/external/us/en/documents/intel-64-architecture-processor-topology-enumeration.pdf) */
    fn get_cache_id(apicid: u32, num_sharing_thread: u32) -> u32 {
        /* find last bit set */
        let index_msb = u32::BITS - num_sharing_thread.leading_zeros();

        return apicid & !((1 << index_msb) - 1);
    }
}

struct TopoPartInfo {
   core_type: HybridCoreType,
   num_logical_proc: u32,
   num_physical_proc: u32,
   l1i_cache: CachePropCount,
   l1d_cache: CachePropCount,
   l2_cache: CachePropCount,
   l3_cache: CachePropCount,
   l4_cache: CachePropCount,
}

impl TopoPartInfo {
    fn check_hybrid_flag() -> bool {
        let cpuid = (cpuid!(0x7, 0x0).edx >> 15) & 0b1;

        return cpuid == 1;
    }

    fn get_core_type() -> HybridCoreType {
        let hybrid_flag = Self::check_hybrid_flag();

        return if hybrid_flag {
            let leaf_1ah = cpuid!(0x1A, 0x0);

            match HybridInfo::get_core_type(leaf_1ah) {
                Some(t) => t,
                None => HybridCoreType::Invalid,
            }
        } else {
            HybridCoreType::Invalid
        };
    }
    
    fn get_core_type_only_list(core_type: &HybridCoreType) -> Vec<usize> {
        let cpu_list = cpu_set_list().unwrap();
        let mut type_only_list: Vec<usize> = Vec::with_capacity(64);

        for cpu in cpu_list {
            pin_thread(cpu).unwrap();
            let leaf_1ah = cpuid!(0x1A, 0x0);

            if HybridInfo::get_core_type(leaf_1ah).as_ref() == Some(core_type) {
                type_only_list.push(cpu);
            }
        }

        return type_only_list;
    }
}
