use crate::*;

pub trait ParseIntel {
    fn clock_speed_intel_00_16h(&self) -> String;
    fn intel_hybrid_1ah(&self) -> String;
    fn v2_ext_topo_intel_1fh(&self) -> String;
}

impl ParseIntel for CpuidResult {
    fn clock_speed_intel_00_16h(&self) -> String {
        format!(" [{}/{}/{} MHz]",
            self.eax & 0xFFFF,
            self.ebx & 0xFFFF,
            self.ecx & 0xFFFF
        )
    }

    fn intel_hybrid_1ah(&self) -> String {
        let (core_type, native_model_id) =
            libcpuid_dump::HybridInfo::get_hybrid_info_from_cpuid(*self);

        let core_type = match core_type {
            Some(v) => v,
            None => return "".to_string(),
        };

        return format!(" [{core_type} (0x{native_model_id:x})]");
    }

    fn v2_ext_topo_intel_1fh(&self) -> String {
        let topo = libcpuid_dump::IntelExtTopo::from_cpuid(self);

        return [
            format!(" [{}]", topo.level_type.to_string()),
            lnpad!(),
            format!(" [x2apic id: {}]", topo.x2apic_id),
        ].concat();
    }
}
