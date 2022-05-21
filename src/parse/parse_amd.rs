use crate::*;

pub trait ParseAMD {
    fn pkgtype_amd_80_01h(&self) -> String;
    fn l1l2tlb_1g_amd_80_19h(&self) -> String;
    fn cpu_topo_amd_80_1eh(&self) -> String;
    fn l1_amd_80_05h(&self) -> String;
    fn l2_amd_80_06h(&self) -> String;
    fn apmi_amd_80_07h(&self) -> String;
    fn spec_amd_80_08h(&self) -> String;
    fn size_amd_80_08h(&self) -> String;
    fn rev_id_amd_80_0ah(&self) -> String;
    fn fpu_width_amd_80_1ah(&self) -> String;
    fn ibs_amd_80_1bh(&self) -> String;
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
        return format!(" [PkgType: {}({:#X})]",
            pkg_dec, pkg_type);
    }

    fn l1l2tlb_1g_amd_80_19h(&self) -> String {
        let [eax, ebx] = [self.eax, self.ebx];

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

    fn cpu_topo_amd_80_1eh(&self) -> String {
        let [ebx, ecx] = [self.ebx, self.ecx];
        let v = [
            format!(" [Core ID: {}]", ebx & 0xFF),
            padln!(),
            format!(" [Thread(s) per core: {}]", ((ebx >> 8) & 0xFF) + 1),
            padln!(),
            format!(" [Node ID: {}]", ecx & 0xFF),
        ];

        return concat_string_from_slice(&v);
    }

    fn l1_amd_80_05h(&self) -> String {
        let [eax, ebx, ecx, edx] = [
            self.eax,
            self.ebx,
            self.ecx,
            self.edx,
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

    fn l2_amd_80_06h(&self) -> String {
        let [eax, ebx, ecx, edx] = [
            self.eax,
            self.ebx,
            self.ecx,
            self.edx,
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

    fn apmi_amd_80_07h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, FTR_AMD_80_07_EDX_X0))
    }

    fn spec_amd_80_08h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.ebx, FTR_AMD_80_08_EBX_X0))
    }

    fn size_amd_80_08h(&self) -> String {
        format!(" [NC: {}]", (self.ecx & 0xFF) + 1)
    }

    fn rev_id_amd_80_0ah(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.edx, FTR_AMD_80_0A_EBX_X0))
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
            format!("{} [MemEncryptPhysAddWidth: {}-bits]",
                padln!(), reduction_size)
        } else {
            "".to_string()
        }
    }

    fn ext_amd_80_21h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, FTR_AMD_80_21_EAX_X0))
    }
}
