#![feature(asm)]
#![allow(dead_code)]

// use std::ascii;
// use std::str;

static _AX: u32 = 0x8000_0000;

fn line() {
    for _i in 0..75 {
        print!("=");
    }
    println!("");
}

fn cpuid_dump() {
    println!("CPUID Dump");
    line();

    let mut a: [u32; 4] = [0; 4];

    for i in 0x0..=0x10 {
        unsafe {
            asm!(
                "cpuid",
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
            asm!(
                "cpuid",
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

fn cpu_feature() {
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
    let _has_sse        = bitflag!(a[3], 25);
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
    let _has_popcnt     = bitflag!(a[2], 23);
    let _has_movbe      = bitflag!(a[2], 22);
    let _has_sse4_2     = bitflag!(a[2], 20);
    let _has_sse4_1     = bitflag!(a[2], 19);
    let _has_pcid       = bitflag!(a[2], 17);
    let _has_cx16       = bitflag!(a[2], 13);
    let _has_fma3       = bitflag!(a[2], 12);
    let _has_ssse3      = bitflag!(a[2], 9);
    let _has_smx        = bitflag!(a[2], 6);
    let _has_vmx        = bitflag!(a[2], 5);
    let _has_sse3       = bitflag!(a[2], 0);

    // 0x00000007_EBX_x0
    let _has_avx512_vl      = bitflag!(b[1], 31);
    let _has_avx512_bw      = bitflag!(b[1], 30);
    let _has_sha            = bitflag!(b[1], 29);
    let _has_avx512_cd      = bitflag!(b[1], 28);
    let _has_avx512_er      = bitflag!(b[1], 27);
    let _has_avx512_pf      = bitflag!(b[1], 26);
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
    let _has_lahf           = bitflag!(d[2], 0);

    // AVX512
    // Skylake server
    let _skx_feature        = _has_avx512_f && _has_avx512_dq && _has_avx512_ifma
                                && _has_avx512_pf && _has_avx512_er && _has_avx512_cd
                                && _has_avx512_bw && _has_avx512_vl;
    // Cannon Lake
    let _cnl_feature        = _skx_feature && _has_avx512_ifma && _has_avx512_vbmi;
    // Cascade Lake
    let _clx_feature        = _cnl_feature && _has_avx512_vnni;
    // Cooper Lake
    let _cpx_feature        = _clx_feature && _has_avx512_bf16;
    // Ice Lake client/server
    let _icl_feature        = _clx_feature && _has_avx512_vpopcntdq && _has_avx512_bitalg
                                && _has_gfni && _has_vaes;
    // Tiger Lake
    let _tgl_feature        = _icl_feature && _has_avx512_vp2intersect;
    // Sapphire Rapids
    let _spr_feature        = _tgl_feature && _has_avx512_bf16 && _has_avx512_fp16
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
    println!(" HTT: {}", _has_htt);

    println!("ECX=0b{:032b}", a[2]);
    println!(" SSE(: {}, 2: {}, 3: {}, 4.1: {}, 4.2: {})\n SSSE3: {}",
        _has_sse, _has_sse2, _has_sse3, _has_sse4_1, _has_sse4_2, _has_ssse3);
    println!(" AVX(: {}, 2: {},) (XSAVE: {}, OSXSAVE: {})",
        _has_avx, _has_avx2, _has_xsave, _has_osxsave);
    println!(" AVX512(_F: {}, _DQ: {}, _IFMA: {})",
        _has_avx512_f, _has_avx512_dq, _has_avx512_ifma);
    println!(" Skylake-X feature (AVX512): {}",
        _skx_feature);
    println!(" x86-64-v1: {}\n x86-64-v2: {}\n x86-64-v3: {}\n x86-64-v4: {}",
                _x86_64_v1, _x86_64_v2, _x86_64_v3, _x86_64_v4);

    println!();
}


fn get_processor_name() {
    println!("Processor Name");
    line();

    let mut a: [u32; 4] = [0; 4];
    let mut name: Vec<u8> = vec![0x20; 48];

    for i in 0..=2 {
        unsafe {
            asm!(
                "cpuid",
                inout("eax") _AX + i + 0x2 => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }
        for j in 0..=3 {
            name[(i*16+j*4) as usize]   = (a[j as usize] & 0xff) as u8;
            name[(i*16+j*4+1) as usize] = ((a[j as usize] >> 8)  & 0xff) as u8;
            name[(i*16+j*4+2) as usize] = ((a[j as usize] >> 16) & 0xff) as u8;
            name[(i*16+j*4+3) as usize] = ((a[j as usize] >> 24) & 0xff) as u8;
        }
    }
    println!("{}", String::from_utf8(name).unwrap());
    println!();
}

fn cache_info() {
    println!("Cache info");
    line();

    let mut a: [u32; 4] = [0; 4];

    for i in 0..=4 {
        unsafe {
            asm!(
                "cpuid",
                inout("eax") _AX + 0x1d => a[0],
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
        asm!(
            "cpuid",
            in("eax") _AX + 0x1d,
            lateout("ebx") a,
            in("ecx") ecx,
            out("edx") _,
        );
    }
    return (a >> 22) + 1;
}

fn cache_info_amd() {
    println!("Cache info 06h");
    line();

    let mut a: [u32; 4] = [0; 4];

    for i in 5..=6 {
        unsafe {
            asm!(
                "cpuid",
                inout("eax") _AX + i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }

        match i {
            5 => {
                println!("L1 Data Cache: {:3} KiB, {:3}-byte line, {:2}-way",
                    (a[2] >> 24), a[2] & 0xff, (a[2] >> 16) & 0xff);
                println!("L1 Inst Cache: {:3} KiB, {:3}-byte line, {:2}-way",
                    (a[3] >> 24), a[3] & 0xff, (a[3] >> 16) & 0xff);
            },
            6 => {
                let mut b: [u32; 2] = [0; 2];
                b[0] = amd_cache_way(2); // L2cache ways
                b[1] = amd_cache_way(3); // L3cache ways

                println!("L2 Cache:      {:3} KiB, {:3}-byte line, {:2}-way",
                    (a[2] >> 16), a[2] & 0xff, b[0]);
                println!("L3 Cache:      {:3} MiB, {:3}-byte line, {:2}-way",
                    (a[3] >> 18) / 2, a[3] & 0xff, b[1]);
            },
            _ => panic!(),
        }
    }
    println!();
}

fn fam_mod_step() {
    println!("Family, Model, Stepping Identifiers");
    line();

    let mut a: u32;

    unsafe {
        asm!(
            "cpuid",
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

fn core_count() {

}

fn get_vendor_name() {
    println!("Vendor Name");
    line();

    let mut a: [u32; 4] = [0; 4];

    unsafe {
        asm!(
            "cpuid",
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

    println!(" Vendor: {}", vendor_name);
    println!();
}

fn main() {
    println!();
/*
    let a: [u32; 4] = [0; 4];
    let _AX: u32 = 0x80000000;
    
    cpuid_dump();
    cache_info();
*/
    get_vendor_name();
    get_processor_name();
    cache_info_amd();
    fam_mod_step();
    cpu_feature();
}
