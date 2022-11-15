use super::*;
use libcpuid_dump::{TlbType, TlbInfo, Tlb};

trait PrintEntryWay {
    fn print_entry_way(&self) -> String;
}

impl PrintEntryWay for TlbInfo {
    fn print_entry_way(&self) -> String {
        format!("{:>4}_entry, {:>4}_way", self.size, self.assoc)
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
    fn size_id_amd_80_08h(&self) -> String;
    fn svm_rev_amd_80_0ah_eax_ebx(&self) -> String;
    fn svm_ftr_amd_80_0ah_edx(&self) -> String;
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
        let pkg_type = libcpuid_dump::AmdPkgType::from(self);

        format!("[PkgType: {pkg_type}]")
    }

    fn l1_amd_80_05h(&self) -> String {
        let CpuidResult { eax, ebx, ecx, edx } = self;

        let l1d_size = ecx >> 24; // KiB
        let l1i_size = edx >> 24; // KiB

        let l1dtlb = Tlb::reg(TlbType::L1d, (ebx >> 16) as u16, (eax >> 16) as u16);
        let l1itlb = Tlb::reg(TlbType::L1i, (ebx & 0xFFFF) as u16, (eax & 0xFFFF) as u16);

        [
            format!("[L1D {l1d_size}K/L1I {l1i_size}K]"),
            l1itlb.print_tlb(),
            l1dtlb.print_tlb(),
        ].concat()
    }

    fn l2_amd_80_06h(&self) -> String {
        let CpuidResult { eax, ebx, ecx, edx } = self;

        let l2_size = ecx >> 16; // KiB
        let l3_size = (edx >> 18) / 2; // 512 KiB

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

    fn size_id_amd_80_08h(&self) -> String {
        let size_id = libcpuid_dump::AmdSizeId::from(self);
        
        [
            format!(
                "[Num Threads: {}] [APIC ID: {}-bits]",
                size_id.num_thread,
                size_id.apic_id_size,
            ),
            lnpad!(),
            format!("[Perf TSC size: {}-bits]", size_id.perf_tsc_size),
        ].concat()
    }

    fn svm_rev_amd_80_0ah_eax_ebx(&self) -> String {
        let rev = self.eax & 0xFF;
        let nasid = self.ebx;

        format!("[SVM Rev: {rev:#X}] [NASID: {nasid:#X}]")
    }

    fn svm_ftr_amd_80_0ah_edx(&self) -> String {
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
        let topo = libcpuid_dump::AmdProcTopo::from(self);

        [
            format!("[NodeID: {}, CoreID: {}]", topo.node_id, topo.core_id),
            lnpad!(),
            format!("[Thread(s) per core: {}]", topo.threads_per_core),
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
        let ucode_patch_size = self.ebx & 0xFFF;

        if 0 < ucode_patch_size {
            [
                ftr,
                lnpad!(),
                format!("[uCodePatchSize: {ucode_patch_size}]"),
            ].concat()
        } else {
            ftr
        }
    }
}
