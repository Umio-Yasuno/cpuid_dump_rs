//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{__cpuid_count, CpuidResult};

extern crate cpuid_asm;
use cpuid_asm::{cpuid, bitflag, cpuid_macro};
use cpuid_macro::*;

use std::io::Write;

#[path = "./_parse/parse_amd.rs"]
    mod parse_amd;      pub use parse_amd::*;
#[path = "./_parse/parse_intel.rs"]
    mod parse_intel;    pub use parse_intel::*;

#[macro_export]
macro_rules! print_cpuid {
    ($in_eax: expr, $in_ecx: expr, $cpuid: expr) => {
        print!("  0x{:08X}_x{:1X}:  0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X} ",
            $in_eax, $in_ecx,
            $cpuid.eax, $cpuid.ebx, $cpuid.ecx, $cpuid.edx)
    };

    ($out: expr, $in_eax: expr, $in_ecx: expr, $cpuid: expr) => {
        write!($out,
            "    0x{:08X} 0x{:1X}: eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ",
            $in_eax, $in_ecx,
            $cpuid.eax, $cpuid.ebx, $cpuid.ecx, $cpuid.edx).unwrap()
    };
}

#[macro_export]
macro_rules! has_ftr {
    ($ftr_bool: expr, $name_str: expr) =>
        { if $ftr_bool { $name_str } else { "" } };
    ($ftr_bool: expr, $name_str: expr, $else_ftr: expr, $else_name: expr) =>
        { if $ftr_bool { $name_str } else if $else_ftr { $else_name } else { "" } };
}

macro_rules! push { ($buff: expr, $str: expr) =>
    { $buff.push($str.to_string()) }
}

#[macro_export]
macro_rules! flag { ($pos: expr, $reg: expr) =>
    { $pos & $reg != 0 }
}

#[macro_export]
macro_rules! pad { () =>
    { format!("{:62}", "") }
}

#[macro_export]
macro_rules! padln { () =>
    { format!("\n{}", pad!()) }
}

fn print_feature(buff: Vec<String>) {

    macro_rules! if_else { ($expr:expr, $if:expr, $else:expr) =>
        { if $expr {$if} else {$else} }
    }

    let out = std::io::stdout();
    let mut out = out.lock();

    let len = buff.len();
    
    for (c, v) in buff.iter().enumerate() {
        let c = c + 1;

        if 9 < v.len() {
            write!(out, "{0} [{1}]{2}",
                if_else!((c % 3) != 1, padln!(), format!("")),
                v,
                if_else!((c % 3) != 0 && c != len, padln!(), format!(""))
            ).unwrap();
        } else {
            write!(out, " [{}]", v).unwrap();
        }

        if (c % 3) == 0 && c != len {
            write!(out, "{}", padln!()).unwrap();
        }
    }
}

pub fn info_00_01h(eax: u32, ebx: u32) {
    let x86_fam  = ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF);
    let x86_mod  = ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0);
    let x86_step = eax & 0xF;

    print!(" [F: 0x{:X}, M: 0x{:X}, S: 0x{:X}]",
        x86_fam, x86_mod, x86_step);
    let codename = cpuid_asm::get_codename(x86_fam, x86_mod, x86_step).codename;
    print!("{} [{}]", padln!(), codename);
            
    print!("{} [APIC ID: {}]",      padln!(), ebx >> 24);
    print!("{} [Total {} thread]",  padln!(), (ebx >> 16) & 0xFF);
    print!("{} [CLFlush: {}B]",     padln!(), ((ebx >> 8) & 0xFF) * 8);
    //  print!("\n{}", pad!());
    print!("{}", padln!());
}

pub fn feature_00_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = Vec::with_capacity(16);

    // 0x0000_0007_EDX_x0
    if flag!(edx, CPUID_FPU)  { push!(buff, "FPU");  }
    if flag!(edx, CPUID_MMX)  { push!(buff, "MMX");  }
    if flag!(edx, CPUID_FXSR) { push!(buff, "FXSR"); }
    if flag!(edx, CPUID_SSE) {
        buff.push(format!("SSE{0}{1}{2}{3}",
            has_ftr!(flag!(edx, CPUID_SSE2),  "/2"),
            has_ftr!(flag!(edx, CPUID_SSE3),  "/3"),
            has_ftr!(flag!(edx, CPUID_SSE41), "/4.1"),
            has_ftr!(flag!(edx, CPUID_SSE42), "/4.2"),
        ));
    }
    if flag!(edx, CPUID_HTT) { push!(buff, "HTT");  }

    // 0x0000_0007_ECX_x0
    if flag!(ecx, CPUID_FMA)     { push!(buff, "FMA");     }
    if flag!(ecx, CPUID_PCID)    { push!(buff, "PCID")     }
    if flag!(ecx, CPUID_POPCNT)  { push!(buff, "POPCNT");  }
    if flag!(ecx, CPUID_AES)     { push!(buff, "AES");     }
    if flag!(ecx, CPUID_XSAVE)   { push!(buff, "XSAVE");   }
    if flag!(ecx, CPUID_OSXSAVE) { push!(buff, "OSXSAVE"); }
    if flag!(ecx, CPUID_AVX)     { push!(buff, "AVX");     }
    if flag!(ecx, CPUID_F16C)    { push!(buff, "F16C");    }
    if flag!(ecx, CPUID_RDRAND)  { push!(buff, "RDRAND");  }

    print_feature(buff);
}

pub fn feature_00_07h_x0() {
    let tmp = cpuid!(0x7, 0x0);
    print_cpuid!(0x7, 0x0, tmp);

    let [ebx, ecx, edx] = [tmp.ebx, tmp.ecx, tmp.edx];
    let mut buff: Vec<String> = Vec::with_capacity(48);

    // 0x00000007_EBX_x0
    if flag!(ebx, CPUID_FSGSBASE) { push!(buff, "FSGSBASE"); }
    if flag!(ebx, CPUID_SGX)      { push!(buff, "SGX");      }
    if flag!(ebx, CPUID_BMI1) {
        buff.push(format!("BMI1{}", has_ftr!(flag!(ebx, CPUID_BMI2), "/2")));
    }
    if flag!(ebx, CPUID_AVX2)     { push!(buff, "AVX2");       }
    if flag!(ebx, CPUID_SMEP)     { push!(buff, "SMEP");       }
    if flag!(ebx, CPUID_INVPCID)  { push!(buff, "INVPCID");    }
    if flag!(ebx, CPUID_RDSEED)   { push!(buff, "RDSEED");     }
    if flag!(ebx, CPUID_SMAP)     { push!(buff, "SMAP");       }
    if flag!(ebx, CPUID_CLFSHOPT) { push!(buff, "CLFLUSHOPT"); }
    if flag!(ebx, CPUID_CLWB)     { push!(buff, "CLWB");       }
    if flag!(ebx, CPUID_SHA)      { push!(buff, "SHA");        }

    let avx512_f    = flag!(ebx, CPUID_AVX512_F);
    let avx512_dq   = flag!(ebx, CPUID_AVX512_DQ);
    let avx512_ifma = flag!(ebx, CPUID_AVX512_IFMA);
    let avx512_cd   = flag!(ebx, CPUID_AVX512_CD);
    let avx512_bw   = flag!(ebx, CPUID_AVX512_BW);
    let avx512_vl   = flag!(ebx, CPUID_AVX512_VL);

    if avx512_f || avx512_dq || avx512_ifma
    || avx512_cd || avx512_bw || avx512_vl {
        buff.push(
            format!("AVX512_{0}{1}{2}{3}{4}{5}",
                has_ftr!(avx512_f,    "F/"),
                has_ftr!(avx512_dq,   "DQ/"),
                has_ftr!(avx512_ifma, "IFMA/"),
                has_ftr!(avx512_cd,   "CD/"),
                has_ftr!(avx512_bw,   "BW/"),
                has_ftr!(avx512_vl,   "VL/")
            ).trim_end_matches("/").to_string()
        );
    }

    /*  Xeon Phi only */
    if flag!(ebx, CPUID_AVX512_PF) && flag!(ebx, CPUID_AVX512_ER) {
        push!(buff, "AVX512PF/ER");
    }

    // 0x00000007_ECX_x0
    let avx512_vbmi1     = flag!(ebx, CPUID_AVX512_VBMI);
    let avx512_vbmi2     = flag!(ebx, CPUID_AVX512_VBMI2);
    let avx512_vnni      = flag!(ebx, CPUID_AVX512_VNNI);
    let avx512_bitalg    = flag!(ebx, CPUID_AVX512_BITALG);
    let avx512_vpopcntdq = flag!(ebx, CPUID_AVX512_VPOPCNTDQ);

    if avx512_vbmi1 || avx512_vbmi2 || avx512_vnni
    || avx512_bitalg || avx512_vpopcntdq {
        buff.push(format!("AVX512_{0}{1}{2}{3}{4}",
            has_ftr!(avx512_vbmi1,     "VBMI/"),
            has_ftr!(avx512_vbmi2,     "VBMI2/"),
            has_ftr!(avx512_vnni,      "VNNI/"),
            has_ftr!(avx512_bitalg,    "BITALG/"),
            has_ftr!(avx512_vpopcntdq, "VPOPCNTDQ/"),
        ).trim_end_matches("/").to_string())
    }

    if flag!(ecx, CPUID_PKU)       { push!(buff, "PKU");        }
    if flag!(ecx, CPUID_CET_SS)    { push!(buff, "CET_SS");     }
    if flag!(ecx, CPUID_GFNI)      { push!(buff, "GFNI");       }
    if flag!(ecx, CPUID_VAES)      { push!(buff, "VAES");       }
    if flag!(ecx, CPUID_VPCLMULDQ) { push!(buff, "VPCLMULQDQ"); }
    //  if flag!(ecx, CPUID_RDPID) { push!(buff, "RDPID");        }
    if flag!(ecx, CPUID_KL)        { push!(buff, "KL");         }
    if flag!(ecx, CPUID_CLDEMOTE)  { push!(buff, "CLDEMOTE");   }
    if flag!(ecx, CPUID_MOVDIRI) {
        buff.push(format!("MOVDIRI{}", has_ftr!(flag!(ecx, CPUID_MOVDIRI64B), "/64B")))
    }
    if flag!(ecx, CPUID_ENQCMD)    { push!(buff, "ENQCMD"); }

    // 0x00000007_EDX_x0
    let avx512_4vnniw_4fmaps = flag!(edx, CPUID_AVX512_4VNNIW)
                            && flag!(edx, CPUID_AVX512_4FMAPS);
    let avx512_vp2intersect  = flag!(edx, CPUID_AVX512_VP2INTERSECT);
    let avx512_fp16          = flag!(edx, CPUID_AVX512_FP16);

    if avx512_4vnniw_4fmaps || avx512_vp2intersect || avx512_fp16 {
        buff.push(format!("AVX512_{0}{1}{2}",
            /*  Xeon Phi only */
            has_ftr!(avx512_4vnniw_4fmaps, "4VNNIW/4FMAPS/"),
            has_ftr!(avx512_vp2intersect,  "VP2INTERSECT/"),
            has_ftr!(avx512_fp16,          "FP16/"),
        ).trim_end_matches("/").to_string())
    }
    if flag!(edx, CPUID_FSRM)      { push!(buff, "FSRM");      }
    if flag!(edx, CPUID_UINTR)     { push!(buff, "UINTR");     }
    if flag!(edx, CPUID_MD_CLEAR)  { push!(buff, "MD_CLEAR");  }
    if flag!(edx, CPUID_SERIALIZE) { push!(buff, "SERIALIZE"); }
    /*  Currently Intel Sapphire Rapids only */
    if flag!(edx, CPUID_AMX_BF16) && flag!(edx, CPUID_AMX_TILE) && flag!(edx, CPUID_AMX_INT8) {
        push!(buff, format!("AMX-BF16/TILE/INT8"));
    }
/*  Intel CPU only
    if flag!(edx, CPUID_IBPB)      { push!(buff, "IBPB");      }
    if flag!(edx, CPUID_STIBP)     { push!(buff, "STIBP");     }
    if flag!(edx, CPUID_L1D_FLUSH) { push!(buff, "L1D_FLUSH"); }
    if flag!(edx, CPUID_SSBD)      { push!(buff, "SSBD");      }
*/

    print_feature(buff);
    println!();
}

pub fn feature_00_07h_x1() {
    let tmp = cpuid!(0x7, 0x1);
    let eax = tmp.eax;
    print_cpuid!(0x7, 0x1, tmp);
    
    let mut buff: Vec<String> = Vec::with_capacity(4);

    // https://github.com/torvalds/linux/commit/b85a0425d8056f3bd8d0a94ecdddf2a39d32a801
    if flag!(eax, CPUID_AVX_VNNI)    { push!(buff, "AVX_VNNI");    }
    if flag!(eax, CPUID_AVX512_BF16) { push!(buff, "AVX512_BF16"); }
    if flag!(eax, CPUID_HRESET)      { push!(buff, "HRESET");      }
    if flag!(eax, CPUID_LAM)         { push!(buff, "LAM");         }

    print_feature(buff);
    println!();
}

pub fn feature_80_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = Vec::with_capacity(8);

    // 0x8000_0001_EDX_x0
    if flag!(edx, CPUID_3DNOW) {
        buff.push(format!("3DNow!{}", has_ftr!(flag!(edx, CPUID_3DNOW_EXT), "/EXT")));
    }

    // 0x8000_0001_ECX_x0
    if flag!(ecx, CPUID_LAHF_SAHF) { push!(buff, "LAHF/SAHF");      }
    if flag!(ecx, CPUID_LZCNT)     { push!(buff, "LZCNT");          }
    if flag!(ecx, CPUID_SSE4A)     { push!(buff, "SSE4A");          }
    if flag!(ecx, CPUID_3DNOW_PF)  { push!(buff, "3DNow!Prefetch"); }
    if flag!(ecx, CPUID_FMA4)      { push!(buff, "FMA4");           }

    print_feature(buff);
    println!();
}

pub fn cache_prop(in_eax: u32) {
    for ecx in 0..=4 {
        let tmp = cpuid!(in_eax, ecx);

        let cache_type = match tmp.eax & 0b11111 {
            0x1 => "Data",
            0x2 => "Inst",
            0x3 => "Unified",
            0x0|
            _   => "",
        };

        let cache_level = (tmp.eax >> 5) & 0b111;
        let cache_share_thread = ((tmp.eax >> 14) & 0xFFF) + 1;

        let cache_line = (tmp.ebx & 0xFFF) + 1;
        let cache_way  = (tmp.ebx >> 22) + 1;
        let cache_set  = tmp.ecx + 1;
        let cache_size = cache_line * cache_way * cache_set;
        let cache_size_unit =
            if cache_size < 1000_000 {
                format!("{}K", cache_size / (1 << 10))
            } else if cache_size < 1000_000_000 {
                format!("{}M", cache_size / (1 << 20))
            } else {
                format!("{}B", cache_size)
            };

        if cache_level == 0 || cache_type.len() == 0 {
            continue;
        }

        print_cpuid!(in_eax, ecx, tmp);
        print!(" [L{} {:>7}: {:>3}-way, {:>4}]",
            cache_level, cache_type, cache_way, cache_size_unit);
        print!("{} [Shared {}T]", padln!(), cache_share_thread);

        let cache_inclusive = tmp.edx & (1 << 1) != 0;
        if cache_inclusive {
            print!("{} [Inclusive]", padln!());
        }
        println!();
    }    
    
}

pub fn cpu_name(tmp: CpuidResult) {
    let reg = vec![tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];
    let mut name = vec![0x20u8; 16];

    for j in 0..=3usize {
        name[(j*4)]    =  (reg[j] & 0xFF) as u8;
        name[(j*4+1)]  = ((reg[j] >> 8)  & 0xFF) as u8;
        name[(j*4+2)]  = ((reg[j] >> 16) & 0xFF) as u8;
        name[(j*4+3)]  = ((reg[j] >> 24) & 0xFF) as u8;
    }

    print!(" [{}]", String::from_utf8(name).unwrap());
}

