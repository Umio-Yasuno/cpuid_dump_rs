use crate::{_AX, cpuid, CpuidResult};

pub struct ProcName;

impl ProcName {
    pub fn dec_reg(reg: u32) -> Vec<u8> {
        reg.to_le_bytes().iter().map(
            /* replace from <Control> to \u0020 (<Space>) */
            |&byte| if char::from(byte).is_control() { 0x20 } else { byte }
        ).collect()
    }
    pub fn dec_cpuid(cpuid: &CpuidResult) -> Vec<u8> {
        /* u32 ([u8; 4]) * 4 (E{A,B,C,D}X) */
        let mut tmp: Vec<u8> = Vec::with_capacity(16);

        [cpuid.eax, cpuid.ebx, cpuid.ecx, cpuid.edx].iter().for_each(
            |&reg| tmp.extend(Self::dec_reg(reg))
        );

        return tmp;
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
        let mut name: Vec<u8> = Vec::with_capacity(48);

        for cpuid in array {
            name.extend(Self::dec_cpuid(&cpuid));
        }

        return String::from_utf8(name).unwrap();
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
