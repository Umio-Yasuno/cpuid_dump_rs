use crate::*;

pub fn clock_speed_intel_00_16h(tmp: CpuidResult) {
    print!(" [{}/{}/{} MHz]",
        tmp.eax & 0xFFFF,
        tmp.ebx & 0xFFFF,
        tmp.ecx & 0xFFFF
    );
}

pub fn intel_hybrid_1ah(eax: u32) {
    let core_type = format!("{}",
        match eax >> 24 {
            0x20 => "Atom",
            0x40 => "Core",
            _    => "",
        }
    );

    if core_type.len() != 0 {
        print!(" [{}]", core_type);
    }
}
