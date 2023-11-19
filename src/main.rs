//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::CpuidResult;

use libcpuid_dump::TopoId;
use libcpuid_dump::{cpuid, CpuVendor};

pub const INPUT_WIDTH: usize = "  0x00000000 0x0:  ".len();
pub const OUTPUT_WIDTH: usize = "0x00000000 ".len() * 4;
pub const TOTAL_WIDTH: usize = 100;
pub const PARSE_WIDTH: usize = TOTAL_WIDTH - INPUT_WIDTH - OUTPUT_WIDTH - 1; // " ".len()
// pub const VERSION_HEAD: &str = concat!("CPUID Dump ", env!("CARGO_PKG_VERSION"), "\n");

mod raw_cpuid;
pub use raw_cpuid::*;

mod parse;
pub use parse::*;

mod args;
use args::*;

mod load_aida64_log;

/// Main flow:
///    pub struct RawCpuid {
///        pub leaf: u32,
///        pub sub_leaf: u32,
///        // https://doc.rust-lang.org/core/arch/x86_64/struct.CpuidResult.html
///        pub result: CpuidResult {
///            pub eax: u32,
///            pub ebx: u32,
///            pub ecx: u32,
///            pub edx: u32,
///        },
///    } 
///    // src/main.rs
///    MainOpt::parse() -> MainOpt
///            |
///    opt.rawcpuid_pool(&leaf_pool()) -> Vec<RawCpuid>
///            |
///    // src/raw_cpuid.rs
///    let parsed_pool: Vec<u8>;
///    cpuid_parse: {
///        for raw_cpuid in cpuid_pool {
///            // src/raw_cpuid.rs, src/parse/*
///            let cpuid_parsed: String = raw_cpuid.parse();
///            parsed_pool.extend(cpuid_parsed.into_bytes());
///        }
///    }
///            |
///    // src/main.rs
///    dump_write(&parsed_pool) // print, write stdout
///    

fn leaf_pool() -> Vec<(u32, u32)> {
    let mut leaf_pool: Vec<(u32, u32)> = Vec::with_capacity(64);

    /* LFuncStd: largest standard function */
    let max_std_leaf = RawCpuid::exe(0x0, 0x0).result.eax;
    /* LFuncExt: largest extended function */
    let max_ext_leaf = RawCpuid::exe(0x8000_0000, 0x0).result.eax;

    /* Base */
    for leaf in 0x0..=max_std_leaf {
        match leaf {
            /* Cache Properties, Intel */
            0x4 => for sub_leaf in 0x0..=0x4 {
                leaf_pool.push((leaf, sub_leaf))
            },
            0x7 => {
                /* CPUID[Leaf=0x7, SubLeaf=0x0].EAX, StructExtFeatIdMax */
                let max_sub_leaf = RawCpuid::exe(0x7, 0x0).result.eax;

                for sub_leaf in 0x0..=max_sub_leaf {
                    leaf_pool.push((leaf, sub_leaf))
                }
            },
            /*  Extended Topology Enumeration, Intel, AMD Zen 2 <=
                SMT_LEVEL = 0,
                CORE_LEVEL = 1,
            */
            0xB => for sub_leaf in 0x0..=0x1 {
                leaf_pool.push((leaf, sub_leaf))
            },
            /* 0xD: Processor Extended State Enumeration */
            0xD => for sub_leaf in 0x0..0xF {
                leaf_pool.push((leaf, sub_leaf))
            },
            /* 0x18: Deterministic Address Translation Parameters, Intel */
            0x18 => {
                let max_sub_leaf = RawCpuid::exe(0x18, 0x0).result.eax;

                for sub_leaf in 0x0..max_sub_leaf {
                    leaf_pool.push((leaf, sub_leaf))
                }
            },
            /* 0x1F: V2 Extended Topology Enumeration Leaf, Intel */
            0x1F => for sub_leaf in 0x0..=0x4 {
                leaf_pool.push((0x1F, sub_leaf))
            },
            _ => leaf_pool.push((leaf, 0x0)),
        }
    }

    /* Ext */
    for leaf in 0x8000_0000..=max_ext_leaf {
        match leaf {
            /* Cache Properties, AMD, same format as Intel Leaf 0x4 */
            0x8000_001D => for sub_leaf in 0x0..=0x4 {
                leaf_pool.push((leaf, sub_leaf))
            },
            /* AMD Platform QoS Enforcement for Memory Bandwidth */
            0x8000_0020 => for sub_leaf in 0x0..=0x1 {
                leaf_pool.push((leaf, sub_leaf))
            },
            /* AMD Extended CPU Topology */
            0x8000_0026 => for sub_leaf in 0x0..=0x4 {
                leaf_pool.push((leaf, sub_leaf))
            },
            _ => leaf_pool.push((leaf, 0x0)),
        }
    }

    leaf_pool
}

#[derive(Debug, Clone)]
struct CpuidDump {
    pub cpu_vendor: CpuVendor,
    pub rawcpuid_pool: Vec<RawCpuid>,
    pub topo_id: Option<TopoId>,
    pub thread_id: Option<usize>,
}

impl CpuidDump {
    fn new(leaf_pool: &[(u32, u32)], skip_zero: bool) -> Self {
        let cpu_vendor = CpuVendor::get();

        let rawcpuid_pool = leaf_pool.iter().filter_map(|(leaf, sub_leaf)| {
            let rawcpuid = RawCpuid::exe(*leaf, *sub_leaf);
            
            if skip_zero && rawcpuid.check_result_zero() {
                None
            } else {
                Some(rawcpuid)
            }
        }).collect();
        let topo_id = TopoId::get_topo_info();

        Self {
            cpu_vendor,
            rawcpuid_pool,
            topo_id,
            thread_id: None,
        }
    }

    fn new_with_thread_id(leaf_pool: &[(u32, u32)], skip_zero: bool, thread_id: usize) -> Self {
        let mut tmp = Self::new(leaf_pool, skip_zero);
        tmp.thread_id = Some(thread_id);

        tmp
    }

    fn top_disp(&self, dump_fmt: DumpFormat) -> String {
        [
            self.topo_info_head(),
            dump_fmt.head_fmt(),
            self.select_pool(dump_fmt),
        ].concat()
    }

    fn disp(&self, dump_fmt: DumpFormat) -> String {
        [
            self.topo_info_head(),
            // dump_fmt.head_fmt(),
            self.select_pool(dump_fmt),
        ].concat()
    }

    fn select_pool(&self, dump_fmt: DumpFormat) -> String {
        let fmt_func = dump_fmt.rawcpuid_fmt_func();

        self.rawcpuid_pool
            .iter()
            .map(|rawcpuid| fmt_func(rawcpuid, &self.cpu_vendor))
            .collect()
    }

    fn topo_info_head(&self) -> String {
        match (&self.topo_id, &self.thread_id) {
            (Some(topo), Some(thread_id)) => {
                let TopoId { pkg_id, core_id, smt_id, x2apic_id } = topo;

                format!("\n[\
                    Pkg: {pkg_id:03}, \
                    Core: {core_id:03}, \
                    SMT: {smt_id:03}, \
                    x2APIC: {x2apic_id:03}, \
                    Thread: {thread_id:03}\
                ]\n")
            },
            (Some(topo), None) => {
                let TopoId { pkg_id, core_id, smt_id, x2apic_id } = topo;

                format!("\n[\
                    Pkg: {pkg_id:03}, \
                    Core: {core_id:03}, \
                    SMT: {smt_id:03}, \
                    x2APIC: {x2apic_id:03}\
                ]\n")
            },
            (_, Some(thread_id)) => format!("[Thread: {thread_id:03}]\n"),
            (_, _) => String::new(),
        }
    }
}

fn dump_all_threads(
    leaf_pool: &[(u32, u32)],
    skip_zero: bool,
    dump_fmt: DumpFormat,
    diff: bool,
) -> String {
    use std::thread;
    use std::sync::Arc;
    use libcpuid_dump::util;

    let leaf_pool = Arc::from(leaf_pool);
    let cpu_list = util::cpu_set_list().unwrap();
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(cpu_list.len());

    let first = {
        /* To confine the effects of pin_thread */
        thread::scope(|s| s.spawn(|| {
            let cpu = cpu_list[0];
            util::pin_thread(cpu).unwrap();

            Arc::new(CpuidDump::new_with_thread_id(&leaf_pool, skip_zero, cpu))
        }).join().unwrap())
    };

    for cpu in &cpu_list[1..] {
        let cpu = *cpu;
        let leaf_pool = Arc::clone(&leaf_pool);
        let first = Arc::clone(&first);

        handles.push(thread::spawn(move || {
            util::pin_thread(cpu).unwrap();

            let cpuid_dump = {
                let mut sub = CpuidDump::new_with_thread_id(&leaf_pool, skip_zero, cpu);

                if diff {
                    let mut first = first.rawcpuid_pool.iter();
                    sub.rawcpuid_pool.retain(|sub| first.next().unwrap() != sub );
                }

                sub
            };

            cpuid_dump
        }));
    }

    let s = first.top_disp(dump_fmt);
    let ss: String = handles.into_iter().filter_map(|h| {
        let cpuid_dump = h.join().ok()?;
        Some(cpuid_dump.disp(dump_fmt))
    }).collect();

    format!("{s}{ss}")
}

fn main() {
    let opt = MainOpt::main_parse();

    match opt {
        MainOpt { leaf: Some(leaf), .. } => {
            opt.only_leaf(leaf.0, leaf.1).expect("faild only_leaf")
        },
        MainOpt { save_path: Some(ref path), .. } => {
            opt.save_file(path).expect("faild save_file")
        },
        MainOpt { load_aida64: Some(ref path), .. } => {
            opt.load_aida64(path).expect("faild load_aida64")
        },
        _ => {
            dump_write(&opt.dump_pool()).expect("faild dump_write")
        },
    }

}
