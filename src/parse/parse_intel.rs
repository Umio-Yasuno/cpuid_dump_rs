use super::*;

pub trait ParseIntel {
    fn clock_speed_intel_00_16h(&self) -> String;
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
