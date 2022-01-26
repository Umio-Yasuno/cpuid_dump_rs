use crate::*;

pub fn pkgtype_amd_80_01h(ebx: &u32) -> String {
    let pkg_type = *ebx >> 28;
    let pkg_dec = match pkg_type {
        0x0 => "FP5/FP6",
        0x2 => "AM4",
        _   => "Unknown",
    };
    return format!(" [PkgType: {}({:#X})]",
        pkg_dec, pkg_type);
}

pub fn l1l2tlb_1g_amd_80_19h(cpuid: &CpuidResult) -> String {
    let [eax, ebx,/* ecx, edx*/] = [
        cpuid.eax,
        cpuid.ebx,
    /*
        cpuid.ecx,
        cpuid.edx,
    */
    ];
    let v = &[
        format!(" [L1TLB 1G: Data {:>3}, Inst {:>3}]",
            (eax >> 16) & 0xFFF, eax & 0xFFF,
        ),
        format!("{} [L2TLB 1G: Data {:>3}, Inst {:>3}]",
            padln!(),
            (ebx >> 16) & 0xFFF, ebx & 0xFFF,
        ),
    ];

    return concat_string_from_slice(v);
}

pub fn cpu_topo_amd_80_1eh(cpuid: &CpuidResult) -> String {
    let [ebx, ecx] = [
        cpuid.ebx,
        cpuid.ecx,
    ];
    let v = &[
        format!(" [Core ID: {}]", ebx & 0xFF),
        format!("{} [{} thread(s) per core]",
            padln!(),
            ((ebx >> 8) & 0xFF) + 1,
        ),
        format!("{} [Node ID: {}]",
            padln!(),
            ecx & 0xFF,
        ),
    ];

    return concat_string_from_slice(v);
}

pub fn l1_amd_80_05h(cpuid: &CpuidResult) -> String {
    let [eax, ebx, ecx, edx] = [
        cpuid.eax,
        cpuid.ebx,
        cpuid.ecx,
        cpuid.edx,
    ];
    let v = &[
        format!(" [L1D {}K/L1I {}K]",
            ecx >> 24, (edx >> 24) & 0xFF,
        ),
        format!("{} [L1dTLB: 4K {:>3}, 2M/4M {:>3}]",
            padln!(),
            (ebx >> 16) & 0xFF, (eax >> 16) & 0xFF,
        ),
        format!("{} [L1iTLB: 4K {:>3}, 2M/4M {:>3}]",
            padln!(),
            ebx & 0xFF, eax & 0xFF,
        ),
    ];

    return concat_string_from_slice(v);
}

pub fn l2_amd_80_06h(cpuid: &CpuidResult) -> String {
    let [eax, ebx, ecx, edx] = [
        cpuid.eax,
        cpuid.ebx,
        cpuid.ecx,
        cpuid.edx,
    ];

    let v = &[
        format!(" [L2 {}K/L3 {}M]",
            (ecx >> 16), (edx >> 18) / 2,
        ),
        format!("{} [L2dTLB: 4K {}, 2M {}",
            padln!(),
            (ebx >> 16) & 0xFFF, (eax >> 16) & 0xFFF,
        ),
        format!("{}{} 4M {:4}]",
            padln!(), " ".repeat(9),
            ((eax >> 16) & 0xFFF) / 2,
        ),
        format!("{} [L2iTLB: 4K {}, 2M {}",
            padln!(),
            ebx & 0xFFF, eax & 0xFFF,
        ),
        format!("{}{} 4M {:4}]",
            padln!(), " ".repeat(9),
            (eax & 0xFFF) / 2
        ),
    ];

    return concat_string_from_slice(v);
}


pub fn enum_amd_0dh(cpuid: &RawCpuid) -> String {
    let x0 = |eax: u32| -> String {
        let ftr = &[
            "X87", "SSE", "AVX256", "",
            "", "", "", "",
            "", "MPK",
            /* Reserved Bit10-31 */
        ];

        return align_mold_ftr(&str_detect_ftr(eax, ftr));
    };
    let x1 = |eax: u32| -> String {
        let ftr = &[
            "XSAVEOPT", "XSAVEC", "XGETBV", "XSAVES",
            /* "Reserved Bit4-31" */
        ];

        return align_mold_ftr(&str_detect_ftr(eax, ftr));
    };

    let size = |eax: u32, txt: &str| -> String {
        if eax != 0 {
            format!(" [{}: size({})]", txt, eax)
        } else {
            "".to_string()
        }
    };

    let eax = cpuid.result.eax;

    return match cpuid.sub_leaf {
        0x0 => x0(eax),
        0x1 => x1(eax),
        0x2 => size(eax, "XSTATE"),
        0x9 => size(eax, "MPK"),
        0xB => size(eax, "CET_U"),
        0xC => size(eax, "CET_S"),
        _ => unreachable!(),
    };
}

pub fn apmi_amd_80_07h(edx: &u32) -> String {
    let ftr = &[
        "TS", "", "", "TTP",
        "TM", "", "OneHundredMHzSteps", "HwPstate",
        "TscInvariant", "CPB", "EffFreqRO", "ProcFeedbackInterface",
        "ProcPowerReporting", "ConnectedStandbyl", "RAPL",
        /* "Reserved Bit15-31" */
    ];

    return align_mold_ftr(&str_detect_ftr(*edx, ftr));
}

pub fn spec_amd_80_08h(ebx: &u32) -> String {
    let ftr = &[
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

    return align_mold_ftr(&str_detect_ftr(*ebx, ftr));
}

pub fn size_amd_80_08h(ecx: &u32) -> String {
    format!(" [NC: {}]", (ecx & 0xFF) + 1)
}

pub fn rev_id_amd_80_0ah(cpuid: &CpuidResult) -> String {
    let ftr_edx = &[
        "NestedPage", "LbrVirt", "SvmLock", "NRIP Save",
        "TscRateMsr", "VmcbClean", "FlushByAsid", "DecodeAssists",
        "", "", "PauseFilter", "",
        "PauseFilterThreshold", "AVIC", "", "V_VMSAVE_VMLOAD",
        "vGIF", "GMET", "", "SupervisorShadowStack",
        "GuestSpecCtrl", "", "", "HOST_MCE_OVERRIDE",
        /* */
    ];

    return align_mold_ftr(&str_detect_ftr(cpuid.edx, ftr_edx));
}

pub fn fpu_width_amd_80_1ah(eax: &u32) -> String {
    let ftr = &[
        "FP128", "MOVU", "FP256",
    ];

    return align_mold_ftr(&str_detect_ftr(*eax, ftr));
}

pub fn ibs_amd_80_1bh(eax: &u32) -> String {
    let ftr = &[
        "IBSFFV", "FetchSam", "OpSam", "RdWrOpCnt",
        "OpCnt", "BrnTrgt", "OpCntExt", "RipInvalidChk",
        "OpBrnFuse", "IbsFetchCtlExtd", "IbsOpData4", /* */
    ];

    return align_mold_ftr(&str_detect_ftr(*eax, ftr));
}

pub fn secure_amd_80_1fh(eax: &u32) -> String {
    let ftr = &[
        "SME", "SEV", "VmPgFlush", "SevEs",
        "SNP", "VMPL", "", "",
        "", "", "CoherencyEnforced", "Req64BitHypervisor",
        "RestrictInjection", "AlternateInjection", "DebugStateSwap", "PreventHostIBS",
        "VTE", /* "Reserved Bit17-31" */
    ];

    return align_mold_ftr(&str_detect_ftr(*eax, ftr));
}
