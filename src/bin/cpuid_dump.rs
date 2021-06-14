//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{__cpuid_count, CpuidResult};

extern crate cpuid_asm;
use cpuid_asm::{_AX, cpuid, bitflag, Vendor};
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
        print!(" {:08X}h_x{:X}:  {:08X}h {:08X}h {:08X}h {:08X}h ",
            $in_eax, $in_ecx,
            $out.eax, $out.ebx, $out.ecx, $out.edx);
    };
}

macro_rules! has_ftr {
    ($ftr_bool: expr, $name_str: expr) => {
        if $ftr_bool { $name_str } else { "" }
    };
    ($ftr_bool: expr, $name_str: expr, $else_ftr: expr, $else_name: expr) => {
        if $ftr_bool { $name_str } else if $else_ftr { $else_name } else { "" }
    };
}
/*
macro_rules! pad { () => {
    format!("{:70}", " ");
} }
*/

fn pad() -> String {
    format!("{:56}", "")
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
    let mut buff: Vec<String> = vec![String::new(); 0];

    // 0x0000_0007_EDX_x0
    if bitflag!(edx,  0) { buff.push(format!("FPU"));  }
    if bitflag!(edx, 23) { buff.push(format!("MMX"));  }
    if bitflag!(edx, 24) { buff.push(format!("FXSR")); }
    if bitflag!(edx, 28) { buff.push(format!("HTT"));  }
    if bitflag!(edx, 25) {
        buff.push(format!("SSE{0}{1}{2}{3}",
            has_ftr!(bitflag!(edx, 26), "/2"),
            // 0x0000_0007_ECX_x0
            has_ftr!(bitflag!(ecx,  0), "/3"),
            has_ftr!(bitflag!(ecx, 19), "/4.1"),
            has_ftr!(bitflag!(ecx, 20), "/4.2"),
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
        let tmp = cpuid!(0x7, j);
        print_cpuid!(0x7, j, tmp);

        let mut buff: Vec<String> = vec![String::new(); 0];

        match j {
            0 => {
                // 0x00000007_EBX_x0
                if bitflag!(tmp.ebx,  0) { buff.push(format!("FSGSBASE")); }
                if bitflag!(tmp.ebx,  2) { buff.push(format!("SGX"));      }
                if bitflag!(tmp.ebx,  3) {
                    buff.push(format!("BMI1{}",
                        has_ftr!(bitflag!(tmp.ebx, 8), "/2"),
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
                            has_ftr!(bitflag!(tmp.ebx, 16), "F/"),
                            has_ftr!(bitflag!(tmp.ebx, 17), "DQ/"),
                            has_ftr!(bitflag!(tmp.ebx, 21), "IFMA/"),
                            has_ftr!(bitflag!(tmp.ebx, 28), "CD/"),
                            has_ftr!(bitflag!(tmp.ebx, 30), "BW/"),
                            has_ftr!(bitflag!(tmp.ebx, 31), "VL/"),
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
                            has_ftr!(bitflag!(tmp.ecx,  1), "VBMI/"),
                            has_ftr!(bitflag!(tmp.ecx,  6), "VBMI2/"),
                            has_ftr!(bitflag!(tmp.ecx, 11), "VNNI/"),
                            has_ftr!(bitflag!(tmp.ecx, 12), "BITALG/"),
                            has_ftr!(bitflag!(tmp.ecx, 14), "VPOPCNTDQ/"),
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
                        has_ftr!(bitflag!(tmp.ecx, 28), "/64B"),
                    ));
                }
                if bitflag!(tmp.ecx, 29) { buff.push(format!("ENQCMD"));       }

                // 0x00000007_EDX_x0
                if bitflag!(tmp.edx,  2) || bitflag!(tmp.edx,  3) || bitflag!(tmp.edx, 8)
                || bitflag!(tmp.edx, 23) {
                    buff.push(
                        format!("AVX512_{0}{1}{2}",
                            /*  Xeon Phi only */
                            has_ftr!((bitflag!(tmp.edx, 2) && bitflag!(tmp.edx, 3)),
                                "4VNNIW/4FMAPS/"),
                            has_ftr!(bitflag!(tmp.edx,  8), "VP2INTERSECT/"),
                            has_ftr!(bitflag!(tmp.edx, 23), "FP16/"),
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
            _ => unreachable!(),
        }
        print_feature(buff);
        println!();
    }
}

fn feature_80_01h(ecx: u32, edx: u32) {
    let mut buff: Vec<String> = vec![String::new(); 0];

    // 0x8000_0001_EDX_x0
    if bitflag!(edx, 31) {
        buff.push(format!("3DNow!{}",
            has_ftr!(bitflag!(edx, 30), "/EXT"),
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

fn cpu_name(tmp: CpuidResult) {
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

fn cache_prop(in_eax: u32) {
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
        print!(" [L{} {:>7}: {:>2}-way, {:>4}]",
            cache_level, cache_type, cache_way, cache_size_unit);
        print!("\n{} [shared {}T]", pad(), cache_share_thread);

        let cache_inclusive = (tmp.edx >> 1) & 0b1;
        if cache_inclusive == 1 {
            print!("\n{} [inclusive]", pad());
        }
        println!();
    }    
    
}

fn enum_amd_0dh() {
    let in_ecx: [u32; 6] = [0x0, 0x1, 0x2, 0x9, 0xB, 0xC];

    for ecx in in_ecx {
        let tmp = cpuid!(0xD, ecx);
        print_cpuid!(0xD, ecx, tmp);

        match ecx {
            0x0 => {
                let x87 = bitflag!(tmp.eax, 0);
                let sse = bitflag!(tmp.eax, 1);
                let avx = bitflag!(tmp.eax, 2);
                let pku = bitflag!(tmp.eax, 9);

                let buff = format!("{0}{1}{2}{3}",
                    has_ftr!(x87, "X87 "),
                    has_ftr!(sse, "SSE "),
                    has_ftr!(avx, "AVX "),
                    has_ftr!(pku, "PKU "),
                );

                print!(" [{}]", buff.trim_end());
            },
            0x2 => if tmp.eax != 0 {
                print!(" [XSTATE: size({})]",   tmp.eax);
            },
            0x9 => if tmp.eax != 0 {
                print!(" [MPK: size({})]",      tmp.eax);
            },
            0xB => if tmp.eax != 0 {
                print!(" [CET_U: size({})]",    tmp.eax);
            },
            0xC => if tmp.eax != 0 {
                print!(" [CET_S: size({})]",    tmp.eax);
            },
            _   => {},
        }
        println!();
    }
}

fn intel_hybrid_1ah(eax: u32) {
    let core_type = format!("{}",
        match eax >> 24 {
            0x20 => "Atom",
            0x40 => "Core",
            _    => "",
        }
    );

    if core_type.len() != 0 {
        print!(" [{}]", core_type);
    }
}

fn apmi_amd_80_07h(edx: u32) {
    let cpb  = bitflag!(edx, 9);
    let rapl = bitflag!(edx, 14);

    let buff = format!("{0}{1}",
        has_ftr!(cpb,  "CPB "),
        has_ftr!(rapl, "RAPL "),
    );

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn spec_amd_80_08h(ebx: u32) {
    let ibpb    = bitflag!(ebx, 12);
    let stibp   = bitflag!(ebx, 15);
    let ssbd    = bitflag!(ebx, 24);
    let psfd    = bitflag!(ebx, 28);

    let buff = format!("{0}{1}{2}{3}",
        has_ftr!(ibpb,  "IBPB "),
        has_ftr!(stibp, "STIBP "),
        has_ftr!(ssbd,  "SSBD "),
        has_ftr!(psfd,  "PSFD "),
    );

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn fpu_width_amd_80_1ah(eax: u32) {
    let fp256 = bitflag!(eax, 3);
    let movu  = bitflag!(eax, 1);
    let fp128 = bitflag!(eax, 0);

    let buff = format!("{0}{1}",
        has_ftr!(fp256, "FP256 ", fp128, "FP128 "),
        has_ftr!(movu,  "MOVU "),
    );

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn secure_amd_80_1fh(eax: u32) {
    let sme     =  (eax & 1) == 1;
    let sev     = ((eax >> 1) & 1) == 1;
    let sev_es  = ((eax >> 3) & 1) == 1;
    let snp     = ((eax >> 4) & 1) == 1;

    let buff = format!("{0}{1}{2}{3}",
        has_ftr!(sme, "SME "),
        has_ftr!(sev, "SEV"),
        has_ftr!(sev && sev_es, "(-ES) "),
        has_ftr!(sev && snp,    "SNP "),
    );

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

fn dump() {
    println!("CPUID Dump");
    println!(" (in)EAX_xECX:  {:<9} {:<9} {:<9} {:<9}",
        "(out)EAX", "(out)EBX", "(out)ECX", "(out)EDX");
    
    let mut buff = String::new();
    for _i in 0..80 {
        buff.push_str("=");
    }
    println!("{}", buff);

    let ck = cpuid!(0, 0);
    let vendor = Vendor {
                    ebx: ck.ebx,
                    ecx: ck.ecx,
                    edx: ck.edx,
                };

    let vendor_amd   = (vendor == Vendor::amd());
    let vendor_intel = (vendor == Vendor::intel());

    for i in 0..=0x20 {
        if (0x2 <= i && i <= 0x4)
        || (0x8 <= i && i <= 0xA)
        || (0xC == i) || (0xE == i)
        || (0x11 <= i)
        && vendor_amd {
            continue;
        } else if i == 0x4 && vendor_intel {
            cache_prop(0x4);
            continue;
        } else if i == 0x7 {
            feature_00_07h();
            continue;
        } else if i == 0xB {
            for j in 0..=1 {
                let tmp = cpuid!(i, j);
                print_cpuid!(i, j, tmp);
                println!();
            }
            continue;
        } else if i == 0xD && vendor_amd {
            enum_amd_0dh();
            continue;
        }

        let tmp = cpuid!(i, 0);
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
        } else if i == 0x1D && vendor_amd {
            cache_prop(_AX + 0x1D);
            continue;
        }

        let tmp = cpuid!(_AX + i, 0);
        print_cpuid!(_AX + i, 0, tmp);

        if i == 0x1 {
            if vendor_amd {
                let pkg_type = tmp.ebx >> 28;
                let pkg_dec = match pkg_type {
                    0x2 => "AM4",
                    _   => "Unknown",
                };
                print!(" [PkgType: {}({:#X})]", pkg_dec, pkg_type);
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
        } else if i == 0x1E && vendor_amd {
            print!(" [Core ID: {}]", tmp.ebx & 0xFF);
            print!("\n{} [{} thread per core]",
                pad(), ((tmp.ebx >> 8) & 0xFF) + 1);
            print!("\n{} [Node ID: {}]",
                pad(), tmp.ecx & 0xFF);
        } else if i == 0x1F && vendor_amd {
            secure_amd_80_1fh(tmp.eax);
        }
        println!();
    }
    println!();
}

fn dump_all() {
    let thread_count = cpuid_asm::CpuCoreCount::get().total_thread;

    for i in 0..(thread_count) as usize {
        thread::spawn(move || {
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

            let id = cpuid_asm::CpuCoreCount::get().core_id;
            println!("Core ID: {:<3} / Thread: {:<3}", id, i);

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
