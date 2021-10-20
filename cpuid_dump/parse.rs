//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{CpuidResult, __cpuid_count};

extern crate cpuid_asm;
use cpuid_asm::{cpuid};

use std::io::Write;

#[path = "./_parse/parse_amd.rs"]
mod parse_amd;
pub use parse_amd::*;
#[path = "./_parse/parse_intel.rs"]
mod parse_intel;
pub use parse_intel::*;

#[macro_export]
macro_rules! print_cpuid {
    ($in_eax: expr, $in_ecx: expr, $cpuid: expr) => {
        print!(
            "  0x{:08X}_x{:1X}:  0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X} ",
            $in_eax, $in_ecx, $cpuid.eax, $cpuid.ebx, $cpuid.ecx, $cpuid.edx
        )
    };

    ($out: expr, $in_eax: expr, $in_ecx: expr, $cpuid: expr) => {
        write!(
            $out,
            "    0x{:08X} 0x{:1X}: eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ",
            $in_eax, $in_ecx, $cpuid.eax, $cpuid.ebx, $cpuid.ecx, $cpuid.edx
        )
        .unwrap()
    };
}

#[macro_export]
macro_rules! has_ftr {
    ($ftr_bool: expr, $name_str: expr) => {
        if $ftr_bool {
            $name_str
        } else {
            ""
        }
    };
    ($ftr_bool: expr, $name_str: expr, $else_ftr: expr, $else_name: expr) => {
        if $ftr_bool {
            $name_str
        } else if $else_ftr {
            $else_name
        } else {
            ""
        }
    };
}

macro_rules! push {
    ($buff: expr, $str: expr) => {
        $buff.push($str.to_string())
    };
}

#[macro_export]
macro_rules! flag {
    ($pos: expr, $reg: expr) => {
        $pos & $reg != 0
    };
}

#[macro_export]
macro_rules! pad {
    () => {
        format!("{:62}", "")
    };
}

#[macro_export]
macro_rules! padln {
    () => {
        format!("\n{}", pad!())
    };
}

struct Reg { reg: u32 }

impl Reg {
    fn new(reg: u32) -> Reg {
        Reg { reg }
    }

    fn to_bitvec(self) -> Vec<u8> {
        let mut bit_vec = vec![0u8; 32];
        for i in 0..32 {
            bit_vec[i] = ((self.reg >> i) & 1) as u8;
        }
        return bit_vec;
    }

    fn to_boolvec(self) -> Vec<bool> {
        self.to_bitvec().iter().map(|&x| x == 1 ).collect()
    }
}

fn print_feature(buff: Vec<String>) {
    macro_rules! if_else { ($expr:expr, $if:expr, $else:expr) => {
        if $expr { $if } else { $else }
    }}

    let out = std::io::stdout();
    let mut out = out.lock();

    let len = buff.len();

    for (c, v) in buff.iter().enumerate() {
        let c = c + 1;

        if 9 < v.len() {
            write!(out,
                "{0} [{1}]{2}",
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
    let x86_fam = ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF);
    let x86_mod = ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0);
    let x86_step = eax & 0xF;

    print!(" [F: 0x{:X}, M: 0x{:X}, S: 0x{:X}]",
        x86_fam, x86_mod, x86_step);
    let codename = cpuid_asm::get_codename(x86_fam, x86_mod, x86_step).codename;
    print!("{} [{}]", padln!(), codename);

    print!("{} [APIC ID: {}]", padln!(), ebx >> 24);
    print!("{} [Total {} thread(s)]", padln!(), (ebx >> 16) & 0xFF);
    print!("{} [CLFlush: {}B]", padln!(), ((ebx >> 8) & 0xFF) * 8);
    //  print!("\n{}", pad!());
    print!("{}", padln!());
}

pub fn feature_00_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = Vec::with_capacity(16);

    // 0x0000_0007_EDX_x0
    let edx = Reg::new(edx).to_boolvec();
    let ecx = Reg::new(ecx).to_boolvec();

    if edx[ 0] { push!(buff, "FPU")     }
    if edx[23] { push!(buff, "MMX")     }
    if edx[24] { push!(buff, "FXSR")    }
    if edx[28] { push!(buff, "HTT")     }

    // 0x0000_0007_ECX_x0
    if ecx[12] { push!(buff, "FMA")     }
    if ecx[17] { push!(buff, "PCID")    }
    if ecx[23] { push!(buff, "POPCNT")  }
    if ecx[25] { push!(buff, "AES")     }
    if ecx[26] { push!(buff, "XSAVE")   }
    if ecx[27] { push!(buff, "OSXSAVE") }
    if ecx[28] { push!(buff, "AVX")     }
    if ecx[29] { push!(buff, "F16C")    }
    if ecx[30] { push!(buff, "RDRAND")  }

    if edx[25] {
        buff.push(format!(
            "SSE{0}{1}{2}{3}",
            has_ftr!(edx[26], "/2"),
            has_ftr!(ecx[ 0], "/3"),
            has_ftr!(ecx[19], "/4.1"),
            has_ftr!(ecx[20], "/4.2"),
        ));
    }

    print_feature(buff);
}

pub fn feature_00_07h_x0() {
    let tmp = cpuid!(0x7, 0x0);
    print_cpuid!(0x7, 0x0, tmp);

    let [ebx, ecx, edx] = [
        Reg::new(tmp.ebx).to_boolvec(),
        Reg::new(tmp.ecx).to_boolvec(),
        Reg::new(tmp.edx).to_boolvec(),
    ];
    let mut buff: Vec<String> = Vec::with_capacity(48);

    // 0x00000007_EBX_x0
    if ebx[ 0] { push!(buff, "FSGSBASE") }

    if ebx[ 2] { push!(buff, "SGX")      }
    if ebx[ 3] {
        buff.push(
            format!("BMI1{}", has_ftr!(ebx[8], "/2"))
        );
    }
    if ebx[ 5] { push!(buff, "AVX2")       }
    if ebx[ 7] { push!(buff, "SMEP")       }
    if ebx[10] { push!(buff, "INVPCID")    }
    if ebx[18] { push!(buff, "RDSEED")     }
    if ebx[20] { push!(buff, "SMAP")       }
    if ebx[23] { push!(buff, "CLFLUSHOPT") }
    if ebx[24] { push!(buff, "CLWB")       }
    if ebx[29] { push!(buff, "SHA")        }

    let avx512_f    = ebx[16];
    let avx512_dq   = ebx[17];
    let avx512_ifma = ebx[21];
    let avx512_cd   = ebx[28];
    let avx512_bw   = ebx[30];
    let avx512_vl   = ebx[31];

    if avx512_f || avx512_dq || avx512_ifma || avx512_cd
    || avx512_bw || avx512_vl {
        buff.push(
            format!(
                "AVX512_{0}{1}{2}{3}{4}{5}",
                has_ftr!(avx512_f, "F/"),
                has_ftr!(avx512_dq, "DQ/"),
                has_ftr!(avx512_ifma, "IFMA/"),
                has_ftr!(avx512_cd, "CD/"),
                has_ftr!(avx512_bw, "BW/"),
                has_ftr!(avx512_vl, "VL/")
            )
            .trim_end_matches("/")
            .to_string(),
        )
    }

    /*  Xeon Phi only */
    if ebx[26] && ebx[27] {
        push!(buff, "AVX512PF/ER");
    }

    // 0x00000007_ECX_x0
    let avx512_vbmi1     = ebx[ 1];
    let avx512_vbmi2     = ebx[ 6];
    let avx512_vnni      = ebx[11];
    let avx512_bitalg    = ebx[12];
    let avx512_vpopcntdq = ebx[14];

    if avx512_vbmi1 || avx512_vbmi2 || avx512_vnni
    || avx512_bitalg || avx512_vpopcntdq {
        buff.push(
            format!(
                "AVX512_{0}{1}{2}{3}{4}",
                has_ftr!(avx512_vbmi1,     "VBMI/"),
                has_ftr!(avx512_vbmi2,     "VBMI2/"),
                has_ftr!(avx512_vnni,      "VNNI/"),
                has_ftr!(avx512_bitalg,    "BITALG/"),
                has_ftr!(avx512_vpopcntdq, "VPOPCNTDQ/"),
            )
            .trim_end_matches("/")
            .to_string(),
        )
    }

    if ecx[ 3] { push!(buff, "PKU")        }
    if ecx[ 7] { push!(buff, "CET_SS")     }
    if ecx[ 8] { push!(buff, "GFNI")       }
    if ecx[ 9] { push!(buff, "VAES")       }
    if ecx[10] { push!(buff, "VPCLMULQDQ") }
    // if ecx[22] { push!(buff, "RDPID") }
    if ecx[23] { push!(buff, "KL")         }
    if ecx[25] { push!(buff, "CLDEMOTE")   }
    if ecx[27] {
        buff.push(
            format!("MOVDIRI{}", has_ftr!(ecx[28], "/64B"))
        )
    }
    if ecx[29] { push!(buff, "ENQCMD")     }

    // 0x00000007_EDX_x0
    /* Xeon Phi Only */
    let avx512_4vnniw_4fmaps = edx[ 2] && edx[ 3];

    let avx512_vp2intersect  = edx[ 8];
    let avx512_fp16          = edx[23];

    if avx512_4vnniw_4fmaps || avx512_vp2intersect || avx512_fp16 {
        buff.push(
            format!("AVX512_{0}{1}{2}",
                has_ftr!(avx512_4vnniw_4fmaps, "4VNNIW/4FMAPS/"),
                has_ftr!(avx512_vp2intersect,  "VP2INTERSECT/"),
                has_ftr!(avx512_fp16,          "FP16/"),
            )
            .trim_end_matches("/")
            .to_string(),
        )
    }
    /* Fast Short REP MOV */
    if edx[ 4] { push!(buff, "FSRM") }
    if edx[ 5] { push!(buff, "UINTR") }
    if edx[10] { push!(buff, "MD_CLEAR") }
    if edx[14] { push!(buff, "SERIALIZE") }

    /*  Currently Intel Sapphire Rapids only
        Bit 22: AMX-BF16,
        Bit 24: AMX-TILE,
        Bit 25: AMX-INT8
    */
    if edx[22] && edx[24] && edx[25] {
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
    print_cpuid!(0x7, 0x1, tmp);

    let eax = Reg::new(tmp.eax).to_boolvec();
    let mut buff: Vec<String> = Vec::with_capacity(4);

    // https://github.com/torvalds/linux/commit/b85a0425d8056f3bd8d0a94ecdddf2a39d32a801
    if eax[ 4] { push!(buff, "AVX_VNNI")    }
    if eax[ 5] { push!(buff, "AVX512_BF16") }
    if eax[22] { push!(buff, "HRESET")      }
    if eax[26] { push!(buff, "LAM")         }

    print_feature(buff);
    println!();
}

pub fn feature_80_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = Vec::with_capacity(8);
    let ecx = Reg::new(ecx).to_boolvec();
    let edx = Reg::new(edx).to_boolvec();

    // 0x8000_0001_EDX_x0
    if edx[31] {
        buff.push(
            format!("3DNow!{}", has_ftr!(edx[30], "/EXT"))
        )
    }

    // 0x8000_0001_ECX_x0
    if ecx[ 0] { push!(buff, "LAHF/SAHF") }
    if ecx[ 5] { push!(buff, "LZCNT") }
    if ecx[ 6] { push!(buff, "SSE4A") }
    if ecx[ 8] { push!(buff, "3DNow!Prefetch") }
    if ecx[16] { push!(buff, "FMA4") }

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
            0x0 | _ => "",
        };

        let cache_level = (tmp.eax >> 5) & 0b111;
        let cache_line  = (tmp.ebx & 0xFFF) + 1;
        let cache_way   = (tmp.ebx >> 22) + 1;
        let cache_set   = tmp.ecx + 1;
        let cache_size  = cache_line * cache_way * cache_set;

        let cache_share_thread = ((tmp.eax >> 14) & 0xFFF) + 1;

        let cache_size_unit = if cache_size < 1000_000 {
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

    for j in 0..4usize {
        name[(j * 4)] = (reg[j] & 0xFF) as u8;
        name[(j * 4 + 1)] = ((reg[j] >> 8) & 0xFF) as u8;
        name[(j * 4 + 2)] = ((reg[j] >> 16) & 0xFF) as u8;
        name[(j * 4 + 3)] = ((reg[j] >> 24) & 0xFF) as u8;
    }

    print!(" [{}]", String::from_utf8(name).unwrap());
}
