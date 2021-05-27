//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

#![feature(asm)]
#![allow(dead_code)]

pub mod feature_detect;
pub mod codename;

pub const _AX: u32 = 0x8000_0000;

#[macro_export]
macro_rules! cpuid {
    ($out_eax: expr, $out_ebx: expr, $out_ecx: expr, $out_edx: expr,
    $in_eax: expr, $in_ecx: expr) => {
        unsafe {
            asm!("cpuid",
                inlateout("eax") $in_eax => $out_eax,
                lateout("ebx") $out_ebx,
                inlateout("ecx") $in_ecx => $out_ecx,
                lateout("edx") $out_edx,
            );
        }
    }
}

pub struct cpuid_out {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl cpuid_out {
    pub fn get(in_eax: u32, in_ecx: u32) -> cpuid_out {
        let mut tmp = [0u32; 4];
        cpuid!(tmp[0], tmp[1], tmp[2], tmp[3], in_eax, in_ecx);

        return cpuid_out {
            eax: tmp[0],
            ebx: tmp[1],
            ecx: tmp[2],
            edx: tmp[3],
        }
    }
}

pub fn get_processor_name() -> String {
    let mut name: Vec<u8> = vec![0x20; 48];

    for i in 0..=2 as usize {
        let tmp = cpuid_out::get(_AX + 0x2 + i as u32, 0);
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

fn amd_cache_way(ecx: u32) -> u32 {
    return (cpuid_out::get(_AX + 0x1D, ecx).ebx >> 22) + 1;
}

fn get_clflush_size() -> u32 {
    return ((cpuid_out::get(0x1, 0).ebx >> 8) & 0xFF) * 8;
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

    let sub00h = cpuid_out::get(0x4, 0);
    let sub01h = cpuid_out::get(0x4, 0x1);
    let sub02h = cpuid_out::get(0x4, 0x2);
    let sub03h = cpuid_out::get(0x4, 0x3);

    return CacheInfo {
        l1d_size:   ((sub00h.ebx >> 22) + 1) * ((sub00h.ebx & 0xfff) + 1) * (sub00h.ecx + 1),
        l1d_line:   (sub00h.ebx & 0xfff) + 1,
        l1d_way:    (sub00h.ebx >> 22) + 1,

        l1i_size:   ((sub01h.ebx >> 22) + 1) * ((sub01h.ebx & 0xfff) + 1) * (sub01h.ecx + 1),
        l1i_line:   (sub01h.ebx & 0xfff) + 1,
        l1i_way:    (sub01h.ebx >> 22) + 1,

        l2_size:    ((sub02h.ebx >> 22) + 1) * ((sub02h.ebx & 0xfff) + 1) * (sub02h.ecx + 1),
        l2_line:    (sub02h.ebx & 0xfff) + 1,
        l2_way:     (sub02h.ebx >> 22) + 1,

        l3_size:    ((sub03h.ebx >> 22) + 1) * ((sub03h.ebx & 0xfff) + 1) * (sub03h.ecx + 1),
        l3_line:    (sub03h.ebx & 0xfff) + 1,
        l3_way:     (sub03h.ebx >> 22) + 1,

        clflush_size: get_clflush_size(),
    }
}

fn cache_info_amd() -> CacheInfo {
    let lf_80_05h = cpuid_out::get(_AX + 0x5, 0);
    let lf_80_06h = cpuid_out::get(_AX + 0x6, 0);

    return CacheInfo {
        l1d_size:   (lf_80_05h.ecx >> 24),
        l1d_line:   (lf_80_05h.ecx & 0xFF),
        l1d_way:    (lf_80_05h.ecx >> 16) & 0xFF,

        l1i_size:   (lf_80_05h.edx >> 24),
        l1i_line:   (lf_80_05h.edx & 0xFF),
        l1i_way:    (lf_80_05h.edx >> 16) & 0xFF,

        l2_size:    (lf_80_06h.ecx >> 16),
        l2_line:    lf_80_06h.ecx & 0xFF,
        l2_way:     amd_cache_way(2),

        l3_size:    (lf_80_06h.edx >> 18) / 2,
        l3_line:    lf_80_06h.edx & 0xFF,
        l3_way:     amd_cache_way(3),

        clflush_size: get_clflush_size(),
    };
}

impl CacheInfo {
    pub fn get() -> CacheInfo {
        let fam = FamModStep::get().syn_fam;
        if fam == 0x6 {
            return cache_info_intel();
        } else if 0x15 <= fam && fam <= 0x19 {
            return cache_info_amd();
        } else {
            return CacheInfo {
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
        let eax = cpuid_out::get(0x1, 0).eax;
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
        let lf_01h = cpuid_out::get(0x1, 0);
        let lf_04h = cpuid_out::get(0x4, 0);
        //  let lf_0bh = cpuid_out::get(0xB, 0);
        let lf_80_1eh = cpuid_out::get(_AX + 0x1E, 0);

        let _has_htt            = ((lf_01h.edx >> 28) & 0b1) == 1;
        let _total_thread       = (lf_01h.ebx >> 16) & 0xFF;
        let _thread_per_core    = if _has_htt && 1 < ((lf_80_1eh.ebx >> 8) & 0xFF) + 1 {
                                    ((lf_80_1eh.ebx >> 8) & 0xFF) + 1
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

pub fn get_vendor_name() -> String {

    let tmp = cpuid_out::get(0, 0);

    // TODO: add other vendor
    let vendor_name =
        if tmp.ebx == 0x6874_7541
        && tmp.ecx == 0x444D_4163
        && tmp.edx == 0x6974_6E65 {
            format!("AuthenticAMD")
        } else if tmp.ebx == 0x756E_6547
        && tmp.ecx == 0x4965_6E69
        && tmp.edx == 0x6C65_746E { 
            format!("GenuineIntel")
        } else {
            format!("Unknown")
        };

    return vendor_name;
}

