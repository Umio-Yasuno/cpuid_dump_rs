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
        let eax = self.eax;

        let core_type = match eax >> 24 {
            0x10 => "Reserved_1",
            0x20 => "Atom",
            0x30 => "Reserved_2",
            0x40 => "Core",
            _    => return "".to_string(),
        };

        return format!(" [{}]", core_type);
    }

    fn v2_ext_topo_intel_1fh(&self) -> String {
        let topo = libcpuid_dump::IntelExtTopo::dec(self);

        return [
            format!(" [{}]", topo.level_type.to_string()),
            lnpad!(),
            format!(" [x2apic id: {}]", topo.x2apic_id),
        ].concat();
    }
}
