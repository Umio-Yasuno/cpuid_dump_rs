//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::CpuidResult;
use std::io;

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

/*
#[path = "./load_file.rs"]
mod load_file;
pub use crate::load_file::*;
*/

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
    /* CPUID[Leaf=0x7, SubLeaf=0x0].EAX, StructExtFeatIdMax */
    let leaf_07h_subc = RawCpuid::exe(0x7, 0x0).result.eax;
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
                for sub_leaf in 0x0..=leaf_07h_subc {
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
            _ => leaf_pool.push((leaf, 0x0)),
        }
    }

    leaf_pool
}

const LEAF_HEAD: &str = "       [Leaf.Sub]";
const LEAF_LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; LEAF_HEAD.len()]) };

fn hex_head() -> String {
    const EAX: &str = "  [EAX]   ";
    const EBX: &str = "  [EBX]   ";
    const ECX: &str = "  [ECX]   ";
    const EDX: &str = "  [EDX]   ";
    const LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; TOTAL_WIDTH]) };

    format!("\
        {LEAF_HEAD}  {EAX} {EBX} {ECX} {EDX}\n\
        {LINE}\
    \n")
}

fn bin_head() -> String {
    const OUTPUT_LEN: usize = 35; // 32 [bits] + '_' * 3
    const PAD_LEN: usize = (OUTPUT_LEN - "[EAX / ECX]".len()) / 2;

    const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; PAD_LEN-1]) };
    const OUTPUT_LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; OUTPUT_LEN]) };

    format!("\
        {LEAF_HEAD}  {PAD} [EAX / ECX] {PAD} \
        {PAD}  [EBX / EDX]\n\
        {LEAF_LINE}  {OUTPUT_LINE}  {OUTPUT_LINE}\
    \n")
}

fn topo_info_head() -> String {
    use libcpuid_dump::TopoId;

    let topo_info = match TopoId::get_topo_info() {
        Some(topo) => topo,
        None => return "".to_string(),
    };

    let TopoId { pkg_id, core_id, smt_id, x2apic_id } = topo_info;

    format!("[\
        Pkg: {pkg_id:03}, \
        Core: {core_id:03}, \
        SMT: {smt_id:03}, \
        x2APIC: {x2apic_id:03}\
    ]\n")
}

fn topo_info_thread_id_head(thread_id: usize) -> String {
    use libcpuid_dump::TopoId;

    let topo_info = match TopoId::get_topo_info() {
        Some(topo) => topo,
        None => return format!("[Thread: {thread_id:03}]\n"),
    };

    let TopoId { pkg_id, core_id, smt_id, x2apic_id } = topo_info;

    format!("[\
        Pkg: {pkg_id:03}, \
        Core: {core_id:03}, \
        SMT: {smt_id:03}, \
        x2APIC: {x2apic_id:03}, \
        Thread: {thread_id:03}\
    ]\n")
}

fn dump_write(pool: &[u8]) -> io::Result<()> {
    use std::io::{Write, stdout};
    let mut out = stdout().lock();

    out.write_all(pool)?;
    Ok(())
}

fn default_name() -> String {
    let proc_name = libcpuid_dump::ProcName::get_trim_name().replace(' ', "_");
    /* Family, Model, Stepping */
    let fms = cpuid!(0x1, 0x0).eax;

    /* like "AMD_Ryzen_5_5600G_with_Radeon_Graphics_00A50F00.txt" */
    format!("{proc_name}_{fms:08X}.txt")
}

fn help_msg() {
    const MSG: &str = concat!(
        "CPUID Dump ", env!("CARGO_PKG_VERSION"), "\n",
        "https://github.com/Umio-Yasuno/cpuid_dump_rs\n",
        "\n",
        "USAGE:\n",
        "    cargo run -- [options ..] or <cpuid_dump> [options ..]\n",
        "\n",
        "FLAGS:\n",
        "    -a, -all\n",
        "        Display result for all threads.\n",
        "    -r, -raw\n",
        "        Display raw/hex result.\n",
        "    -bin\n",
        "        Display binary result.\n",
        "    -c, -compat\n",
        "        Display the same format as `cpuid -r` (cpuid by Todd Allen)\n",
        "    -full\n",
        "        Combine \"-disp-zero\" and \"-no-diff\"\n",
        "    -disp-zero\n",
        "        Display result even if E[ABCD]X are zero.\n",
        "    -no-diff\n",
        "        Do not omit diff when all threads execution\n",
        "\n",
        "OPTIONS:\n",
        "    --l <u32>, --leaf <u32>\n",
        "        Display result only for the specified value, the value is Leaf/InputEAX <u32>.\n",
        "        e.g. --leaf 1, --leaf 0x8000_0008,\n",
        "    --sub_leaf <u32>, --subleaf <u32>\n",
        "        Display result only for the specified value, the value is Sub_Leaf/InputECX <u32>.\n",
        "    --s <path/filename>, --save <path/filename>\n",
        "        Save dump result to text file.\n",
        "        If there is no path/filename argument, will be used \"./<processor_name>\".",
    );

    println!("{MSG}")
}

#[derive(Debug, Clone)]
enum DumpFormat {
    Raw,
    Binary,
    Parse,
    CompatCpuid,
    Debug,
}

#[derive(Debug, Clone)]
struct MainOpt {
    fmt: DumpFormat,
    dump_all: bool,
    save_path: Option<String>,
    leaf: Option<(u32, u32)>,
    skip_zero: bool,
    diff: bool,
}

impl Default for MainOpt {
    fn default() -> Self {
        Self {
            fmt: DumpFormat::Parse,
            dump_all: false,
            save_path: None,
            leaf: None,
            skip_zero: true,
            diff: true,
        }
    }
}

impl MainOpt {
    fn parse_value(raw_value: &str) -> u32 {
        /* for like "0x8000_0000" */
        let raw_value = raw_value.replace('_', "");

        if let Some(stripped) = raw_value.strip_prefix("0x") {
            u32::from_str_radix(stripped, 16).unwrap()
        } else {
            raw_value.parse::<u32>().unwrap()
        }
    }

    fn main_parse() -> Self {
        let mut opt = MainOpt::default();
        let mut skip = false;

        let args: Vec<String> = std::env::args().collect();

        for (idx, arg) in args.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            if !arg.starts_with('-') {
                // eprintln!("Unknown option: {}", args[i]);
                continue;
            }

            let arg = arg.trim_start_matches('-');

            match arg {
                "a" | "all" => {
                    opt.dump_all = true;
                },
                "r" | "raw" => {
                    opt.fmt = DumpFormat::Raw;
                    // opt.skip_zero = false;
                },
                "s" | "save" => {
                    use std::path::Path;
                    let mut path = default_name();

                    if let Some(v) = args.get(idx+1) {
                        if v.starts_with('-') { 
                            opt.save_path = Some(path);
                            continue;
                        }

                        path = if Path::new(v).is_dir() {
                            format!("{v}{path}")
                        } else {
                            v.to_string()
                        };
                    }

                    opt.save_path = Some(path);
                },
                "leaf" => {
                    if let Some(v) = args.get(idx+1) {
                        let leaf = Self::parse_value(v);
                        opt.leaf = Some((leaf, 0x0));
                    } else {
                        eprintln!("missing argument <u32> to \"--leaf\"");
                    };
                },
                "subleaf" | "sub_leaf" | "sub-leaf" => {
                    if let (Some((leaf, _)), Some(sub_leaf)) = (opt.leaf, args.get(idx+1)) {
                        let sub_leaf = Self::parse_value(sub_leaf);
                        opt.leaf = Some((leaf, sub_leaf));
                    } else {
                        eprintln!("missing argument \"--sub_leaf <u32>\"");
                    };
                }
                "bin" => {
                    opt.fmt = DumpFormat::Binary;
                },
                "c" | "compat" => {
                    opt.dump_all = true;
                    opt.fmt = DumpFormat::CompatCpuid;
                    opt.skip_zero = false;
                    opt.diff = false;
                },
                "debug" => {
                    opt.fmt = DumpFormat::Debug
                },
                "h" | "help" => {
                    help_msg();
                    std::process::exit(0);
                },
                "disp-zero" => {
                    opt.skip_zero = false;
                },
                "no-diff" => {
                    opt.diff = false;
                },
                "full" => {
                    opt.skip_zero = false;
                    opt.diff = false;
                },
                _ => eprintln!("Unknown option: {}", arg),
            }
        }

        opt
    }

    fn rawcpuid_pool(&self, leaf_pool: &[(u32, u32)]) -> Vec<RawCpuid> {
        let mut cpuid_pool: Vec<RawCpuid> = Vec::with_capacity(leaf_pool.len());

        for (leaf, sub_leaf) in leaf_pool {
            let cpuid = RawCpuid::exe(*leaf, *sub_leaf);

            if self.skip_zero && cpuid.check_result_zero() {
                continue;
            }

            cpuid_pool.push(cpuid)
        }

        cpuid_pool
    }

    fn thread_id_head(&self, thread_id: usize) -> String {
        match self.fmt {
            DumpFormat::CompatCpuid => format!("CPU {thread_id}:\n"),
            _ => topo_info_thread_id_head(thread_id),
        }
    }

    fn head_fmt(&self) -> String {
        match self.fmt {
            DumpFormat::Binary => bin_head(),
            DumpFormat::Debug |
            DumpFormat::CompatCpuid => "".to_string(),
            _ => hex_head(),
        }
    }

    fn select_pool(&self, rawcpuid_pool: &[RawCpuid]) -> Vec<u8> {
        let len = rawcpuid_pool.len();
        let (cap, fmt_func): (usize, fn(&RawCpuid, &CpuVendor) -> String) = match self.fmt {
            DumpFormat::Raw => (
                len * TOTAL_WIDTH,
                RawCpuid::raw_fmt
            ),
            DumpFormat::Binary => (
                len * TOTAL_WIDTH * 2,
                RawCpuid::bin_fmt
            ),
            DumpFormat::Parse => (
                len * TOTAL_WIDTH * 3,
                RawCpuid::parse_fmt
            ),
            DumpFormat::CompatCpuid => (
                len * TOTAL_WIDTH,
                RawCpuid::compat_fmt,
            ),
            DumpFormat::Debug => (
                len * TOTAL_WIDTH * 2,
                RawCpuid::debug_fmt,
            ),
        };

        let mut parse_pool: Vec<u8> = Vec::with_capacity(cap);
        let vendor = CpuVendor::get();

        for rawcpuid in rawcpuid_pool {
            parse_pool.extend(fmt_func(rawcpuid, &vendor).into_bytes())
        }

        parse_pool
    }

    fn pool_all_thread(&self) -> Vec<u8> {
        use std::thread;
        use std::sync::Arc;
        use libcpuid_dump::util;

        let opt = Arc::new(self.clone());
        let leaf_pool = Arc::new(leaf_pool());
        let cpu_list = util::cpu_set_list().unwrap();
        /* this with_capacity is experiental */
        let mut main_pool = Vec::<u8>::with_capacity( if opt.diff {
            TOTAL_WIDTH * leaf_pool.len() * cpu_list.len() / 2
        } else {
            TOTAL_WIDTH * leaf_pool.len() * cpu_list.len() * 2
        });
        let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(cpu_list.len());

        let (first_pool, topo_head) = {
            /* To confine the effects of pin_thread */
            thread::scope(|s| s.spawn(|| {
                let cpu = cpu_list[0];
                util::pin_thread(cpu).unwrap();

                let topo_head = opt.thread_id_head(cpu);

                (
                    Arc::new(opt.rawcpuid_pool(&leaf_pool)),
                    topo_head.into_bytes(),
                )
            }).join().unwrap())
        };

        main_pool.extend(topo_head);
        main_pool.extend(opt.head_fmt().into_bytes());
        main_pool.extend(opt.select_pool(&first_pool));

        for cpu in &cpu_list[1..] {
            let cpu = *cpu;
            let opt = Arc::clone(&opt);
            let leaf_pool = Arc::clone(&leaf_pool);
            let first_pool = Arc::clone(&first_pool);

            handles.push(thread::spawn(move || {
                util::pin_thread(cpu).unwrap();

                let diff = {
                    let mut sub_pool = opt.rawcpuid_pool(&leaf_pool);

                    if opt.diff {
                        let mut first_pool = first_pool.iter();
                        sub_pool.retain(|sub| first_pool.next().unwrap() != sub );
                    }

                    sub_pool
                };

                let topo_head = opt.thread_id_head(cpu);

                [
                    topo_head.into_bytes(),
                    opt.select_pool(&diff),
                ].concat()
            }));
        }

        for h in handles {
            let v = h.join().unwrap();
            main_pool.extend(v);
        }

        main_pool
    }

    fn dump_pool(&self) -> Vec<u8> {
        if self.dump_all {
            return self.pool_all_thread();
        }

        let rawcpuid_pool = self.rawcpuid_pool(&leaf_pool());

        [
            topo_info_head().into_bytes(),
            self.head_fmt().into_bytes(),
            self.select_pool(&rawcpuid_pool),
        ].concat()
    }

    fn only_leaf(&self, leaf: u32, sub_leaf: u32) -> io::Result<()> {
        let raw_result = RawCpuid::exe(leaf, sub_leaf);
        let vendor = CpuVendor::get();
        let dump_fmt = match self.fmt {
            DumpFormat::Raw => RawCpuid::raw_fmt,
            DumpFormat::Binary => RawCpuid::bin_fmt,
            DumpFormat::Parse => RawCpuid::parse_fmt,
            DumpFormat::CompatCpuid => RawCpuid::compat_fmt,
            DumpFormat::Debug => RawCpuid::debug_fmt,
        };

        let tmp = [
            topo_info_head(),
            self.head_fmt(),
            dump_fmt(&raw_result, &vendor),
        ]
        .concat()
        .into_bytes();

        dump_write(&tmp)?;

        Ok(())
    }

    fn save_file(&self, save_path: &String) -> io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let pool = self.dump_pool();

        let mut f = File::create(save_path)?;

        f.write_all(&pool)?;
        println!("Output to \"{save_path}\"");

        Ok(())
    }

    fn run(&self) {
        match self {
            Self { leaf: Some(leaf), .. } => {
                self.only_leaf(leaf.0, leaf.1).expect("faild only_leaf")
            },
            Self { save_path: Some(path), .. } => {
                self.save_file(path).expect("faild save_file")
            },
            _ => {
                dump_write(&self.dump_pool()).expect("faild dump_write")
            },
        }
    }
}

fn main() {
    MainOpt::main_parse().run();
}
