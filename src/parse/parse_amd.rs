use super::*;
use libcpuid_dump::{TlbType, TlbInfo, Tlb};

trait PrintTlb {
    fn print_tlb(&self) -> String;
}

impl PrintTlb for Tlb {
    fn print_tlb(&self) -> String {
        const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; 7]) };

        format!("\
            {LN_PAD}[{}TLB 4K: {}\
            {LN_PAD}{PAD} 2M: {}\
            {LN_PAD}{PAD} 4M: {}]\
        ", self.type_, self.page_4k, self.page_2m, self.page_4m)
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
    fn amd_ext_topo_80_26h(&self) -> String;
}

impl ParseAMD for CpuidResult {
    fn pkgtype_amd_80_01h(&self) -> String {
        let pkg_type = libcpuid_dump::AmdPkgType::from(self);

        format!("[PkgType: {pkg_type}]")
    }

    fn l1_amd_80_05h(&self) -> String {
        let l1d_size = self.ecx >> 24; // KiB
        let l1i_size = self.edx >> 24; // KiB

        let l1itlb = Tlb::reg(
            TlbType::L1i,
            (self.ebx & 0xFFFF) as u16,
            (self.eax & 0xFFFF) as u16
        ).print_tlb();
        let l1dtlb = Tlb::reg(
            TlbType::L1d,
            (self.ebx >> 16) as u16,
            (self.eax >> 16) as u16
        ).print_tlb();

        format!("\
            [L1D {l1d_size}K/L1I {l1i_size}K]\
            {l1itlb}\
            {l1dtlb}\
        ")
    }

    fn l2_amd_80_06h(&self) -> String {
        let l2_size = self.ecx >> 16; // KiB
        let l3_size = (self.edx >> 18) / 2; // 512 KiB

        let l2itlb = Tlb::reg(
            TlbType::L2i,
            (self.ebx & 0xFFFF) as u16,
            (self.eax & 0xFFFF) as u16
        ).print_tlb();
        let l2dtlb = Tlb::reg(
            TlbType::L2d,
            (self.ebx >> 16) as u16,
            (self.eax >> 16) as u16
        ).print_tlb();

        format!("\
            [L2 {l2_size}K/L3 {l3_size}M]\
            {l2itlb}\
            {l2dtlb}\
        ")
    }

    fn apmi_amd_80_07h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, &ftr_amd_80_07_edx_x0()))
    }

    fn size_id_amd_80_08h(&self) -> String {
        use libcpuid_dump::AmdSizeId;

        let AmdSizeId {
            perf_tsc_size,
            apic_id_size,
            num_thread,
            rdpru_max_input,
            invlpgb_max_page,
        } = AmdSizeId::from(self);

        format!("\
            [Num Threads: {num_thread}] [APIC ID: {apic_id_size}-bits]\
            {LN_PAD}[Perf TSC size: {perf_tsc_size}-bits]\
            {LN_PAD}[RDPRU max input: {rdpru_max_input}]\
            {LN_PAD}[INVLPGB max page: {invlpgb_max_page}]\
        ")
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
            TlbInfo::from_reg_l2(reg as u16)
        );

        format!("\
            [L1iTLB 1G: {l1itlb}]\
            {LN_PAD}\
            [L1dTLB 1G: {l1dtlb}]\
            {LN_PAD}\
            [L2iTLB 1G: {l2itlb}]\
            {LN_PAD}\
            [L2dTLB 1G: {l2dtlb}]\
        ")
    }

    fn fpu_width_amd_80_1ah(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_amd_80_1a_eax_x0()))
    }

    fn ibs_amd_80_1bh(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_amd_80_1b_eax_x0()))
    }

    fn cpu_topo_amd_80_1eh(&self) -> String {
        use libcpuid_dump::AmdProcTopo;
        let AmdProcTopo {
            ext_apic_id: _,
            threads_per_core,
            core_id,
            nodes_per_processor: _,
            node_id,
        } = AmdProcTopo::from(self);

        format!("\
            [NodeId: {node_id}, CoreId: {core_id}]\
            {LN_PAD}\
            [threads per core: {threads_per_core}]\
        ")
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
            format!("\
                {ftr}\
                {LN_PAD}[uCodePatchSize: {ucode_patch_size} Bytes]\
            ")
        } else {
            ftr
        }
    }

    fn amd_ext_topo_80_26h(&self) -> String {
        let ext_topo = libcpuid_dump::AmdExtTopo::from(self);

        let core_type = match ext_topo.core_type {
            Some(core_type) => format!("{LN_PAD}[CoreType: {core_type}]"),
            None => "".to_string(),
        };

        let nid = match ext_topo.native_model_id {
            Some(nid) => format!("{LN_PAD}[Model: {nid}]"),
            None => "".to_string(),
        };

        format!("\
            [LevelType: {}, NumProc: {}]\
            {nid}\
            {core_type}\
        ",
            ext_topo.level_type,
            ext_topo.num_proc,
        )
    }
}
