pub(crate) const fn ftr_00_01_edx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "FPU";
    ftr[1] = "VME";
    ftr[2] = "DE"; // DebugExt
    ftr[3] = "PSE";
    ftr[4] = "TSC";
    ftr[5] = "MSR";
    ftr[6] = "PAE";
    ftr[7] = "MCE";
    ftr[8] = "CX8";
    ftr[9] = "APIC";
    ftr[10] = "";
    ftr[11] = "SEP";
    ftr[12] = "MTRR";
    ftr[13] = "PGE";
    ftr[14] = "MCA";
    ftr[15] = "CMOV";
    ftr[16] = "PAT";
    ftr[17] = "PSE36";
    ftr[18] = "PSN";
    ftr[19] = "CLFLUSH";
    ftr[20] = "";
    ftr[21] = "DS";
    ftr[22] = "ACPI";
    ftr[23] = "MMX";
    ftr[24] = "FXSR";
    ftr[25] = "SSE";
    ftr[26] = "SSE2";
    ftr[27] = "SS"; // Self Snoop
    ftr[28] = "HTT";
    ftr[29] = "TM"; // Thermal Monitor
    ftr[30] = "";
    ftr[31] = "PBE"; // Pending Break Enable

    ftr
}

pub(crate) const fn ftr_00_01_ecx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "SSE3";
    ftr[1] = "PCLMULQDQ";
    ftr[2] = "DTES64"; // 64-bit DS Area
    ftr[3] = "MONITOR"; // MONITOR/MWAIT
    ftr[4] = "DS-CPL"; // CPL Qualified Debug Store
    ftr[5] = "VMX"; // Virtual Machine Extensions
    ftr[6] = "SMX"; // Safer Mode Extensions
    ftr[7] = "EST"; // Enhanced Intel SpeedStep Technology
    ftr[8] = "TM2"; // Thermal Monito 2
    ftr[9] = "SSSE3";
    ftr[10] = "CNXT-ID"; // L1 Context ID
    ftr[11] = "SDBG"; // Silicon Debug
    ftr[12] = "FMA";
    ftr[13] = "CX16"; // CMPXCHG16B
    ftr[14] = "xTPR Update Control";
    ftr[15] = "PDCM"; // Perfmon and Debug Capability
    ftr[16] = "";
    ftr[17] = "PCID";
    ftr[18] = "DCA";
    ftr[19] = "SSE4.1";
    ftr[20] = "SSE4.2";
    ftr[21] = "x2APIC";
    ftr[22] = "MOVBE";
    ftr[23] = "POPCNT";
    ftr[24] = "TSC-Deadline";
    ftr[25] = "AES"; // AESNI
    ftr[26] = "XSAVE"; // XSAVE/XRSTOR
    ftr[27] = "OSXSAVE"; // XSETBV/XGETBV
    ftr[28] = "AVX";
    ftr[29] = "F16C";
    ftr[30] = "RDRAND";
    ftr[31] = "";

    ftr
}

pub(crate) const fn ftr_00_06_eax_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "DiditalTempSensor";
    ftr[1] = "TurboBoost";
    ftr[2] = "ARAT"; // APIC-Timer-always-running, always running APIC timer
    ftr[3] = "";
    ftr[4] = "PLN"; // Power Limit Management
    ftr[5] = "ECMD"; // Clock modulation duty cycle extension
    ftr[6] = "PTM"; // Package Thermal Management
    ftr[7] = "HWP";
    ftr[8] = "HWP_Notification";
    ftr[9] = "HWP_Activity_Window";
    ftr[10] = "HWP_Energy_Performance_Preference";
    ftr[11] = "HWP_Package_Level_Request";
    ftr[12] = "";
    ftr[13] = "HDC";
    ftr[14] = "TurboBoostMax";
    ftr[15] = "HWP_Capabilities";
    ftr[16] = "HWP_PECI";
    ftr[17] = "Flexible_HWP";
    ftr[18] = "FastAccessMode";
    ftr[19] = "HFI"; // Hardware Feedback Interface
    ftr[20] = ""; // Ignoring Idle Logical Processor HWP req
    ftr[21] = "";
    ftr[22] = "";
    ftr[23] = "EHFI";

    ftr
}

pub(crate) const fn ftr_00_07_ebx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "FSGSBASE";
    ftr[1] = "TSC_Adjust";
    ftr[2] = "SGX";
    ftr[3] = "BMI1";
    ftr[4] = "HLE";
    ftr[5] = "AVX2";
    ftr[6] = "FDP_EXCPTN_ONLY";
    ftr[7] = "SMEP";
    ftr[8] = "BMI2";
    ftr[9] = "ERMS"; // Enhanced REP MOVSB/STOSB
    ftr[10] = "INVPCID";
    ftr[11] = "RTM";
    ftr[12] = "PQM"; // AMD: Platform QoS Monitoring, Intel: RDT-M (Resource Director Technology - Monitoring)
    ftr[13] = "FPU_CS_DS"; // Deprecates FPU CS and FPU DS
    ftr[14] = "MemoryProtectionExtensions";
    ftr[15] = "PQE"; // AMD: PQE (Cache Allocation Technology, Platform QoS Allocation?), Intel: RTD-A (Allocation),
    ftr[16] = "AVX512F";
    ftr[17] = "AVX512DQ";
    ftr[18] = "RDSEED";
    ftr[19] = "ADX";
    ftr[20] = "SMAP";
    ftr[21] = "AVX512IFMA";
    // ftr[22] = "";
    ftr[23] = "CLFLUSHOPT";
    ftr[24] = "CLWB";
    ftr[25] = "ProcessorTrace";
    ftr[26] = "AVX512PF";
    ftr[27] = "AVX512ER";
    ftr[28] = "AVX512CD";
    ftr[29] = "SHA";
    ftr[30] = "AVX512BW";
    ftr[31] = "AVX512VL";
                 
    ftr
}

pub(crate) const fn ftr_00_07_ecx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "PREFETCHWT1"; // Intel Xeon Phi only
    ftr[1] = "AVX512_VBMI";
    ftr[2] = "UMIP";
    ftr[3] = "PKU";
    ftr[4] = "OSPKE";
    ftr[5] = "WAITPKG";
    ftr[6] = "AVX512_VBMI2";
    ftr[7] = "CET_SS";
    ftr[8] = "GFNI";
    ftr[9] = "VAES";
    ftr[10] = "VPCLMULQDQ";
    ftr[11] = "AVX512_VNNI";
    ftr[12] = "AVX512_BITALG";
    ftr[13] = "TME_EN";
    ftr[14] = "AVX512_VPOPCNTDQ";
    // ftr[15] = "";
    ftr[16] = "LA57"; // 57-bit linear addresses
    /*  The value of MAWAU used by the BNDLDX and BNDSTX instructions in 64-bit mode.
    ftr[17] = "";
    ftr[18] = "";
    ftr[19] = "";
    ftr[20] = "";
    ftr[21] = "";
    */
    ftr[22] = "RDPID";
    ftr[23] = "KL"; // Key Locker
    ftr[24] = "BUS_LOCK_DETECT";
    ftr[25] = "CLDEMOTE";
    // ftr[26] = "";
    ftr[27] = "MOVDIRI";
    ftr[28] = "MOVDIRI64B";
    ftr[29] = "ENQCMD"; // Enqueue Stores
    ftr[30] = "SGX_LC"; // SGX Launch Configuration
    ftr[31] = "PKS"; // protection keys for supervisor-mode pages

    ftr
}

pub(crate) const fn ftr_00_07_edx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[2] = "AVX512_4VNNIW"; // Intel Xeon Phi only
    ftr[3] = "AVX512_4FMAPS"; // Intel Xeon Phi only
    ftr[4] = "FSRM"; // Fast Short REP MOV
    ftr[5] = "UINTR"; // the processor supports user interrupts
    // ftr[6] = "";
    // ftr[7] = "";
    ftr[8] = "AVX512_VP2INTERSECT";
    ftr[9] = "SRBDS_CTRL";
    ftr[10] = "MD_CLEAR";
    ftr[11] = "RTM_ALWAYS_ABORT";
    // ftr[12] = "";
    ftr[13] = "RTM_FORCE_ABORT";
    ftr[14] = "SERIALIZE";
    ftr[15] = "Hybrid";
    ftr[16] = "TSXLDTRK"; // Intel TSX suspend load address tracking
    // ftr[17] = "";
    ftr[18] = "PCONFIG";
    ftr[19] = "ArchitecturalLBR";
    ftr[20] = "CET_IBT";
    // ftr[21] = "";
    ftr[22] = "AMX-BF16";
    ftr[23] = "AVX512_FP16";
    ftr[24] = "AMX-TILE";
    ftr[25] = "AMX-INT8";
    ftr[26] = "IBRS";
    ftr[27] = "STIBP";
    ftr[28] = "L1D_FLUSH";
    // ftr[29] = "IA32_ARCH_CAPABILITIES";
    // ftr[30] = "IA32_CORE_CAPABILITIES";
    ftr[31] = "SSBD";

    ftr
}

pub(crate) const fn ftr_00_07_eax_x1() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[3] = "RAO-INT";
    ftr[4] = "AVX-VNNI";
    ftr[5] = "AVX512_BF16";
    ftr[7] = "CMPCCXADD";
    ftr[8] = "ArchPerfmonExt";
    ftr[10] = "FZRM"; // fast zero-length MOVSB
    ftr[11] = "FSRS"; // fast short STOSB
    ftr[12] = "FSRC"; // fast short CMPSB, SCASB
    /* https://lore.kernel.org/lkml/20221006154041.13001-2-xin3.li@intel.com/ */
    ftr[18] = "LKGS"; // Load "kernel" (userspace) gs
    ftr[19] = "WRMSRNS"; // Non-Serializing WRMSR
    ftr[21] = "AMX-FP16";
    ftr[22] = "HRESET";
    ftr[23] = "AVX-IFMA";
    ftr[26] = "LAM"; // Linear Address Masking
    ftr[27] = "MSRLIST"; // [RD,WR]MSRLIST

    ftr
}

pub(crate) const fn ftr_00_07_edx_x1() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[4] = "AVX-VNNI-INT8";
    ftr[5] = "AVX-NE-CONVERT";
    ftr[14] = "PREFETCHITI";

    ftr
}

/* Ref: https://github.com/torvalds/linux/blob/master/arch/x86/kernel/fpu/xstate.c */
pub(crate) const fn xfeature_mask_00_0d_eax_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "X87";
    ftr[1] = "SSE";
    ftr[2] = "AVX256";
    ftr[3] = "MPX bounds";
    ftr[4] = "MPX CSR";
    ftr[5] = "AVX512 opmask"; // KREGS
    ftr[6] = "AVX512 Hi256";
    ftr[7] = "AVX512 ZMM_Hi256";
    ftr[8] = "Processor Trace"; // unused
    ftr[9] = "Protection Key User";
    ftr[10] = "PASID";
    ftr[11] = "";
    ftr[12] = "";
    ftr[13] = "";
    ftr[14] = "";
    ftr[15] = "";
    ftr[16] = "";
    ftr[17] = "AMX Tile config";
    ftr[18] = "AMX Tile data";

    ftr
}

pub(crate) const fn xsave_00_0d_eax_x1() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "XSAVEOPT";
    ftr[1] = "XSAVEC";
    ftr[2] = "XGETBV";
    ftr[3] = "XSAVES";
    ftr[4] = "XFD";

    ftr
}

pub(crate) const fn xsave_00_0d_ecx_x1() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[11] = "CET User";
    ftr[12] = "CET SuperVisor";

    ftr
}

pub(crate) const fn ftr_80_01_ecx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "LAHF/SAHF";
    ftr[1] = "CmpLegacy";
    ftr[2] = "SVM";
    ftr[3] = "Ext_APIC_Space";
    ftr[4] = "Alt_MOV_CR8";
    ftr[5] = "ABM";
    ftr[6] = "SSE4A";
    ftr[7] = "MisAlignSSE";
    ftr[8] = "3DNowPrefetch";
    ftr[9] = "OSVW"; // OS visible workaround
    ftr[10] = "IBS"; // Instruction based sampling
    ftr[11] = "XOP"; // Extended operation
    ftr[12] = "SKINIT"; // SKINIT and STGI
    ftr[13] = "WDT"; // Watchdog timer
    ftr[14] = "";
    ftr[15] = "LWP"; // Lightweight profiling
    ftr[16] = "FMA4";
    ftr[17] = "TCE"; // Translation Cache Extension
    ftr[18] = "";
    ftr[19] = "";
    ftr[20] = "";
    ftr[21] = "TBM"; // Trailing bit manipulation
    ftr[22] = "TopologyExtensions"; // CPUID Fn8000_001D_EAX_x[N:0]-CPUID Fn8000_001E_EDX
    ftr[23] = "PerfCtrExtCore"; // Processor performance counter extensions
    ftr[24] = "PerfCtrExtNB"; // NB performance counter extensions
    ftr[25] = "";
    ftr[26] = "DataBkptExt"; // Data access breakpoint extension
    ftr[27] = "PerfTSC"; // Performance time-stamp counter
    ftr[28] = "PerfCtrExtLLC"; // L3 performance counter extension
    ftr[29] = "MONITORX"; // MWAITX/MONITORX
    ftr[30] = "AddrMaskExt";

    ftr
}

pub(crate) const fn ftr_80_01_edx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[11] = "SYSCALL/SYSRET";
    ftr[20] = "NXbit";
    ftr[26] = "Page1GB";
    ftr[27] = "RDTSCP";
    ftr[29] = "LongMode";
    ftr[30] = "3DNow!Ext";
    ftr[31] = "3DNow!";

    ftr
}

pub(crate) const fn ftr_amd_80_07_edx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "TS"; // Temperature Sensor
    ftr[1] = "";
    ftr[2] = "";
    ftr[3] = "TTP"; // THERMTRIP
    ftr[4] = "TM"; // Hardware thermal control
    ftr[5] = "";
    ftr[6] = "100MHzSteps";
    ftr[7] = "HwPstate";
    ftr[8] = "TscInvariant";
    ftr[9] = "CPB"; // Core Performance Boost
    ftr[10] = "EffFreqRO"; // read-only effective frequency interface
    ftr[11] = "ProcFeedbackInterface";
    ftr[12] = "ProcPowerReporting";
    ftr[13] = "ConnectedStandby";
    ftr[14] = "RAPL";

    ftr
}

pub(crate) const fn ftr_80_08_ebx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "CLZERO";
    ftr[1] = "InstRetCntMsr";
    ftr[2] = "RstrFpErrPtrs";
    ftr[3] = "INVLPGB"; // INVLPGB and TLBSYNC instruction
    ftr[4] = "RDPRU";
    ftr[5] = "";
    ftr[6] = "";
    ftr[7] = "";
    ftr[8] = "MCOMMIT";
    ftr[9] = "WBNOINVD";
    ftr[10] = "";
    ftr[11] = "";
    ftr[12] = "IBPB";
    ftr[13] = "INT_WBINVD";
    ftr[14] = "IBRS";
    ftr[15] = "STIBP";
    ftr[16] = "IBRS_Always_On";
    ftr[17] = "STIBP_Always_On";
    ftr[18] = "IBRS_Preferred";
    ftr[19] = "IBRS_Same_Mode";
    ftr[20] = "EFER_LMSLE_Unsupported";
    ftr[21] = "INVLPGB_Nested_Pages";
    ftr[22] = "";
    ftr[23] = "";
    ftr[24] = "SSBD"; // Speculative Store Bypass Disable
    ftr[25] = "SSBD_Virt_Spec_Ctrl";
    ftr[26] = "SSBD_Not_Required";
    ftr[27] = "";
    ftr[28] = "PSFD"; // Predictive Store Forward Disable

    ftr
}

pub const fn ftr_amd_80_0a_edx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "NestedPaging";
    ftr[1] = "LbrVirt";
    ftr[2] = "SVM_Lock";
    ftr[3] = "NRIP_Save";
    ftr[4] = "TSC_Rate_MSR";
    ftr[5] = "VMCB_Clean";
    ftr[6] = "FlushByASID";
    ftr[7] = "DecodeAssists";
    ftr[8] = "";
    ftr[9] = "";
    ftr[10] = "PauseFilter";
    ftr[11] = "";
    ftr[12] = "PauseFilterThreshold";
    ftr[13] = "AVIC"; // AMD virtual interrupt controller
    ftr[14] = "";
    ftr[15] = "V_VMSAVE_VMLOAD";
    ftr[16] = "vGIF"; // Virtualized GIF
    ftr[17] = "GMET"; // Guest Mode Execute Trap
    ftr[18] = "x2AVIC";
    ftr[19] = "SupervisorShadowStack";
    ftr[20] = "GuestSpecCtrl";
    ftr[21] = "ROGPT"; // Read-Only Guest Page Table
    ftr[22] = "";
    ftr[23] = "HOST_MCE_OVERRIDE";
    ftr[25] = "VNMI"; // NMI Virtualization
    ftr[26] = "IbsVirt";

    ftr
}

pub(crate) const fn ftr_amd_80_1a_eax_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "FP128";
    ftr[1] = "MOVU";
    ftr[2] = "FP256";

    ftr
}

pub const fn ftr_amd_80_1b_eax_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "IBSFFV"; // IBS feature flags valid
    ftr[1] = "FetchSam"; // IBS fetch sampling
    ftr[2] = "OpSam"; // IBS execution sampling
    ftr[3] = "RdWrOpCnt"; // Read write of op counter
    ftr[4] = "OpCnt"; // Op counting mode
    ftr[5] = "BrnTrgt"; // Branch target address reporting
    ftr[6] = "OpCntExt"; // IbsOpCurCnt and IbsOpMaxCnt extend by 7 bits
    ftr[7] = "RipInvalidChk"; // Invalid RIP indication
    ftr[8] = "OpBrnFuse"; // Fused branch micro-op indication
    ftr[11] = "IbsL3MissFiltering";

    ftr
}

pub const fn ftr_amd_80_1f_eax_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "SME"; // Secure Memory Encryption
    ftr[1] = "SEV"; // Secure Encrypted Virtualization
    ftr[2] = "PageFlushMSR";
    ftr[3] = "SEV-ES"; // SEV Encrypted State
    ftr[4] = "SEV-SNP"; // SEV Secure Nested Paging
    ftr[5] = "VMPL"; // VM Permission Levels
    ftr[6] = "";
    ftr[7] = "";
    ftr[8] = "SecureTSC";
    ftr[9] = "";
    ftr[10] = "HwEnfCacheCoh"; // Hardware cache coherency across encryption domains enforced
    ftr[11] = "64BitHost";
    ftr[12] = "RestrictedInjection";
    ftr[13] = "AlternateInjection";
    ftr[14] = "DebugSwap";
    ftr[15] = "PreventHostIBS";
    ftr[16] = "VTE"; // Virtual Transparent Encryption
    ftr[17] = "VmgexitParameter";
    ftr[18] = "VirtualTomMsr";
    ftr[19] = "IbsVirtGuestCtl";
    /* .. */
    ftr[24] = "VMSARegProt"; // VMSA Register Protection
    ftr[25] = "SmtProtection";
    /* .. */
    ftr[28] = "SvsmCommPageMSR"; // SVSM Communication Page MSR
    ftr[29] = "NestedVirtSnpMsr";

    ftr
}

/*
pub(crate) const fn ftr_amd_80_20_ebx_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[1] = "MBE"; // Memory bandwidth enforcement

    return ftr;
}
*/

pub(crate) const fn ftr_amd_80_21_eax_x0() -> [&'static str; 32] {
    let mut ftr = [""; 32];

    ftr[0] = "NoNestedDataBp"; // Processor ignores nested data breakpoints
    ftr[1] = "FsGsKernelGsBaseNonSerializing";
    ftr[2] = "LFenceAlwaysSerializing";
    ftr[3] = "SmmPgCfgLock";
    // ftr[4] = "";
    // ftr[5] = "";
    ftr[6] = "NullSelectClearsBase";
    ftr[7] = "UpperAddressIgnore";
    ftr[8] = "AutomaticIBRS";
    ftr[9] = "NoSmmCtlMSR";
    ftr[10] = "FSRS"; // Fast Short Rep Stosb
    ftr[11] = "FSRC"; // Fast Short Repe Cmpsb
    // ftr[12] = "";
    ftr[13] = "PrefetchCtlMsr";
    /* */
    ftr[17] = "CpuidUserDis"; // GpOnUserCpuid
    ftr[18] = "EPSF"; // Enhanced Predictive Store Forwarding

    ftr
}
