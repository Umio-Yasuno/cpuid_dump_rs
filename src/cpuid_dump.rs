use super::{_AX, cpuid};
use crate::feature_detect::CpuFeature;

macro_rules! print_cpuid {
    ($in_eax: expr, $in_ecx: expr,
    $out_eax: expr, $out_ebx: expr, $out_ecx: expr, $out_edx: expr) => {
        print!(" {:08X}h_x{:X}:  eax={:08X}h ebx={:08X}h ecx={:08X}h edx={:08X}h",
            $in_eax, $in_ecx,
            $out_eax, $out_ebx, $out_ecx, $out_edx);
    }
}

fn cpuid_feature_07h() {
    let mut a: [u32; 4] = [0; 4];

    for j in 0x0..=0x1 {
        cpuid!(a[0], a[1], a[2], a[3], 0x7, j);
        print_cpuid!(0x7, j, a[0], a[1], a[2], a[3]);

        if j == 0 {
            let abi = CpuFeature::get();

            let mut buff = String::new();

            if abi.x86_64_v1 { buff.push_str("v1"); }
            if abi.x86_64_v2 { buff.push_str("/2"); }
            if abi.x86_64_v3 { buff.push_str("/3"); }
            if abi.x86_64_v4 { buff.push_str("/4"); }

            print!(" [x86-64-{}]", buff);
        }
        println!();
    }
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
    let ecx = [0x0, 0x1, 0x2, 0x9, 0xB, 0xC];

    for j in &ecx {
        cpuid!(a[0], a[1], a[2], a[3], 0xD, *j as u32);
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
            0x2 => print!(" [XSTATE: size {}]", a[0]),
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

    if core_type != "" {
        print!(" [{}]", core_type);
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

    if buff != "" {
        print!(" [{}]", buff.trim_end());
    }
}

fn fpu_width_amd_80_1ah(a: [u32; 4]) {
    let fp256 = ((a[0] >> 3) & 0b1) == 1;
    let fp128 = (a[0] & 0b1) == 1;

    let mut buff = String::new();
            
    if fp256 {
        buff.push_str("FP256");
    } else if fp128 {
        buff.push_str("FP128");
    }

    if buff != "" {
        print!(" [{}]", buff);
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

    if buff != "" {
        print!(" [{}]", buff.trim_end());
    }
}

pub fn dump() {
    println!("CPUID Dump");

    let mut buff = String::new();
    for _i in 0..72 {
        buff.push_str("=");
    }
    println!("{}", buff);

    let mut a: [u32; 4] = [0; 4];

    cpuid!(a[0], a[1], a[2], a[3], 0, 0);

    let vendor_amd =    a[1] == 0x6874_7541 
                        && a[2] == 0x444D_4163
                        && a[3] == 0x6974_6E65;
    let vendor_intel =  a[1] == 0x756E_6547
                        && a[2] == 0x4965_6E69
                        && a[3] == 0x6C65_746E;

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
            cpuid_feature_07h();
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
            print!(" [{}]", super::get_vendor_name());
        } else if i == 0x1 {
            print!(" [F: {}, M: {}, S: {}]",
                ((a[0] >> 8) & 0xF) + ((a[0] >> 20) & 0xFF),
                ((a[0] >> 4) & 0xF) + ((a[0] >> 12) & 0xF0),
                a[0] & 0xF);
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

        if 0x2 <= i && i <= 0x4 {
            cpu_name(a);
        } else if i == 0x5 && vendor_amd {
            print!(" [L1D {}K/L1I {}K]",
                a[2] >> 24, (a[3] >> 24) & 0xFF);
            print!("\n{:71} [L1TLB: {} entry]",
                " ", a[1] & 0xFF);

        } else if i == 0x6 && vendor_amd {
            print!(" [L2 {}K/L3 {}M]",
                (a[2] >> 16), (a[3] >> 18) / 2);

            print!("\n{:71} [L2dTLB: 4K {}, 2M {}",
                " ", ((a[1] >> 16) & 0xFFF), ((a[0] >> 16) & 0xFFF));
            print!("\n{:80} 4M {:4}]",
                " ", ((a[0] >> 16) & 0xFFF) / 2);

            print!("\n{:71} [L2iTLB: 4K {}, 2M {}",
                " ", a[1] & 0xFFF, a[0] & 0xFFF);
            print!("\n{:80} 4M {:4}]",
                " ", (a[0] & 0xFFF) / 2);

        } else if i == 0x7 && vendor_amd {
            if ((a[0] >> 9) & 1) == 1 {
                print!(" [CPB]");
            }
        } else if i == 0x8 && vendor_amd {
            spec_amd_80_08h(a);
        } else if i == 0x19 && vendor_amd {
            print!(" [L2TLB 1G: D {}, I {}]",
                (a[1] >> 16) & 0xFFF, a[1] & 0xFFF);
        } else if i == 0x1A && vendor_amd {
            fpu_width_amd_80_1ah(a);
        } else if i == 0x1e && vendor_amd {
            print!(" [{} thread per core]",
                ((a[1] >> 8) & 0xFF) + 1);
        } else if i == 0x1f && vendor_amd {
            secure_amd_80_1fh(a);
        }
        println!();
    }
    println!();
}
