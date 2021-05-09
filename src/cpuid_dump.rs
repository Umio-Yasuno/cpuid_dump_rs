use super::_AX;
use super::cpuid;

fn line() {
    for _i in 0..75 {
        print!("=");
    }
    println!();
}

macro_rules! print_cpuid {
    ($in_eax: expr, $in_ecx: expr,
    $out_eax: expr, $out_ebx: expr, $out_ecx: expr, $out_edx: expr) => {
        print!(" {:08X}h_x{}:  eax={:08X}h ebx={:08X}h ecx={:08X}h edx={:08X}h",
            $in_eax, $in_ecx,
            $out_eax, $out_ebx, $out_ecx, $out_edx);
    }
}

fn cpuid_feature_07h() {
    let mut a: [u32; 4] = [0; 4];
    for j in 0x0..=0x1 {
        cpuid!(a[0], a[1], a[2], a[3], 0x7, j);
        print_cpuid!(0x7, j, a[0], a[1], a[2], a[3]);
        println!();
    }
}

fn cpu_name(a: [u32; 4]) {
    let mut name: Vec<u8> = vec![0x20; 16];

    for j in 0..=3 {
        name[(j*4) as usize]   =  (a[j as usize] & 0xff) as u8;
        name[(j*4+1) as usize] = ((a[j as usize] >> 8)  & 0xff) as u8;
        name[(j*4+2) as usize] = ((a[j as usize] >> 16) & 0xff) as u8;
        name[(j*4+3) as usize] = ((a[j as usize] >> 24) & 0xff) as u8;
    }

    print!(" [{}]", String::from_utf8(name).unwrap());
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
                a[0] = 0;
                a[1] = 0;
                a[2] = 0;
                a[3] = 0;
            },
            _ => {},
        }
    */
        let cache_level = (a[0] >> 5) & 0b111;
        let cache_type =
            match (a[0] & 0b11111) {
                1 => "D", // Data
                2 => "I", // Instruction
                3 => "U", // Unified
                0 | _ => "",
        };
        let cache_line = (a[1] & 0xfff) + 1;
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

pub fn dump() {
    println!("CPUID Dump");
    line();

    let mut a: [u32; 4] = [0; 4];

    cpuid!(a[0], a[1], a[2], a[3], 0, 0);

    let vendor_amd =    a[1] == 0x6874_7541 
                        && a[2] == 0x444D_4163
                        && a[3] == 0x6974_6E65;
    let vendor_intel =  a[1] == 0x756E_6547
                        && a[2] == 0x4965_6E69
                        && a[3] == 0x6C65_746E;

    for i in 0..=0x10 {

        if i == 0x4 /* && vendor_intel */ {
            cache_prop_intel_04h();
            continue;
        } else if i == 0x7 {
            cpuid_feature_07h();
            continue;
        }

        cpuid!(a[0], a[1], a[2], a[3], i, 0);
        print_cpuid!(i, 0, a[0], a[1], a[2], a[3]);

        if i == 0 {
            print!(" [{}]", super::get_vendor_name());
        } else if i == 0x1 {
            print!(" [F: {}, M: {}, S: {}]",
                ((a[0] >> 8) & 0xf) + ((a[0] >> 20) & 0xff),
                ((a[0] >> 4) & 0xf) + ((a[0] >> 12) & 0xf0),
                a[0] & 0xf);
        }

        println!();
    }

    println!();

    for i in 0x0..=0x20 {

        cpuid!(a[0], a[1], a[2], a[3], _AX + i, 0);
        print_cpuid!(_AX + i, 0, a[0], a[1], a[2], a[3]);

        if 0x2 <= i && i <= 0x4 {
            cpu_name(a);
        } else if i == 0x5 && vendor_amd {
            print!(" [L1D {}K/L1I {}K]", (a[2] >> 24) & 0xff, (a[3] >> 24) & 0xff);
        } else if i == 0x6 && vendor_amd {
            print!(" [L2 {}K/L3 {}M]", (a[2] >> 16), (a[3] >> 18) / 2);
        } else if i == 0x1e && vendor_amd {
            print!(" [{} thread per core]", ((a[1] >> 8) & 0xff) + 1);
        }

        println!();
    }
    println!();
}
