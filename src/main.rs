//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::CpuidResult;

// extern crate libcpuid_dump;
use libcpuid_dump::{cpuid, Vendor, VendorFlag, _AX};

#[path = "./parse_mod.rs"]
mod parse_mod;
pub use crate::parse_mod::*;

#[path = "./raw_cpuid.rs"]
mod raw_cpuid;
pub use crate::raw_cpuid::*;

/*
#[path = "./load_file.rs"]
mod load_file;
pub use crate::load_file::*;
*/

const VERSION: f32 = 0.1;

fn cpuid_pool() -> Vec<RawCpuid> {
    let mut pool: Vec<RawCpuid> = Vec::with_capacity(64);

    /* Base */
    for leaf in 0x0..=0xC {
        match leaf {
            /* Cache Properties, Intel */
            0x4 => for sub_leaf in 0..=4 {
                pool.push(RawCpuid::exe(leaf, sub_leaf));
            },
            0x7 => for sub_leaf in 0..=1 {
                pool.push(RawCpuid::exe(leaf, sub_leaf));
            },
            /* Extended Topology Enumeration, Intel, AMD Family19h <= */
            0xB => for sub_leaf in 0..=2 {
                pool.push(RawCpuid::exe(leaf, sub_leaf));
            },
            _ => pool.push(RawCpuid::exe(leaf, 0x0)),
        }
    }

    /* 0xD: Processor Extended State Enumeration */
    for sub_leaf in [0x0, 0x1, 0x2, 0x9, 0xB, 0xC] {
        pool.push(RawCpuid::exe(0xD, sub_leaf));
    }

    /* 0x1F: V2 Extended Topology Enumeration Leaf, Intel */
    for sub_leaf in 0..6 {
        pool.push(RawCpuid::exe(0x1F, sub_leaf));
    }

    /* Ext */
    for leaf in _AX+0x0..=_AX+0xA {
        pool.push(RawCpuid::exe(leaf, 0x0));
    }
    for leaf in _AX+0x19..=_AX+0x21 {
        /* Cache Properties, AMD, same format as Intel Leaf:0x4 */
        const LF_80_1D: u32 = _AX + 0x1D;

        match leaf {
            LF_80_1D => for sub_leaf in 0x0..=0x4 {
                pool.push(RawCpuid::exe(leaf, sub_leaf));
            },
            _ => pool.push(RawCpuid::exe(leaf, 0x0)),
        }
    }

    return pool;
}

fn version_head() -> String {
    format!("CPUID Dump v{VERSION:.1}\n")
}

fn hex_head() -> String {
    const HEAD: &str = "  {LEAF}_x{SUB}:  (out)EAX   (out)EBX   (out)ECX   (out)EDX";

    format!("{}\n{}\n",
        HEAD,
        "=".repeat(TOTAL_WIDTH)
    )
}

fn bin_head() -> String {
    const INPUT_LEN: usize = 16;
    const OUTPUT_LEN: usize = 35;
    const PAD_LEN: usize = (OUTPUT_LEN - "(out)EAX / (out)ECX".len()) / 2;

    let p = " ".repeat(PAD_LEN - 1);

    let head = format!("  {{LEAF}}_x{{SUB}}:  {p} (out)EAX / (out)ECX {p} {p}  (out)EBX / (out)EDX");
    let line = format!("{}  {}  {}",
        "=".repeat(INPUT_LEN),
        "=".repeat(OUTPUT_LEN),
        "=".repeat(OUTPUT_LEN),
    );

    format!("{head}\n{line}\n")
}

fn dump_write(pool: &[u8]) {
    use std::io::{BufWriter, Write, stdout};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

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
    bin_fmt: bool,
}

impl MainOpt {
    fn init() -> Self {
        Self {
            raw: false,
            dump_all: false,
            save: SaveOpt {
                flag: false,
                path: format!("{}.txt", libcpuid_dump::ProcName::get_trim_name().replace(" ", "_")),
            },
            // load: (false, "cpuid_dump.txt".to_string()),
            only_leaf: OnlyLeaf {
                flag: false,
                leaf: 0u32,
                sub_leaf: 0u32,
            },
            skip_zero: true,
            bin_fmt: false,
        }
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
            cpuid_dump v{VERSION:.1}\n\
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
            \x20    -disp-zero\n\
            \x20        Display result even if E[ABCD]X are zero.\n\
            \n\
            OPTIONS:\n\
            \x20    --l <u32>, --leaf <u32>\n\
            \x20        Display result only for the specified value, the value is Leaf/InputEAX <u32>.\n\
            \x20        e.g. --leaf 1, --leaf 0x8000_0008,\n\
            \x20    --sub_leaf <u32>, --sub-leaf <u32>\n\
            \x20        Display result only for the specified value, the value is Sub-Leaf/InputECX <u32>.\n\
            \x20    --pin <usize>, --pin_threads <usize>\n\
            \x20        Display result for the specified thread.\n\
            \x20    --s <path/filename>, --save <path/filename>\n\
            \x20        Save dump result to text file.\n\
            \x20        If there is no path/filename argument, will be used \"./<processor_name>\".
        \n");
    }

    pub fn main_parse() -> Self {
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
                "r" | "raw" => opt.raw = true,
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
                "sub-leaf" | "sub_leaf" => {
                    if !opt.only_leaf.flag {
                        eprintln!("Please \"--leaf <u32>\" argument");
                        continue;
                    }
                    opt.only_leaf.sub_leaf = match args.get(idx+1) {
                        Some(v) => Self::parse_value(v.to_string(), "sub-leaf"),
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
                // TODO: "taskset" option?
                // cpuid_dump --taskset <list>
                // same `taskset -c <list> cpuid_dump -a`
                _ => eprintln!("Unknown option: {}", arg),
            }
        }

        return opt;
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
                raw_result.parse_fmt(&VendorFlag::all_true()),
            ]
        }.concat();

        dump_write(&tmp.into_bytes())
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
            if self.skip_zero && cpuid.check_result_zero() {
                continue;
            }

            let fmt = if self.bin_fmt {
                cpuid.bin_fmt()
            } else {
                cpuid.parse_fmt(&vendor)
            }.into_bytes();

            parse_pool.extend(fmt);
        }

        return parse_pool;
    }

    fn pool_select(&self, cpuid_pool: &[RawCpuid]) -> Vec<u8> {
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

        let first_pool = {
            libcpuid_dump::pin_thread(cpu_list[0]).unwrap();

            Arc::new(cpuid_pool())
        };

        let v_0 = {
            let mut tmp: Vec<u8> = Vec::with_capacity(16384 * cpu_list.len());
            tmp.extend(opt_0.pool_select(&first_pool));

            Arc::new(Mutex::new(tmp))
        };

        for i in &cpu_list[1..] {
            let i = *i;
            let v_1 = Arc::clone(&v_0);
            let opt_1 = Arc::clone(&opt_0);
            let first_pool = Arc::clone(&first_pool);

            thread::spawn(move || {
                libcpuid_dump::pin_thread(i).unwrap();

                let id = libcpuid_dump::CpuCoreCount::get().core_id;
                let ct_head = format!("Core ID: {id:>3} / Thread: {i:>3}\n").into_bytes();

                let sub_pool = cpuid_pool();

                let mut diff: Vec<RawCpuid> = Vec::with_capacity(32);
                for (x, y) in first_pool.iter().zip(sub_pool) {
                    if *x != y { diff.push(y) }
                }

                let pool = opt_1.pool_select(&diff);

                let mut v_1 = v_1.lock().unwrap();

                v_1.extend(ct_head);
                v_1.extend(pool);
            }).join().unwrap();
        }

        return Arc::try_unwrap(v_0).unwrap().into_inner().unwrap();
    }

    fn dump(&self) {
        let pool = [
            self.head_fmt().into_bytes(),
            if self.dump_all {
                self.pool_all_thread()
            } else {
                self.pool_select(&cpuid_pool())
            },
        ].concat();

        dump_write(&pool);
    }

    fn save_file(&self) {
        use std::fs::File;
        use std::io::Write;
        
        let pool = [
            version_head().into_bytes(),
            self.head_fmt().into_bytes(),
            if self.dump_all {
                self.pool_all_thread()
            } else {
                self.pool_select(&cpuid_pool())
            },
        ].concat();

        let path = &self.save.path;

        let mut f = File::create(path).expect("File::create {path} faild.");

        f.write(&pool).expect("fs::write() faild.");

        println!("Output to \"{path}\"");
    }

    fn run(&self) {
        print!("{}", version_head());

        match self {
            Self { only_leaf: OnlyLeaf { flag: true, .. }, .. }
                => self.only_leaf(),
            Self { save: SaveOpt { flag: true, .. }, .. }
                => self.save_file(),
            _ => self.dump(),
        }
    }
}

fn main() {
    MainOpt::main_parse().run();
}
