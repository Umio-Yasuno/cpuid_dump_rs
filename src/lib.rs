#![feature(asm)]
#![allow(dead_code)]

pub mod cpuid_dump;
use cpuid_dump::{line,_AX};
pub mod feature_detect;
// use feature_detect::CpuFeature;

/*
static _AX: u32 = 0x8000_0000;

fn line() {
    for _i in 0..75 {
        print!("=");
    }
    println!();
}
*/

pub fn get_processor_name() -> String {
    let mut a: [u32; 4] = [0; 4];
    let mut name: Vec<u8> = vec![0x20; 48];

    for i in 0..=2 {
        unsafe {
            asm!("cpuid",
                inout("eax") _AX + i + 0x2 => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }
        for j in 0..=3 {
            name[(i*16+j*4) as usize]   =  (a[j as usize] & 0xff) as u8;
            name[(i*16+j*4+1) as usize] = ((a[j as usize] >> 8)  & 0xff) as u8;
            name[(i*16+j*4+2) as usize] = ((a[j as usize] >> 16) & 0xff) as u8;
            name[(i*16+j*4+3) as usize] = ((a[j as usize] >> 24) & 0xff) as u8;
        }
    }

    return String::from_utf8(name).unwrap();
}

pub fn cache_info() {
    println!("Cache info");
    line();

    let mut a: [u32; 4] = [0; 4];

    for i in 0..=4 {
        unsafe {
            asm!("cpuid",
                inlateout("eax") _AX + 0x1d => a[0],
                lateout("ebx") a[1],
                inlateout("ecx") i => a[2],
                lateout("edx") a[3]
            );
        }
        let cache_type =
            match a[0] & 0b11111 {
                1 => "Data",
                2 => "Inst",
                3 => "Unified",
                _    => "unknown",
        };
        let cache_line = (a[1] & 0xfff) + 1;
        let cache_way  = (a[1] >> 22) + 1;
        let cache_set  = a[2] + 1;

        let cache_size = cache_line * cache_way * cache_set;
        let cache_size_str =
            if cache_size < 1000_000 {
                format!("{} KiB,", cache_size / (1 << 10))
            } else if cache_size < 1000_000_000 {
                format!("{} MiB,", cache_size / (1 << 20))
            } else {
                format!("{} B,", cache_size)
            };

        println!("{}: eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
            i, a[0], a[1], a[2], a[3]);
        println!("  Level{0} {1:7} cache: {5:8} {2:3}-byte line size, {3:2}-way {4}-sets",
            a[0] >> 5 & 0b111, cache_type, cache_line, cache_way, cache_set, cache_size_str);
    }
    println!();
}

fn amd_cache_way(ecx: u32) -> u32 {
    let mut a: u32;

    unsafe {
        asm!("cpuid",
            in("eax") _AX + 0x1d,
            lateout("ebx") a,
            in("ecx") ecx,
            out("edx") _,
        );
    }
    return (a >> 22) + 1;
}

pub struct CacheInfo {
    pub l1d_size:   u32, // KiB
    pub l1d_line:   u32,
    pub l1d_way:    u32,
    pub l1i_size:   u32, // KiB
    pub l1i_line:   u32,
    pub l1i_way:    u32,
    pub l2_size:    u32, // KiB
    pub l2_line:    u32,
    pub l2_way:     u32,
    pub l3_size:    u32, // MiB
    pub l3_line:    u32,
    pub l3_way:     u32,
//  pub has_l4:     bool,
}

fn cache_info_amd() -> CacheInfo {
    let mut a: [u32; 4] = [0; 4];
    let mut b: [u32; 4] = [0; 4];

    unsafe {
        asm!("cpuid",
            in("eax") _AX + 0x5,
            lateout("ebx") _,
            lateout("ecx") a[2],
            lateout("edx") a[3]
        );
        asm!("cpuid",
            in("eax") _AX + 0x6,
            lateout("ebx") _,
            lateout("ecx") b[2],
            lateout("edx") b[3]
        );
    }
    
    return CacheInfo {
        l1d_size:   (a[2] >> 24),
        l1d_line:   (a[2] & 0xff),
        l1d_way:    (a[2] >> 16) & 0xff,
        l1i_size:   (a[3] >> 24),
        l1i_line:   (a[3] & 0xff),
        l1i_way:    (a[3] >> 16) & 0xff,
        l2_size:    (b[2] >> 16),
        l2_line:    b[2] & 0xff,
        l2_way:     amd_cache_way(2),
        l3_size:    (b[3] >> 18) / 2,
        l3_line:    b[3] & 0xff,
        l3_way:     amd_cache_way(3),
    };
}

impl CacheInfo {
    pub fn get(fam: u32) -> CacheInfo {
        if 0x15 <= fam && fam <= 0x19 {
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
            }
        }
    }
}

pub struct FamModStep {
   pub syn_fam: u32,
   pub syn_mod: u32,
   pub step:    u32,
}

impl FamModStep {
    pub fn get() -> FamModStep {
        let mut a: u32;

        unsafe {
            asm!("cpuid",
                inout("eax") _AX + 0x1 => a,
                lateout("ebx") _,
                lateout("ecx") _,
                lateout("edx") _,
            );
        }
        return FamModStep {
            syn_fam: ((a >> 8) & 0xf) + ((a >> 20) & 0xff),
            syn_mod: ((a >> 4) & 0xf) + ((a >> 12) & 0xf0),
            step: a & 0xf,
        };
    }
}


pub fn fam_mod_step() {
    let mut a: u32;

    unsafe {
        asm!("cpuid",
            inout("eax") _AX + 0x1 => a,
            lateout("ebx") _,
            lateout("ecx") _,
            lateout("edx") _,
        );
    }

    let base_fam = (a >> 8) & 0xf;
    let ext_fam  = (a >> 20) & 0xff;
    let base_mod = (a >> 4) & 0xf;
    let ext_mod  = (a >> 12) & 0xf0;
    let step     = a & 0xf;

    println!("Base.Family: 0x{0:X} ({0})", base_fam);
    println!("Ext.Family:  0x{0:X} ({0})", ext_fam);
    println!("Base.Model:  0x{0:X} ({0})", base_mod);
    println!("Ext.Model:   0x{0:X} ({0})", ext_mod);
    println!("Family: 0x{0:X} ({0}), Model: 0x{1:X} ({1}), Stepping: {2}",
        base_fam + ext_fam, base_mod + ext_mod, step);
    println!();
}

pub struct CpuCoreCount {
    pub has_htt:            bool,
    pub phy_core:           u32,
    pub total_thread:       u32,
    pub thread_per_core:    u32,
    pub core_id:            u32,
}

impl CpuCoreCount {
    pub fn get() -> CpuCoreCount {
        let mut a: [u32; 4] = [0; 4];
        let mut b: [u32; 4] = [0; 4];

        unsafe {
            asm!("cpuid",
                inlateout("eax") 0x1 => a[0],
                lateout("ebx") a[1],
                lateout("ecx") _,
                lateout("edx") a[3],
            );
            asm!("cpuid",
                in("eax") _AX + 0x1e,
                lateout("ebx") b[1],
                lateout("ecx") b[2],
                lateout("edx") _,
            );
        }

        let _has_htt            = ((a[3] >> 28) & 0x1) == 1;
        let _total_thread       = (a[1] >> 16) & 0xff;
        let _thread_per_core    = ((b[1] >> 8) & 0xff) + 1;
        let _phy_core           = if _has_htt && 1 < _thread_per_core {
                                    _total_thread / _thread_per_core
                                } else if _has_htt {
                                    _total_thread / 2
                                } else {
                                    _total_thread
                                };
        return CpuCoreCount {
            has_htt:            _has_htt,
            total_thread:       _total_thread,
            thread_per_core:    _thread_per_core,
            phy_core:           _phy_core,
            core_id:            (b[1] & 0xff),
        }
    }
}

pub fn core_count() {
    println!("Core Count");
    line();

    let mut a: [u32; 4] = [0; 4];
    let mut b: [u32; 4] = [0; 4];

    unsafe {
        asm!("cpuid",
            inlateout("eax") 0x1 => a[0],
            lateout("ebx") a[1],
            lateout("ecx") _,
            lateout("edx") a[3],
        );
        asm!("cpuid",
            in("eax") _AX + 0x1e,
            lateout("ebx") b[1],
            lateout("ecx") b[2],
            lateout("edx") _,
        );
    }

    let has_htt             = ((a[3] >> 28) & 0x1) == 1;
    let logical_core        = (a[1] >> 16) & 0xff;
    let threads_per_core    = ((b[1] >> 8) & 0xff) + 1;
    let phy_core            = if has_htt && 1 < threads_per_core {
                                logical_core / threads_per_core
                            } else if has_htt {
                                logical_core / 2
                            } else {
                                logical_core
                            };
    let core_id             = b[1] & 0xff;

    println!("0x{:08x}: eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
        0x1, a[0], a[1], a[2], a[3]);
    println!("0x{:08x}: eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
        _AX + 0x1e, b[0], b[1], b[2], b[3]);
    println!("HTT/SMT: {}", has_htt);
    println!("CoreID: {}", core_id);
    println!("{}-Core/{}-Thread", phy_core, logical_core);
    println!("Threads per Core: {}", threads_per_core);
    println!();
}

pub fn get_vendor_name() -> String {
    let mut a: [u32; 4] = [0; 4];

    unsafe {
        asm!("cpuid",
            in("eax") 0,
            lateout("ebx") a[1],
            lateout("ecx") a[2],
            lateout("edx") a[3],
        );
    }

    // TODO: add other vendor
    let vendor_name =
        if a[1] == 0x6874_7541 && a[2] == 0x444D_4163 && a[3] == 0x6974_6E65 {
            format!("AuthenticAMD")
        } else if a[1] == 0x756E_6547 && a[2] == 0x4965_6E69 && a[3] == 0x6C65_746E { 
            format!("GenuineIntel")
        } else {
            format!("Unknown")
        };

    return vendor_name;
}

