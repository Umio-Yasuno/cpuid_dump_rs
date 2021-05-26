//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use super::{_AX, cpuid_out};

#[macro_export]
macro_rules! bitflag {
    ($x: expr, $pos: expr) => {
        (($x >> $pos) & 1) == 1;
    }
}

pub struct Bmi {
    pub has_popcnt: bool,
    pub has_lzcnt:  bool,
    pub has_bmi1:   bool,
    pub has_bmi2:   bool,
    pub has_tbm:    bool,
}

impl Bmi {
    fn get() -> Bmi {
        let lf_01h    = cpuid_out::get(0x1, 0);
        let lf_07h    = cpuid_out::get(0x7, 0);
        let lf_80_01h = cpuid_out::get(_AX + 0x1, 0);

        let _popcnt  = bitflag!(lf_01h.ecx, 23);

        let _bmi1    = bitflag!(lf_07h.ebx,  3);
        let _bmi2    = bitflag!(lf_07h.ebx,  8);

        let _lzcnt   = bitflag!(lf_80_01h.ecx,  5);
        let _tbm     = bitflag!(lf_80_01h.ecx, 21);

        return Bmi {
            has_popcnt: _popcnt,
            has_lzcnt:  _lzcnt,
            has_bmi1:   _bmi1,
            has_bmi2:   _bmi2,
            has_tbm:    _tbm,
        }
    }
}

// https://gitlab.com/x86-psABIs/x86-64-ABI
pub struct x86_64_abi {
    pub v1:     bool,
    pub v2:     bool,
    pub v3:     bool,
    pub v4:     bool,
}

impl x86_64_abi {
    pub fn get() -> x86_64_abi {
        let lf_01h      = cpuid_out::get(0x1, 0);
        let lf_07h      = cpuid_out::get(0x7, 0);
        let lf_80_01h   = cpuid_out::get(_AX + 0x1, 0);
    
        //  0x00000001_EDX
        //  let _has_htt        = bitflag!(lf_01h.edx, 28);
        let _has_sse2       = bitflag!(lf_01h.edx, 26);
        let _has_sse        = bitflag!(lf_01h.edx, 25); // Streaming SIMD Extensions
        let _has_fxsr       = bitflag!(lf_01h.edx, 24);
        let _has_mmx        = bitflag!(lf_01h.edx, 23);
        let _has_cmov       = bitflag!(lf_01h.edx, 15);
        let _has_syscall    = bitflag!(lf_01h.edx, 11);
        let _has_cx8        = bitflag!(lf_01h.edx, 8);
        let _has_fpu        = bitflag!(lf_01h.edx, 0);
    
        let _x86_64_v1  =  _has_fpu && _has_cx8 && _has_syscall 
                        && _has_cmov && _has_mmx && _has_fxsr
                        && _has_sse && _has_sse2;

        //  0x00000001_ECX
        //  let _has_hypervisor = bitflag!(lf_01h.ecx, 31);
        //  let _has_rdrnd      = bitflag!(lf_01h.ecx, 30);
        let _has_f16c       = bitflag!(lf_01h.ecx, 29);
        let _has_avx        = bitflag!(lf_01h.ecx, 28);
        let _has_osxsave    = bitflag!(lf_01h.ecx, 27);
        let _has_xsave      = bitflag!(lf_01h.ecx, 26);
        //  let _has_aes        = bitflag!(lf_01h.ecx, 25);
        //  let _has_tsc_deadline = bitflag!(lf_01h.ecx, 24);
        let _has_popcnt     = bitflag!(lf_01h.ecx, 23);
        let _has_movbe      = bitflag!(lf_01h.ecx, 22);
        //  let _has_x2apic     = bitflag!(lf_01h.ecx, 21);
        let _has_sse4_2     = bitflag!(lf_01h.ecx, 20);
        let _has_sse4_1     = bitflag!(lf_01h.ecx, 19);
        //  let _has_pcid       = bitflag!(lf_01h.ecx, 17); // Process-context identifiers
        //  Reserved: let _has_       = bitflag!(lf_01h.ecx, 16);
        //  let _has_pdcm       = bitflag!(lf_01h.ecx, 15);
        //  let _has_xtpr       = bitflag!(lf_01h.ecx, 14);
        let _has_cx16       = bitflag!(lf_01h.ecx, 13);
        let _has_fma3       = bitflag!(lf_01h.ecx, 12);
        //  let _has_sbdg       = bitflag!(lf_01h.ecx, 11);
        //  let _has_cnxt_id    = bitflag!(lf_01h.ecx, 10);
        let _has_ssse3      = bitflag!(lf_01h.ecx, 9); // Supplemental SSE3
        //  let _has_tm2        = bitflag!(lf_01h.ecx, 8); // Thermal Monitor 2
        //  let _has_est        = bitflag!(lf_01h.ecx, 7); // Enhaced SpeedStep
        //  let _has_smx        = bitflag!(lf_01h.ecx, 6); // Safer Mode Extensions
        //  let _has_vmx        = bitflag!(lf_01h.ecx, 5); // Virtual Machine Extensions
        //  let _has_ds_cpl     = bitflag!(lf_01h.ecx, 4); // CPL Qualified Debug Store
        //  let _has_monitor    = bitflag!(lf_01h.ecx, 3); // MONITOR/MWAIT
        //  let _has_dtes64     = bitflag!(lf_01h.ecx, 2); // Debus Store
        //  let _has_pclmulqdq  = bitflag!(lf_01h.ecx, 1);
        let _has_sse3       = bitflag!(lf_01h.ecx, 0); 
    
        //  0x80000001_ECX
        //  let _has_prefetchhw     = bitflag!(lf_80_01h.ecx, 8); // 3dnowprefetch
        let _has_lzcnt          = bitflag!(lf_80_01h.ecx, 5); // abm
        let _has_lahf           = bitflag!(lf_80_01h.ecx, 0); // LAHF/SAHF
   
        let _x86_64_v2  = _x86_64_v1 && _has_cx16 && _has_lahf 
                        && _has_popcnt && _has_sse3 && _has_sse4_1
                        && _has_sse4_2 && _has_ssse3;

        // 0x00000007_EBX_x0
        let _has_avx512_vl      = bitflag!(lf_07h.ebx, 31);
        let _has_avx512_bw      = bitflag!(lf_07h.ebx, 30);
        //  let _has_sha            = bitflag!(lf_07h.ebx, 29);
        let _has_avx512_cd      = bitflag!(lf_07h.ebx, 28);
        //  let _has_avx512_er      = bitflag!(lf_07h.ebx, 27); // Xeon Phi only
        //  let _has_avx512_pf      = bitflag!(lf_07h.ebx, 26); // Xeon Phi only
        //  let _has_intel_pt       = bitflag!(lf_07h.ebx, 25);
        //  let _has_clwb           = bitflag!(lf_07h.ebx, 24);
        //  let _has_clflushopt     = bitflag!(lf_07h.ebx, 23);
        //  let _has_pcommit        = bitflag!(lf_07h.ebx, 22);
        //  let _has_avx512_ifma    = bitflag!(lf_07h.ebx, 21);
        let _has_avx512_dq      = bitflag!(lf_07h.ebx, 17);
        let _has_avx512_f       = bitflag!(lf_07h.ebx, 16);
        let _has_bmi2           = bitflag!(lf_07h.ebx, 8);
        let _has_avx2           = bitflag!(lf_07h.ebx, 5);
        let _has_bmi1           = bitflag!(lf_07h.ebx, 3);

        let _x86_64_v3  = _x86_64_v2 && _has_avx && _has_avx2
                        && _has_bmi1 && _has_bmi2 && _has_f16c
                        && _has_fma3 && _has_lzcnt && _has_movbe
                        && _has_osxsave;
        let _x86_64_v4  = _x86_64_v3 && _has_avx512_f && _has_avx512_bw
                        && _has_avx512_cd && _has_avx512_dq && _has_avx512_vl;

        return x86_64_abi {
            v1:     _x86_64_v1,
            v2:     _x86_64_v2,
            v3:     _x86_64_v3,
            v4:     _x86_64_v4,
        }
        // 0x00000007_ECX_x0
    /*
        let _has_avx512_vpopcntdq   = bitflag!(b[2], 14);
        let _has_avx512_bitalg      = bitflag!(b[2], 12);
        let _has_avx512_vnni        = bitflag!(b[2], 11);
        let _has_vaes               = bitflag!(b[2], 9);
        let _has_gfni               = bitflag!(b[2], 8);
        let _has_avx512_vbmi2       = bitflag!(b[2], 6);
        let _has_avx512_vbmi        = bitflag!(b[2], 1);
    */
        // 0x00000007_EDX_x0
    /*
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
    */
        // 0x00000007_EAX_x1
    /*
        let _has_lam            = bitflag!(c[0], 26);
        let _has_hreset         = bitflag!(c[0], 22);
        let _has_avx512_bf16    = bitflag!(c[0], 5);
        let _has_avx_vnni       = bitflag!(c[0], 4);
    */
    }
}

pub struct intel_avx512 {
    pub skx_avx512: bool,   // Skylake server
    pub cnl_avx512: bool,   // Cannon Lake, Palm Cove
    pub clx_avx512: bool,   // Cascade Lake
    pub cpx_avx512: bool,   // Cooper lake
    pub icl_avx512: bool,   // Ice Lake client/server, Sunny Cove
    pub tgl_avx512: bool,   // Tiger Lake, Willow Cove
    pub spr_avx512: bool,   // Sapphire Rapids, Golden Cove
}

impl intel_avx512 {
    pub fn get() -> intel_avx512 {
        let lf_07h              = cpuid_out::get(0x7, 0);
        let lf_07h_sub_01h      = cpuid_out::get(0x7, 0x1);

        //  0x00000007_EBX_x0
        let _has_avx512_vl      = bitflag!(lf_07h.ebx, 31);
        let _has_avx512_bw      = bitflag!(lf_07h.ebx, 30);
        let _has_avx512_cd      = bitflag!(lf_07h.ebx, 28);
        //  let _has_avx512_er      = bitflag!(lf_07h.ebx, 27); // Xeon Phi only
        //  let _has_avx512_pf      = bitflag!(lf_07h.ebx, 26); // Xeon Phi only
        let _has_avx512_ifma    = bitflag!(lf_07h.ebx, 21);
        let _has_avx512_dq      = bitflag!(lf_07h.ebx, 17);
        let _has_avx512_f       = bitflag!(lf_07h.ebx, 16);
        
        //  0x00000007_ECX_x0
        let _has_avx512_vpopcntdq   = bitflag!(lf_07h.ecx, 14);
        let _has_avx512_bitalg      = bitflag!(lf_07h.ecx, 12);
        let _has_avx512_vnni        = bitflag!(lf_07h.ecx, 11);
        let _has_vaes               = bitflag!(lf_07h.ecx, 9);
        let _has_gfni               = bitflag!(lf_07h.ecx, 8);
        let _has_avx512_vbmi2       = bitflag!(lf_07h.ecx, 6);
        let _has_avx512_vbmi        = bitflag!(lf_07h.ecx, 1);

        //  0x00000007_EDX_x0
        //  let _has_amx_int8               = bitflag!(lf_07h.edx, 25);
        //  let _has_amx_tile               = bitflag!(lf_07h.edx, 24);
        let _has_avx512_fp16            = bitflag!(lf_07h.edx, 23);
        let _has_amx_bf16               = bitflag!(lf_07h.edx, 22);
        let _has_avx512_vp2intersect    = bitflag!(lf_07h.edx, 8);

        //  0x00000007_EAX_x1
        let _has_avx512_bf16    = bitflag!(lf_07h_sub_01h.eax,  5);
        //  let _has_avx_vnni            = bitflag!(lf_07h_sub_01h.eax,  4);

        let _skx_avx512     = _has_avx512_f && _has_avx512_dq && _has_avx512_ifma
                            && _has_avx512_cd && _has_avx512_bw && _has_avx512_vl;
        let _cnl_avx512     = _skx_avx512 && _has_avx512_ifma && _has_avx512_vbmi;
        let _clx_avx512     = _cnl_avx512 && _has_avx512_vnni;
        let _cpx_avx512     = _clx_avx512 && _has_avx512_bf16;
        let _icl_avx512     = _clx_avx512 && _has_avx512_vpopcntdq && _has_avx512_bitalg
                            && _has_gfni && _has_vaes;
        let _tgl_avx512     = _icl_avx512 && _has_avx512_vp2intersect;
        let _spr_avx512     = _tgl_avx512 && _has_avx512_bf16 && _has_avx512_fp16
                            /* && _has_avx_vnni */;

        return intel_avx512 {
            skx_avx512:     _skx_avx512,
            cnl_avx512:     _cnl_avx512,
            clx_avx512:     _clx_avx512,
            cpx_avx512:     _cpx_avx512,
            icl_avx512:     _icl_avx512,
            tgl_avx512:     _tgl_avx512,
            spr_avx512:     _spr_avx512,
        }
    }
}
    

