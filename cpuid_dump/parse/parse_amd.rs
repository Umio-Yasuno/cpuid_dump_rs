use crate::*;

pub fn pkgtype_amd_80_01h(ebx: &u32) -> String {
    let pkg_type = *ebx >> 28;
    let pkg_dec = match pkg_type {
        0x0 => "FP5/FP6",
        0x2 => "AM4",
        _ => return "".to_string(),
    };
    return format!(" [PkgType: {}({:#X})]",
        pkg_dec, pkg_type);
}

pub fn l1l2tlb_1g_amd_80_19h(cpuid: &CpuidResult) -> String {
    let [eax, ebx] = [cpuid.eax, cpuid.ebx];

    /* Inst TLB number of entries for 1-GB pages, size: Bit00-11, assoc: Bit12-15 */
    /* Data TLB number of entries for 1-GB pages, size: Bit16-27, assoc: Bit28-31 */
    let v = [
        format!(" [L1TLB 1G: Data {:>4}, Inst {:>4}]",
            (eax >> 16) & 0xFFF, eax & 0xFFF,
        ),
        padln!(),
        format!(" [L2TLB 1G: Data {:>4}, Inst {:>4}]",
            (ebx >> 16) & 0xFFF, ebx & 0xFFF,
        ),
    ];

    return concat_string_from_slice(&v);
}

pub fn cpu_topo_amd_80_1eh(cpuid: &CpuidResult) -> String {
    let [ebx, ecx] = [cpuid.ebx, cpuid.ecx];
    let v = [
        format!(" [Core ID: {}]", ebx & 0xFF),
        padln!(),
        format!(" [Thread(s) per core: {}T]", ((ebx >> 8) & 0xFF) + 1),
        padln!(),
        format!(" [Node ID: {}]", ecx & 0xFF),
    ];

    return concat_string_from_slice(&v);
}

pub fn l1_amd_80_05h(cpuid: &CpuidResult) -> String {
    let [eax, ebx, ecx, edx] = [
        cpuid.eax,
        cpuid.ebx,
        cpuid.ecx,
        cpuid.edx,
    ];
    let v = [
        format!(" [L1D {}K/L1I {}K]",
            ecx >> 24, (edx >> 24) & 0xFF,
        ),
        padln!(),
        format!(" [L1dTLB: 4K {:>4}, 2M/4M {:>4}]",
            (ebx >> 16) & 0xFF, (eax >> 16) & 0xFF,
        ),
        padln!(),
        format!(" [L1iTLB: 4K {:>4}, 2M/4M {:>4}]",
            ebx & 0xFF, eax & 0xFF,
        ),
    ];

    return concat_string_from_slice(&v);
}

pub fn l2_amd_80_06h(cpuid: &CpuidResult) -> String {
    let [eax, ebx, ecx, edx] = [
        cpuid.eax,
        cpuid.ebx,
        cpuid.ecx,
        cpuid.edx,
    ];

    let v = [
        format!(" [L2 {}K/L3 {}M]",
            (ecx >> 16), (edx >> 18) / 2,
        ),
        padln!(),
        format!(" [L2dTLB: 4K {:>4}, 2M {:>4}",
            (ebx >> 16) & 0xFFF, (eax >> 16) & 0xFFF,
        ),
        padln!(), 
        format!("{} 4M {:>4}]",
            " ".repeat(9),
            ((eax >> 16) & 0xFFF) / 2,
        ),
        padln!(),
        format!(" [L2iTLB: 4K {:>4}, 2M {:>4}",
            ebx & 0xFFF, eax & 0xFFF,
        ),
        padln!(), 
        format!("{} 4M {:>4}]",
            " ".repeat(9),
            (eax & 0xFFF) / 2
        ),
    ];

    return concat_string_from_slice(&v);
}


pub fn enum_amd_0dh(cpuid: &RawCpuid) -> String {
    let x0 = |eax: u32| -> String {
        align_mold_ftr(&str_detect_ftr(eax, FTR_AMD_00_0D_EAX_X0))
    };
    let x1 = |eax: u32| -> String {
        align_mold_ftr(&str_detect_ftr(eax, FTR_AMD_00_0D_EAX_X1))
    };

    let size = |eax: u32, txt: &str| -> String {
        /* 00_0D_X{SUB}:EAX is the state size, EAX = 0 indicates not supported it */
        if eax != 0x0 {
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
        _ => "".to_string(),
    };
}

pub fn apmi_amd_80_07h(edx: &u32) -> String {
    align_mold_ftr(&str_detect_ftr(*edx, FTR_AMD_80_07_EDX_X0))
}

pub fn spec_amd_80_08h(ebx: &u32) -> String {
    align_mold_ftr(&str_detect_ftr(*ebx, FTR_AMD_80_08_EBX_X0))
}

pub fn size_amd_80_08h(ecx: &u32) -> String {
    format!(" [NC: {}]", (ecx & 0xFF) + 1)
}

pub fn rev_id_amd_80_0ah(cpuid: &CpuidResult) -> String {
    align_mold_ftr(&str_detect_ftr(cpuid.edx, FTR_AMD_80_0A_EBX_X0))
}

pub fn fpu_width_amd_80_1ah(eax: &u32) -> String {
    align_mold_ftr(&str_detect_ftr(*eax, FTR_AMD_80_1A_EAX_X0))
}

pub fn ibs_amd_80_1bh(eax: &u32) -> String {
    align_mold_ftr(&str_detect_ftr(*eax, FTR_AMD_80_1B_EAX_X0))
}

pub fn encrypt_ftr_amd_80_1fh(eax: &u32) -> String {
    align_mold_ftr(&str_detect_ftr(*eax, FTR_AMD_80_1F_EAX_X0))
}

pub fn reduction_phys_addr_amd_80_1fh(ebx: &u32) -> String {
    // Reduction of physical address space in bits when 
    // memory encryption is enabled (0 indicates no reduction).
    // [Reserved]: Bit16-31
    // VmplSupported: Bit12-15
    // MemEncryptPhysAddWidth: Bit6-11
    // CBit: Bit00-05
    let reduction_size = (ebx >> 6) & 0x3F;

    if 0 < reduction_size {
        format!("{} [MemEncryptPhysAddWidth: {}-bits]",
            padln!(), reduction_size)
    } else {
        "".to_string()
    }
}

pub fn ext_amd_80_21h(eax: &u32) -> String {
    align_mold_ftr(&str_detect_ftr(*eax, FTR_AMD_80_21_EAX_X0))
}
