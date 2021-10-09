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
    print!("{} [L1dTLB: 4K {:>3}, 2M/4M {:>3}]",
        padln!(), (tmp.ebx >> 16) & 0xFF, (tmp.eax >> 16) & 0xFF);
    print!("{} [L1iTLB: 4K {:>3}, 2M/4M {:>3}]",
        padln!(), tmp.ebx & 0xFF, tmp.eax & 0xFF);
}

pub fn l2_amd_80_06h(tmp: CpuidResult) {
    print!(" [L2 {}K/L3 {}M]",
        (tmp.ecx >> 16), (tmp.edx >> 18) / 2);

    print!("{} [L2dTLB: 4K {}, 2M {}",
        padln!(), ((tmp.ebx >> 16) & 0xFFF), ((tmp.eax >> 16) & 0xFFF));
    print!("{}{:9} 4M {:4}]",
        padln!(), " ", ((tmp.eax >> 16) & 0xFFF) / 2);

    print!("{} [L2iTLB: 4K {}, 2M {}",
        padln!(), tmp.ebx & 0xFFF, tmp.eax & 0xFFF);
    print!("{}{:9} 4M {:4}]",
        padln!(), "", (tmp.eax & 0xFFF) / 2);
}

pub fn l1l2tlb_1g_amd_80_19h(eax: u32, ebx: u32) {
    print!(" [L1TLB 1G: Data {:>3}, Inst {:>3}]",
        (eax >> 16) & 0xFFF, eax & 0xFFF);
    print!("{} [L2TLB 1G: Data {:>3}, Inst {:>3}]",
        padln!(), (ebx >> 16) & 0xFFF, ebx & 0xFFF);
}

pub fn cpu_topo_amd_80_1eh(ebx: u32, ecx: u32) {
    print!(" [Core ID: {}]", ebx & 0xFF);
    print!("{} [{} thread(s) per core]",
        padln!(), ((ebx >> 8) & 0xFF) + 1);
    print!("{} [Node ID: {}]",
        padln!(), ecx & 0xFF);
}

pub fn enum_amd_0dh() {
    let x0 = |eax: u32| -> _ {
        let x87 = flag!(eax, 1 << 0);
        let sse = flag!(eax, 1 << 1);
        let avx = flag!(eax, 1 << 2);
        let pku = flag!(eax, 1 << 9);

        let buff = format!("{0}{1}{2}{3}",
            has_ftr!(x87, "X87 "),
            has_ftr!(sse, "SSE "),
            has_ftr!(avx, "AVX "),
            has_ftr!(pku, "PKU "),
        );
                
        if buff.len() != 0 {
            print!(" [{}]", buff.trim_end());
        }
    };
    let x1 = |eax: u32| -> _ {
        let xsaves   = flag!(eax, 1 << 3);
        let xgetbv   = flag!(eax, 1 << 2);
        let xsavec   = flag!(eax, 1 << 1);
        let xsaveopt = flag!(eax, 1);

        let buff = format!("{0}{1}{2}{3}",
            has_ftr!(xsaves, "XSAVES "),
            has_ftr!(xgetbv, "XGETBV "),
            has_ftr!(xsavec, "XSAVEC "),
            has_ftr!(xsaveopt, "XSAVEOPT"),
        );

        if buff.len() != 0 {
            print!(" [{}]", buff.trim_end());
        }
    };

    macro_rules! size { ($eax: expr, $str :expr) =>
        { if $eax != 0 { print!(" [{0}: size({1})]", $str, $eax) } }
    }

    for ecx in [0x0, 0x1, 0x2, 0x9, 0xB, 0xC] {
        let tmp = cpuid!(0xD, ecx);
        print_cpuid!(0xD, ecx, tmp);
        let eax = tmp.eax;

        match ecx {
            0x0 => { x0(eax) },
            0x1 => { x1(eax) },
            0x2 => { size!(eax, "XSTATE")   },
            0x9 => { size!(eax, "MPK")      },
            0xB => { size!(eax, "CET_U")    },
            0xC => { size!(eax, "CET_S")    },
            _   => {},
        }
        println!();
    }
}

pub fn apmi_amd_80_07h(edx: u32) {
    let cpb  = flag!(edx, 1 << 9);
    let rapl = flag!(edx, 1<< 14);

    let buff = format!("{0}{1}",
        has_ftr!(cpb,  "CPB "),
        has_ftr!(rapl, "RAPL "),
    );

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

pub fn spec_amd_80_08h(ebx: u32) {
    let ibpb  = flag!(ebx, 1 << 12);
    let stibp = flag!(ebx, 1 << 15);
    let ssbd  = flag!(ebx, 1 << 24);
    let psfd  = flag!(ebx, 1 << 28);

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
    let fp256 = flag!(eax, 1 << 2);
    let movu  = flag!(eax, 1 << 1);
    let fp128 = flag!(eax, 1 << 0);

    let buff = format!("{0}{1}",
        has_ftr!(fp256, "FP256 ", fp128, "FP128 "),
        has_ftr!(movu,  "MOVU "),
    );

    if buff.len() != 0 {
        print!(" [{}]", buff.trim_end());
    }
}

pub fn secure_amd_80_1fh(eax: u32) {
    let sme    = flag!(eax, 1 << 0);
    let sev    = flag!(eax, 1 << 1);
    let sev_es = flag!(eax, 1 << 3);
    let snp    = flag!(eax, 1 << 4);

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
