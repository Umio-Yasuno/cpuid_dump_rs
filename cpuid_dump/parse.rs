//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{CpuidResult};

#[path = "./_parse/parse_amd.rs"]
mod parse_amd;
pub use parse_amd::*;
#[path = "./_parse/parse_intel.rs"]
mod parse_intel;
pub use parse_intel::*;
#[path = "./parse_util.rs"]
#[macro_use]
mod parse_util;
pub use parse_util::*;

pub fn info_00_01h(cpuid: &CpuidResult) -> String {
    use cpuid_asm::*;
    let mut buff: Vec<String> = Vec::new();

    let [eax, ebx] = [
        cpuid.eax,
        cpuid.ebx,
    ];

    let fms = FamModStep::from_cpuid(&eax);
    let codename = fms.codename();

    buff.push(
        format!(" [F: 0x{:X}, M: 0x{:X}, S: 0x{:X}]", fms.syn_fam, fms.syn_mod, fms.step)
    );
    buff.push(format!("{} [{}]", padln!(), codename));

    buff.push(format!("{} [APIC ID: {}]", padln!(), ebx >> 24));
    buff.push(format!("{} [Total {} thread(s)]", padln!(), (ebx >> 16) & 0xFF));
    buff.push(format!("{} [CLFlush: {}B]", padln!(), ((ebx >> 8) & 0xFF) * 8));

    return concat_string(buff);
}

pub fn feature_00_01h(cpuid: &CpuidResult) -> String {
    let mut buff: Vec<String> = Vec::with_capacity(64);
    //  0x0000_0007_EDX_x0
    let ftr_edx = vec![
        "FPU", "VME", "DebugExt", "PSE",
        "TSC", "MSR", "PAE", "MCE",
        "CMPXCHG8B", "APIC", "", "SysCallSysRet",
        "MTRR", "PGE", "MCA", "CMOV",
        "PAT", "PSE36", "", "",
        "", "", "", "MMX",
        "FXSR", "", "",  "", /* Bit25: SSE, Bit26: SSE2 */
        "HTT", /* */
    ];
    //  0x0000_0007_ECX_x0
    let ftr_ecx = vec![
        "", "PCLMULQDQ", "", "Monitor/Mwait", /* Bit0: SSE3 */
        "", "", "", "",
        "", "", "", "", /* Bit9: SSSE3 */
        "FMA", "CMPXCHG16B", "", "",
        "", "PCID", "", "", /* Bit19: SSE41 */
        "", "X2APIC", "MOVBE", "POPCNT", /* Bit24: SSE42 */
        "", "AES", "XSAVE", "OSXSAVE",
        "AVX", "F16C", "RDRAND", "",
    ];

    let [ftr_edx, ftr_ecx] = [
        to_vstring(ftr_edx),
        to_vstring(ftr_ecx),
    ];

    let [edx, ecx] = [
        cpuid.edx,
        cpuid.ecx,
    ];

    buff.extend(detect_ftr(edx, ftr_edx));
    buff.extend(detect_ftr(ecx, ftr_ecx));

    let [edx, ecx] = [
        Reg::new(edx).to_boolvec(),
        Reg::new(ecx).to_boolvec(),
    ];

    if edx[25] { buff.push(format!(
        "SSE{0}{1}{2}{3}",
            has_ftr!(edx[26], "/2"),
            has_ftr!(ecx[ 0], "/3"),
            has_ftr!(ecx[19], "/4.1"),
            has_ftr!(ecx[20], "/4.2"),
        )
    )}
    if ecx[9] { push!(buff, "SSSE3"); }

    return mold_ftr(buff);
}

pub fn feature_00_07h_x0(cpuid: &CpuidResult) -> String {
    //  let tmp = cpuid!(0x7, 0x0);
    //  print_cpuid!(0x7, 0x0, tmp);

    let mut buff: Vec<String> = Vec::with_capacity(96);
    let [ebx, ecx, edx] = [
        cpuid.ebx,
        cpuid.ecx,
        cpuid.edx,
    ];

    // 0x00000007_EBX_x0
    let ftr_ebx = vec![
        "FSGSBASE", "", "SGX", "", /* Bit3: BMI1 */
        "", "", "", "SMEP", /* Bit5: AVX2 */
        "", "ERMS", "INVPCID", "", /* Bit8: BMI2 */
        "PQM", "", "", "PQE",
        "", "", "RDSEED", "ADX",
        "SMAP", "", "", "CLFSHOPT",
        "CLWB", "", "", "",
        "", "SHA", "", "",
    ];
    let ftr_ecx = vec![
        "", "", "UMIP", "PKU",
        "OSPKE", "", "", "CET_SS",
        "GFNI", "VAES", "VPCLMULQDQ", "",
        "", "", "", "",
        "", "", "", "",
        "", "", "RDPID", "KL",
        "", "CLDEMOTE", "", "MOVDIRI",
        "MOVDIRI64B", "ENQCMD", /* */
    ];
    let ftr_edx = vec![
        "", "", "", "",
        "FSRM", "UINTR", "", "",
        "", "", "MD_CLEAR", "",
        "", "", "SERIALIZE", "",
        /* */
    ];

    let [ftr_ebx, ftr_ecx, ftr_edx] = [
        to_vstring(ftr_ebx),
        to_vstring(ftr_ecx),
        to_vstring(ftr_edx),
    ];
    
    buff.extend(detect_ftr(ebx, ftr_ebx));
    buff.extend(detect_ftr(ecx, ftr_ecx));
    buff.extend(detect_ftr(edx, ftr_edx));

    let [ebx, ecx, edx] = [
        Reg::new(ebx).to_boolvec(),
        Reg::new(ecx).to_boolvec(),
        Reg::new(edx).to_boolvec(),
    ];

    let avx512_f    = ebx[16];
    let avx512_dq   = ebx[17];
    let avx512_ifma = ebx[21];
    let avx512_cd   = ebx[28];
    let avx512_bw   = ebx[30];
    let avx512_vl   = ebx[31];

    if avx512_f || avx512_dq || avx512_ifma || avx512_cd
    || avx512_bw || avx512_vl {
        buff.push(
            format!("AVX512_{0}{1}{2}{3}{4}{5}",
                has_ftr!(avx512_f, "F/"),
                has_ftr!(avx512_dq, "DQ/"),
                has_ftr!(avx512_ifma, "IFMA/"),
                has_ftr!(avx512_cd, "CD/"),
                has_ftr!(avx512_bw, "BW/"),
                has_ftr!(avx512_vl, "VL/")
            )
            .trim_end_matches("/")
            .to_string(),
        )
    }

    /*  Xeon Phi only */
    if ebx[26] && ebx[27] {
        push!(buff, "AVX512PF/ER");
    }

    // 0x00000007_ECX_x0
    let avx512_vbmi1     = ecx[ 1];
    let avx512_vbmi2     = ecx[ 6];
    let avx512_vnni      = ecx[11];
    let avx512_bitalg    = ecx[12];
    let avx512_vpopcntdq = ecx[14];

    if avx512_vbmi1 || avx512_vbmi2 || avx512_vnni
    || avx512_bitalg || avx512_vpopcntdq {
        buff.push(
            format!("AVX512_{0}{1}{2}{3}{4}",
                has_ftr!(avx512_vbmi1,     "VBMI/"),
                has_ftr!(avx512_vbmi2,     "VBMI2/"),
                has_ftr!(avx512_vnni,      "VNNI/"),
                has_ftr!(avx512_bitalg,    "BITALG/"),
                has_ftr!(avx512_vpopcntdq, "VPOPCNTDQ/"),
            )
            .trim_end_matches("/")
            .to_string(),
        )
    }

    // 0x00000007_EDX_x0
    /* Xeon Phi Only */
    let avx512_4vnniw_4fmaps = edx[ 2] && edx[ 3];

    let avx512_vp2intersect  = edx[ 8];
    let avx512_fp16          = edx[23];

    if avx512_4vnniw_4fmaps || avx512_vp2intersect || avx512_fp16 {
        buff.push(
            format!("AVX512_{0}{1}{2}",
                has_ftr!(avx512_4vnniw_4fmaps, "4VNNIW/4FMAPS/"),
                has_ftr!(avx512_vp2intersect,  "VP2INTERSECT/"),
                has_ftr!(avx512_fp16,          "FP16/"),
            )
            .trim_end_matches("/")
            .to_string(),
        )
    }

    /*  Currently Intel Sapphire Rapids only
        Bit 22: AMX-BF16,
        Bit 24: AMX-TILE,
        Bit 25: AMX-INT8
    */
    if edx[22] && edx[24] && edx[25] {
        push!(buff, format!("AMX-BF16/TILE/INT8"));
    }

    return mold_ftr(buff);
}

pub fn feature_00_07h_x1(eax: &u32) -> String {
    /*
    let tmp = cpuid!(0x7, 0x1);
    print_cpuid!(0x7, 0x1, tmp);
    */
    let eax = Reg::new(*eax).to_boolvec();
    let mut buff: Vec<String> = Vec::with_capacity(4);

    // https://github.com/torvalds/linux/commit/b85a0425d8056f3bd8d0a94ecdddf2a39d32a801
    if eax[ 4] { push!(buff, "AVX_VNNI")    }
    if eax[ 5] { push!(buff, "AVX512_BF16") }
    if eax[22] { push!(buff, "HRESET")      }
    if eax[26] { push!(buff, "LAM")         }

    return mold_ftr(buff);
}

pub fn feature_80_01h(cpuid: &CpuidResult) -> String {
    let [ecx, edx] = [
        cpuid.ecx,
        cpuid.edx,
    ];
    // 0x8000_0001_ECX_x0
    let ftr = vec![
        "LAHF/SAHF", "CmpLegacy", "SVM", "ExtApicSpace",
        "AltMovCr8", "ABM (LZCNT)", "SSE4A", "MisAlignSse",
        "3DNow!Prefetch", "OSVW", "IBS", "XOP",
        "SKINIT", "WDT", "", "LWP",
        "FMA4", "TCE", "", "",
        "", "", "TopologyExtensions", "PerfCtrExtCore",
        "PerfCtrExtDFl", "", "DataBreakpointExtension", "PerfTsc",
        "PerfCtrExtLLC", "MwaitExtended", "AdMskExtn", "",
    ];
    let ftr = to_vstring(ftr);

    let mut buff = detect_ftr(ecx, ftr);

    // 0x8000_0001_EDX_x0
    let edx = Reg::new(edx).to_boolvec();
    if edx[31] {
        buff.push(format!("3DNow!{}", has_ftr!(edx[30], "/EXT")))
    }

    return mold_ftr(buff);
}

#[allow(dead_code)]
struct CacheProp {
    cache_type: String,
    level: u32,
    line_size: u32,
    way: u32,
    set: u32,
    size: u32,
    share_thread: u32,
    size_unit: u32,
    size_unit_string: String,
    inclusive: bool,
}

impl CacheProp {
    fn dec(tmp: &CpuidResult) -> CacheProp {
        let [eax, ebx, ecx, edx] = [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];

        let cache_type = match eax & 0b11111 {
            0x1 => "Data",
            0x2 => "Inst",
            0x3 => "Unified",
            0x0 | _ => "",
        }.to_string();

        let level = (eax >> 5) & 0b111;
        let line_size = (ebx & 0xFFF) + 1;
        let way = (ebx >> 22) + 1;
        let set = ecx + 1;
        let size = line_size * way * set;

        let share_thread = ((eax >> 14) & 0xFFF) + 1;

        let mut size_unit = 1;
        let mut size_unit_string = "B";

        if size < 1000_000 {
            size_unit = 1 << 10;
            size_unit_string = "KiB";
        } else if size < 1000_000_000 {
            size_unit = 1 << 20;
            size_unit_string = "MiB";
        };

        let size_unit_string = size_unit_string.to_string();

        let inclusive = (edx & 0b10) != 0;

        CacheProp {
            cache_type,
            level,
            line_size,
            way,
            set,
            size,
            share_thread,
            size_unit,
            size_unit_string,
            inclusive,
        }
    }
}

pub fn cache_prop(cpuid: &CpuidResult) -> String {
    let cache = CacheProp::dec(cpuid);

    if cache.level == 0 || cache.cache_type.len() == 0 {
        return "".to_string();
    }

    let mut v = vec![
        format!(" [L{} {:<7} {:>3}-way, {:>4}{}]",
            cache.level, cache.cache_type, cache.way,
            cache.size / cache.size_unit, cache.size_unit_string),
        format!("{} [Shared {}T]",
            padln!(), cache.share_thread),
    ];

    if cache.inclusive {
        v.push(" [Inclusive]".to_string());
    }
   
    return concat_string(v);
}
