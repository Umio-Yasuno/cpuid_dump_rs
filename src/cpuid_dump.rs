static _AX: u32 = 0x8000_0000;

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
            $in_eax, $in_ecx, $out_eax, $out_ebx, $out_ecx, $out_edx);
    }
}

fn cpuid_feature_07h() {
    let mut a: [u32; 4] = [0; 4];
    for j in 0x0..=0x1 {
        unsafe {
            asm!("cpuid",
                inlateout("eax") 0x7 => a[0],
                lateout("ebx") a[1],
                inlateout("ecx") j => a[2],
                lateout("edx") a[3]
            );
        }
        print_cpuid!(0x7, j, a[0], a[1], a[2], a[3]);
        println!();
    }
}

fn cache_prop_amd() {
    let mut a: [u32; 4] = [0; 4];
    for j in 0x0..=0x4 {
        unsafe {
            asm!("cpuid",
                inlateout("eax") _AX + 0x1d => a[0],
                lateout("ebx") a[1],
                inlateout("ecx") j => a[2],
                lateout("edx") a[3]
            );
        }

        let cache_level = (a[0] >> 5) & 0b111;
        let cache_type =
            match a[0] & 0b11111 {
                1 => "D",
                2 => "I",
                3 => "U",
                _ => "",
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

        print_cpuid!(_AX + 0x1d, j, a[0], a[1], a[2], a[3]);
        print!(" [L{}{} {}] ",
            cache_level, cache_type, cache_size_str);
        println!();
    }
}

pub fn dump() {
    println!("CPUID Dump");
    line();

    let mut a: [u32; 4] = [0; 4];

    unsafe {
        asm!("cpuid",
            in("eax") 0,
            lateout("ebx") a[1],
            lateout("ecx") a[2],
            lateout("edx") a[3],
        );
    }

    let vendor_amd =    a[1] == 0x6874_7541 
                        && a[2] == 0x444D_4163
                        && a[3] == 0x6974_6E65;
    let vendor_intel =  a[1] == 0x756E_6547
                        && a[2] == 0x4965_6E69
                        && a[3] == 0x6C65_746E;

    for i in 0x0..=0x10 {

        if i == 0x7 {
            cpuid_feature_07h();
            continue;
        }

        unsafe {
            asm!("cpuid",
                inlateout("eax") i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }
        print_cpuid!(i, 0, a[0], a[1], a[2], a[3]);

        if i == 0x1 {
            print!(" [F: {}, M: {}, S: {}]",
                ((a[0] >> 8) & 0xf) + ((a[0] >> 20) & 0xff),
                ((a[0] >> 4) & 0xf) + ((a[0] >> 12) & 0xf0),
                a[0] & 0xf);
        }

        println!();
    }

    println!();

    for i in 0x0..=0x20 {

        if i == 0x1d && vendor_amd {
            cache_prop_amd();
            continue;
        }

        unsafe {
            asm!("cpuid",
                inlateout("eax") _AX + i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }
        print_cpuid!(_AX + i, 0, a[0], a[1], a[2], a[3]);
        println!();
    }
    println!();
}
