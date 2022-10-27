use crate::*;

pub trait ParseIntel {
    fn clock_speed_intel_00_16h(&self) -> String;
    fn intel_hybrid_1ah(&self) -> String;
    fn v2_ext_topo_intel_1fh(&self) -> String;
}

impl ParseIntel for CpuidResult {
    fn clock_speed_intel_00_16h(&self) -> String {
        format!("[{}/{}/{} MHz]",
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

        format!("[{core_type} ({native_model_id:#x})]")
    }

    fn v2_ext_topo_intel_1fh(&self) -> String {
        let topo = libcpuid_dump::IntelExtTopo::from_cpuid(self);

        [
            format!("[{}]", topo.level_type),
            lnpad!(),
            format!("[x2apic id: {}]", topo.x2apic_id),
        ].concat()
    }
}
