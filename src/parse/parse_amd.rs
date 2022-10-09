use crate::*;
use libcpuid_dump::{TlbType, TlbInfo, Tlb};

trait PrintEntryWay {
    fn print_entry_way(&self) -> String;
}

impl PrintEntryWay for TlbInfo {
    fn print_entry_way(&self) -> String {
        format!("{:>4}_entry, {:>3}_way", self.size, self.assoc)
    }
}

trait PrintTlb {
    fn print_tlb(&self) -> String;
}

impl PrintTlb for Tlb {
    fn print_tlb(&self) -> String {
        const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; 7]) };

        [
            lnpad!(),
            format!("[{}TLB 4K: {}", self.type_, self.page_4k.print_entry_way()),
            lnpad!(),
            format!("{PAD} 2M: {}", self.page_2m.print_entry_way()),
            lnpad!(),
            format!("{PAD} 4M: {}]", self.page_4m.print_entry_way()),
        ].concat()
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
        let pkg_type = self.ebx >> 28;
        /*
        // Family 17h, 19h
        let pkg_dec = match pkg_type {
            0x0 => "FP5/FP6",
            0x2 => "AM4",
            _ => return "".to_string(),
        };
        */

        format!("[PkgType: {pkg_type:#X}]")
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

        [
            format!("[L1D {l1d_size}K/L1I {l1i_size}K]"),
            l1itlb.print_tlb(),
            l1dtlb.print_tlb(),
        ].concat()
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

        [
            format!("[L2 {l2_size}K/L3 {l3_size}M]"),
            l2itlb.print_tlb(),
            l2dtlb.print_tlb(),
        ].concat()
    }

    fn apmi_amd_80_07h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, &ftr_amd_80_07_edx_x0()))
    }

    fn spec_amd_80_08h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.ebx, &ftr_amd_80_08_ebx_x0()))
    }

    fn size_amd_80_08h(&self) -> String {
        let num_t = (self.ecx & 0xFF) + 1;
        let apicid_size = (self.ecx >> 12) & 0xF;
        
        [
            format!("[Num Threads: {num_t}]"),
            lnpad!(),
            format!("[APIC ID size: {apicid_size}-bits]"),
        ].concat()
    }

    fn rev_id_amd_80_0ah(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, &ftr_amd_80_0a_edx_x0()))
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

        [
            format!("[L1iTLB 1G: {l1itlb}]"),
            lnpad!(),
            format!("[L1dTLB 1G: {l1dtlb}]"),
            lnpad!(),
            format!("[L2iTLB 1G: {l2itlb}]"),
            lnpad!(),
            format!("[L2dTLB 1G: {l2dtlb}]"),
        ].concat()
    }

    fn fpu_width_amd_80_1ah(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_amd_80_1a_eax_x0()))
    }

    fn ibs_amd_80_1bh(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_amd_80_1b_eax_x0()))
    }

    fn cpu_topo_amd_80_1eh(&self) -> String {
        let core_id = self.ebx & 0xFF;
        let th_per_core = ((self.ebx >> 8) & 0xFF) + 1;
        let node_id = self.ecx & 0xFF;

        [
            format!("[NodeID: {node_id}, CoreID: {core_id}]"),
            lnpad!(),
            format!("[Thread(s) per core: {th_per_core}]"),
        ].concat()
    }

    fn encrypt_ftr_amd_80_1fh(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_amd_80_1f_eax_x0()))
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
            format!("{LN_PAD}[MemEncryptPhysAddWidth: {reduction_size}-bits]")
        } else {
            "".to_string()
        }
    }

    fn ext_amd_80_21h(&self) -> String {
        let ftr = align_mold_ftr(&str_detect_ftr(self.eax, &ftr_amd_80_21_eax_x0()));

        if 0 < self.ebx {
            [
                ftr,
                lnpad!(),
                format!("[uCodePatchSize: {}]", self.ebx),
            ].concat()
        } else {
            ftr
        }
    }
}
