pub const INPUT_WIDTH: usize = "  0x00000000_x0: ".len();
pub const OUTPUT_WIDTH: usize = "0x00000000 ".len() * 4;
pub const PARSE_WIDTH: usize = 32;
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
    "FPU", "VME", "DebugExt", "PSE",
    "TSC", "MSR", "PAE", "MCE",
    "CMPXCHG8B", "APIC", "", "SysCallSysRet",
    "MTRR", "PGE", "MCA", "CMOV",
    "PAT", "PSE36", "", "",
    "", "", "", "MMX",
    "FXSR", "", "",  "", /* Bit25: SSE, Bit26: SSE2 */
    "HTT", /* */
];
pub const FTR_00_01_ECX_X0: &[&str] = &[
    "", "PCLMULQDQ", "", "Monitor/Mwait", /* Bit0: SSE3 */
    "", "", "", "",
    "", "", "", "", /* Bit9: SSSE3 */
    "FMA", "CMPXCHG16B", "", "",
    "", "PCID", "", "", /* Bit19: SSE41 */
    "", "X2APIC", "MOVBE", "POPCNT", /* Bit24: SSE42 */
    "", "AES", "XSAVE", "OSXSAVE",
    "AVX", "F16C", "RDRAND", "",
];

pub const FTR_00_07_EBX_X0: &[&str] = &[
    "FSGSBASE", "", "SGX", "BMI1",
    "", "AVX2", "", "SMEP",
    "BMI2", "ERMS", "INVPCID", "",
    "PQM", "", "", "PQE",
    "", "", "RDSEED", "ADX",
    "SMAP", "", "", "CLFSHOPT",
    "CLWB", "", "", "",
    "", "SHA", "", "",
];
pub const FTR_00_07_ECX_X0: &[&str] = &[
    "", "", "UMIP", "PKU",
    "OSPKE", "", "", "CET_SS",
    "GFNI", "VAES", "VPCLMULQDQ", "",
    "", "", "", "",
    "", "", "", "",
    "", "", "RDPID", "KL",
    "", "CLDEMOTE", "", "MOVDIRI",
    "MOVDIRI64B", "ENQCMD", /* */
];
pub const FTR_00_07_EDX_X0: &[&str] = &[
    "", "", "", "",
    "FSRM", "UINTR", "", "",
    "", "", "MD_CLEAR", "",
    "", "", "SERIALIZE", "",
    /* */
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

