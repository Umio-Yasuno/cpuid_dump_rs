use crate::{CpuidDump, CpuidResult, CpuVendor, RawCpuid};
use libcpuid_dump::{TopoLevelType, TopoId};

pub(crate) fn parse_aida64(log: &String) -> Vec<CpuidDump> {
    let mut rawcpuid_pool: Vec<RawCpuid> = Vec::new();
    let mut pre_leaf = u32::MAX;
    let mut sub_leaf = 0u32;
    let mut cpu_vendor: Option<CpuVendor> = None;
    let [mut smt_cpuid, mut core_cpuid]: [Option<CpuidResult>; 2] = [None, None];
    let mut vec_cpuid_dump = Vec::new();

    for line in log.lines().skip(1) {
        if line.starts_with("Group:") || line.starts_with("------[ CPUID Registers") {
            let Some(cpu_vendor) = cpu_vendor else { continue };
            let topo_id = if let [Some(smt_cpuid), Some(core_cpuid)] = [smt_cpuid, core_cpuid] {
                Some(TopoId::get_topo_info_with_smt_core_cpuid(&smt_cpuid, &core_cpuid))
            } else {
                None
            };

            vec_cpuid_dump.push(CpuidDump {
                cpu_vendor,
                rawcpuid_pool: rawcpuid_pool.clone(),
                topo_id,
                thread_id: None,
            });

            rawcpuid_pool.clear();
            smt_cpuid = None;
            core_cpuid = None;
        }
        if line == "------[ All CPUs ]------" {
            break;
        }
        if !line.starts_with("CPUID") {
            continue;
        }

        let split: Vec<&str> = line.splitn(4, ' ').collect();
        let Ok(leaf) = u32::from_str_radix(&split[1][..8], 16) else { continue };
        let Some(result) = parse_reg(split[2]) else { continue };

        if pre_leaf == leaf {
            sub_leaf += 1;
        } else {
            sub_leaf = 0;
        }

        pre_leaf = leaf;

        if leaf == 0x0 {
            cpu_vendor = Some(CpuVendor::from(&result));
        }

        if leaf == 0xB {
            let level = TopoLevelType::from(&result);

            if level == TopoLevelType::SMT {
                smt_cpuid = Some(result);
            } else if level == TopoLevelType::Core {
                core_cpuid = Some(result);
            }
        }

        let rawcpuid = RawCpuid {
            leaf,
            sub_leaf,
            result,
        };

        rawcpuid_pool.push(rawcpuid);
    }

    vec_cpuid_dump
}

fn parse_reg(reg: &str) -> Option<CpuidResult> {
    let result: Vec<u32> = reg
        .splitn(4, '-')
        .filter_map(|str_reg| u32::from_str_radix(str_reg, 16).ok())
        .collect();

    if result.len() < 4 { return None }

    Some(CpuidResult {
        eax: result[0],
        ebx: result[1],
        ecx: result[2],
        edx: result[3],
    })
}
