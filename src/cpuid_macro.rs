/// 0x0000_0001_ECX_x0
// pub const CPUID_RESERVE :u32 = 1 << 31;
pub const CPUID_RDRAND  :u32 = 1 << 30;
pub const CPUID_F16C    :u32 = 1 << 29;
pub const CPUID_AVX     :u32 = 1 << 28;
pub const CPUID_OSXSAVE :u32 = 1 << 27;
pub const CPUID_XSAVE   :u32 = 1 << 26;
pub const CPUID_AES     :u32 = 1 << 25;
// pub const CPUID_RESERVE :u32 = 1 << 24;
pub const CPUID_POPCNT  :u32 = 1 << 23;
pub const CPUID_MOVBE   :u32 = 1 << 22;
pub const CPUID_X2APIC  :u32 = 1 << 21;
pub const CPUID_SSE42   :u32 = 1 << 20;
pub const CPUID_SSE41   :u32 = 1 << 19;
// pub const CPUID_RESERVE :u32 = 1 << 18;
pub const CPUID_PCID    :u32 = 1 << 17;
// pub const CPUID_RESERVE :u32 = 1 << 16;
// pub const CPUID_RESERVE :u32 = 1 << 15;
// pub const CPUID_RESERVE :u32 = 1 << 14;
pub const CPUID_CMPXCHG16B :u32 = 1 << 13;
pub const CPUID_FMA     :u32 = 1 << 12;
// pub const CPUID_RESERVE :u32 = 1 << 11;
// pub const CPUID_RESERVE :u32 = 1 << 10;
pub const CPUID_SSSE3   :u32 = 1 <<  9;
// pub const CPUID_RESERVE :u32 = 1 <<  8;
// pub const CPUID_RESERVE :u32 = 1 <<  7;
// pub const CPUID_RESERVE :u32 = 1 <<  6;
// pub const CPUID_RESERVE :u32 = 1 <<  5;
// pub const CPUID_RESERVE :u32 = 1 <<  4;
pub const CPUID_MONITOR :u32 = 1 <<  3;
// pub const CPUID_RESERVE :u32 = 1 <<  2;
pub const CPUID_PCLMULQDQ :u32 = 1 <<  1;
pub const CPUID_SSE3    :u32 = 1;
/*
pub const CPUID :u32 = 1 << ;
pub const CPUID :u32 = 1 << ;
pub const CPUID :u32 = 1 << ;
pub const CPUID :u32 = 1 << ;
pub const CPUID :u32 = 1 << ;
pub const CPUID :u32 = 1 << ;
*/
/// 0x0000_0007_EDX_x0
// pub const CPUID_RESERVE :u32 = 1 << 31;
// pub const CPUID_RESERVE :u32 = 1 << 30;
// pub const CPUID_RESERVE :u32 = 1 << 29;
pub const CPUID_HTT     :u32 = 1 << 28;
// pub const CPUID_RESERVE :u32 = 1 << 27;
pub const CPUID_SSE2    :u32 = 1 << 26;
pub const CPUID_SSE     :u32 = 1 << 25;
pub const CPUID_FXSR    :u32 = 1 << 24;
pub const CPUID_MMX     :u32 = 1 << 23;
// pub const CPUID_RESERVE :u32 = 1 << 22;
// pub const CPUID_RESERVE :u32 = 1 << 21;
// pub const CPUID_RESERVE :u32 = 1 << 20;
pub const CPUID_CLFSH   :u32 = 1 << 19;
// pub const CPUID_RESERVE :u32 = 1 << 18;
pub const CPUID_PSE36   :u32 = 1 << 17;
pub const CPUID_PAT     :u32 = 1 << 16;
pub const CPUID_CMOV    :u32 = 1 << 15;
pub const CPUID_MCA     :u32 = 1 << 14;
pub const CPUID_PGE     :u32 = 1 << 13;
pub const CPUID_MTRR    :u32 = 1 << 12;
pub const CPUID_SYSENTER_EXIT   :u32 = 1 << 11;
// pub const CPUID_RESERVE :u32 = 1 << 10;
pub const CPUID_APIC    :u32 = 1 <<  9;
pub const CPUID_CMPXCHG8B       :u32 = 1 <<  8;
pub const CPUID_MCE     :u32 = 1 <<  7;
pub const CPUID_PAE     :u32 = 1 <<  6;
pub const CPUID_MSR     :u32 = 1 <<  5;
pub const CPUID_TSC     :u32 = 1 <<  4;
pub const CPUID_PSE     :u32 = 1 <<  3;
pub const CPUID_DE      :u32 = 1 <<  2;
pub const CPUID_VME     :u32 = 1 <<  1;
pub const CPUID_FPU     :u32 = 1;

/// 0x0000_0007_EBX_x0
pub const CPUID_AVX512_VL   :u32 = 1 << 31;
pub const CPUID_AVX512_BW   :u32 = 1 << 30;
pub const CPUID_SHA         :u32 = 1 << 29;
pub const CPUID_AVX512_CD   :u32 = 1 << 28;
pub const CPUID_AVX512_ER   :u32 = 1 << 27;
pub const CPUID_AVX512_PF   :u32 = 1 << 26;
pub const CPUID_CLWB        :u32 = 1 << 24;
pub const CPUID_CLFSHOPT    :u32 = 1 << 23;
pub const CPUID_AVX512_IFMA :u32 = 1 << 21;
pub const CPUID_SMAP        :u32 = 1 << 20;
pub const CPUID_ADX         :u32 = 1 << 19;
pub const CPUID_RDSEED      :u32 = 1 << 18;
pub const CPUID_AVX512_DQ   :u32 = 1 << 17;
pub const CPUID_AVX512_F    :u32 = 1 << 16;
pub const CPUID_INVPCID     :u32 = 1 << 10;
pub const CPUID_BMI2        :u32 = 1 <<  8;
pub const CPUID_SMEP        :u32 = 1 <<  7;
pub const CPUID_AVX2        :u32 = 1 <<  5;
pub const CPUID_BMI1        :u32 = 1 <<  3;
pub const CPUID_SGX         :u32 = 1 <<  2;
pub const CPUID_FSGSBASE    :u32 = 1;

/// 0x0000_0007_ECX_x0
pub const CPUID_PKS                 :u32 = 1 << 31;
pub const CPUID_SGX_LC              :u32 = 1 << 30;
pub const CPUID_ENQCMD              :u32 = 1 << 29;
pub const CPUID_MOVDIRI64B          :u32 = 1 << 28;
pub const CPUID_MOVDIRI             :u32 = 1 << 27;
pub const CPUID_CLDEMOTE            :u32 = 1 << 25;
pub const CPUID_KL                  :u32 = 1 << 23;
pub const CPUID_RDPID               :u32 = 1 << 22;
pub const CPUID_LA57                :u32 = 1 << 16;
pub const CPUID_AVX512_VPOPCNTDQ    :u32 = 1 << 14;
pub const CPUID_TME_EN              :u32 = 1 << 13;
pub const CPUID_AVX512_BITALG       :u32 = 1 << 12;
pub const CPUID_AVX512_VNNI         :u32 = 1 << 11;
pub const CPUID_VPCLMULDQ           :u32 = 1 << 10;
pub const CPUID_VAES                :u32 = 1 <<  9;
pub const CPUID_GFNI                :u32 = 1 <<  8;
pub const CPUID_CET_SS              :u32 = 1 <<  7;
pub const CPUID_AVX512_VBMI2        :u32 = 1 <<  6;
pub const CPUID_WAITPKG             :u32 = 1 <<  5;
pub const CPUID_OSPKE               :u32 = 1 <<  4;
pub const CPUID_PKU                 :u32 = 1 <<  3;
pub const CPUID_UMIP                :u32 = 1 <<  2;
pub const CPUID_AVX512_VBMI1        :u32 = 1 <<  1;
pub const CPUID_AVX512_VBMI         :u32 = 1 <<  1;

/// 0x0000_0007_EDX_x0
pub const CPUID_AMX_INT8            :u32 = 1 << 25;
pub const CPUID_AMX_TILE            :u32 = 1 << 24;
pub const CPUID_AVX512_FP16         :u32 = 1 << 23;
pub const CPUID_AMX_BF16            :u32 = 1 << 22;
pub const CPUID_SERIALIZE           :u32 = 1 << 14;
pub const CPUID_MD_CLEAR            :u32 = 1 << 10;
pub const CPUID_AVX512_VP2INTERSECT :u32 = 1 <<  8;
pub const CPUID_UINTR               :u32 = 1 <<  5;
pub const CPUID_FSRM                :u32 = 1 <<  4;
pub const CPUID_AVX512_4FMAPS       :u32 = 1 <<  3;
pub const CPUID_AVX512_4VNNIW       :u32 = 1 <<  2;

/// 0x0000_0007_EAX_x1
pub const CPUID_LAM         :u32 = 1 << 26;
pub const CPUID_HRESET      :u32 = 1 << 22;
pub const CPUID_AVX512_BF16 :u32 = 1 <<  5;
pub const CPUID_AVX_VNNI    :u32 = 1 <<  4;

/// 0x8000_0001_ECX_x0
pub const CPUID_FMA4        :u32 = 1 << 16;
pub const CPUID_3DNOW_PF    :u32 = 1 <<  8;
pub const CPUID_SSE4A       :u32 = 1 <<  6;
pub const CPUID_LZCNT       :u32 = 1 <<  5;
pub const CPUID_LAHF_SAHF   :u32 = 1;

/// 0x8000_0001_EDX_x0
pub const CPUID_3DNOW       :u32 = 1 << 31;
pub const CPUID_3DNOW_EXT   :u32 = 1 << 30;
