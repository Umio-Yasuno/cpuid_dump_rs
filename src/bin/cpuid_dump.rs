//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

#![feature(asm)]

extern crate cpuid_asm;
use cpuid_asm::{_AX, cpuid};
use cpuid_asm::{feature_detect::CpuFeature, bitflag};
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
    ($in_eax: expr, $in_ecx: expr,
    $out_eax: expr, $out_ebx: expr, $out_ecx: expr, $out_edx: expr) => {
        print!(" {:08X}h_x{:X}: eax={:08X}h ebx={:08X}h ecx={:08X}h edx={:08X}h",
            $in_eax, $in_ecx,
            $out_eax, $out_ebx, $out_ecx, $out_edx);
    }
}

macro_rules! pad {
    () => {
        format!("{:70}", " ");
    }
}

fn print_feature(buff: Vec<String>) {
    let mut c: usize = 1;
    let len = buff.len();

    for v in buff {
        if 9 < v.len() {
            print!("{} [{}]{}",
                if (c % 3) != 1 {
                    format!("\n{}", pad!())
                } else {
                    format!("")
                },
                v.trim_end_matches('/'),
                if (c % 3) != 0 && c != len {
                    format!("\n{}", pad!())
                } else {
                    format!("") 
                },
            );
        } else {
            print!(" [{}]", v.trim_end_matches('/'));
        }

        if (c % 3) == 0 && c != len {
            print!("\n{}", pad!());
        }

        c += 1;
    }
}

fn feature_00_01h(a: [u32; 4]) {
    let mut buff: Vec<String> = vec![format!(""); 0];

    // 0x0000_0007_EDX_x0
    if bitflag!(a[3],  0) { buff.push(format!("FPU"));  }
    if bitflag!(a[3], 23) { buff.push(format!("MMX"));  }
    if bitflag!(a[3], 24) { buff.push(format!("FXSR")); }
    if bitflag!(a[3], 28) { buff.push(format!("HTT"));  }
    if bitflag!(a[3], 25) {
        buff.push(format!("SSE{0}{1}{2}{3}",
            if bitflag!(a[3], 26) { "/2" }   else { "" },
            // 0x0000_0007_ECX_x0
            if bitflag!(a[2],  0) { "/3" }   else { "" },
            if bitflag!(a[2], 19) { "/4.1" } else { "" },
            if bitflag!(a[2], 20) { "/4.2" } else { "" },
        ));
    }
    // 0x0000_0007_ECX_x0
    if bitflag!(a[2], 12) { buff.push(format!("FMA"));      }
    if bitflag!(a[2], 17) { buff.push(format!("PCID"));     }
    if bitflag!(a[2], 23) { buff.push(format!("POPCNT"));   }
    if bitflag!(a[2], 25) { buff.push(format!("AES"));      }
    if bitflag!(a[2], 26) { buff.push(format!("XSAVE"));    }
    if bitflag!(a[2], 27) { buff.push(format!("OSXSAVE"));  }
    if bitflag!(a[2], 28) { buff.push(format!("AVX"));      }
    if bitflag!(a[2], 29) { buff.push(format!("F16C"));     }
    if bitflag!(a[2], 30) { buff.push(format!("RDRAND"));   }

    print_feature(buff);
}

fn feature_00_07h() {
    let mut a: [u32; 4] = [0; 4];

    for j in 0x0..=0x1 {
        cpuid!(a[0], a[1], a[2], a[3], 0x7, j);
        print_cpuid!(0x7, j, a[0], a[1], a[2], a[3]);

        let mut buff: Vec<String> = vec![format!(""); 0];

        match j {
            0 => {
                // 0x00000007_EBX_x0
                if bitflag!(a[1],  0) { buff.push(format!("FSGSBASE")); }
                if bitflag!(a[1],  2) { buff.push(format!("SGX"));      }
                if bitflag!(a[1],  3) {
                    buff.push(format!("BMI1{}",
                        if bitflag!(a[1], 8) { "/2" } else { "" },
                    ));
                }
                if bitflag!(a[1],  5) { buff.push(format!("AVX2"));         }
                if bitflag!(a[1],  7) { buff.push(format!("SMEP"));         }
                if bitflag!(a[1], 10) { buff.push(format!("INVPCID"));      }
                if bitflag!(a[1], 18) { buff.push(format!("RDSEED"));       }
                if bitflag!(a[1], 20) { buff.push(format!("SMAP"));         }
                if bitflag!(a[1], 23) { buff.push(format!("CLFLUSHOPT"));   }
                if bitflag!(a[1], 24) { buff.push(format!("CLWB"));         }
                if bitflag!(a[1], 29) { buff.push(format!("SHA"));          }

                if bitflag!(a[1], 16) || bitflag!(a[1], 17) || bitflag!(a[1], 21)
                || bitflag!(a[1], 28) || bitflag!(a[1], 30) || bitflag!(a[1], 31) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}{3}{4}{5}",
                            if bitflag!(a[1], 16) { "F/"    } else { "" },
                            if bitflag!(a[1], 17) { "DQ/"   } else { "" },
                            if bitflag!(a[1], 21) { "IFMA/" } else { "" },
                            if bitflag!(a[1], 28) { "CD/"   } else { "" },
                            if bitflag!(a[1], 30) { "BW/"   } else { "" },
                            if bitflag!(a[1], 31) { "VL/"   } else { "" },
                        )
                    );
                }
                /*  Xeon Phi only */
                if bitflag!(a[1], 26) && bitflag!(a[1], 27) {
                    buff.push(format!("AVX512PF/ER"));
                }

                // 0x00000007_ECX_x0
                if bitflag!(a[2],  1) || bitflag!(a[2],  6) || bitflag!(a[2], 11)
                || bitflag!(a[2], 12) || bitflag!(a[2], 14) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}{3}{4}",
                            if bitflag!(a[2],  1) { "VBMI/" }      else { "" },
                            if bitflag!(a[2],  6) { "VBMI2/" }     else { "" },
                            if bitflag!(a[2], 11) { "VNNI/" }      else { "" },
                            if bitflag!(a[2], 12) { "BITALG/" }    else { "" },
                            if bitflag!(a[2], 14) { "VPOPCNTDQ/" } else { "" },
                    ));
                }

                if bitflag!(a[2],  3) { buff.push(format!("PKU"));          }
                if bitflag!(a[2],  7) { buff.push(format!("CET_SS"));       }
                if bitflag!(a[2],  8) { buff.push(format!("GFNI"));         }
                if bitflag!(a[2],  9) { buff.push(format!("VAES"));         }
                if bitflag!(a[2], 10) { buff.push(format!("VPCLMULQDQ"));   }
                //  if bitflag!(a[2], 22) { buff.push(format!("RDPID"));        }
                if bitflag!(a[2], 23) { buff.push(format!("KL"));           }
                if bitflag!(a[2], 25) { buff.push(format!("CLDEMOTE"));     }
                if bitflag!(a[2], 27) {
                    buff.push(format!("MOVDIRI{}",
                        if bitflag!(a[2], 28) { "/64B" } else { "" },
                    ));
                }
                if bitflag!(a[2], 29) { buff.push(format!("ENQCMD"));       }

                // 0x00000007_EDX_x0
                if bitflag!(a[3],  2) || bitflag!(a[3],  3) || bitflag!(a[3], 8)
                || bitflag!(a[3], 23) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}",
                            /*  Xeon Phi only */
                            if bitflag!(a[3],  2) && bitflag!(a[3],  3) {
                                "4VNNIW/4FMAPS/" } else { "" },
                            if bitflag!(a[3],  8) { "VP2INTERSECT/" } else { "" },
                            if bitflag!(a[3], 23) { "FP16/" }         else { "" },
                    ));
                }
                if bitflag!(a[3],  4) { buff.push(format!("FSRM"));         }
                if bitflag!(a[3],  5) { buff.push(format!("UINTR"));        }
                //  if bitflag!(a[3],  8) { buff.push(format!("AVX512_VP2INTERSECT"));  }
                if bitflag!(a[3], 10) { buff.push(format!("MD_CLEAR"));     }
                if bitflag!(a[3], 14) { buff.push(format!("SERIALIZE"));    }
                /*  Currently Sapphire Rapids only */
                if bitflag!(a[3], 22) && bitflag!(a[3], 24) && bitflag!(a[3], 25) {
                    buff.push(format!("AMX-BF16/TILE/INT8"));
                }
                //  if bitflag!(a[3], 23) { buff.push(format!("AVX512_FP16"));  }
                if bitflag!(a[3], 26) { buff.push(format!("IBPB"));         }
                if bitflag!(a[3], 27) { buff.push(format!("STIBP"));        }
                if bitflag!(a[3], 28) { buff.push(format!("L1D_FLUSH"));    }
                if bitflag!(a[3], 31) { buff.push(format!("SSBD"));         }
            },
            1 => {
                if bitflag!(a[0],  4) { buff.push(format!("AVX_VNNI"));     }
                if bitflag!(a[0],  5) { buff.push(format!("AVX512_BF16"));  }
                if bitflag!(a[0], 22) { buff.push(format!("HRESET"));       }
                if bitflag!(a[0], 26) { buff.push(format!("LAM"));          }
            },
            _ => {},
        }
        print_feature(buff);
        println!();
    }
}

fn feature_80_01h(a: [u32; 4]) {
    let mut buff: Vec<String> = vec![format!(""); 0];

    // 0x8000_0001_EDX_x0
    if bitflag!(a[3], 31) {
        buff.push(format!("3DNow!{}",
            if bitflag!(a[3], 30) { "/Ext" } else { "" }
        ));
    }

    // 0x8000_0001_ECX_x0
    if bitflag!(a[2],  0) { buff.push(format!("LAHF/SAHF"));            }
    if bitflag!(a[2],  5) { buff.push(format!("LZCNT"));                }
    if bitflag!(a[2],  6) { buff.push(format!("SSE4A"));                }
    if bitflag!(a[2],  8) { buff.push(format!("3DNow!Prefetch"));       }
    if bitflag!(a[2], 16) { buff.push(format!("FMA4"));                 }

    print_feature(buff);
}

fn cpu_name(a: [u32; 4]) {
    let mut name: [u8; 16] = [0x20; 16];

    for j in 0..=3 as usize {
        name[(j*4)]    =  (a[j] & 0xFF) as u8;
        name[(j*4+1)]  = ((a[j] >> 8)  & 0xFF) as u8;
        name[(j*4+2)]  = ((a[j] >> 16) & 0xFF) as u8;
        name[(j*4+3)]  = ((a[j] >> 24) & 0xFF) as u8;
    }

    print!(" [{}]", String::from_utf8(name.to_vec()).unwrap());
}

fn cache_prop_intel_04h() {
    let mut a: [u32; 4] = [0; 4];

    for j in 0x0..=0x4 {
        cpuid!(a[0], a[1], a[2], a[3], 0x4, j);
    /* for debug
        match j {
            0 => {
                a[0] = 0x1C004121;
                a[1] = 0x02C0003F;
                a[2] = 0x0000003F;
                a[3] = 0;
            },
            1 => {
                a[0] = 0x1C004122;
                a[1] = 0x01C0003F;
                a[2] = 0x0000003F;
                a[3] = 0;
            },
            2 => {
                a[0] = 0x1C004143;
                a[1] = 0x01C0003F;
                a[2] = 0x000003FF;
                a[3] = 0;
            },
            3 => {
                a[0] = 0x1C03C163;
                a[1] = 0x03C0003F;
                a[2] = 0x00003FFF;
                a[3] = 0;
            },
            4 => {
                a = [0; 4];
            },
            _ => {},
        }
    */
        let cache_level = (a[0] >> 5) & 0b111;
        let cache_type =
            match a[0] & 0b1 {
                1 => "D", // Data
                2 => "I", // Instruction
                3 => "U", // Unified
                0 | _ => "",
        };
        let cache_line = (a[1] & 0xFFF) + 1;
        let cache_way  = (a[1] >> 22) + 1;
        let cache_set  = a[2] + 1;
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

        print_cpuid!(0x4, j, a[0], a[1], a[2], a[3]);
        print!(" [L{}{} {}] ",
            cache_level, cache_type, cache_size_str);
        println!();
    }
}

fn enum_amd_0dh() {
    let mut a: [u32; 4] = [0; 4];
    let ecx: Vec<u32> = vec![0x0, 0x1, 0x2, 0x9, 0xB, 0xC];

    for j in ecx {
        cpuid!(a[0], a[1], a[2], a[3], 0xD, j);
        print_cpuid!(0xD, j, a[0], a[1], a[2], a[3]);

        match j {
            0x0 => {
                let x87 = (a[0] & 0b1) == 1;
                let sse = ((a[0] >> 1) & 0b1) == 1;
                let avx = ((a[0] >> 2) & 0b1) == 1;
                let pku = ((a[0] >> 9) & 0b1) == 1;

                let mut buff = String::new();

                if x87 { buff.push_str("X87 ") }
                if sse { buff.push_str("SSE ") }
                if avx { buff.push_str("AVX ") }
                if pku { buff.push_str("PKU ") }

                print!(" [{}]", buff.trim_end());
            },
            0x2 => print!(" [XSTATE: size({})]", a[0]),
            _   => {},
        }
        println!();
    }
}

fn intel_hybrid_1ah(a: [u32; 4]) {
    let core_type = match a[0] >> 24 {
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

fn spec_amd_80_08h(a: [u32; 4]) {
    let ibpb    = ((a[1] >> 12) & 1) == 1;
    let stibp   = ((a[1] >> 15) & 1) == 1;
    let ssbd    = ((a[1] >> 24) & 1) == 1;
    let psfd    = ((a[1] >> 28) & 1) == 1;

    let mut buff = String::new();

    if ibpb  { buff.push_str("IBPB "); }
    if stibp { buff.push_str("STIBP "); }
    if ssbd  { buff.push_str("SSBD "); }
    if psfd  { buff.push_str("PSFD "); }

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn fpu_width_amd_80_1ah(a: [u32; 4]) {
    let fp256 = ((a[0] >> 3) & 0b1) == 1;
    let movu  = ((a[0] >> 1) & 0b1) == 1;
    let fp128 = (a[0] & 0b1) == 1;

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

fn secure_amd_80_1fh(a: [u32; 4]) {
    let sme     =  (a[0] & 1) == 1;
    let sev     = ((a[0] >> 1) & 1) == 1;
    let sev_es  = ((a[0] >> 3) & 1) == 1;
    let snp     = ((a[0] >> 4) & 1) == 1;

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

    let mut a: [u32; 4] = [0; 4];

    cpuid!(a[0], a[1], a[2], a[3], 0, 0);

    let vendor_amd   = a[1] == 0x6874_7541 && a[2] == 0x444D_4163 && a[3] == 0x6974_6E65;
    let vendor_intel = a[1] == 0x756E_6547 && a[2] == 0x4965_6E69 && a[3] == 0x6C65_746E;

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
            for j in 0..=3 {
                cpuid!(a[0], a[1], a[2], a[3], i, j);
                print_cpuid!(i, j, a[0], a[1], a[2], a[3]);
                println!();
            }
            continue;
        } else if i == 0xD && vendor_amd {
            enum_amd_0dh();
            continue;
        }

        cpuid!(a[0], a[1], a[2], a[3], i, 0);
        print_cpuid!(i, 0, a[0], a[1], a[2], a[3]);

        if i == 0 {
            print!(" [{}]", cpuid_asm::get_vendor_name());
        } else if i == 0x1 {
            print!(" [F: {:X}h, M: {:X}h, S: {}]",
                ((a[0] >> 8) & 0xF) + ((a[0] >> 20) & 0xFF),
                ((a[0] >> 4) & 0xF) + ((a[0] >> 12) & 0xF0),
                a[0] & 0xF);
            print!("\n{} [APIC ID: {}]", pad!(), a[1] >> 24);
            print!("\n{} [Total {} thread]", pad!(), (a[1] >> 16) & 0xFF);
            print!("\n{} [CLFlush: {}B]", pad!(), ((a[1] >> 8) & 0xFF) * 8);
            print!("\n{}", pad!());
            feature_00_01h(a);
        } else if i == 0x16 && vendor_intel {
            print!(" [{}/{}/{} MHz]",
                a[0] & 0xFFFF, a[1] & 0xFFFF, a[2] & 0xFFFF);
        } else if i == 0x1A && vendor_intel {
            intel_hybrid_1ah(a);
        }
        println!();
    }

    println!();

    for i in 0x0..=0x21 {
        if (0xB <= i && i <= 0x18) && vendor_amd {
            continue;
        }

        cpuid!(a[0], a[1], a[2], a[3], _AX + i, 0);
        print_cpuid!(_AX + i, 0, a[0], a[1], a[2], a[3]);

        if i == 0x1 {
            if vendor_amd {
                print!(" [PkgType: {}]", a[1] >> 28);
                print!("\n{}", pad!());
            }
            feature_80_01h(a);
        } else if 0x2 <= i && i <= 0x4 {
            cpu_name(a);
        } else if i == 0x5 && vendor_amd {
            print!(" [L1D {}K/L1I {}K]",
                a[2] >> 24, (a[3] >> 24) & 0xFF);
            print!("\n{} [L1TLB: {} entry]",
                pad!(), a[1] & 0xFF);

        } else if i == 0x6 && vendor_amd {
            print!(" [L2 {}K/L3 {}M]",
                (a[2] >> 16), (a[3] >> 18) / 2);

            print!("\n{} [L2dTLB: 4K {}, 2M {}",
                pad!(), ((a[1] >> 16) & 0xFFF), ((a[0] >> 16) & 0xFFF));
            print!("\n{}{:9} 4M {:4}]",
                pad!(), " ", ((a[0] >> 16) & 0xFFF) / 2);

            print!("\n{} [L2iTLB: 4K {}, 2M {}",
                pad!(), a[1] & 0xFFF, a[0] & 0xFFF);
            print!("\n{}{:9} 4M {:4}]",
                pad!(), "", (a[0] & 0xFFF) / 2);

        } else if i == 0x7 && vendor_amd {
            apmi_amd_80_07h(a[3]);
        } else if i == 0x8 && vendor_amd {
            spec_amd_80_08h(a);
        } else if i == 0x19 && vendor_amd {
            print!(" [L2TLB 1G: D {}, I {}]",
                (a[1] >> 16) & 0xFFF, a[1] & 0xFFF);
        } else if i == 0x1A && vendor_amd {
            fpu_width_amd_80_1ah(a);
        } else if i == 0x1e && vendor_amd {
            print!(" [Core ID: {}]", a[1] & 0xFF);
            print!("\n{} [{} thread per core]",
                pad!(), ((a[1] >> 8) & 0xFF) + 1);
        } else if i == 0x1f && vendor_amd {
            secure_amd_80_1fh(a);
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
