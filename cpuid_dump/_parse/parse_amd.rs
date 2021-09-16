use crate::*;

pub fn pkgtype_amd_80_01h(ebx: u32) {
    let pkg_type = ebx >> 28;
    let pkg_dec = match pkg_type {
        0x0 => "FP5/FP6",
        0x2 => "AM4",
        _   => "Unknown",
    };
    print!(" [PkgType: {}({:#X})]", pkg_dec, pkg_type);
    print!("{}", padln!());
}

pub fn l1_amd_80_05h(tmp: CpuidResult) {
    print!(" [L1D {}K/L1I {}K]",
        tmp.ecx >> 24, (tmp.edx >> 24) & 0xFF);
    print!("\n{} [L1dTLB: 4K {:>3}, 2M/4M {:>3}]",
        pad!(), (tmp.ebx >> 16) & 0xFF, (tmp.eax >> 16) & 0xFF);
    print!("\n{} [L1iTLB: 4K {:>3}, 2M/4M {:>3}]",
        pad!(), tmp.ebx & 0xFF, tmp.eax & 0xFF);
}

pub fn l2_amd_80_06h(tmp: CpuidResult) {
    print!(" [L2 {}K/L3 {}M]",
        (tmp.ecx >> 16), (tmp.edx >> 18) / 2);

    print!("\n{} [L2dTLB: 4K {}, 2M {}",
        pad!(), ((tmp.ebx >> 16) & 0xFFF), ((tmp.eax >> 16) & 0xFFF));
    print!("\n{}{:9} 4M {:4}]",
        pad!(), " ", ((tmp.eax >> 16) & 0xFFF) / 2);

    print!("\n{} [L2iTLB: 4K {}, 2M {}",
        pad!(), tmp.ebx & 0xFFF, tmp.eax & 0xFFF);
    print!("\n{}{:9} 4M {:4}]",
        pad!(), "", (tmp.eax & 0xFFF) / 2);
}

pub fn l1l2tlb_1g_amd_80_19h(eax: u32, ebx: u32) {
    print!(" [L1TLB 1G: Data {:>3}, Inst {:>3}]",
        (eax >> 16) & 0xFFF, eax & 0xFFF);
    print!("\n{} [L2TLB 1G: Data {:>3}, Inst {:>3}]",
        pad!(), (ebx >> 16) & 0xFFF, ebx & 0xFFF);
}

pub fn cpu_topo_amd_80_1eh(ebx: u32, ecx: u32) {
    print!(" [Core ID: {}]", ebx & 0xFF);
    print!("\n{} [{} thread(s) per core]",
        pad!(), ((ebx >> 8) & 0xFF) + 1);
    print!("\n{} [Node ID: {}]",
        pad!(), ecx & 0xFF);
}

pub fn enum_amd_0dh() {
    for ecx in [0x0, 0x1, 0x2, 0x9, 0xB, 0xC] {
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

pub fn apmi_amd_80_07h(edx: u32) {
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

pub fn spec_amd_80_08h(ebx: u32) {
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

pub fn fpu_width_amd_80_1ah(eax: u32) {
    let fp256 = bitflag!(eax, 2);
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

pub fn secure_amd_80_1fh(eax: u32) {
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
