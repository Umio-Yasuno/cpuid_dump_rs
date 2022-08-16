use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
enum TlbType {
    L1d,
    L1i,
    L2d,
    L2i,
}

impl TlbType {
    fn get_offset(&self) -> u16 {
        match self {
            Self::L1d |
            Self::L1i => 0xFF,
            Self::L2d |
            Self::L2i => 0xFFF,
        }
    }
}

impl fmt::Display for TlbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L1d => write!(f, "L1d"),
            Self::L1i => write!(f, "L1i"),
            Self::L2d => write!(f, "L2d"),
            Self::L2i => write!(f, "L2i"),
        }
    }
}

#[derive(Debug, Clone)]
struct TlbInfo {
    size: u16,
    assoc: u16,
}

impl TlbInfo {
    fn from_reg(reg: u16, offset: u16) -> Self {
        Self {
            size: reg & offset,
            assoc: reg >> offset.trailing_ones(),
        }
    }

    fn print_entry_way(&self) -> String {
        format!("{:>4}_entry, {:>3}_way", self.size, self.assoc)
    }
}

#[derive(Debug, Clone)]
struct Tlb {
    type_: TlbType,
    page_4k: TlbInfo,
    page_2m: TlbInfo,
    page_4m: TlbInfo,
    // page_1g
}

impl Tlb {
    fn reg(type_: TlbType, reg_4k: u16, reg_2m4m: u16) -> Self {
        let offset = type_.get_offset();
        let page_4k = TlbInfo::from_reg(reg_4k, offset);
        let page_2m = TlbInfo::from_reg(reg_2m4m, offset);
        let page_4m = TlbInfo {
            size: page_2m.size / 2,
            assoc: page_2m.assoc,
        };

        Self {
            type_,
            page_4k,
            page_2m,
            page_4m,
        }
    }

    fn disp(&self) -> String {
        const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; 8]) };

        return [
            lnpad!(),
            format!(" [{}TLB 4K: {}", self.type_, self.page_4k.print_entry_way()),
            lnpad!(),
            format!("{PAD} 2M: {}", self.page_2m.print_entry_way()),
            lnpad!(),
            format!("{PAD} 4M: {}]", self.page_4m.print_entry_way()),
        ].concat();
    }
}

pub trait ParseAMD {
    fn pkgtype_amd_80_01h(&self) -> String;
    fn l1_amd_80_05h(&self) -> String;
    fn l2_amd_80_06h(&self) -> String;
    fn apmi_amd_80_07h(&self) -> String;
    fn spec_amd_80_08h(&self) -> String;
    fn size_amd_80_08h(&self) -> String;
    fn rev_id_amd_80_0ah(&self) -> String;
    fn l1l2tlb_1g_amd_80_19h(&self) -> String;
    fn fpu_width_amd_80_1ah(&self) -> String;
    fn ibs_amd_80_1bh(&self) -> String;
    fn cpu_topo_amd_80_1eh(&self) -> String;
    fn encrypt_ftr_amd_80_1fh(&self) -> String;
    fn reduction_phys_addr_amd_80_1fh(&self) -> String;
    fn ext_amd_80_21h(&self) -> String;
}

impl ParseAMD for CpuidResult {
    fn pkgtype_amd_80_01h(&self) -> String {
        let ebx = self.ebx;

        let pkg_type = ebx >> 28;
        let pkg_dec = match pkg_type {
            0x0 => "FP5/FP6",
            0x2 => "AM4",
            _ => return "".to_string(),
        };

        return format!(" [PkgType: {pkg_dec}({pkg_type:#X})]")
    }

    fn l1l2tlb_1g_amd_80_19h(&self) -> String {
        let [eax, ebx] = [self.eax, self.ebx];

        /* Inst TLB number of entries for 1-GB pages, size: Bit00-11, assoc: Bit12-15 */
        /* Data TLB number of entries for 1-GB pages, size: Bit16-27, assoc: Bit28-31 */
        let [l1dtlb, l1itlb, l2dtlb, l2itlb] = [
            (eax >> 16),
            (eax & 0xFFFF),
            (ebx >> 16),
            (ebx & 0xFFFF),
        ].map(|reg|
            TlbInfo::from_reg(reg as u16, 0xFFF).print_entry_way()
        );

        return [
            format!(" [L1dTLB 1G: {}]", l1dtlb),
            lnpad!(),
            format!(" [L1iTLB 1G: {}]", l1itlb),
            lnpad!(),
            format!(" [L2dTLB 1G: {}]", l2dtlb),
            lnpad!(),
            format!(" [L2iTLB 1G: {}]", l2itlb),
        ].concat();
    }

    fn cpu_topo_amd_80_1eh(&self) -> String {
        let [ebx, ecx] = [self.ebx, self.ecx];

        let core_id = ebx & 0xFF;
        let th_per_core = ((ebx >> 8) & 0xFF) + 1;
        let node_id = ecx & 0xFF;

        return [
            format!(" [Core ID: {core_id}]"),
            lnpad!(),
            format!(" [Thread(s) per core: {th_per_core}]"),
            lnpad!(),
            format!(" [Node ID: {node_id}]"),
        ].concat();
    }

    fn l1_amd_80_05h(&self) -> String {
        let [eax, ebx, ecx, edx] = [
            self.eax,
            self.ebx,
            self.ecx,
            self.edx,
        ];

        let l1d_size = ecx >> 24;
        let l1i_size = edx >> 24;

        let l1dtlb = Tlb::reg(TlbType::L1d, (ebx >> 16) as u16, (eax >> 16) as u16);
        let l1itlb = Tlb::reg(TlbType::L1i, (ebx & 0xFFFF) as u16, (eax & 0xFFFF) as u16);

        return [
            format!(" [L1D {l1d_size}K/L1I {l1i_size}K]"),
            l1dtlb.disp(),
            l1itlb.disp(),
        ].concat();
    }

    fn l2_amd_80_06h(&self) -> String {
        let [eax, ebx, ecx, edx] = [
            self.eax,
            self.ebx,
            self.ecx,
            self.edx,
        ];

        let l2_size = ecx >> 16;
        let l3_size = (edx >> 18) / 2;

        let l2dtlb = Tlb::reg(TlbType::L2d, (ebx >> 16) as u16, (eax >> 16) as u16);
        let l2itlb = Tlb::reg(TlbType::L2i, (ebx & 0xFFFF) as u16, (eax & 0xFFFF) as u16);

        return [
            format!(" [L2 {l2_size}K/L3 {l3_size}M]"),
            l2dtlb.disp(),
            l2itlb.disp(),
        ].concat();
    }

    fn apmi_amd_80_07h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, FTR_AMD_80_07_EDX_X0))
    }

    fn spec_amd_80_08h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.ebx, FTR_AMD_80_08_EBX_X0))
    }

    fn size_amd_80_08h(&self) -> String {
        format!(" [Num threads: {}]", (self.ecx & 0xFF) + 1)
    }

    fn rev_id_amd_80_0ah(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, FTR_AMD_80_0A_EDX_X0))
    }

    fn fpu_width_amd_80_1ah(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, FTR_AMD_80_1A_EAX_X0))
    }

    fn ibs_amd_80_1bh(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, FTR_AMD_80_1B_EAX_X0))
    }

    fn encrypt_ftr_amd_80_1fh(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, FTR_AMD_80_1F_EAX_X0))
    }

    fn reduction_phys_addr_amd_80_1fh(&self) -> String {
        // Reduction of physical address space in bits when 
        // memory encryption is enabled (0 indicates no reduction).
        // [Reserved]: Bit16-31
        // VmplSupported: Bit12-15
        // MemEncryptPhysAddWidth: Bit6-11
        // CBit: Bit00-05
        let reduction_size = (self.ebx >> 6) & 0x3F;

        if 0 < reduction_size {
            format!("{LN_PAD} [MemEncryptPhysAddWidth: {reduction_size}-bits]")
        } else {
            "".to_string()
        }
    }

    fn ext_amd_80_21h(&self) -> String {
        let ftr = align_mold_ftr(&str_detect_ftr(self.eax, FTR_AMD_80_21_EAX_X0));

        if 0 < self.ebx {
            [
                ftr,
                lnpad!(),
                format!(" [uCodePatchSize: {}]", self.ebx),
            ].concat()
        } else {
            ftr
        }
    }
}
