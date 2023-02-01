use crate::{CpuidResult, CpuVendor, RawCpuid, Vendor};

pub(crate) fn parse_aida64(log: &String) -> (Vec<RawCpuid>, CpuVendor) {
    let mut rawcpuid_pool: Vec<RawCpuid> = Vec::new();
    let mut pre_leaf = u32::MAX;
    let mut sub_leaf = 0u32;
    let mut cpu_vendor = CpuVendor::from(&Vendor {
        ebx: 0x0,
        ecx: 0x0,
        edx: 0x0,
    }); // dummy

    for line in log.lines() {
        if line.starts_with("L1") {
            break;
        }
        if !line.starts_with("CPUID") {
            continue;
        }

        let split: Vec<&str> = line.splitn(4, ' ').collect();
        let leaf = u32::from_str_radix(&split[1][..8], 16).unwrap();
        let result = parse_reg(split[2]);

        if pre_leaf == leaf {
            sub_leaf += 1;
        } else {
            sub_leaf = 0;
        }

        pre_leaf = leaf;

        if leaf == 0x0 {
            cpu_vendor = CpuVendor::from(&result);
        }

        let rawcpuid = RawCpuid {
            leaf,
            sub_leaf,
            result,
        };

        rawcpuid_pool.push(rawcpuid);
    }

    (rawcpuid_pool, cpu_vendor)
}

fn parse_reg(reg: &str) -> CpuidResult {
    let result: Vec<u32> = reg
        .splitn(4, '-')
        .map(|str_reg| u32::from_str_radix(str_reg, 16).unwrap())
        .collect();

    CpuidResult {
        eax: result[0],
        ebx: result[1],
        ecx: result[2],
        edx: result[3],
    }
}
