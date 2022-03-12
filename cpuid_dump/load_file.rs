use crate::*;

#[derive(Debug)]
enum RegType {
    EAX = 0,
    EBX = 1,
    ECX = 2,
    EDX = 3,
}

impl RegType {
    fn step(&mut self) {
        *self = match self {
            // TODO: use add?
            Self::EAX => Self::EBX,
            Self::EBX => Self::ECX,
            Self::ECX => Self::EDX,
            Self::EDX => Self::EAX,
        }
    }
}

pub fn load_file(load_path: String) {
    use std::fs;
    // println!("path: {}", load_path);
    let v = fs::read_to_string(load_path)
        .expect("Load error");
    let mut pool: Vec<u8> = Vec::new();

    pool.extend(
        format!("   (in)EAX_xECX:  {:<10} {:<10} {:<10} {:<10}\n",
            "(out)EAX", "(out)EBX", "(out)ECX", "(out)EDX").into_bytes()
    );
    pool.extend(
        format!("{}\n", "=".repeat(TOTAL_WIDTH)).into_bytes()
    );

    let vendor_flag = VendorFlag::all_true();


    for ln in v.lines() {
        let raw_cpuid_set = load_cpuid_dump_rs(ln);
        if !raw_cpuid_set.check_all_zero() {
            pool.extend(
                raw_cpuid_set
                    .parse_fmt(&vendor_flag)
                    .into_bytes()
            );
        }
    }

    dump_write(&pool);
}

fn load_cpuid_dump_rs(line: &str) -> RawCpuid {

    let mut raw = RawCpuid::zero();
    let mut cpuid = CpuidResult { eax: 0x0, ebx: 0x0, ecx: 0x0, edx: 0x0 };
    let mut reg_type = RegType::EAX;

    // for ignore parsed strings
    const LEN: usize = INPUT_WIDTH + OUTPUT_WIDTH;

    let line = match line.get(..LEN) {
        Some(v) => v,
        _ => {
            panic!("Load error: Possibly not the result file of cpuid_dump_rs")
        },
    };

    for v in line.split(' ') {
        match v.len() {
            // "0x80000000".len()
            10 => {
                //println!("Reg: {:?} [{v}]", reg_type);
                let v = u32::from_str_radix(&v[2..], 16).unwrap();
                match reg_type {
                    RegType::EAX => cpuid.eax = v,
                    RegType::EBX => cpuid.ebx = v,
                    RegType::ECX => cpuid.ecx = v,
                    RegType::EDX => {
                        cpuid.edx = v;
                        raw.result = cpuid;
                        continue;
                    },
                }
                reg_type.step();
            },
            // "0x8000001F_x0:".len()
            14 => {
                // Base: "0x8000001F_x0:"
                // to split: ("0x8000001F", "_x0:")
                let (leaf, sub_leaf) = v.split_at(10);

                // &leaf[2..]: "8000001F" => 0x8000001F
                // &sub_leaf[2..3]: "0" => 0x0
                let leaf = u32::from_str_radix(&leaf[2..], 16).unwrap();
                let sub_leaf = u32::from_str_radix(&sub_leaf[2..3], 16).unwrap();
                // println!("LEAFv2: 0x{:08x}, SUBv2: 0x{:08x}", leaf, sub_leaf);
                raw.leaf = leaf;
                raw.sub_leaf = sub_leaf;
            }
            _ => {},
        }
        // println!("SPLIT: \"{}\", LEN: {}", v, v.len());
    }

    return raw;
}
