//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{__cpuid_count, CpuidResult};

pub mod feature_detect;
mod codename;   pub use codename::*;

pub const _AX: u32 = 0x8000_0000;

#[macro_export]
macro_rules! cpuid {
    ($in_eax: expr, $in_ecx: expr) => {
        unsafe {
            __cpuid_count($in_eax, $in_ecx)
        }
    }
}

#[macro_export]
macro_rules! pin_thread { ($cpu: expr) => {
    #[cfg(target_os = "linux")]
    use libc::{cpu_set_t, CPU_SET, CPU_ISSET, CPU_ZERO, sched_setaffinity, sched_getaffinity};
    #[cfg(target_os = "windows")]
    use kernel32::{GetCurrentThread, SetThreadAffinityMask};

    #[cfg(target_os = "linux")]
    unsafe {
        let mut set = mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);
        CPU_SET($cpu, &mut set);

        let status = sched_setaffinity(0, mem::size_of::<cpu_set_t>(), &set);
        if status == -1 {
            eprintln!("sched_setaffinity failed");
            return;
        }
    }
    #[cfg(target_os = "windows")]
    unsafe {
        SetThreadAffinityMask(GetCurrentThread(), 1 << $cpu);
    }
}}

pub fn get_processor_name() -> String {
    let mut name = vec![0x20u8; 48];

    for i in 0..=2 as usize {
        let tmp = cpuid!(_AX + 0x2 + i as u32, 0);
        let reg = [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];

        for j in 0..=3 {
            name[(i*16+j*4)]   =  (reg[j] & 0xff) as u8;
            name[(i*16+j*4+1)] = ((reg[j] >> 8)  & 0xff) as u8;
            name[(i*16+j*4+2)] = ((reg[j] >> 16) & 0xff) as u8;
            name[(i*16+j*4+3)] = ((reg[j] >> 24) & 0xff) as u8;
        }
    }

    return String::from_utf8(name).unwrap();
}

fn get_clflush_size() -> u32 {
    ((cpuid!(0x1, 0).ebx >> 8) & 0xFF) * 8
}

pub struct CacheInfo {
    pub l1d_size: u32, // KiB
    pub l1d_line: u32,
    pub l1d_way:  u32,

    pub l1i_size: u32, // KiB
    pub l1i_line: u32,
    pub l1i_way:  u32,

    pub l2_size:  u32, // KiB
    pub l2_line:  u32,
    pub l2_way:   u32,

    pub l3_size:  u32, // MiB
    pub l3_line:  u32,
    pub l3_way:   u32,

    pub clflush_size: u32, // B
//  pub has_l4:     bool,
}

fn cache_info_intel() -> CacheInfo {

    let sb00h = cpuid!(0x4, 0);
    let sb01h = cpuid!(0x4, 0x1);
    let sb02h = cpuid!(0x4, 0x2);
    let sb03h = cpuid!(0x4, 0x3);

    let size_calc = |ebx: u32, ecx: u32| -> u32 {
        ((ebx >> 22) + 1) * (((ebx >> 12) & 0x3FF) + 1) * ((ebx & 0xFFF) + 1) * (ecx + 1)
    };
    let line = |ebx: u32| -> u32 { (ebx & 0xFFF) + 1 };
    let way  = |ebx: u32| -> u32 { (ebx >> 22) + 1 };

    return CacheInfo {
        l1d_size:   size_calc(sb00h.ebx, sb00h.ecx),
        l1d_line:   line(sb00h.ebx),
        l1d_way:    way(sb00h.ebx),

        l1i_size:   size_calc(sb01h.ebx, sb01h.ecx),
        l1i_line:   line(sb01h.ebx),
        l1i_way:    way(sb01h.ebx),

        l2_size:    size_calc(sb02h.ebx, sb02h.ecx),
        l2_line:    line(sb02h.ebx),
        l2_way:     way(sb02h.ebx),

        l3_size:    size_calc(sb03h.ebx, sb03h.ecx),
        l3_line:    line(sb03h.ebx),
        l3_way:     way(sb03h.ebx),

        clflush_size: get_clflush_size(),
    };
}

fn cache_info_amd() -> CacheInfo {
    let lf_80_05h = cpuid!(_AX + 0x5, 0);
    let lf_80_06h = cpuid!(_AX + 0x6, 0);

    let cache_way = |sub_lf: u32| -> u32 {
        (cpuid!(_AX + 0x1D, sub_lf).ebx >> 22) + 1
    };

    return CacheInfo {
        l1d_size:   (lf_80_05h.ecx >> 24),
        l1d_line:   (lf_80_05h.ecx & 0xFF),
        l1d_way:    (lf_80_05h.ecx >> 16) & 0xFF,

        l1i_size:   (lf_80_05h.edx >> 24),
        l1i_line:   (lf_80_05h.edx & 0xFF),
        l1i_way:    (lf_80_05h.edx >> 16) & 0xFF,

        l2_size:    (lf_80_06h.ecx >> 16),
        l2_line:    lf_80_06h.ecx & 0xFF,
        l2_way:     cache_way(2),

        l3_size:    (lf_80_06h.edx >> 18) / 2,
        l3_line:    lf_80_06h.edx & 0xFF,
        l3_way:     cache_way(3),

        clflush_size: get_clflush_size(),
    };
}

impl CacheInfo {
    fn zero() -> CacheInfo {
        CacheInfo {
            l1d_size:   0,
            l1d_line:   0,
            l1d_way:    0,

            l1i_size:   0,
            l1i_line:   0,
            l1i_way:    0,

            l2_size:    0,
            l2_line:    0,
            l2_way:     0,

            l3_size:    0,
            l3_line:    0,
            l3_way:     0,

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

pub struct FamModStep {
   pub syn_fam: u32,
   pub syn_mod: u32,
   pub step:    u32,
   pub raw_eax: u32,
}

impl FamModStep {
    pub fn get() -> FamModStep {
        let eax = cpuid!(0x1, 0).eax;
        return FamModStep {
            syn_fam:    ((eax >> 8) & 0xf) + ((eax >> 20) & 0xff),
            syn_mod:    ((eax >> 4) & 0xf) + ((eax >> 12) & 0xf0),
            step:       eax & 0xf,
            raw_eax:    eax,
        };
    }
}

pub struct CpuCoreCount {
    pub has_htt:            bool,
    pub phy_core:           u32,
    pub total_thread:       u32,
    pub thread_per_core:    u32,
    pub core_id:            u32,
    pub apic_id:            u32,
}

impl CpuCoreCount {
    pub fn get() -> CpuCoreCount {
        let lf_01h = cpuid!(0x1, 0);
        let lf_04h = cpuid!(0x4, 0);
        //  let lf_0bh = cpuid!(0xB, 0);
        let lf_80_1eh = cpuid!(_AX + 0x1E, 0);

        let _has_htt            = ((lf_01h.edx >> 28) & 0b1) == 1;
        let _total_thread       = (lf_01h.ebx >> 16) & 0xFF;

        let amd_td_per_core = ((lf_80_1eh.ebx >> 8) & 0xFF) + 1;
        let intel_shared_dc = ((lf_04h.eax >> 14) & 0xFFF) + 1;

        let _thread_per_core    =
            if _has_htt && 1 < amd_td_per_core {
                amd_td_per_core
            } else if _has_htt && 1 < intel_shared_dc {
                intel_shared_dc
            } else if _has_htt {
                2
            } else {
                1
            };
        let _phy_core           = _total_thread / _thread_per_core;
        let _apic_id            = (lf_01h.ebx >> 24) & 0xFF;
        //  TODO: CoreID for Intel CPU
        let _core_id            = lf_80_1eh.ebx & 0xFF;

        return CpuCoreCount {
            has_htt:            _has_htt,
            total_thread:       _total_thread,
            thread_per_core:    _thread_per_core,
            phy_core:           _phy_core,
            apic_id:            _apic_id,
            core_id:            _core_id,
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vendor {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl Vendor {
    pub fn get() -> Vendor {
        let tmp = cpuid!(0, 0);
        return Vendor {
            ebx: tmp.ebx,
            ecx: tmp.ecx,
            edx: tmp.edx,
        };
    }
    pub fn amd() -> Vendor {
        Vendor {
            ebx: 0x6874_7541,
            ecx: 0x444D_4163,
            edx: 0x6974_6E65,
        }
    }
    pub fn check_amd() -> bool {
        return Vendor::get() == Vendor::amd();
    }
    pub fn intel() -> Vendor {
        Vendor {
            ebx: 0x756E_6547,
            ecx: 0x4965_6E69,
            edx: 0x6C65_746E,
        }
    }
    pub fn check_intel() -> bool {
        return Vendor::get() == Vendor::intel();
    }
}

pub fn get_vendor_name() -> String {
    let tmp = cpuid!(0, 0);
    let vendor = Vendor {
                    ebx: tmp.ebx,
                    ecx: tmp.ecx,
                    edx: tmp.edx,
                };

    // TODO: add other vendor
    return if vendor == Vendor::amd() {
        format!("AuthenticAMD")
    } else if vendor == Vendor::intel() {
        format!("GenuineIntel")
    } else {
        format!("Unknown")
    };
}

