//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{__cpuid_count, CpuidResult};

extern crate cpuid_asm;
use cpuid_asm::{cpuid, bitflag};

use std::io::Write;

#[path = "./parse_amd.rs"]
    mod parse_amd;  pub use parse_amd::*;
#[path = "./parse_intel.rs"]
    mod parse_intel;  pub use parse_intel::*;

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
    ($ftr_bool: expr, $name_str: expr) => {
        if $ftr_bool { $name_str } else { "" }
    };
    ($ftr_bool: expr, $name_str: expr, $else_ftr: expr, $else_name: expr) => {
        if $ftr_bool { $name_str } else if $else_ftr { $else_name } else { "" }
    };
}

macro_rules! push {
    ($buff: expr, $str: expr) => {
        $buff.push($str.to_string())
    };
}

#[macro_export]
macro_rules! pad {
    () => { format!("{:62}", "") };
}

#[macro_export]
macro_rules! padln {
    () => { format!("\n{}", pad!()); };
}

fn print_feature(buff: Vec<String>) {

    macro_rules! if_else {
        ($expr:expr, $if:expr, $else:expr) => {
            if $expr {$if} else {$else}
        };
    }

    let out = std::io::stdout();
    let mut out = out.lock();

    let mut c: usize = 1;
    let len = buff.len();

    for v in buff {
        if 9 < v.len() {
            write!(out, "{0} [{1}]{2}",
                // {0}
                if_else!((c%3) != 1, padln!(), format!("")),

                // {1}
                v,

                // {2}
                if_else!((c % 3) != 0 && c != len, padln!(), format!(""))
            ).unwrap();
        } else {
            write!(out, " [{}]", v).unwrap();
        }

        if (c % 3) == 0 && c != len {
            write!(out, "{}", padln!()).unwrap();
        }

        c += 1;
    }
}

pub fn info_00_01h(eax: u32, ebx: u32) {
    let x86_fam  = ((eax >> 8) & 0xF) + ((eax >> 20) & 0xFF);
    let x86_mod  = ((eax >> 4) & 0xF) + ((eax >> 12) & 0xF0);
    let x86_step = eax & 0xF;

    print!(" [F: 0x{:X}, M: 0x{:X}, S: 0x{:X}]",
        x86_fam, x86_mod, x86_step);
    let codename = cpuid_asm::get_codename(x86_fam, x86_mod, x86_step).codename;
    print!("\n{} [{}]", pad!(), codename);
            
    print!("\n{} [APIC ID: {}]", pad!(), ebx >> 24);
    print!("\n{} [Total {} thread]", pad!(), (ebx >> 16) & 0xFF);
    print!("\n{} [CLFlush: {}B]", pad!(), ((ebx >> 8) & 0xFF) * 8);
    //  print!("\n{}", pad!());
    print!("{}", padln!());
}

pub fn feature_00_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = Vec::with_capacity(16);

    // 0x0000_0007_EDX_x0
    if bitflag!(edx,  0) { push!(buff, "FPU");  }
    if bitflag!(edx, 23) { push!(buff, "MMX");  }
    if bitflag!(edx, 24) { push!(buff, "FXSR"); }
    if bitflag!(edx, 28) { push!(buff, "HTT");  }
    if bitflag!(edx, 25) {
        buff.push(
            format!("SSE{0}{1}{2}{3}",
                has_ftr!(bitflag!(edx, 26), "/2"),
                // 0x0000_0007_ECX_x0
                has_ftr!(bitflag!(ecx,  0), "/3"),
                has_ftr!(bitflag!(ecx, 19), "/4.1"),
                has_ftr!(bitflag!(ecx, 20), "/4.2"),
            )
        );
    }
    // 0x0000_0007_ECX_x0
    if bitflag!(ecx, 12) { push!(buff, "FMA");      }
    if bitflag!(ecx, 17) { push!(buff, "PCID");     }
    if bitflag!(ecx, 23) { push!(buff, "POPCNT");   }
    if bitflag!(ecx, 25) { push!(buff, "AES");      }
    if bitflag!(ecx, 26) { push!(buff, "XSAVE");    }
    if bitflag!(ecx, 27) { push!(buff, "OSXSAVE");  }
    if bitflag!(ecx, 28) { push!(buff, "AVX");      }
    if bitflag!(ecx, 29) { push!(buff, "F16C");     }
    if bitflag!(ecx, 30) { push!(buff, "RDRAND");   }

    print_feature(buff);
}

pub fn feature_00_07h() {
    for j in 0x0..=0x1 {
        let tmp = cpuid!(0x7, j);
        print_cpuid!(0x7, j, tmp);

        let mut buff: Vec<String> = Vec::with_capacity(48);

        match j {
            0 => {
                // 0x00000007_EBX_x0
                if bitflag!(tmp.ebx,  0) { push!(buff, "FSGSBASE"); }
                if bitflag!(tmp.ebx,  2) { push!(buff, "SGX");      }
                if bitflag!(tmp.ebx,  3) {
                    buff.push(format!("BMI1{}",
                        has_ftr!(bitflag!(tmp.ebx, 8), "/2"),
                    ));
                }
                if bitflag!(tmp.ebx,  5) { push!(buff, "AVX2");         }
                if bitflag!(tmp.ebx,  7) { push!(buff, "SMEP");         }
                if bitflag!(tmp.ebx, 10) { push!(buff, "INVPCID");      }
                if bitflag!(tmp.ebx, 18) { push!(buff, "RDSEED");       }
                if bitflag!(tmp.ebx, 20) { push!(buff, "SMAP");         }
                if bitflag!(tmp.ebx, 23) { push!(buff, "CLFLUSHOPT");   }
                if bitflag!(tmp.ebx, 24) { push!(buff, "CLWB");         }
                if bitflag!(tmp.ebx, 29) { push!(buff, "SHA");          }

                if bitflag!(tmp.ebx, 16) || bitflag!(tmp.ebx, 17) || bitflag!(tmp.ebx, 21)
                || bitflag!(tmp.ebx, 28) || bitflag!(tmp.ebx, 30) || bitflag!(tmp.ebx, 31) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}{3}{4}{5}",
                            has_ftr!(bitflag!(tmp.ebx, 16), "F/"),
                            has_ftr!(bitflag!(tmp.ebx, 17), "DQ/"),
                            has_ftr!(bitflag!(tmp.ebx, 21), "IFMA/"),
                            has_ftr!(bitflag!(tmp.ebx, 28), "CD/"),
                            has_ftr!(bitflag!(tmp.ebx, 30), "BW/"),
                            has_ftr!(bitflag!(tmp.ebx, 31), "VL/")
                        ).trim_end_matches("/").to_string()
                    );
                }

                /*  Xeon Phi only */
                if bitflag!(tmp.ebx, 26) && bitflag!(tmp.ebx, 27) {
                    push!(buff, "AVX512PF/ER");
                }

                // 0x00000007_ECX_x0
                if bitflag!(tmp.ecx,  1) || bitflag!(tmp.ecx,  6) || bitflag!(tmp.ecx, 11)
                || bitflag!(tmp.ecx, 12) || bitflag!(tmp.ecx, 14) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}{3}{4}",
                            has_ftr!(bitflag!(tmp.ecx,  1), "VBMI/"),
                            has_ftr!(bitflag!(tmp.ecx,  6), "VBMI2/"),
                            has_ftr!(bitflag!(tmp.ecx, 11), "VNNI/"),
                            has_ftr!(bitflag!(tmp.ecx, 12), "BITALG/"),
                            has_ftr!(bitflag!(tmp.ecx, 14), "VPOPCNTDQ/"),
                        ).trim_end_matches("/").to_string()
                    );
                }

                if bitflag!(tmp.ecx,  3) { push!(buff, "PKU");          }
                if bitflag!(tmp.ecx,  7) { push!(buff, "CET_SS");       }
                if bitflag!(tmp.ecx,  8) { push!(buff, "GFNI");         }
                if bitflag!(tmp.ecx,  9) { push!(buff, "VAES");         }
                if bitflag!(tmp.ecx, 10) { push!(buff, "VPCLMULQDQ");   }
                //  if bitflag!(tmp.ecx, 22) { push!(buff, "RDPID");        }
                if bitflag!(tmp.ecx, 23) { push!(buff, "KL");           }
                if bitflag!(tmp.ecx, 25) { push!(buff, "CLDEMOTE");     }
                if bitflag!(tmp.ecx, 27) {
                    buff.push(format!("MOVDIRI{}",
                        has_ftr!(bitflag!(tmp.ecx, 28), "/64B"),
                    ));
                }
                if bitflag!(tmp.ecx, 29) { push!(buff, "ENQCMD");   }

                // 0x00000007_EDX_x0
                if bitflag!(tmp.edx,  2) || bitflag!(tmp.edx,  3) || bitflag!(tmp.edx, 8)
                || bitflag!(tmp.edx, 23) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}",
                            /*  Xeon Phi only */
                            has_ftr!( bitflag!(tmp.edx, 2) && bitflag!(tmp.edx, 3) ,
                                "4VNNIW/4FMAPS/"),
                            has_ftr!(bitflag!(tmp.edx,  8), "VP2INTERSECT/"),
                            has_ftr!(bitflag!(tmp.edx, 23), "FP16/"),
                        ).trim_end_matches("/").to_string()
                    );
                }
                if bitflag!(tmp.edx,  4) { push!(buff, "FSRM");         }
                if bitflag!(tmp.edx,  5) { push!(buff, "UINTR");        }
                //  if bitflag!(tmp.edx,  8) { push!(buff, "AVX512_VP2INTERSECT");  }
                if bitflag!(tmp.edx, 10) { push!(buff, "MD_CLEAR");     }
                if bitflag!(tmp.edx, 14) { push!(buff, "SERIALIZE");    }
                /*  Currently Intel Sapphire Rapids only */
                if bitflag!(tmp.edx, 22) && bitflag!(tmp.edx, 24) && bitflag!(tmp.edx, 25) {
                    push!(buff, format!("AMX-BF16/TILE/INT8"));
                }
                //  if bitflag!(tmp.edx, 23) { push!(buff, format!("AVX512_FP16"));  }
                if bitflag!(tmp.edx, 26) { push!(buff, "IBPB");         }
                if bitflag!(tmp.edx, 27) { push!(buff, "STIBP");        }
                if bitflag!(tmp.edx, 28) { push!(buff, "L1D_FLUSH");    }
                if bitflag!(tmp.edx, 31) { push!(buff, "SSBD");         }
            },
            1 => {
                if bitflag!(tmp.eax,  4) { push!(buff, "AVX_VNNI");     }
                if bitflag!(tmp.eax,  5) { push!(buff, "AVX512_BF16");  }
                if bitflag!(tmp.eax, 22) { push!(buff, "HRESET");       }
                if bitflag!(tmp.eax, 26) { push!(buff, "LAM");          }
            },
            _ => unreachable!(),
        }
        print_feature(buff);
        println!();
    }
}

pub fn feature_80_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = Vec::with_capacity(8);

    // 0x8000_0001_EDX_x0
    if bitflag!(edx, 31) {
        buff.push(format!("3DNow!{}",
            has_ftr!(bitflag!(edx, 30), "/EXT"),
        ));
    }

    // 0x8000_0001_ECX_x0
    if bitflag!(ecx,  0) { push!(buff, "LAHF/SAHF");        }
    if bitflag!(ecx,  5) { push!(buff, "LZCNT");            }
    if bitflag!(ecx,  6) { push!(buff, "SSE4A");            }
    if bitflag!(ecx,  8) { push!(buff, "3DNow!Prefetch");   }
    if bitflag!(ecx, 16) { push!(buff, "FMA4");             }

    print_feature(buff);
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
        print!("\n{} [shared {}T]", pad!(), cache_share_thread);

        let cache_inclusive = (tmp.edx >> 1) & 0b1;
        if cache_inclusive == 1 {
            print!("\n{} [inclusive]", pad!());
        }
        println!();
    }    
    
}

pub fn cpu_name(tmp: CpuidResult) {
    let reg = [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];
    let mut name = vec![0x20u8; 16];

    for j in 0..=3 as usize {
        name[(j*4)]    =  (reg[j] & 0xFF) as u8;
        name[(j*4+1)]  = ((reg[j] >> 8)  & 0xFF) as u8;
        name[(j*4+2)]  = ((reg[j] >> 16) & 0xFF) as u8;
        name[(j*4+3)]  = ((reg[j] >> 24) & 0xFF) as u8;
    }

    print!(" [{}]", String::from_utf8(name).unwrap());
}

