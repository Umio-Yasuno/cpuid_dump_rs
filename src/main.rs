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
    let mut pool: Vec<RawCpuid> = Vec::new();

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

fn raw_pool() -> Vec<u8> {
    let mut pool: Vec<u8> = Vec::new();
    let cpuid_pool = cpuid_pool();

    for cpuid in cpuid_pool {
        pool.extend(
            cpuid.raw_fmt().into_bytes()
        );
    }

    return pool;
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

    let pad = " ".repeat(PAD_LEN - 1);

    let head = format!("  {{LEAF}}_x{{SUB}}: {pad} (out)EAX / (out)ECX {pad}{pad} (out)EBX / (out)EDX");
    let line = format!("{} {} {}",
        "=".repeat(INPUT_LEN),
        "=".repeat(OUTPUT_LEN),
        "=".repeat(OUTPUT_LEN),
    );

    format!("{head}\n{line}\n")
}

fn raw_dump() {
    dump_write(&raw_pool())
}

fn raw_dump_all() {
    use std::thread;
    let cpu_list = libcpuid_dump::cpu_set_list();

    for i in cpu_list {
        thread::spawn(move || {
            libcpuid_dump::pin_thread!(i);

            let mut local: Vec<u8> = Vec::new();
            local.extend(
                format!("CPU {:>3}:\n", i).into_bytes()
            );
            local.extend(raw_pool());

            dump_write(&local);
        }).join().unwrap();
    }
}

fn dump_write(pool: &[u8]) {
    use std::io::{BufWriter, Write, stdout};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    out.write(pool).unwrap();
}

fn save_file(save_path: String, pool: &[u8]) {
    use std::fs::File;
    use std::io::Write;

    let mut f = File::create(save_path).unwrap();
    //  let pool = parse_pool();
    f.write(pool).unwrap();
}

#[derive(Debug, Clone)]
struct MainOpt {
    raw: bool,
    dump_all: bool,
    save: (bool, String),
    // load: (bool, String),
    only_leaf: (bool, u32, u32, bool),
    skip_zero: bool,
}

/*
#[derive(Debug, Clone)]
struct OnlyLeaf {
    flag: bool,
    leaf: bool,
    sub_leaf: bool,
    bin_fmt: bool,
}
*/

impl MainOpt {
    fn init() -> Self {
        Self {
            raw: false,
            dump_all: false,
            save: (false, format!("{}.txt",
                libcpuid_dump::ProcName::get_trim_name().replace(" ", "_")
            )),
            // load: (false, "cpuid_dump.txt".to_string()),
            only_leaf: (false, 0x0, 0x0, false),
            skip_zero: true,
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
            OPTIONS:\n\
            \x20    -a, -all\n\
            \x20        Display result for all threads.\n\
            \x20    -r, -raw\n\
            \x20        Display raw/hex result.\n\
            \x20    --l <u32>, --leaf <u32>\n\
            \x20        Display result only for the specified value, the value is Leaf/InputEAX <u32>.\n\
            \x20        e.g. --leaf 1, --leaf 0x8000_0008,
            \x20    --sub_leaf <u32>, --sub-leaf <u32>\n\
            \x20        Display result only for the specified value, the value is Sub-Leaf/InputECX <u32>.\n\
            \x20    -bin\n\
            \x20        Display binary result, for --leaf/--sub_leaf option.\n\
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
                    opt.save.0 = true;
                    opt.save.1 = match args.get(idx+1) {
                        Some(v) => {
                            if v.starts_with("-") {
                                skip = true;
                                continue;
                            }

                            if std::path::Path::new(v).is_dir() {
                                format!("{}{}", v, opt.save.1)
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
                    opt.only_leaf.0 = true;
                    opt.only_leaf.1 = match args.get(idx+1) {
                        Some(v) => Self::parse_value(v.to_string(), "leaf"),
                        _ => continue,
                    };
                },
                "sub-leaf" | "sub_leaf" => {
                    if !opt.only_leaf.0 {
                        eprintln!("Please \"--leaf <u32>\" argument");
                        continue;
                    }
                    opt.only_leaf.2 = match args.get(idx+1) {
                        Some(v) => Self::parse_value(v.to_string(), "sub-leaf"),
                        _ => continue,
                    };
                }
                "bin" => {
                    opt.only_leaf.3 = true
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
                    libcpuid_dump::pin_thread!(cpu);
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

    fn only_leaf(&self) {
        let opt = self.only_leaf;
        let raw_result = RawCpuid::exe(opt.1, opt.2);

        let tmp = if opt.3 {
            [
                bin_head(),
                raw_result.bin_fmt(),
            ]
        } else {
            [
                hex_head(),
                raw_result.parse_fmt(&VendorFlag::all_true()),
            ]
        };

        let tmp = concat_string_from_slice(&tmp);

        dump_write(&tmp.into_bytes())
    }

    fn parse_pool(&self) -> Vec<u8> {
        let mut parse_pool: Vec<u8> = Vec::new();
        let cpuid_pool = cpuid_pool();
        let vendor = VendorFlag::check();
        
        for cpuid in cpuid_pool {
            if self.skip_zero && cpuid.check_result_zero() {
                continue;
            }

            parse_pool.extend(
                cpuid.parse_fmt(&vendor).into_bytes()
            );
        }

        return parse_pool;
    }

    fn dump(&self) {
        let mut pool: Vec<u8> = Vec::new();

        pool.extend(hex_head().into_bytes());
        pool.extend(self.parse_pool());
        pool.extend(b"\n");

        dump_write(&pool);
    }

    fn dump_all(&self) {
        use std::thread;
        let cpu_list = libcpuid_dump::cpu_set_list();

        println!("{}", hex_head());

        for i in cpu_list {
            let opt = self.clone();

            thread::spawn(move || {
                libcpuid_dump::pin_thread!(i);

                let mut local: Vec<u8> = Vec::new();
                let id = libcpuid_dump::CpuCoreCount::get().core_id;
                let ct_head = format!("Core ID: {:>3} / Thread: {:>3}\n", id, i)
                    .into_bytes();

                local.extend(ct_head);
                local.extend(opt.parse_pool());

                dump_write(&local);
            }).join().unwrap();
        }
    }

    fn run(&self) {
        println!("CPUID Dump v{VERSION:.1}");

        match self {
            Self { only_leaf: (true, ..), .. }
                => self.only_leaf(),
            /*
            MainOpt { load: (true, load_path), .. }
                => load_file(load_path),
            */
            Self { raw: true, save: (true, save_path), .. }
                => save_file(save_path.to_string(), &raw_pool()),
            Self { raw: true, dump_all: true, .. }
                => raw_dump_all(),
            Self { dump_all: true, .. }
                => self.dump_all(),
            Self { raw: true, .. }
                => raw_dump(),
            Self { save: (true, save_path), .. }
                => save_file(save_path.to_string(), &self.parse_pool()),
            _ => {
                self.dump();
            },
        }
    }
}

/*
TODO: load & parse,
pub enum CpuidDumpType {
    CpuidDumpRs,
    LibCpuid,
    EtallenCpuid,
    Last,
}
*/

fn main() {
    MainOpt::main_parse().run();
}
