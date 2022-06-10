pub const INPUT_WIDTH: usize = "  0x00000000_x0: ".len();
pub const OUTPUT_WIDTH: usize = "0x00000000 ".len() * 4;
pub const PARSE_WIDTH: usize = 36;
pub const TOTAL_WIDTH: usize = PARSE_WIDTH + INPUT_WIDTH + OUTPUT_WIDTH;

/** FTR_{leaf first 2}_{leaf last 2}_{register}_{sub-leaf}
    Bit00-03
    Bit04-07
    Bit08-11
    Bit12-15
    Bit16-19
    Bit20-23
    Bit24-27
    Bit28-31
*/

pub const FTR_00_01_EDX_X0: &[&str] = &[
    "FPU", "VME", "DE", "PSE",
    "TSC", "MSR", "PAE", "MCE",
    "CX8", "APIC", "", "SYSCALL",
    "MTRR", "PGE", "MCA", "CMOV",
    "PAT", "PSE36", "", "",
    "", "", "", "MMX",
    "FXSR", "", "",  "SS",
    "HTT",
    /*
        Bit02 => DebugExt
        Bit08 => CMPXCHG8B,

        Bit25 => SSE,
        Bit26 => SSE2,

        use `ftr_variant_expand("SSE", &[(flag: bool, end_name: &str)])`
    */
];
pub const FTR_00_01_ECX_X0: &[&str] = &[
    "", "PCLMULQDQ", "", "MONITOR",
    "", "", "", "",
    "", "", "", "",
    "FMA", "CX16", "", "",
    "", "PCID", "", "",
    "", "X2APIC", "MOVBE", "POPCNT",
    "", "AES", "XSAVE", "OSXSAVE",
    "AVX", "F16C", "RDRAND", "",
    /*
        Bit13 => CMPXCHG16B,

        Bit00 => SSE3,
        Bit19 => SSE41,
        Bit24 => SSE42,

        use `ftr_variant_expand("SSE", &[(flag: bool, end_name: &str)])`
    */
    /* Bit09 => SSSE3, */
];

pub const FTR_00_07_EBX_X0: &[&str] = &[
    "FSGSBASE", "", "SGX", "BMI1",
    "HLE", "AVX2", "", "SMEP",
    "BMI2", "ERMS", "INVPCID", "",
    "", "", "", "",
    "", "", "RDSEED", "ADX",
    "SMAP", "", "", "CLFSHOPT",
    "CLWB", "", "", "",
    "", "SHA", "", "",
    /*
        Bit11 => Intel: RTM, AMD: [Reserved]
        Bit12 => Intel RDT-M (Resource Director Technology - Monitoring),
                 AMD: PQM (Platform QoS Monitoring)
        Bit15 => Intel RTD-A (Allocation),
                 AMD: PQE (Cache Allocation Technology, Platform QoS Allocation?)
    */
    /*
        Bit16 => AVX512F,
        Bit17 => AVX512DQ,
        Bit21 => AVX512_IFMA,
        Bit28 => AVX512CD,
        Bit30 => AVX512BW,
        Bit31 => AVX512VL,

        use `ftr_variant_expand("AVX512", &[(flag: bool, end_name: &str)])`
    */
    /*
        Intel Xeon Phi only:
        Bit26 => AVX512PF,
        Bit27 => AVX512ER,
    */
];

pub const FTR_00_07_ECX_X0: &[&str] = &[
    "", "", "UMIP", "PKU",
    "OSPKE", "", "", "CET_SS",
    "GFNI", "VAES", "VPCLMULQDQ", "",
    "", "", "", "",
    "", "", "", "",
    "", "", "RDPID", "KL",
    "", "CLDEMOTE", "", "MOVDIRI",
    "MOVDIRI64B", "ENQCMD",
    /*
        Bit01 => AVX512_VBMI,
        Bit06 => AVX512_VBMI2,
        Bit11 => AVX512_VNNI,
        Bit12 => AVX512_BITALG,
        Bit14 => AVX512_VPOPCNTDQ,

        use `ftr_variant_expand("AVX512", &[(flag: bool, end_name: &str)])`
    */
];

pub const FTR_00_07_EDX_X0: &[&str] = &[
    "", "", "", "",
    "FSRM", "UINTR", "", "",
    "", "", "MD_CLEAR", "",
    "", "", "SERIALIZE", "Hybrid",
    "TSXLDTRK", "", "PCONFIG", "",
    "CET_IBT",
    /*
        Bit08 => AVX512_VP2INTERSECT,
        Bit23 => AVX512_FP16,

        use `ftr_variant_expand("AVX512", &[(flag: bool, end_name: &str)])`
    */
    /*
        Bit22 => AMX-BF16,
        Bit24 => AMX-TILE,
        Bit25 => AMX-INT8,
    */
    /*
        Intel Xeon Phi only:
        Bit02 => AVX512_4VNNIW,
        Bit03 => AVX512_4FMAPS,
    */
];

pub const XFEATURE_MASK_00_0D_EAX_X0: &[&str] = &[
    "X87", "SSE", "AVX256", "MPX bound",
    "MPX CSR", "AVX512 opmask", "AVX512 Hi256", "AVX512 ZMM_Hi256",
    "", "Protection Key", "PASID", "",
    "", "", "", "",
    "", "AMX Tile config", "AMX Tile data",
    /*  Reference:
        https://github.com/torvalds/linux/blob/master/arch/x86/kernel/fpu/xstate.c
    */
];

pub const XSAVE_00_0D_EAX_X1: &[&str] = &[
    "XSAVEOPT", "XSAVEC", "XGETBV", "XSAVES",
    "XFD",
];


pub const FTR_AMD_80_07_EDX_X0: &[&str] = &[
    "TS", "", "", "TTP",
    "TM", "", "OneHundredMHzSteps", "HwPstate",
    "TscInvariant", "CPB", "EffFreqRO", "ProcFeedbackInterface",
    "ProcPowerReporting", "ConnectedStandby", "RAPL",
];

pub const FTR_AMD_80_08_EBX_X0: &[&str] = &[
    "CLZERO", "InstRetCntMsr", "RstrFpErrPtrs", "INVLPGB",
    "RDPRU", "", "MBE", "",
    "MCOMMIT", "WBNOINVD", "", "",
    "IBPB", "INT_WBINVD", "IBRS", "STIBP",
    "", "StibpAlwaysOn", "IbrsPreferred", "IbrsProvidesSameModeProtection",
    "EferLmsleUnsupported", "", "", "PPIN",
    "SpeculativeStoreBypassDisable", "VirtSSBD", "SSB_HW_FIXED", "CPPC",
    "PSFD", "", "", "BranchSampling"
];

pub const FTR_AMD_80_0A_EBX_X0: &[&str] = &[
    "NestedPage", "LbrVirt", "SvmLock", "NRIP Save",
    "TscRateMsr", "VmcbClean", "FlushByAsid", "DecodeAssists",
    "", "", "PauseFilter", "",
    "PauseFilterThreshold", "AVIC", "", "V_VMSAVE_VMLOAD",
    "vGIF", "GMET", "", "SupervisorShadowStack",
    "GuestSpecCtrl", "", "", "HOST_MCE_OVERRIDE",
];

pub const FTR_80_01_ECX_X0: &[&str] = &[
    "LAHF/SAHF", "CmpLegacy", "SVM", "ExtApicSpace",
    "AltMovCr8", "ABM (LZCNT)", "SSE4A", "MisAlignSse",
    "3DNow!Prefetch", "OSVW", "IBS", "XOP",
    "SKINIT", "WDT", "", "LWP",
    "FMA4", "TCE", "", "",
    "", "", "TopologyExtensions", "PerfCtrExtCore",
    "PerfCtrExtDFl", "", "DataBreakpointExtension", "PerfTsc",
    "PerfCtrExtLLC", "MwaitExtended", "AdMskExtn", "",
];

pub const FTR_AMD_80_1A_EAX_X0: &[&str] = &[
    "FP128", "MOVU", "FP256",
];

pub const FTR_AMD_80_1B_EAX_X0: &[&str] = &[
    "IBSFFV", "FetchSam", "OpSam", "RdWrOpCnt",
    "OpCnt", "BrnTrgt", "OpCntExt", "RipInvalidChk",
    "OpBrnFuse", "IbsFetchCtlExtd", "IbsOpData4",
];

pub const FTR_AMD_80_1F_EAX_X0: &[&str] = &[
    "SME", "SEV", "VmPgFlush", "SevEs",
    "SNP", "VMPL", "", "",
    "", "", "CoherencyEnforced", "Req64BitHypervisor",
    "RestrictInjection", "AlternateInjection", "DebugStateSwap", "PreventHostIBS",
    "VTE",
];

/*
    [RFC PATCH v0 0/6] x86/AMD: Userspace address tagging
    https://lore.kernel.org/linux-mm/699fb763ac054833bc8c29c9814c63b2@AcuMS.aculab.com/T/#m1b9caa0c700839bc9238a3161ddc5b757062d077
*/
pub const FTR_AMD_80_21_EAX_X0: &[&str] = &[
    "NoNestedDataBp", "", "LFenceAlwaysSerializing", "SmmPgCfgLock",
    "", "", "NullSelectorClearsBase", "UpperAddressIgnore",
];
