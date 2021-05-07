#![feature(asm)]
#![allow(dead_code)]

// use std::ascii;
// use std::str;

static _AX: u32 = 0x8000_0000;

pub fn line() {
    for _i in 0..75 {
        print!("=");
    }
    println!("");
}

pub fn cpuid_dump() {
    println!("CPUID Dump");
    line();

    let mut a: [u32; 4] = [0; 4];

    for i in 0x0..=0x10 {
        unsafe {
            asm!("cpuid",
                inout("eax") i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }

        println!(" 0x{:08X}:  eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
            i, a[0], a[1], a[2], a[3]);
    }

    println!();

    for i in 0x0..=0x20 {
        unsafe {
            asm!("cpuid",
                inout("eax") _AX + i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }

        println!(" 0x{:08X}:  eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
            _AX + i, a[0], a[1], a[2], a[3]);
    }
    println!();
}

macro_rules! bitflag {
    ($x: expr, $pos: expr) => {
        (($x >> $pos) & 1) == 1;
    };
}

pub struct CpuFeature {
    pub skx_avx512:     bool,
    pub cnl_avx512:     bool,
    pub clx_avx512:     bool,
    pub cpx_avx512:     bool,
    pub icl_avx512:     bool,
    pub tgl_avx512:     bool,
    pub spr_avx512:     bool,
    pub x86_64_v1:      bool,
    pub x86_64_v2:      bool,
    pub x86_64_v3:      bool,
    pub x86_64_v4:      bool,
}

impl CpuFeature {
    pub fn get() -> CpuFeature {
        let mut a: [u32; 4] = [0; 4];
        let mut b: [u32; 4] = [0; 4];
        let mut c: [u32; 4] = [0; 4];
        let mut d: [u32; 4] = [0; 4];

        unsafe {
            asm!("cpuid",
                in("eax") 0x1,
                lateout("ebx") _,
                lateout("ecx") a[2],
                lateout("edx") a[3],
            );
            asm! {"cpuid",
                in("eax") 0x7,
                lateout("ebx") b[1],
                inlateout("ecx") 0 => b[2],
                lateout("edx") b[3],
            };
            asm! {"cpuid",
                inlateout("eax") 0x7 => c[0],
                lateout("ebx") _,
                in("ecx") 1,
                lateout("edx") _,
            };
            asm! {"cpuid",
                in("eax") _AX + 0x1,
                lateout("ebx") _,
                lateout("ecx") d[2],
                lateout("edx") d[3],
            };
        }
    
        // 0x00000001_EDX
        let _has_htt        = bitflag!(a[3], 28);
        let _has_sse2       = bitflag!(a[3], 26);
        let _has_sse        = bitflag!(a[3], 25); // Streaming SIMD Extensions
        let _has_fxsr       = bitflag!(a[3], 24);
        let _has_mmx        = bitflag!(a[3], 23);
        let _has_cmov       = bitflag!(a[3], 15);
        let _has_syscall    = bitflag!(a[3], 11);
        let _has_cx8        = bitflag!(a[3], 8);
        let _has_fpu        = bitflag!(a[3], 0);
    
        // 0x00000001_ECX
        let _has_hypervisor = bitflag!(a[2], 31);
        let _has_rdrnd      = bitflag!(a[2], 30);
        let _has_f16c       = bitflag!(a[2], 29);
        let _has_avx        = bitflag!(a[2], 28);
        let _has_osxsave    = bitflag!(a[2], 27);
        let _has_xsave      = bitflag!(a[2], 26);
        let _has_aes        = bitflag!(a[2], 25);
        let _has_tsc_deadline = bitflag!(a[2], 24);
        let _has_popcnt     = bitflag!(a[2], 23);
        let _has_movbe      = bitflag!(a[2], 22);
        let _has_x2apic     = bitflag!(a[2], 21);
        let _has_sse4_2     = bitflag!(a[2], 20);
        let _has_sse4_1     = bitflag!(a[2], 19);
        let _has_pcid       = bitflag!(a[2], 17); // Process-context identifiers
        //  Reserved: let _has_       = bitflag!(a[2], 16);
        let _has_pdcm       = bitflag!(a[2], 15);
        let _has_xtpr       = bitflag!(a[2], 14);
        let _has_cx16       = bitflag!(a[2], 13);
        let _has_fma3       = bitflag!(a[2], 12);
        let _has_sbdg       = bitflag!(a[2], 11);
        let _has_cnxt_id    = bitflag!(a[2], 10);
        let _has_ssse3      = bitflag!(a[2], 9); // Supplemental SSE3
        let _has_tm2        = bitflag!(a[2], 8); // Thermal Monitor 2
        let _has_est        = bitflag!(a[2], 7); // Enhaced SpeedStep
        let _has_smx        = bitflag!(a[2], 6); // Safer Mode Extensions
        let _has_vmx        = bitflag!(a[2], 5); // Virtual Machine Extensions
        let _has_ds_cpl     = bitflag!(a[2], 4); // CPL Qualified Debug Store
        let _has_monitor    = bitflag!(a[2], 3); // MONITOR/MWAIT
        let _has_dtes64     = bitflag!(a[2], 2); // Debus Store
        let _has_pclmulqdq  = bitflag!(a[2], 1);
        let _has_sse3       = bitflag!(a[2], 0); 
    
        // 0x00000007_EBX_x0
        let _has_avx512_vl      = bitflag!(b[1], 31);
        let _has_avx512_bw      = bitflag!(b[1], 30);
        let _has_sha            = bitflag!(b[1], 29);
        let _has_avx512_cd      = bitflag!(b[1], 28);
        let _has_avx512_er      = bitflag!(b[1], 27); // Xeon Phi only
        let _has_avx512_pf      = bitflag!(b[1], 26); // Xeon Phi only
        let _has_intel_pt       = bitflag!(b[1], 25);
        let _has_clwb           = bitflag!(b[1], 24);
        let _has_clflushopt     = bitflag!(b[1], 23);
        let _has_pcommit        = bitflag!(b[1], 22);
        let _has_avx512_ifma    = bitflag!(b[1], 21);
        let _has_avx512_dq      = bitflag!(b[1], 17);
        let _has_avx512_f       = bitflag!(b[1], 16);
        let _has_bmi2           = bitflag!(b[1], 8);
        let _has_avx2           = bitflag!(b[1], 5);
        let _has_bmi1           = bitflag!(b[1], 3);
    
        // 0x00000007_ECX_x0
        let _has_avx512_vpopcntdq   = bitflag!(b[2], 14);
        let _has_avx512_bitalg      = bitflag!(b[2], 12);
        let _has_avx512_vnni        = bitflag!(b[2], 11);
        let _has_vaes               = bitflag!(b[2], 9);
        let _has_gfni               = bitflag!(b[2], 8);
        let _has_avx512_vbmi2       = bitflag!(b[2], 6);
        let _has_avx512_vbmi        = bitflag!(b[2], 1);
    
        // 0x00000007_EDX_x0
        let _has_avx512_fp16            = bitflag!(b[3], 23);
        let _has_avx512_vp2intersect    = bitflag!(b[3], 8);
    
        // 0x00000007_ECX_x1
        let _has_lam            = bitflag!(c[0], 26);
        let _has_hreset         = bitflag!(c[0], 22);
        let _has_avx512_bf16    = bitflag!(c[0], 5);
        let _has_avx_vnni       = bitflag!(c[0], 4);
    
        // 0x80000001_ECX
        let _has_prefetchhw     = bitflag!(d[2], 8); // 3dnowprefetch
        let _has_lzcnt          = bitflag!(d[2], 5); // abm
        let _has_lahf           = bitflag!(d[2], 0); // LAHF/SAHF
    
        // AVX512
        // Skylake server
        let _skx_avx512     = _has_avx512_f && _has_avx512_dq && _has_avx512_ifma
                            && _has_avx512_cd && _has_avx512_bw && _has_avx512_vl;
        // Cannon Lake
        let _cnl_avx512     = _skx_avx512 && _has_avx512_ifma && _has_avx512_vbmi;
        // Cascade Lake
        let _clx_avx512     = _cnl_avx512 && _has_avx512_vnni;
        // Cooper Lake
        let _cpx_avx512     = _clx_avx512 && _has_avx512_bf16;
        // Ice Lake client/server
        let _icl_avx512     = _clx_avx512 && _has_avx512_vpopcntdq && _has_avx512_bitalg
                            && _has_gfni && _has_vaes;
        // Tiger Lake
        let _tgl_avx512     = _icl_avx512 && _has_avx512_vp2intersect;
        // Sapphire Rapids
        let _spr_avx512     = _tgl_avx512 && _has_avx512_bf16 && _has_avx512_fp16
                            && _has_avx_vnni;
    
        // https://gitlab.com/x86-psABIs/x86-64-ABI
        let _x86_64_v1  = _has_cmov && _has_cx8 && _has_fpu
                        && _has_fxsr && _has_mmx && _has_syscall
                        && _has_sse && _has_sse2;
        let _x86_64_v2  = _x86_64_v1 && _has_cx16 && _has_popcnt
                        && _has_sse3 && _has_sse4_1 && _has_sse4_2
                        && _has_ssse3;
        let _x86_64_v3  = _x86_64_v2 && _has_avx && _has_avx2
                        && _has_bmi1 && _has_bmi2 && _has_f16c
                        && _has_fma3 && _has_movbe && _has_osxsave;
        let _x86_64_v4  = _x86_64_v3 && _has_avx512_f && _has_avx512_bw
                        && _has_avx512_cd && _has_avx512_dq && _has_avx512_vl;

        return CpuFeature {
            skx_avx512:     _skx_avx512,
            cnl_avx512:     _cnl_avx512,
            clx_avx512:     _clx_avx512,
            cpx_avx512:     _cpx_avx512,
            icl_avx512:     _icl_avx512,
            tgl_avx512:     _tgl_avx512,
            spr_avx512:     _spr_avx512,
            x86_64_v1:      _x86_64_v1,
            x86_64_v2:      _x86_64_v2,
            x86_64_v3:      _x86_64_v3,
            x86_64_v4:      _x86_64_v4,
        }
    }
}

pub fn cpu_feature() {
    println!("CPU Feature Detect");
    line();

    let mut a: [u32; 4] = [0; 4];
    let mut b: [u32; 4] = [0; 4];
    let mut c: [u32; 4] = [0; 4];
    let mut d: [u32; 4] = [0; 4];

    unsafe {
        asm!(
            "cpuid",
            in("eax") 0x1,
            lateout("ebx") _,
            lateout("ecx") a[2],
            lateout("edx") a[3],
        );
        asm! {
            "cpuid",
            in("eax") 0x7,
            lateout("ebx") b[1],
            inlateout("ecx") 0 => b[2],
            lateout("edx") b[3],
        };
        asm! {
            "cpuid",
            inlateout("eax") 0x7 => c[0],
            lateout("ebx") _,
            in("ecx") 1,
            lateout("edx") _,
        };
        asm! {
            "cpuid",
            in("eax") _AX + 0x1,
            lateout("ebx") _,
            lateout("ecx") d[2],
            lateout("edx") d[3],
        };
    }

    // 0x00000001_EDX
    let _has_htt        = bitflag!(a[3], 28);
    let _has_sse2       = bitflag!(a[3], 26);
    let _has_sse        = bitflag!(a[3], 25); // Streaming SIMD Extensions
    let _has_fxsr       = bitflag!(a[3], 24);
    let _has_mmx        = bitflag!(a[3], 23);
    let _has_cmov       = bitflag!(a[3], 15);
    let _has_syscall    = bitflag!(a[3], 11);
    let _has_cx8        = bitflag!(a[3], 8);
    let _has_fpu        = bitflag!(a[3], 0);

    // 0x00000001_ECX
    let _has_hypervisor = bitflag!(a[2], 31);
    let _has_rdrnd      = bitflag!(a[2], 30);
    let _has_f16c       = bitflag!(a[2], 29);
    let _has_avx        = bitflag!(a[2], 28);
    let _has_osxsave    = bitflag!(a[2], 27);
    let _has_xsave      = bitflag!(a[2], 26);
    let _has_aes        = bitflag!(a[2], 25);
    let _has_tsc_deadline = bitflag!(a[2], 24);
    let _has_popcnt     = bitflag!(a[2], 23);
    let _has_movbe      = bitflag!(a[2], 22);
    let _has_x2apic     = bitflag!(a[2], 21);
    let _has_sse4_2     = bitflag!(a[2], 20);
    let _has_sse4_1     = bitflag!(a[2], 19);
    let _has_pcid       = bitflag!(a[2], 17); // Process-context identifiers
    //  Reserved: let _has_       = bitflag!(a[2], 16);
    let _has_pdcm       = bitflag!(a[2], 15);
    let _has_xtpr       = bitflag!(a[2], 14);
    let _has_cx16       = bitflag!(a[2], 13);
    let _has_fma3       = bitflag!(a[2], 12);
    let _has_sbdg       = bitflag!(a[2], 11);
    let _has_cnxt_id    = bitflag!(a[2], 10);
    let _has_ssse3      = bitflag!(a[2], 9); // Supplemental SSE3
    let _has_tm2        = bitflag!(a[2], 8); // Thermal Monitor 2
    let _has_est        = bitflag!(a[2], 7); // Enhaced SpeedStep
    let _has_smx        = bitflag!(a[2], 6); // Safer Mode Extensions
    let _has_vmx        = bitflag!(a[2], 5); // Virtual Machine Extensions
    let _has_ds_cpl     = bitflag!(a[2], 4); // CPL Qualified Debug Store
    let _has_monitor    = bitflag!(a[2], 3); // MONITOR/MWAIT
    let _has_dtes64     = bitflag!(a[2], 2); // Debus Store
    let _has_pclmulqdq  = bitflag!(a[2], 1);
    let _has_sse3       = bitflag!(a[2], 0); 

    // 0x00000007_EBX_x0
    let _has_avx512_vl      = bitflag!(b[1], 31);
    let _has_avx512_bw      = bitflag!(b[1], 30);
    let _has_sha            = bitflag!(b[1], 29);
    let _has_avx512_cd      = bitflag!(b[1], 28);
    let _has_avx512_er      = bitflag!(b[1], 27); // Xeon Phi only
    let _has_avx512_pf      = bitflag!(b[1], 26); // Xeon Phi only
    let _has_intel_pt       = bitflag!(b[1], 25);
    let _has_clwb           = bitflag!(b[1], 24);
    let _has_clflushopt     = bitflag!(b[1], 23);
    let _has_pcommit        = bitflag!(b[1], 22);
    let _has_avx512_ifma    = bitflag!(b[1], 21);
    let _has_avx512_dq      = bitflag!(b[1], 17);
    let _has_avx512_f       = bitflag!(b[1], 16);
    let _has_bmi2           = bitflag!(b[1], 8);
    let _has_avx2           = bitflag!(b[1], 5);
    let _has_bmi1           = bitflag!(b[1], 3);

    // 0x00000007_ECX_x0
    let _has_avx512_vpopcntdq   = bitflag!(b[2], 14);
    let _has_avx512_bitalg      = bitflag!(b[2], 12);
    let _has_avx512_vnni        = bitflag!(b[2], 11);
    let _has_vaes               = bitflag!(b[2], 9);
    let _has_gfni               = bitflag!(b[2], 8);
    let _has_avx512_vbmi2       = bitflag!(b[2], 6);
    let _has_avx512_vbmi        = bitflag!(b[2], 1);

    // 0x00000007_EDX_x0
    let _has_avx512_fp16            = bitflag!(b[3], 23);
    let _has_avx512_vp2intersect    = bitflag!(b[3], 8);

    // 0x00000007_ECX_x1
    let _has_lam            = bitflag!(c[0], 26);
    let _has_hreset         = bitflag!(c[0], 22);
    let _has_avx512_bf16    = bitflag!(c[0], 5);
    let _has_avx_vnni       = bitflag!(c[0], 4);

    // 0x80000001_ECX
    let _has_prefetchhw     = bitflag!(d[2], 8); // 3dnowprefetch
    let _has_lzcnt          = bitflag!(d[2], 5); // abm
    let _has_lahf           = bitflag!(d[2], 0); // LAHF/SAHF

    // AVX512
    // Skylake server
    let _skx_avx512        = _has_avx512_f && _has_avx512_dq && _has_avx512_ifma
                                && _has_avx512_cd && _has_avx512_bw && _has_avx512_vl;
    // Cannon Lake
    let _cnl_avx512        = _skx_avx512 && _has_avx512_ifma && _has_avx512_vbmi;
    // Cascade Lake
    let _clx_avx512        = _cnl_avx512 && _has_avx512_vnni;
    // Cooper Lake
    let _cpx_avx512        = _clx_avx512 && _has_avx512_bf16;
    // Ice Lake client/server
    let _icl_avx512        = _clx_avx512 && _has_avx512_vpopcntdq && _has_avx512_bitalg
                                && _has_gfni && _has_vaes;
    // Tiger Lake
    let _tgl_avx512        = _icl_avx512 && _has_avx512_vp2intersect;
    // Sapphire Rapids
    let _spr_avx512        = _tgl_avx512 && _has_avx512_bf16 && _has_avx512_fp16
                                && _has_avx_vnni;

    // https://gitlab.com/x86-psABIs/x86-64-ABI
    let _x86_64_v1  = _has_cmov && _has_cx8 && _has_fpu
                        && _has_fxsr && _has_mmx && _has_syscall
                        && _has_sse && _has_sse2;
    let _x86_64_v2  = _x86_64_v1 && _has_cx16 && _has_popcnt
                        && _has_sse3 && _has_sse4_1 && _has_sse4_2
                        && _has_ssse3;
    let _x86_64_v3  = _x86_64_v2 && _has_avx && _has_avx2
                        && _has_bmi1 && _has_bmi2 && _has_f16c
                        && _has_fma3 && _has_movbe && _has_osxsave;
    let _x86_64_v4  = _x86_64_v3 && _has_avx512_f && _has_avx512_bw
                        && _has_avx512_cd && _has_avx512_dq && _has_avx512_vl;

    println!("EDX=0b{:032b}", a[3]);
    println!("ECX=0b{:032b}", a[2]);

    println!(" x86-64-v1: {}\n x86-64-v2: {}\n x86-64-v3: {}\n x86-64-v4: {}",
                _x86_64_v1, _x86_64_v2, _x86_64_v3, _x86_64_v4);

    println!();
}

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
                inout("ecx") i => a[2],
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
    pub l1d_size:   u32,
    pub l1d_line:   u32,
    pub l1d_way:    u32,
    pub l1i_size:   u32,
    pub l1i_line:   u32,
    pub l1i_way:    u32,
    pub l2_size:    u32,
    pub l2_line:    u32,
    pub l2_way:     u32,
    pub l3_size:    u32,
    pub l3_line:    u32,
    pub l3_way:     u32,
/*
*/
//    pub has_l4:     bool,
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
            has_htt: _has_htt,
            total_thread: _total_thread,
            thread_per_core: _thread_per_core,
            phy_core: _phy_core,
            core_id: (b[1] & 0xff),
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
        } else if a[1] == 0x756e_6547 && a[2] == 0x4965_6e69 && a[3] == 0x6c65_746e { 
            format!("GenuineIntel")
        } else {
            format!("Unknown")
        };

    return vendor_name;
}

