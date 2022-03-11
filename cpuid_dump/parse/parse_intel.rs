use crate::*;

pub fn clock_speed_intel_00_16h(cpuid: &CpuidResult) -> String {
    format!(" [{}/{}/{} MHz]",
        cpuid.eax & 0xFFFF,
        cpuid.ebx & 0xFFFF,
        cpuid.ecx & 0xFFFF
    )
}

pub fn intel_hybrid_1ah(eax: &u32) -> String {
    let core_type = match *eax >> 24 {
        0x10 => "Reserved 1",
        0x20 => "Atom",
        0x30 => "Reserved 2",
        0x40 => "Core",
        _    => "",
    }.to_string();

    return if core_type.len() != 0 {
        format!(" [{}]", core_type)
    } else {
        core_type.to_string()
    };
}

#[allow(dead_code)]
struct IntelExtTopo {
    //  x2apic_id: u32,
    level_number: u32,
    level_type: u32,
    level_type_string: String,
}

impl IntelExtTopo {
    fn dec(cpuid: &CpuidResult) -> IntelExtTopo {

        let level_number = cpuid.ecx & 0xFF;
        let level_type = (cpuid.ecx >> 8) & 0xFF;
        let level_type_string = match level_type {
            // 0x0 => "Invalid",
            0x1 => "SMT",
            0x2 => "Core",
            0x3 => "Module",
            0x4 => "Tile",
            0x5 => "Die",
            _ => "", /* Invalid or Reserved */
        }.to_string();

        IntelExtTopo {
            level_number,
            level_type,
            level_type_string,
        }
    }
}

pub fn v2_ext_topo_intel_1fh(cpuid: &CpuidResult) -> String {
    let topo = IntelExtTopo::dec(cpuid);
    return format!(" [{}]", topo.level_type_string);
}
