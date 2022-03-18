use crate::{_AX, cpuid, CpuidResult};

pub struct ProcName;

impl ProcName {
    pub fn dec_cpuid(cpuid: CpuidResult) -> Vec<u8> {
        /* u32 ([u8; 4]) * 4 (E{A,B,C,D}X) */
        let mut tmp = Vec::with_capacity(16);

        [cpuid.eax, cpuid.ebx, cpuid.ecx, cpuid.edx].iter().for_each(
            |&reg| tmp.extend(Self::dec_reg(reg))
        );

        return tmp;
    }
    pub fn dec_reg(reg: u32) -> Vec<u8> {
        reg.to_le_bytes().iter().map(
            /* replace from \u0000..\u001F (<Control>) to \u0020 (<Space>) */
            |&byte| if byte < 0x20 { 0x20 } else { byte }
        ).collect()
    }
    pub fn get_name() -> String {
        let mut name: Vec<u8> = Vec::with_capacity(48);

        for i in 0..=2u32 {
            name.extend(
                Self::dec_cpuid(cpuid!(_AX+0x2 + i, 0))
            );
        }

        return String::from_utf8(name).unwrap();
    }
    pub fn get_trim_name() -> String {
        Self::get_name()
            .trim_end()
            .to_string()
    }
}

#[test]
fn test_proc_name() {
    println!("Processor Name       : [{}]", ProcName::get_name());
    println!("Processor Name (trim): [{}]", ProcName::get_trim_name());
}
