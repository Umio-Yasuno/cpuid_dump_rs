use crate::*;
use libcpuid_dump::Vendor;

pub trait ParseGeneric {
    fn vendor_00_00h(&self) -> String;
    fn info_00_01h(&self) -> String;
    fn monitor_mwait_00_05h(&self) -> String;
    fn feature_00_01h(&self) -> String;
    fn thermal_power_00_06h(&self) -> String;
    fn feature_00_07h_x0(&self) -> String;
    fn feature_00_07h_x1(&self) -> String;
    fn topo_ext_00_0bh(&self) -> String;
    fn xstate_00_0dh(&self, sub_leaf: u32) -> String;
    fn feature_80_01h(&self) -> String;
    fn addr_size_80_08h(&self) -> String;
    fn cpu_name(&self) -> String;
    fn cache_prop(&self) -> String;
}

impl ParseGeneric for CpuidResult {
    fn vendor_00_00h(&self) -> String {
        format!(" [{}]", Vendor::from_cpuid(self).get_name())
    }

    fn info_00_01h(&self) -> String {
        let [eax, ebx] = [self.eax, self.ebx];

        let fms = libcpuid_dump::FamModStep::from_cpuid(eax);

        let apic_id = ebx >> 24;
        let total_thread = (ebx >> 16) & 0xFF;
        let clflush_size = ((ebx >> 8) & 0xFF) * 8;

        return [
            format!(" [F: 0x{:X}, M: 0x{:X}, S: 0x{:X}]", fms.syn_fam, fms.syn_mod, fms.step),
            lnpad!(),
            format!(" [Codename: {}]", fms.codename()),
            lnpad!(),
            format!(" [APIC ID: {apic_id}]"),
            lnpad!(),
            format!(" [Total thread(s): {total_thread}]"),
            lnpad!(),
            format!(" [CLFlush (Byte): {clflush_size}]"),
        ].concat();
    }

    fn monitor_mwait_00_05h(&self) -> String {
        let min_mon_line_size = self.eax & 0xFFFF;
        let max_mon_line_size = self.ebx & 0xFFFF;
        let ftr = [
            if (self.ecx & 0b01) == 0b01 { " [EMX]" } else { "" },
            if (self.ecx & 0b10) == 0b10 { " [IBE]" } else { "" },
        ].concat();

        let c_state: String = {
            let mut c = 0;

            [
                (self.edx) & 0xF,
                (self.edx >>  4) & 0xF,
                (self.edx >>  8) & 0xF,
                (self.edx >> 12) & 0xF,
                (self.edx >> 16) & 0xF,
                (self.edx >> 20) & 0xF,
                (self.edx >> 24) & 0xF,
                (self.edx >> 28) & 0xF,
            ].map(|v| {
                let parsed = if v != 0 {
                    format!("{LN_PAD} [C{c} sub-state using MWAIT: {v}]")
                } else {
                    "".to_string()
                };

                c += 1;

                parsed
            }).concat()
        };
        
        return [
            format!(" [MonitorLineSize: {min_mon_line_size}(Min), {max_mon_line_size}(Max)]"),
            lnpad!(),
            ftr,
            c_state,
        ].concat();
    }

    fn feature_00_01h(&self) -> String {
        let buff = [
            str_detect_ftr(self.edx, &ftr_00_01_edx_x0()),
            str_detect_ftr(self.ecx, &ftr_00_01_ecx_x0()),
        ].concat();

        align_mold_ftr(&buff)
    }

    fn thermal_power_00_06h(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_00_06_eax_x0()))
    }

    fn feature_00_07h_x0(&self) -> String {
        let buff = [
            str_detect_ftr(self.ebx, &ftr_00_07_ebx_x0()),
            str_detect_ftr(self.ecx, &ftr_00_07_ecx_x0()),
            str_detect_ftr(self.edx, &ftr_00_07_edx_x0()),
        ].concat();

        align_mold_ftr(&buff)
    }

    fn feature_00_07h_x1(&self) -> String {
        align_mold_ftr(&str_detect_ftr(self.eax, &ftr_00_07_eax_x1()))
    }

    fn topo_ext_00_0bh(&self) -> String {
        let topo = libcpuid_dump::IntelExtTopo::from_cpuid(self);

        format!(" [LevelType: {}, num: {}]", topo.level_type, topo.num_proc)
    }

    fn xstate_00_0dh(&self, sub_leaf: u32) -> String {
        let x0 = |eax: u32| -> String {
            let tmp = align_mold_ftr(&str_detect_ftr(eax, &xfeature_mask_00_0d_eax_x0()));

            if !tmp.is_empty() {
                format!(" [-XFEATURE Mask-]{LN_PAD}{tmp}")
            } else {
                tmp
            }
        };

        let x1 = |eax: u32| -> String {
            align_mold_ftr(&str_detect_ftr(eax, &xsave_00_0d_eax_x1()))
        };

        let size = |eax: u32, txt: &str| -> String {
            /* 00_0D_X{SUB}:EAX is the state size, EAX = 0 indicates not supported it */
            if eax != 0x0 {
                format!(" [{}: size({})]", txt, eax)
            } else {
                "".to_string()
            }
        };

        let eax = self.eax;

        match sub_leaf {
            0x0 => x0(eax),
            0x1 => x1(eax),
            0x2 => size(eax, "XSTATE"),
            0x9 => size(eax, "Protection Key"),
            0xB => size(eax, "CET User"),
            0xC => size(eax, "CET SuperVisor"),
            _ => "".to_string(),
        }
    }

    fn feature_80_01h(&self) -> String {
        // 0x8000_0001_E{CD}X_x0
        let buff = [
            str_detect_ftr(self.ecx, &ftr_80_01_ecx_x0()),
            str_detect_ftr(self.edx, &ftr_80_01_edx_x0()),
        ].concat();

        align_mold_ftr(&buff)
    }

    fn addr_size_80_08h(&self) -> String {
        const LEN: usize = " [Address size:".len();
        const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; LEN]) };
        let pad = format!("{LN_PAD}{PAD}");

        let eax = self.eax;
        let p_size = eax & 0xFF;
        let v_size = (eax >> 8) & 0xFF;

        format!(" [Address size: {p_size:2}-bits physical {pad} {v_size:2}-bits virtual]")
    }

    fn cpu_name(&self) -> String {
        let name = libcpuid_dump::ProcName::dec_cpuid(self);

        String::from_utf8(name).unwrap()
    }

    fn cache_prop(&self) -> String {
        let cache = libcpuid_dump::CacheProp::from_cpuid(self);

        if cache.level == 0 { return "".to_string(); }

        return [
            format!(" [L{}{},{:>3}_way,{:>4}_{}]",
                cache.level,
                &cache.cache_type.to_string()[..1],
                cache.way,
                cache.size / cache.size_unit.to_byte(),
                &cache.size_unit.to_string()[..1]
            ),
            // format!(" [Shared {}T]", cache.share_thread),
            if cache.inclusive {
                " [Inclusive]"
            } else {
                ""
            }.to_string()
        ].concat();
    }
}
