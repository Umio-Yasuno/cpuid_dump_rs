//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

#![feature(asm)]

extern crate cpuid_asm;
use cpuid_asm::{_AX, cpuid_out, bitflag};
/*
#[cfg(target_os = "linux")]
extern crate libc;
*/
#[cfg(target_os = "linux")]
use libc::{cpu_set_t, CPU_SET, CPU_ZERO, sched_setaffinity};

#[cfg(target_os = "windows")]
use kernel32::{GetCurrentThread, SetThreadAffinityMask};

use std::{mem, thread};

macro_rules! print_cpuid {
    ($in_eax: expr, $in_ecx: expr, $out: expr) => {
        print!(" {:08X}h_x{:X}: eax={:08X}h ebx={:08X}h ecx={:08X}h edx={:08X}h",
            $in_eax, $in_ecx,
            $out.eax, $out.ebx, $out.ecx, $out.edx);
    }
}

/*
macro_rules! pad { () => {
    format!("{:70}", " ");
} }
*/

fn pad() -> String {
    return format!("{:70}", "");
}

fn print_feature(buff: Vec<String>) {
    let mut c: usize = 1;
    let len = buff.len();

    for v in buff {
        if 9 < v.len() {
            print!("{} [{}]{}",
                if (c % 3) != 1 {
                    format!("\n{}", pad())
                } else {
                    format!("")
                },

                v.trim_end_matches('/'),

                if (c % 3) != 0 && c != len {
                    format!("\n{}", pad())
                } else {
                    format!("") 
                },
            );
        } else {
            print!(" [{}]", v.trim_end_matches('/'));
        }

        if (c % 3) == 0 && c != len {
            print!("\n{}", pad());
        }

        c += 1;
    }
}

fn feature_00_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = vec![format!(""); 0];

    // 0x0000_0007_EDX_x0
    if bitflag!(edx,  0) { buff.push(format!("FPU"));  }
    if bitflag!(edx, 23) { buff.push(format!("MMX"));  }
    if bitflag!(edx, 24) { buff.push(format!("FXSR")); }
    if bitflag!(edx, 28) { buff.push(format!("HTT"));  }
    if bitflag!(edx, 25) {
        buff.push(format!("SSE{0}{1}{2}{3}",
            if bitflag!(edx, 26) { "/2" }   else { "" },
            // 0x0000_0007_ECX_x0
            if bitflag!(ecx,  0) { "/3" }   else { "" },
            if bitflag!(ecx, 19) { "/4.1" } else { "" },
            if bitflag!(ecx, 20) { "/4.2" } else { "" },
        ));
    }
    // 0x0000_0007_ECX_x0
    if bitflag!(ecx, 12) { buff.push(format!("FMA"));      }
    if bitflag!(ecx, 17) { buff.push(format!("PCID"));     }
    if bitflag!(ecx, 23) { buff.push(format!("POPCNT"));   }
    if bitflag!(ecx, 25) { buff.push(format!("AES"));      }
    if bitflag!(ecx, 26) { buff.push(format!("XSAVE"));    }
    if bitflag!(ecx, 27) { buff.push(format!("OSXSAVE"));  }
    if bitflag!(ecx, 28) { buff.push(format!("AVX"));      }
    if bitflag!(ecx, 29) { buff.push(format!("F16C"));     }
    if bitflag!(ecx, 30) { buff.push(format!("RDRAND"));   }

    print_feature(buff);
}

fn feature_00_07h() {
    for j in 0x0..=0x1 {
        let tmp = cpuid_out::get(0x7, j);
        print_cpuid!(0x7, j, tmp);

        let mut buff: Vec<String> = vec![format!(""); 0];

        match j {
            0 => {
                // 0x00000007_EBX_x0
                if bitflag!(tmp.ebx,  0) { buff.push(format!("FSGSBASE")); }
                if bitflag!(tmp.ebx,  2) { buff.push(format!("SGX"));      }
                if bitflag!(tmp.ebx,  3) {
                    buff.push(format!("BMI1{}",
                        if bitflag!(tmp.ebx, 8) { "/2" } else { "" },
                    ));
                }
                if bitflag!(tmp.ebx,  5) { buff.push(format!("AVX2"));         }
                if bitflag!(tmp.ebx,  7) { buff.push(format!("SMEP"));         }
                if bitflag!(tmp.ebx, 10) { buff.push(format!("INVPCID"));      }
                if bitflag!(tmp.ebx, 18) { buff.push(format!("RDSEED"));       }
                if bitflag!(tmp.ebx, 20) { buff.push(format!("SMAP"));         }
                if bitflag!(tmp.ebx, 23) { buff.push(format!("CLFLUSHOPT"));   }
                if bitflag!(tmp.ebx, 24) { buff.push(format!("CLWB"));         }
                if bitflag!(tmp.ebx, 29) { buff.push(format!("SHA"));          }

                if bitflag!(tmp.ebx, 16) || bitflag!(tmp.ebx, 17) || bitflag!(tmp.ebx, 21)
                || bitflag!(tmp.ebx, 28) || bitflag!(tmp.ebx, 30) || bitflag!(tmp.ebx, 31) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}{3}{4}{5}",
                            if bitflag!(tmp.ebx, 16) { "F/"    } else { "" },
                            if bitflag!(tmp.ebx, 17) { "DQ/"   } else { "" },
                            if bitflag!(tmp.ebx, 21) { "IFMA/" } else { "" },
                            if bitflag!(tmp.ebx, 28) { "CD/"   } else { "" },
                            if bitflag!(tmp.ebx, 30) { "BW/"   } else { "" },
                            if bitflag!(tmp.ebx, 31) { "VL/"   } else { "" },
                        )
                    );
                }
                /*  Xeon Phi only */
                if bitflag!(tmp.ebx, 26) && bitflag!(tmp.ebx, 27) {
                    buff.push(format!("AVX512PF/ER"));
                }

                // 0x00000007_ECX_x0
                if bitflag!(tmp.ecx,  1) || bitflag!(tmp.ecx,  6) || bitflag!(tmp.ecx, 11)
                || bitflag!(tmp.ecx, 12) || bitflag!(tmp.ecx, 14) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}{3}{4}",
                            if bitflag!(tmp.ecx,  1) { "VBMI/" }      else { "" },
                            if bitflag!(tmp.ecx,  6) { "VBMI2/" }     else { "" },
                            if bitflag!(tmp.ecx, 11) { "VNNI/" }      else { "" },
                            if bitflag!(tmp.ecx, 12) { "BITALG/" }    else { "" },
                            if bitflag!(tmp.ecx, 14) { "VPOPCNTDQ/" } else { "" },
                    ));
                }

                if bitflag!(tmp.ecx,  3) { buff.push(format!("PKU"));          }
                if bitflag!(tmp.ecx,  7) { buff.push(format!("CET_SS"));       }
                if bitflag!(tmp.ecx,  8) { buff.push(format!("GFNI"));         }
                if bitflag!(tmp.ecx,  9) { buff.push(format!("VAES"));         }
                if bitflag!(tmp.ecx, 10) { buff.push(format!("VPCLMULQDQ"));   }
                //  if bitflag!(tmp.ecx, 22) { buff.push(format!("RDPID"));        }
                if bitflag!(tmp.ecx, 23) { buff.push(format!("KL"));           }
                if bitflag!(tmp.ecx, 25) { buff.push(format!("CLDEMOTE"));     }
                if bitflag!(tmp.ecx, 27) {
                    buff.push(format!("MOVDIRI{}",
                        if bitflag!(tmp.ecx, 28) { "/64B" } else { "" },
                    ));
                }
                if bitflag!(tmp.ecx, 29) { buff.push(format!("ENQCMD"));       }

                // 0x00000007_EDX_x0
                if bitflag!(tmp.edx,  2) || bitflag!(tmp.edx,  3) || bitflag!(tmp.edx, 8)
                || bitflag!(tmp.edx, 23) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}",
                            /*  Xeon Phi only */
                            if bitflag!(tmp.edx,  2) && bitflag!(tmp.edx,  3) {
                                "4VNNIW/4FMAPS/" } else { "" },
                            if bitflag!(tmp.edx,  8) { "VP2INTERSECT/" } else { "" },
                            if bitflag!(tmp.edx, 23) { "FP16/" }         else { "" },
                    ));
                }
                if bitflag!(tmp.edx,  4) { buff.push(format!("FSRM"));         }
                if bitflag!(tmp.edx,  5) { buff.push(format!("UINTR"));        }
                //  if bitflag!(tmp.edx,  8) { buff.push(format!("AVX512_VP2INTERSECT"));  }
                if bitflag!(tmp.edx, 10) { buff.push(format!("MD_CLEAR"));     }
                if bitflag!(tmp.edx, 14) { buff.push(format!("SERIALIZE"));    }
                /*  Currently Sapphire Rapids only */
                if bitflag!(tmp.edx, 22) && bitflag!(tmp.edx, 24) && bitflag!(tmp.edx, 25) {
                    buff.push(format!("AMX-BF16/TILE/INT8"));
                }
                //  if bitflag!(tmp.edx, 23) { buff.push(format!("AVX512_FP16"));  }
                if bitflag!(tmp.edx, 26) { buff.push(format!("IBPB"));         }
                if bitflag!(tmp.edx, 27) { buff.push(format!("STIBP"));        }
                if bitflag!(tmp.edx, 28) { buff.push(format!("L1D_FLUSH"));    }
                if bitflag!(tmp.edx, 31) { buff.push(format!("SSBD"));         }
            },
            1 => {
                if bitflag!(tmp.eax,  4) { buff.push(format!("AVX_VNNI"));     }
                if bitflag!(tmp.eax,  5) { buff.push(format!("AVX512_BF16"));  }
                if bitflag!(tmp.eax, 22) { buff.push(format!("HRESET"));       }
                if bitflag!(tmp.eax, 26) { buff.push(format!("LAM"));          }
            },
            _ => {},
        }
        print_feature(buff);
        println!();
    }
}

fn feature_80_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = vec![format!(""); 0];

    // 0x8000_0001_EDX_x0
    if bitflag!(edx, 31) {
        buff.push(format!("3DNow!{}",
            if bitflag!(edx, 30) { "/Ext" } else { "" }
        ));
    }

    // 0x8000_0001_ECX_x0
    if bitflag!(ecx,  0) { buff.push(format!("LAHF/SAHF"));            }
    if bitflag!(ecx,  5) { buff.push(format!("LZCNT"));                }
    if bitflag!(ecx,  6) { buff.push(format!("SSE4A"));                }
    if bitflag!(ecx,  8) { buff.push(format!("3DNow!Prefetch"));       }
    if bitflag!(ecx, 16) { buff.push(format!("FMA4"));                 }

    print_feature(buff);
}

fn cpu_name(tmp: cpuid_out) {
    let reg = [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];
    let mut name: [u8; 16] = [0x20; 16];

    for j in 0..=3 as usize {
        name[(j*4)]    =  (reg[j] & 0xFF) as u8;
        name[(j*4+1)]  = ((reg[j] >> 8)  & 0xFF) as u8;
        name[(j*4+2)]  = ((reg[j] >> 16) & 0xFF) as u8;
        name[(j*4+3)]  = ((reg[j] >> 24) & 0xFF) as u8;
    }

    print!(" [{}]", String::from_utf8(name.to_vec()).unwrap());
}

fn cache_prop_intel_04h() {
    for j in 0x0..=0x4 {
        let tmp = cpuid_out::get(0x4, j);
    /* for debug
        match j {
            0 => {
                tmp.eax = 0x1C004121;
                tmp.ebx = 0x02C0003F;
                tmp.ecx = 0x0000003F;
                tmp.edx = 0;
            },
            1 => {
                tmp.eax = 0x1C004122;
                tmp.ebx = 0x01C0003F;
                tmp.ecx = 0x0000003F;
                tmp.edx = 0;
            },
            2 => {
                tmp.eax = 0x1C004143;
                tmp.ebx = 0x01C0003F;
                tmp.ecx = 0x000003FF;
                tmp.edx = 0;
            },
            3 => {
                tmp.eax = 0x1C03C163;
                tmp.ebx = 0x03C0003F;
                tmp.ecx = 0x00003FFF;
                tmp.edx = 0;
            },
            4 => {
                a = [0; 4];
            },
            _ => {},
        }
    */
        let cache_level = (tmp.eax >> 5) & 0b111;
        let cache_type =
            match tmp.eax & 0b11111 {
                1 => "D", // Data
                2 => "I", // Instruction
                3 => "U", // Unified
                0 | _ => "",
        };
        let cache_line = (tmp.ebx & 0xFFF) + 1;
        let cache_way  = (tmp.ebx >> 22) + 1;
        let cache_set  = tmp.ecx + 1;
        let cache_size = cache_line * cache_way * cache_set;
        let cache_size_str =
            if cache_size < 1000_000 {
                format!("{}K", cache_size / (1 << 10))
            } else if cache_size < 1000_000_000 {
                format!("{}M", cache_size / (1 << 20))
            } else {
                format!("{}B", cache_size)
            };

        if cache_level == 0 || cache_type == "" {
            return;
        }

        print_cpuid!(0x4, j, tmp);
        print!(" [L{}{} {}] ",
            cache_level, cache_type, cache_size_str);
        println!();
    }
}

fn enum_amd_0dh() {
    let in_ecx: Vec<u32> = vec![0x0, 0x1, 0x2, 0x9, 0xB, 0xC];

    for j in in_ecx {
        let tmp = cpuid_out::get(0xD, j);
        print_cpuid!(0xD, j, tmp);

        match j {
            0x0 => {
                let x87 = (tmp.eax & 0b1) == 1;
                let sse = ((tmp.eax >> 1) & 0b1) == 1;
                let avx = ((tmp.eax >> 2) & 0b1) == 1;
                let pku = ((tmp.eax >> 9) & 0b1) == 1;

                let mut buff = String::new();

                if x87 { buff.push_str("X87 ") }
                if sse { buff.push_str("SSE ") }
                if avx { buff.push_str("AVX ") }
                if pku { buff.push_str("PKU ") }

                print!(" [{}]", buff.trim_end());
            },
            0x2 => print!(" [XSTATE: size({})]", tmp.eax),
            _   => {},
        }
        println!();
    }
}

fn intel_hybrid_1ah(eax: u32) {
    let core_type = match eax >> 24 {
        0x20    => format!("Atom"),
        0x40    => format!("Core"),
        _       => format!(""),
    };

    if core_type.len() != 0 {
        print!(" [{}]", core_type);
    }
}

fn apmi_amd_80_07h(edx: u32) {
    let cpb  = ((edx >> 9) & 0b1) == 1;
    let rapl = ((edx >> 14) & 0b1) == 1;

    let mut buff = String::new();

    if cpb  { buff.push_str("CPB "); }
    if rapl { buff.push_str("RAPL "); }

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn spec_amd_80_08h(ebx: u32) {
    let ibpb    = ((ebx >> 12) & 1) == 1;
    let stibp   = ((ebx >> 15) & 1) == 1;
    let ssbd    = ((ebx >> 24) & 1) == 1;
    let psfd    = ((ebx >> 28) & 1) == 1;

    let mut buff = String::new();

    if ibpb  { buff.push_str("IBPB "); }
    if stibp { buff.push_str("STIBP "); }
    if ssbd  { buff.push_str("SSBD "); }
    if psfd  { buff.push_str("PSFD "); }

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn fpu_width_amd_80_1ah(eax: u32) {
    let fp256 = ((eax >> 3) & 0b1) == 1;
    let movu  = ((eax >> 1) & 0b1) == 1;
    let fp128 = (eax & 0b1) == 1;

    let mut buff = String::new();
            
    if fp256 {
        buff.push_str("FP256 ");
    } else if fp128 {
        buff.push_str("FP128 ");
    }
    if movu { buff.push_str("MOVU "); }

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn secure_amd_80_1fh(eax: u32) {
    let sme     =  (eax & 1) == 1;
    let sev     = ((eax >> 1) & 1) == 1;
    let sev_es  = ((eax >> 3) & 1) == 1;
    let snp     = ((eax >> 4) & 1) == 1;

    let mut buff = String::new();

    if sme { buff.push_str("SME "); }
    if sev { buff.push_str("SEV");
        if sev_es { buff.push_str("(-ES) "); }
        if snp    { buff.push_str("SNP "); }
    }

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn dump() {
    println!("CPUID Dump");

    let mut buff = String::new();
    for _i in 0..72 {
        buff.push_str("=");
    }
    println!("{}", buff);

    let vendor_check = cpuid_out::get(0, 0);

    let vendor_amd   = vendor_check.ebx == 0x6874_7541
                    && vendor_check.ecx == 0x444D_4163
                    && vendor_check.edx == 0x6974_6E65;

    let vendor_intel = vendor_check.ebx == 0x756E_6547
                    && vendor_check.ecx == 0x4965_6E69
                    && vendor_check.edx == 0x6C65_746E;

    for i in 0..=0x20 {
        if (0x2 <= i && i <= 0x4)
        || (0x8 <= i && i <= 0xA)
        || (0xC == i) || (0xE == i)
        || (0x11 <= i)
        && vendor_amd {
            continue;
        } else if i == 0x4 && vendor_intel {
            cache_prop_intel_04h();
            continue;
        } else if i == 0x7 {
            feature_00_07h();
            continue;
        } else if i == 0xB {
            for j in 0..=1 {
                let tmp = cpuid_out::get(i, j);
                print_cpuid!(i, j, tmp);
                println!();
            }
            continue;
        } else if i == 0xD && vendor_amd {
            enum_amd_0dh();
            continue;
        }

        let tmp = cpuid_out::get(i, 0);
        print_cpuid!(i, 0, tmp);

        if i == 0 {
            print!(" [{}]", cpuid_asm::get_vendor_name());
        } else if i == 0x1 {
            print!(" [F: {:X}h, M: {:X}h, S: {:X}]",
                ((tmp.eax >> 8) & 0xF) + ((tmp.eax >> 20) & 0xFF),
                ((tmp.eax >> 4) & 0xF) + ((tmp.eax >> 12) & 0xF0),
                tmp.eax & 0xF);
            print!("\n{} [APIC ID: {}]", pad(), tmp.ebx >> 24);
            print!("\n{} [Total {} thread]", pad(), (tmp.ebx >> 16) & 0xFF);
            print!("\n{} [CLFlush: {}B]", pad(), ((tmp.ebx >> 8) & 0xFF) * 8);
            print!("\n{}", pad());
            feature_00_01h(tmp.ecx, tmp.edx);
        } else if i == 0x16 && vendor_intel {
            print!(" [{}/{}/{} MHz]",
                tmp.eax & 0xFFFF, tmp.ebx & 0xFFFF, tmp.ecx & 0xFFFF);
        } else if i == 0x1A && vendor_intel {
            intel_hybrid_1ah(tmp.eax);
        }
        println!();
    }

    println!();

    for i in 0x0..=0x21 {
        if (0xB <= i && i <= 0x18) && vendor_amd {
            continue;
        }

        let tmp = cpuid_out::get(_AX + i, 0);
        print_cpuid!(_AX + i, 0, tmp);

        if i == 0x1 {
            if vendor_amd {
                print!(" [PkgType: {}]", tmp.ebx >> 28);
                print!("\n{}", pad());
            }
            feature_80_01h(tmp.ecx, tmp.edx);
        } else if 0x2 <= i && i <= 0x4 {
            cpu_name(tmp);
        } else if i == 0x5 && vendor_amd {
            print!(" [L1D {}K/L1I {}K]",
                tmp.ecx >> 24, (tmp.edx >> 24) & 0xFF);
            print!("\n{} [L1TLB: {} entry]",
                pad(), tmp.ebx & 0xFF);

        } else if i == 0x6 && vendor_amd {
            print!(" [L2 {}K/L3 {}M]",
                (tmp.ecx >> 16), (tmp.edx >> 18) / 2);

            print!("\n{} [L2dTLB: 4K {}, 2M {}",
                pad(), ((tmp.ebx >> 16) & 0xFFF), ((tmp.eax >> 16) & 0xFFF));
            print!("\n{}{:9} 4M {:4}]",
                pad(), " ", ((tmp.eax >> 16) & 0xFFF) / 2);

            print!("\n{} [L2iTLB: 4K {}, 2M {}",
                pad(), tmp.ebx & 0xFFF, tmp.eax & 0xFFF);
            print!("\n{}{:9} 4M {:4}]",
                pad(), "", (tmp.eax & 0xFFF) / 2);

        } else if i == 0x7 && vendor_amd {
            apmi_amd_80_07h(tmp.edx);
        } else if i == 0x8 && vendor_amd {
            spec_amd_80_08h(tmp.ebx);
        } else if i == 0x19 && vendor_amd {
            print!(" [L2TLB 1G: D {}, I {}]",
                (tmp.ebx >> 16) & 0xFFF, tmp.ebx & 0xFFF);
        } else if i == 0x1A && vendor_amd {
            fpu_width_amd_80_1ah(tmp.eax);
        } else if i == 0x1e && vendor_amd {
            print!(" [Core ID: {}]", tmp.ebx & 0xFF);
            print!("\n{} [{} thread per core]",
                pad(), ((tmp.ebx >> 8) & 0xFF) + 1);
            print!("\n{} [Node ID: {}]",
                pad(), tmp.ecx & 0xFF);
        } else if i == 0x1f && vendor_amd {
            secure_amd_80_1fh(tmp.eax);
        }
        println!();
    }
    println!();
}

fn dump_all() {
    let core_count = cpuid_asm::CpuCoreCount::get();

    for i in 0..(core_count.total_thread) as usize {
        thread::spawn( move || {
            #[cfg(target_os = "linux")]
            unsafe {
                let mut set = mem::zeroed::<cpu_set_t>();
                CPU_ZERO(&mut set);
                CPU_SET(i, &mut set);

                sched_setaffinity(0,
                                  mem::size_of::<cpu_set_t>(),
                                  &set);
            }
            #[cfg(target_os = "windows")]
            unsafe {
                SetThreadAffinityMask(GetCurrentThread(), 1 << i);
            }

            let id = cpuid_asm::CpuCoreCount::get();
            println!("Core ID: {:<3} / Thread: {:<3}",
                id.core_id, i);

            dump();

        }).join().unwrap();
    }
}

fn main() {
    println!();

    for opt in std::env::args() {
        if opt == "-a" || opt == "--all" {
            dump_all();
            return;
        }
    }

    dump();
}
