//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use super::{_AX, cpuid};

#[macro_export]
macro_rules! bitflag {
    ($x: expr, $pos: expr) => {
        (($x >> $pos) & 1) == 1;
    }
}

pub struct CpuFeature {
/*
    pub skx_avx512:     bool,
    pub cnl_avx512:     bool,
    pub clx_avx512:     bool,
    pub cpx_avx512:     bool,
    pub icl_avx512:     bool,
    pub tgl_avx512:     bool,
    pub spr_avx512:     bool,
*/
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

        cpuid!(a[0], a[1], a[2], a[3], 0x1, 0);
        cpuid!(b[0], b[1], b[2], b[3], 0x7, 0);
        cpuid!(c[0], c[1], c[2], c[3], 0x7, 0x1);
        cpuid!(d[0], d[1], d[2], d[3], _AX + 0x1, 0);
    
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
        let _has_ssbd                   = bitflag!(b[3], 31);
        let _has_l1dflush               = bitflag!(b[3], 28);
        let _has_stibp                  = bitflag!(b[3], 27);
        let _has_ibpb                   = bitflag!(b[3], 26);
        let _has_amx_int8               = bitflag!(b[3], 25);
        let _has_amx_tile               = bitflag!(b[3], 24);
        let _has_avx512_fp16            = bitflag!(b[3], 23);
        let _has_amx_bf16               = bitflag!(b[3], 22);
        let _has_cet_ibt                = bitflag!(b[3], 20);
        let _has_pconfig                = bitflag!(b[3], 18);
        let _has_tsxldtrk               = bitflag!(b[3], 16);
        let _has_hybrid                 = bitflag!(b[3], 15);
        let _has_serialize              = bitflag!(b[3], 14);
        let _has_md_clear               = bitflag!(b[3], 10);
        let _has_avx512_vp2intersect    = bitflag!(b[3], 8);
        let _has_uintr                  = bitflag!(b[3], 5);
        let _has_fsrm                   = bitflag!(b[3], 4); // Fast Short REP MOV
    
        // 0x00000007_EAX_x1
        let _has_lam            = bitflag!(c[0], 26);
        let _has_hreset         = bitflag!(c[0], 22);
        let _has_avx512_bf16    = bitflag!(c[0], 5);
        let _has_avx_vnni       = bitflag!(c[0], 4);
    
        // 0x80000001_ECX
        let _has_prefetchhw     = bitflag!(d[2], 8); // 3dnowprefetch
        let _has_lzcnt          = bitflag!(d[2], 5); // abm
        let _has_lahf           = bitflag!(d[2], 0); // LAHF/SAHF
    
    /*
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
    */
    
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
        /*
            skx_avx512:     _skx_avx512,
            cnl_avx512:     _cnl_avx512,
            clx_avx512:     _clx_avx512,
            cpx_avx512:     _cpx_avx512,
            icl_avx512:     _icl_avx512,
            tgl_avx512:     _tgl_avx512,
            spr_avx512:     _spr_avx512,
        */
            x86_64_v1:      _x86_64_v1,
            x86_64_v2:      _x86_64_v2,
            x86_64_v3:      _x86_64_v3,
            x86_64_v4:      _x86_64_v4,
        }
    }
}

