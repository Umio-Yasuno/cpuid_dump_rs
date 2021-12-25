use crate::*;

pub fn pkgtype_amd_80_01h(ebx: u32) {
    let pkg_type = ebx >> 28;
    let pkg_dec = match pkg_type {
        0x0 => "FP5/FP6",
        0x2 => "AM4",
        _   => "Unknown",
    };
    print!(" [PkgType: {}({:#X})]{}",
        pkg_dec, pkg_type, padln!());
}

pub fn l1_amd_80_05h(tmp: CpuidResult) {
    print!(" [L1D {}K/L1I {}K]",
        tmp.ecx >> 24,
        (tmp.edx >> 24) & 0xFF
    );
    print!("{} [L1dTLB: 4K {:>3}, 2M/4M {:>3}]",
        padln!(),
        (tmp.ebx >> 16) & 0xFF,
        (tmp.eax >> 16) & 0xFF
    );
    print!("{} [L1iTLB: 4K {:>3}, 2M/4M {:>3}]",
        padln!(),
        tmp.ebx & 0xFF,
        tmp.eax & 0xFF
    );
}

pub fn l2_amd_80_06h(tmp: CpuidResult) {
    print!(" [L2 {}K/L3 {}M]",
        (tmp.ecx >> 16),
        (tmp.edx >> 18) / 2
    );
    print!("{} [L2dTLB: 4K {}, 2M {}",
        padln!(),
        ((tmp.ebx >> 16) & 0xFFF),
        ((tmp.eax >> 16) & 0xFFF)
    );
    print!("{}{:9} 4M {:4}]",
        padln!(),
        " ",
        ((tmp.eax >> 16) & 0xFFF) / 2
    );

    print!("{} [L2iTLB: 4K {}, 2M {}",
        padln!(),
        tmp.ebx & 0xFFF,
        tmp.eax & 0xFFF
    );
    print!("{}{:9} 4M {:4}]",
        padln!(), "",
        (tmp.eax & 0xFFF) / 2
    );
}

pub fn l1l2tlb_1g_amd_80_19h(eax: u32, ebx: u32) {
    print!(" [L1TLB 1G: Data {:>3}, Inst {:>3}]",
        (eax >> 16) & 0xFFF,
        eax & 0xFFF
    );
    print!("{} [L2TLB 1G: Data {:>3}, Inst {:>3}]",
        padln!(),
        (ebx >> 16) & 0xFFF,
        ebx & 0xFFF
    );
}

pub fn cpu_topo_amd_80_1eh(ebx: u32, ecx: u32) {
    print!(" [Core ID: {}]", ebx & 0xFF);
    print!("{} [{} thread(s) per core]",
        padln!(),
        ((ebx >> 8) & 0xFF) + 1
    );
    print!("{} [Node ID: {}]",
        padln!(),
        ecx & 0xFF
    );
}

pub fn enum_amd_0dh() {
    let x0 = |eax: u32| -> _ {
        let ftr = vec![
            "X87", "SSE", "AVX", "",
            "", "", "", "",
            "", "MPK",
            /* Reserved Bit10-31 */
        ];

        let ftr = to_vstring(ftr);
        let buff = detect_ftr(eax, ftr);

        print_feature(buff);
    };
    let x1 = |eax: u32| -> _ {
        let ftr = vec![
            "XSAVEOPT", "XSAVEC", "XGETBV", "XSAVES",
            /* "Reserved Bit4-31" */
        ];
        let ftr = to_vstring(ftr);
        let buff = detect_ftr(eax, ftr);

        print_feature(buff);
    };

    macro_rules! size {
        ($eax: expr, $str :expr) => {
            if $eax != 0 {
                print!(" [{}: size({})]", $str, $eax)
            }
        };
    }

    for ecx in [0x0, 0x1, 0x2, 0x9, 0xB, 0xC] {
        let tmp = cpuid!(0xD, ecx);
        print_cpuid!(0xD, ecx, tmp);

        let eax = tmp.eax;

        match ecx {
            0x0 => x0(eax),
            0x1 => x1(eax),
            0x2 => { size!(eax, "XSTATE") }
            0x9 => { size!(eax, "MPK")    }
            0xB => { size!(eax, "CET_U")  }
            0xC => { size!(eax, "CET_S")  }
            _ => {}
        }
        println!();
    }
}

pub fn apmi_amd_80_07h(edx: u32) {
    let ftr = vec![
        "TS", "", "", "TTP",
        "TM", "", "OneHundredMHzSteps", "HwPstate",
        "TscInvariant", "CPB", "EffFreqRO", "ProcFeedbackInterface",
        "ProcPowerReporting", "ConnectedStandbyl", "RAPL",
        /* "Reserved Bit15-31" */
    ];
    let ftr = to_vstring(ftr);

    let buff = detect_ftr(edx, ftr);
    print_feature(buff);
}

pub fn spec_amd_80_08h(ebx: u32) {
    let ftr = vec![
        "CLZERO", "InstRetCntMsr", "RstrFpErrPtrs", "INVLPGB",
        "RDPRU", "", "MBE", "",
        "MCOMMIT", "WBNOINVD", "", "",
        "IBPB", "INT_WBINVD", "IBRS", "STIBP",
        "", "StibpAlwaysOn", "IbrsPreferred", "IbrsProvidesSameModeProtection",
        "EferLmsleUnsupported", "", "", "PPIN",
        "SSBD", "", "", "CPPC",
        "PSFD",
        /* "Reserved Bit29-31", */
    ];
    let ftr = to_vstring(ftr);
    let buff = detect_ftr(ebx, ftr);

    print_feature(buff);
}

pub fn fpu_width_amd_80_1ah(eax: u32) {
    let ftr = vec![
        "FP128", "MOVU", "FP256",
    ];
    let ftr = to_vstring(ftr);

    let buff = detect_ftr(eax, ftr);
    print_feature(buff);
}

pub fn secure_amd_80_1fh(eax: u32) {
    let ftr = vec![
        "SME", "SEV", "VmPgFlush", "SevEs",
        "SNP", "VMPL", "", "",
        "", "", "CoherencyEnforced", "Req64BitHypervisor",
        "RestrictInjection", "AlternateInjection", "DebugStateSwap", "PreventHostIBS",
        "VTE", /* "Reserved Bit17-31" */
    ];
    let ftr = to_vstring(ftr);

    let buff = detect_ftr(eax, ftr);
    print_feature(buff);
}
