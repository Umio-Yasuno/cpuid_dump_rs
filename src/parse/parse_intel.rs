use super::*;
use crate::PARSE_WIDTH;

pub trait ParseIntel {
    fn clock_speed_intel_00_16h(&self) -> String;
    fn intel_tlb_param_00_18h(&self) -> String;
    fn intel_hybrid_1ah(&self) -> String;
    fn v2_ext_topo_intel_1fh(&self) -> String;
}

impl ParseIntel for CpuidResult {
    fn clock_speed_intel_00_16h(&self) -> String {
        format!(
            "[Base {}, Max {}, Bus {} MHz]",
            self.eax & 0xFFFF,
            self.ebx & 0xFFFF,
            self.ecx & 0xFFFF
        )
    }

    fn intel_tlb_param_00_18h(&self) -> String {
        let tlb_param = libcpuid_dump::IntelTlbParam::from(self);

        if let libcpuid_dump::IntelTlbType::Null = tlb_param.cache_type {
            return "".to_string();
        }

        let mut support_page = String::with_capacity(PARSE_WIDTH);
        {
            if tlb_param.support_4k { support_page.push_str("[4K]") }
            if tlb_param.support_2m { support_page.push_str("[2M]") }
            if tlb_param.support_4m { support_page.push_str("[4M]") }
            if tlb_param.support_1g { support_page.push_str("[1GB]") }
        }

        let fully_assoc = if tlb_param.fully_assoc {
            "{LN_PAD}[Fully Assoc]"
        } else {
            ""
        }.to_string();

        let partitioning = if tlb_param.partitioning == 0 {
            format!("{LN_PAD}[Soft partitioning]")
        } else {
            "".to_string()
        };

        format!("\
            [Type: {cache_type}]\
            {LN_PAD}{support_page}\
            {fully_assoc}\
            {partitioning}\
            {LN_PAD}[way: {way:>3}, set: {set:>3}]\
        ",
            cache_type = tlb_param.cache_type,
            way = tlb_param.way,
            set = tlb_param.set,
        )
    }

    fn intel_hybrid_1ah(&self) -> String {
        use libcpuid_dump::HybridInfo;

        let core_type = match HybridInfo::get_core_type(self) {
            Some(v) => v,
            None => return "".to_string(),
        };

        let native_model_id = HybridInfo::get_native_model_id(self);

        format!("[Type: {core_type}, Model: {native_model_id}]")
    }

    fn v2_ext_topo_intel_1fh(&self) -> String {
        let topo = libcpuid_dump::IntelExtTopo::from(self);

        format!("[LevelType: {}, num: {}]", topo.level_type, topo.num_proc)
    }
}
