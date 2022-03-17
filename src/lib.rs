//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::CpuidResult;

//  pub mod feature_detect;
#[path = "./codename_mod.rs"]
mod codename_mod;
pub use codename_mod::*;

#[path = "./vendor.rs"]
mod vendor;
pub use vendor::*;

#[path = "./micro_arch_level.rs"]
mod micro_arch_level;
pub use micro_arch_level::*;

pub mod cpuid_macro;

pub const _AX: u32 = 0x8000_0000;

#[macro_export]
macro_rules! cpuid {
    ($leaf: expr, $sub_leaf: expr) => {
        unsafe { std::arch::x86_64::__cpuid_count($leaf, $sub_leaf) }
    };
}

#[macro_export]
macro_rules! pin_thread {
    ($cpu: expr) => {
        #[cfg(target_os = "windows")]
        use kernel32::{GetCurrentThread, SetThreadAffinityMask};
        #[cfg(target_os = "linux")]
        use libc::{cpu_set_t, sched_getaffinity, sched_setaffinity, CPU_ALLOC_SIZE, CPU_SET, CPU_ZERO};

        #[cfg(target_os = "linux")]
        unsafe {
            let mut set = std::mem::zeroed::<cpu_set_t>();
            CPU_ZERO(&mut set);
            CPU_SET($cpu, &mut set);

            let status = sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &set);
            if status == -1 {
                eprintln!("sched_setaffinity failed.");
            }
        }
        #[cfg(target_os = "windows")]
        unsafe {
            SetThreadAffinityMask(GetCurrentThread(), 1 << $cpu);
        }
    };
}

pub fn get_proc_name() -> String {
    let mut reg: Vec<u32> = Vec::with_capacity(12);
    let mut name: Vec<u8> = Vec::with_capacity(48);

    for i in 0..=2 {
        let tmp = cpuid!(_AX + 0x2 + i as u32, 0);
        reg.extend([tmp.eax, tmp.ebx, tmp.ecx, tmp.edx]);
    }

    reg.iter().for_each(
        |&val| name.extend( &val.to_le_bytes() )
    );

    return String::from_utf8(name).unwrap();
}

pub fn get_trim_proc_name() -> String {
    //  let pat = ('\u{00}' ..= '\u{20}').collect::<Vec<char>>();
    let pat = ['\u{00}', '\u{20}'];

    return get_proc_name()
        .trim_end_matches(&pat[..])
        .to_string();
}

fn get_clflush_size() -> u32 {
    ((cpuid!(0x1, 0).ebx >> 8) & 0xFF) * 8
}

/*
enum CoreType {
    Core = 0x20, // big
    Atom = 0x40, // small
}

enum CacheType {
    Null = 0,
    Data = 1,
    Inst = 2,
    Unified = 3,
}

struct CacheProp {
    cache_type: String,
    level: u32,
    line_size: u32,
    way: u32,
    set: u32,
    size: u32,
    share_thread: u32,
    size_unit: u32,
    size_unit_string: String,
    inclusive: bool,
}

impl CacheProp {
    fn dec(tmp: CpuidResult) -> CacheProp {
        let [eax, ebx, ecx, edx] = [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];

        let cache_type = match eax & 0b11111 {
            0x1 => "Data",
            0x2 => "Inst",
            0x3 => "Unified",
            0x0 | _ => "",
        }.to_string();

        let level = (eax >> 5) & 0b111;
        let line_size = (ebx & 0xFFF) + 1;
        let way = (ebx >> 22) + 1;
        let set = ecx + 1;
        let size = line_size * way * set;

        let share_thread = ((eax >> 14) & 0xFFF) + 1;

        let mut size_unit = 1;
        let mut size_unit_string = "B";

        if size < 1000_000 {
            size_unit = 1 << 10;
            size_unit_string = "KiB";
        } else if size < 1000_000_000 {
            size_unit = 1 << 20;
            size_unit_string = "MiB";
        };

        let size_unit_string = size_unit_string.to_string();

        let inclusive = (edx & 0b10) != 0;

        CacheProp {
            cache_type,
            level,
            line_size,
            way,
            set,
            size,
            share_thread,
            size_unit,
            size_unit_string,
            inclusive,
        }
    }
}
*/

pub struct CacheInfo {
    pub l1d_size: u32, // KiB
    pub l1d_line: u32,
    pub l1d_way: u32,

    pub l1i_size: u32, // KiB
    pub l1i_line: u32,
    pub l1i_way: u32,

    pub l2_size: u32, // KiB
    pub l2_line: u32,
    pub l2_way: u32,

    pub l3_size: u32, // MiB
    pub l3_line: u32,
    pub l3_way: u32,

    pub clflush_size: u32, // B
                           //  pub has_l4:     bool,
}

fn cache_info_intel() -> CacheInfo {
    let sb00h = cpuid!(0x4, 0);
    let sb01h = cpuid!(0x4, 0x1);
    let sb02h = cpuid!(0x4, 0x2);
    let sb03h = cpuid!(0x4, 0x3);

    let line = |ebx: u32| -> u32 {
        (ebx & 0xFFF) + 1
    };
    let way = |ebx: u32| -> u32 {
        (ebx >> 22) + 1
    };
    let size_calc = |ebx: u32, ecx: u32| -> u32 {
        way(ebx) * (((ebx >> 12) & 0x3FF) + 1) * line(ebx) * (ecx + 1)
    };

    CacheInfo {
        l1d_size: size_calc(sb00h.ebx, sb00h.ecx),
        l1d_line: line(sb00h.ebx),
        l1d_way: way(sb00h.ebx),

        l1i_size: size_calc(sb01h.ebx, sb01h.ecx),
        l1i_line: line(sb01h.ebx),
        l1i_way: way(sb01h.ebx),

        l2_size: size_calc(sb02h.ebx, sb02h.ecx),
        l2_line: line(sb02h.ebx),
        l2_way: way(sb02h.ebx),

        l3_size: size_calc(sb03h.ebx, sb03h.ecx),
        l3_line: line(sb03h.ebx),
        l3_way: way(sb03h.ebx),

        clflush_size: get_clflush_size(),
    }
}

fn cache_info_amd() -> CacheInfo {
    let lf_80_05h = cpuid!(_AX + 0x5, 0);
    let lf_80_06h = cpuid!(_AX + 0x6, 0);

    let cache_way = |sub_lf: u32| -> u32 {
        (cpuid!(_AX + 0x1D, sub_lf).ebx >> 22) + 1
    };

    CacheInfo {
        l1d_size: (lf_80_05h.ecx >> 24),
        l1d_line: (lf_80_05h.ecx & 0xFF),
        l1d_way: (lf_80_05h.ecx >> 16) & 0xFF,

        l1i_size: (lf_80_05h.edx >> 24),
        l1i_line: (lf_80_05h.edx & 0xFF),
        l1i_way: (lf_80_05h.edx >> 16) & 0xFF,

        l2_size: (lf_80_06h.ecx >> 16),
        l2_line: lf_80_06h.ecx & 0xFF,
        l2_way: cache_way(2),

        l3_size: (lf_80_06h.edx >> 18) / 2,
        l3_line: lf_80_06h.edx & 0xFF,
        l3_way: cache_way(3),

        clflush_size: get_clflush_size(),
    }
}

impl CacheInfo {
    fn zero() -> CacheInfo {
        CacheInfo {
            l1d_size: 0,
            l1d_line: 0,
            l1d_way: 0,

            l1i_size: 0,
            l1i_line: 0,
            l1i_way: 0,

            l2_size: 0,
            l2_line: 0,
            l2_way: 0,

            l3_size: 0,
            l3_line: 0,
            l3_way: 0,

            clflush_size: 0,
        }
    }

    pub fn get() -> CacheInfo {
        let fam = FamModStep::get().syn_fam;
        if fam == 0x6 {
            cache_info_intel()
        } else if 0x15 <= fam && fam <= 0x19 {
            cache_info_amd()
        } else {
            CacheInfo::zero()
        }
    }
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
    pub topo_core_count: u32,
    pub topo_thread_count: u32,
    pub thread_per_core: u32,
    pub topo_cache_info: CacheInfo,

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

/*
struct IntelExtTopo {
    
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
