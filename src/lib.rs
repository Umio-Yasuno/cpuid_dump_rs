#![feature(asm)]
#![allow(dead_code)]

pub mod cpuid_dump;
// use cpuid_dump::line;
pub mod feature_detect;
// use feature_detect::CpuFeature;
static _AX: u32 = 0x8000_0000;

pub fn get_processor_name() -> String {
    let mut a: [u32; 4] = [0; 4];
    let mut name: Vec<u8> = vec![0x20; 48];

    for i in 0..=2 {
        unsafe {
            asm!("cpuid",
                inlateout("eax") _AX + i + 0x2 => a[0],
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

fn amd_cache_way(ecx: u32) -> u32 {
    let mut a: u32;

    unsafe {
        asm!("cpuid",
            in("eax") _AX + 0x1d,
            lateout("ebx") a,
            in("ecx") ecx,
            lateout("edx") _,
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
                inlateout("eax") _AX + 0x1 => a,
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
                in("eax") 0x1,
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

