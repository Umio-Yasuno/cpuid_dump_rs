//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::CpuidResult;

// extern crate libcpuid_dump;
use libcpuid_dump::{cpuid, VendorFlag, _AX};

pub const INPUT_WIDTH: usize = "  0x00000000_x0: ".len();
pub const OUTPUT_WIDTH: usize = "0x00000000 ".len() * 4;
pub const TOTAL_WIDTH: usize = 100;
pub const PARSE_WIDTH: usize = TOTAL_WIDTH - INPUT_WIDTH - OUTPUT_WIDTH - 1; // " ".len()
pub const VERSION_HEAD: &str = concat!("CPUID Dump ", env!("CARGO_PKG_VERSION"), "\n");

mod parse;
pub use crate::parse::*;

mod raw_cpuid;
pub use crate::raw_cpuid::*;

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
///    opt.rawcpuid_pool() -> Vec<RawCpuid>
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

    /* CPUID[Leaf=0x7, SubLeaf=0x0].EAX, StructExtFeatIdMax */
    let leaf_07h_subc = RawCpuid::exe(0x7, 0x0).result.eax;

    /* Base */
    for leaf in 0x0..=0xD {
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
            _ => leaf_pool.push((leaf, 0x0)),
        }
    }

    /* 0x1F: V2 Extended Topology Enumeration Leaf, Intel */
    for sub_leaf in 0x0..=0x4 {
        leaf_pool.push((0x1F, sub_leaf))
    }

    /* Ext */
    for leaf in _AX+0x0..=_AX+0xA {
        leaf_pool.push((leaf, 0x0))
    }

    for leaf in _AX+0x19..=_AX+0x21 {
        /* Cache Properties, AMD, same format as Intel Leaf 0x4 */
        const LF_80_1D: u32 = _AX + 0x1D;
        /* AMD Platform QoS Enforcement for Memory Bandwidth */
        const LF_80_20: u32 = _AX + 0x20;

        match leaf {
            LF_80_1D => for sub_leaf in 0x0..=0x4 {
                leaf_pool.push((leaf, sub_leaf))
            },
            LF_80_20 => for sub_leaf in 0x0..=0x1 {
                leaf_pool.push((leaf, sub_leaf))
            },
            _ => leaf_pool.push((leaf, 0x0)),
        }
    }

    return leaf_pool;
}

fn hex_head() -> String {
    const HEAD: &str = "  {LEAF}_x{SUB}:  (out)EAX   (out)EBX   (out)ECX   (out)EDX";
    const LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; TOTAL_WIDTH]) };

    format!("{HEAD}\n{LINE}\n")
}

fn bin_head() -> String {
    const INPUT_LEN: usize = 16;
    const OUTPUT_LEN: usize = 35;
    const PAD_LEN: usize = (OUTPUT_LEN - "(out)EAX / (out)ECX".len()) / 2;

    const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; PAD_LEN-1]) };
    const INPUT_LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; INPUT_LEN]) };
    const OUTPUT_LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; OUTPUT_LEN]) };

    [
        format!("  {{LEAF}}_x{{SUB}}:  {PAD} (out)EAX / (out)ECX {PAD} {PAD}  (out)EBX / (out)EDX\n"),
        format!("{INPUT_LINE}  {OUTPUT_LINE}  {OUTPUT_LINE}\n"),
    ].concat()
}

fn topo_info_head() -> String {
    let topo_info = match libcpuid_dump::TopoId::get_topo_info() {
        Some(topo) => topo,
        None => return "".to_string(),
    };

    let pkg_id = topo_info.pkg_id;
    let core_id = topo_info.core_id;
    let smt_id = topo_info.smt_id;
    // let x2apic_id = topo_info.x2apic_id;

    format!("[Pkg: {pkg_id:03}, Core: {core_id:03}, SMT: {smt_id:03}]\n")
}

fn topo_info_with_threadid_head(thread_id: usize) -> String {
    let topo_info = match libcpuid_dump::TopoId::get_topo_info() {
        Some(topo) => topo,
        None => return format!("[Thread: {thread_id:03}]\n"),
    };

    let pkg_id = topo_info.pkg_id;
    let core_id = topo_info.core_id;
    let smt_id = topo_info.smt_id;
    // let x2apic_id = topo_info.x2apic_id;

    format!("[Pkg: {pkg_id:03}, Core: {core_id:03}, SMT: {smt_id:03}, Thread: {thread_id:03}]\n")
}

fn dump_write(pool: &[u8]) {
    use std::io::{Write, stdout};
    let mut out = stdout().lock();

    out.write(pool).unwrap();
}

#[derive(Debug, Clone)]
struct SaveOpt {
    flag: bool,
    path: String,
}

#[derive(Debug, Clone)]
struct OnlyLeaf {
    flag: bool,
    leaf: u32,
    sub_leaf: u32,
}

#[derive(Debug, Clone)]
struct MainOpt {
    raw: bool,
    dump_all: bool,
    save: SaveOpt,
    // load: (bool, String),
    // only_leaf: (bool, u32, u32),
    only_leaf: OnlyLeaf,
    skip_zero: bool,
    diff: bool,
    bin_fmt: bool,
}

impl MainOpt {
    fn init() -> Self {
        Self {
            raw: false,
            dump_all: false,
            save: SaveOpt {
                flag: false,
                path: Self::init_name(),
            },
            // load: (false, "cpuid_dump.txt".to_string()),
            only_leaf: OnlyLeaf {
                flag: false,
                leaf: 0x0,
                sub_leaf: 0x0,
            },
            skip_zero: true,
            diff: true,
            bin_fmt: false,
        }
    }

    fn init_name() -> String {
        let proc_name = libcpuid_dump::ProcName::get_trim_name().replace(" ", "_");
        /* Family, Model, Stepping */
        let fms = cpuid!(0x1, 0x0).eax;

        /* like "AMD_Ryzen_5_5600G_with_Radeon_Graphics_00A50F00.txt" */
        format!("{proc_name}_{fms:08X}.txt")
    }

    fn parse_value(raw_value: String, msg: &str) -> u32 {
        let raw_value = raw_value.replace("_", "");

        if raw_value.starts_with("-") {
            eprintln!("Please the value of {msg} <u32>");
            return 0u32;
        }

        if raw_value.starts_with("0x") {
            u32::from_str_radix(&raw_value[2..], 16)
                .expect("Parse error: {msg} <u32>")
        } else {
            raw_value.parse::<u32>()
                .expect("Parse error: {msg} <u32>")
        }
    }

    fn help_msg() {
        print!("\n\
            {VERSION_HEAD}\
            https://github.com/Umio-Yasuno/cpuid_dump_rs\n\
            \n\
            USAGE:\n\
            \x20    cargo run -- [options ..] or <cpuid_dump> [options ..]\n\
            \n\
            FLAGS:\n\
            \x20    -a, -all\n\
            \x20        Display result for all threads.\n\
            \x20    -r, -raw\n\
            \x20        Display raw/hex result.\n\
            \x20    -bin\n\
            \x20        Display binary result.\n\
            \x20    -full\n\
            \x20        Combine \"-disp-zero\" and \"-no-diff\"\n\
            \x20    -disp-zero\n\
            \x20        Display result even if E[ABCD]X are zero.\n\
            \x20    -no-diff\n\
            \x20        Do not omit diff when all threads execution\n\
            \n\
            OPTIONS:\n\
            \x20    --l <u32>, --leaf <u32>\n\
            \x20        Display result only for the specified value, the value is Leaf/InputEAX <u32>.\n\
            \x20        e.g. --leaf 1, --leaf 0x8000_0008,\n\
            \x20    --sub_leaf <u32>, --subleaf <u32>\n\
            \x20        Display result only for the specified value, the value is Sub_Leaf/InputECX <u32>.\n\
            \x20    --pin <usize>, --pin_threads <usize>\n\
            \x20        Display result for the specified thread.\n\
            \x20    --s <path/filename>, --save <path/filename>\n\
            \x20        Save dump result to text file.\n\
            \x20        If there is no path/filename argument, will be used \"./<processor_name>\".
        \n");
    }

    fn main_parse() -> Self {
        let mut opt = MainOpt::init();
        let mut skip = false;

        let args: Vec<String> = std::env::args().collect();

        for (idx, arg) in args.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            if !arg.starts_with("-") {
                // eprintln!("Unknown option: {}", args[i]);
                continue;
            }

            let arg = arg.trim_start_matches("-");

            match arg {
                "a" | "all" => opt.dump_all = true,
                "r" | "raw" => {
                    opt.raw = true;
                    opt.skip_zero = false;
                },
                "s" | "save" => {
                    opt.save.flag = true;
                    opt.save.path = match args.get(idx+1) {
                        Some(v) => {
                            if v.starts_with("-") {
                                skip = true;
                                continue;
                            }

                            if std::path::Path::new(v).is_dir() {
                                format!("{}{}", v, opt.save.path)
                            } else {
                                v.to_string()
                            }
                        },
                        // use default path/file name
                        // save_path: format!("{}.txt",
                        //      libcpuid_dump::get_trim_proc_name().replace(" ", "_")
                        _ => continue,
                    };
                },
                /*
                "l" | "load" => {
                    opt.load.0 = true;
                    opt.load.1 = match args.get(idx+1) {
                        Some(v) => {
                            if v.starts_with("-") {
                                skip = true;
                                continue;
                            }

                            v.to_string()
                        },
                        _ => {
                            eprintln!("Please load path");
                            std::process::exit(1);
                        },
                    };
                },
                */
                "leaf" => {
                    opt.only_leaf.flag = true;
                    opt.only_leaf.leaf = match args.get(idx+1) {
                        Some(v) => Self::parse_value(v.to_string(), "leaf"),
                        _ => continue,
                    };
                },
                "subleaf" | "sub_leaf" | "sub-leaf" => {
                    if !opt.only_leaf.flag {
                        eprintln!("Please \"--leaf <u32>\" argument");
                        continue;
                    }
                    opt.only_leaf.sub_leaf = match args.get(idx+1) {
                        Some(v) => Self::parse_value(v.to_string(), "sub_leaf"),
                        _ => continue,
                    };
                }
                "bin" => {
                    opt.bin_fmt = true
                },
                "pin" | "pin_thread" => {
                    let cpu = match args.get(idx+1) {
                        Some(v) => {
                            v.parse::<usize>()
                                .expect("Parse error: pin/pin_thread")
                        },
                        _ => {
                            eprintln!("Please the value of pin/pin_thread <usize>");
                            continue;
                        },
                    };
                    libcpuid_dump::pin_thread(cpu).unwrap();
                },
                "h" | "help" => {
                    Self::help_msg();
                    std::process::exit(0);
                },
                "disp-zero" => {
                    opt.skip_zero = false
                },
                "no-diff" => {
                    opt.diff = false
                },
                "full" => {
                    opt.skip_zero = false;
                    opt.diff = false;
                },
                // TODO: "taskset" option?
                // cpuid_dump --taskset <list>
                // same `taskset -c <list> cpuid_dump -a`
                _ => eprintln!("Unknown option: {}", arg),
            }
        }

        return opt;
    }

    fn rawcpuid_pool(&self) -> Vec<RawCpuid> {
        let mut cpuid_pool: Vec<RawCpuid> = Vec::with_capacity(64);

        for (leaf, sub_leaf) in leaf_pool() {
            let cpuid = RawCpuid::exe(leaf, sub_leaf);

            if self.skip_zero && cpuid.check_result_zero() {
                continue;
            }

            cpuid_pool.push(cpuid)
        }

        return cpuid_pool;
    }

    fn head_fmt(&self) -> String {
        if self.bin_fmt {
            bin_head()
        } else {
            hex_head()
        }
    }

    fn only_leaf(&self) {
        let raw_result = RawCpuid::exe(self.only_leaf.leaf, self.only_leaf.sub_leaf);

        let tmp = if self.bin_fmt {
            [
                bin_head(),
                raw_result.bin_fmt(),
            ]
        } else {
            [
                hex_head(),
                raw_result.parse_fmt(&VendorFlag::check()),
            ]
        }.concat();

        return dump_write(&tmp.into_bytes());
    }

    fn raw_pool(&self, cpuid_pool: &[RawCpuid]) -> Vec<u8> {
        let mut pool: Vec<u8> = Vec::with_capacity(4096);

        for cpuid in cpuid_pool {
            pool.extend(
                cpuid.raw_fmt().into_bytes()
            );
        }

        return pool;
    }

    fn parse_pool(&self, cpuid_pool: &[RawCpuid]) -> Vec<u8> {
        let mut parse_pool: Vec<u8> = Vec::with_capacity(16384);
        let vendor = VendorFlag::check();
        
        for cpuid in cpuid_pool {
            let fmt = if self.bin_fmt {
                cpuid.bin_fmt()
            } else {
                cpuid.parse_fmt(&vendor)
            }.into_bytes();

            parse_pool.extend(fmt);
        }

        return parse_pool;
    }

    fn select_pool(&self, cpuid_pool: &[RawCpuid]) -> Vec<u8> {
        if self.raw {
            self.raw_pool(cpuid_pool)
        } else {
            self.parse_pool(cpuid_pool)
        }
    }

    fn pool_all_thread(&self) -> Vec<u8> {
        use std::thread;
        use std::sync::{Arc, Mutex};

        let opt_0 = Arc::new(self.clone());

        let cpu_list = libcpuid_dump::cpu_set_list().unwrap();

        let (first_pool, topo_head) = {
            let cpu = cpu_list[0];
            libcpuid_dump::pin_thread(cpu).unwrap();

            (
                Arc::new(opt_0.rawcpuid_pool()),
                topo_info_with_threadid_head(cpu),
            )
        };

        let main_pool = {
            let mut tmp: Vec<u8> = Vec::with_capacity(16384 * cpu_list.len());
            tmp.extend(topo_head.into_bytes());
            tmp.extend(opt_0.head_fmt().into_bytes());
            tmp.extend(opt_0.select_pool(&first_pool));

            Arc::new(Mutex::new(tmp))
        };

        for i in &cpu_list[1..] {
            let i = *i;
            let main_pool = Arc::clone(&main_pool);
            let opt = Arc::clone(&opt_0);
            let first_pool = Arc::clone(&first_pool);

            thread::spawn(move || {
                libcpuid_dump::pin_thread(i).unwrap();

                let topo_head = topo_info_with_threadid_head(i).into_bytes();

                let diff = {
                    let mut sub_pool = opt.rawcpuid_pool();

                    if opt.diff {
                        let mut first_pool = first_pool.iter();
                        sub_pool.retain(|sub| first_pool.next().unwrap() != sub );
                    }

                    sub_pool
                };

                /* for simulator, like Intel SDE */
                if diff.is_empty() { return }

                let pool = opt.select_pool(&diff);

                let mut main_pool = main_pool.lock().unwrap();
                main_pool.extend(topo_head);
                main_pool.extend(pool);

            }).join().unwrap();
        }

        return Arc::try_unwrap(main_pool).unwrap().into_inner().unwrap();
    }

    fn dump_pool(&self) -> Vec<u8> {
        [
            if self.dump_all {
                self.pool_all_thread()
            } else {
                [
                    topo_info_head().into_bytes(),
                    self.head_fmt().into_bytes(),
                    self.select_pool(&self.rawcpuid_pool()),
                ].concat()
            },
        ].concat()
    }

    fn save_file(&self) {
        use std::fs::File;
        use std::io::Write;
        
        let pool = self.dump_pool();

        let path = &self.save.path;

        let mut f = File::create(path).expect("File::create {path} faild.");

        f.write(&pool).expect("fs::write() faild.");

        println!("Output to \"{path}\"");
    }

    fn run(&self) {
        print!("{VERSION_HEAD}");

        match self {
            Self { only_leaf: OnlyLeaf { flag: true, .. }, .. }
                => self.only_leaf(),
            Self { save: SaveOpt { flag: true, .. }, .. }
                => self.save_file(),
            _ => dump_write(&self.dump_pool()),
        }
    }
}

fn main() {
    MainOpt::main_parse().run();
}
