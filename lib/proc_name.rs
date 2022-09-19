use crate::{_AX, cpuid, CpuidResult};

pub struct ProcName;

impl ProcName {
    pub(crate) fn check_reg(reg: u32) -> [u8; 4] {
        let mut bytes = reg.to_le_bytes();

        for b in bytes.iter_mut() {
            /* replace from <Control> to \u0020 (<Space>) */
            if char::from(*b).is_control() {
                *b = 0x20
            }
        }

        bytes
    }

    pub fn dec_cpuid(cpuid: &CpuidResult) -> Vec<u8> {
        [
            cpuid.eax,
            cpuid.ebx,
            cpuid.ecx,
            cpuid.edx,
        ]
        .iter()
        .flat_map(|&reg| Self::check_reg(reg))
        .collect()
    }
    
    fn set_cpuid() -> [CpuidResult; 3] {
        [
            cpuid!(_AX+0x2, 0x0),
            cpuid!(_AX+0x3, 0x0),
            cpuid!(_AX+0x4, 0x0),
        ]
    }
    
    pub fn from_cpuid_array(array: [CpuidResult; 3]) -> String {
        /* 4 (0x8000_0002 .. 0x8000_0004) * u32 ([u8; 4]) * 4 (E{A,B,C,D}X) */
        let name = array.iter().flat_map(|cpuid| Self::dec_cpuid(cpuid)).collect();

        String::from_utf8(name).unwrap()
    }
    
    pub fn get_name() -> String {
        let cpuid = Self::set_cpuid();
        Self::from_cpuid_array(cpuid)
    }
    
    pub fn get_trim_name() -> String {
        Self::get_name()
            .trim_end()
            .to_string()
    }
}

#[test]
fn test_proc_name() {
    let cpuid = [
        CpuidResult { eax: 0x20444D41, ebx: 0x657A7952, ecx: 0x2035206E, edx: 0x30303635 },
        CpuidResult { eax: 0x69772047, ebx: 0x52206874, ecx: 0x6F656461, edx: 0x7247206E },
        CpuidResult { eax: 0x69687061, ebx: 0x20207363, ecx: 0x20202020, edx: 0x00202020 },
    ];
    let name = "AMD Ryzen 5 5600G with Radeon Graphics          ".to_string();

    assert_eq!(name, ProcName::from_cpuid_array(cpuid));

    /*
    println!("Processor Name       : [{}]", ProcName::get_name());
    println!("Processor Name (trim): [{}]", ProcName::get_trim_name());
    */
}
