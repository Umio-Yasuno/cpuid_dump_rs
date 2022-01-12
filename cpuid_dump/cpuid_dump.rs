//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{CpuidResult, __cpuid_count};

extern crate cpuid_asm;
use cpuid_asm::{cpuid, Vendor, _AX};

#[path = "./parse.rs"]
mod parse;
pub use crate::parse::*;
#[path = "./raw_cpuid.rs"]
mod raw_cpuid;
pub use crate::raw_cpuid::*;

use std::{mem, thread};

fn cpuid_pool() -> Vec<RawCpuid> {
    let mut pool: Vec<RawCpuid> = Vec::new();

    // Base
    for leaf in 0x0..=0xC {
        match leaf {
            0x4 => {
                for sub_leaf in 0..=4 {
                    pool.push(RawCpuid::exe(leaf, sub_leaf));
                }
            },
            0x7 => {
                for sub_leaf in 0..=1 {
                    pool.push(RawCpuid::exe(leaf, sub_leaf));
                }
            },
            _ => pool.push(RawCpuid::exe(leaf, 0x0)),
        }
    }

    // 0xD: Processor Extended State Enumeration
    for sub_leaf in [0x0, 0x1, 0x2, 0x9, 0xB, 0xC] {
        pool.push(RawCpuid::exe(0xD, sub_leaf));
    }

    // 0x1F: V2 Extended Topology Enumeration Leaf, Intel
    for sub_leaf in 0..6 {
        pool.push(RawCpuid::exe(0x1F, sub_leaf));
    }

    // Ext
    for leaf in _AX+0x0..=_AX+0x21 {
        if leaf == _AX+0x1D {
            for sub_leaf in 0x0..=0x4 {
                pool.push(RawCpuid::exe(leaf, sub_leaf));
            }
            continue;
        }
        pool.push(RawCpuid::exe(leaf, 0x0));
    }

    return pool;
}

fn parse_pool() -> Vec<u8> {
    let mut parse_pool: Vec<u8> = Vec::new();
    let cpuid_pool = cpuid_pool();
    
    for cpuid in cpuid_pool {
        let v = cpuid.parse();
        parse_pool.extend(
            cpuid.parse_fmt(v).into_bytes()
        );
    }

    return parse_pool;
}

fn raw_pool() -> Vec<u8> {
    let mut pool: Vec<u8> = Vec::new();
    let cpuid_pool = cpuid_pool();

    for cpuid in cpuid_pool {
        let tmp = cpuid.raw_fmt().into_bytes();
        pool.extend(tmp);
    }

    return pool;
}

fn dump() {
    let mut pool: Vec<u8> = Vec::new();

    pool.extend(
        format!("   (in)EAX_xECX:  {:<10} {:<10} {:<10} {:<10}\n",
            "(out)EAX", "(out)EBX", "(out)ECX", "(out)EDX").into_bytes()
    );
    pool.extend(
        format!("{}\n", "=".repeat(80)).into_bytes()
    );
    pool.extend(parse_pool());
    pool.extend(b"\n");

    dump_write(&pool);
}

fn raw_dump() {
    let pool = raw_pool();
    dump_write(&pool);
}

fn dump_all() {
    let thread_count = cpuid_asm::CpuCoreCount::get().total_thread;
    println!("   (in)EAX_xECX:  {:<10} {:<10} {:<10} {:<10}\n{}",
            "(out)EAX", "(out)EBX", "(out)ECX", "(out)EDX",
            "=".repeat(80));

    for i in 0..(thread_count) as usize {
        thread::spawn(move || {
            cpuid_asm::pin_thread!(i);

            let mut local: Vec<u8> = Vec::new();
            let id = cpuid_asm::CpuCoreCount::get().core_id;
            local.extend(
                format!("Core ID: {:>3} / Thread: {:>3}\n", id, i).into_bytes()
            );
            local.extend(parse_pool());

            dump_write(&local);
        }).join().unwrap();
    }
}

fn raw_dump_all() {
    let thread_count = cpuid_asm::CpuCoreCount::get().total_thread;

    for i in 0..(thread_count) as usize {
        thread::spawn(move || {
            cpuid_asm::pin_thread!(i);

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

fn save_file(save_path: String) {
    use std::fs::File;
    use std::io::Write;

    let mut f = File::create(save_path).unwrap();
    let pool = parse_pool();
    f.write(&pool).unwrap();
}

struct MainOpt {
    raw: bool,
    dump_all: bool,
    save: bool,
    save_path: String,
}

impl MainOpt {
    fn init() -> MainOpt {
        MainOpt {
            raw: false,
            dump_all: false,
            save: false,
            save_path: format!("./cpuid_dump.txt"),
        }
    }
    fn parse() -> MainOpt {
        let mut opt = MainOpt::init();

        let args: Vec<String> = std::env::args().collect();

        for i in 1..args.len() {
            let arg = args[i].trim_start_matches("-");

            match arg {
                "a" | "all" => opt.dump_all = true,
                "r" | "raw" => opt.raw = true,
                "save" => {
                    opt.save = true;
                    opt.save_path = match args.get(i+1) {
                        Some(v) => v.parse::<String>().expect("Parse error"),
                        _ => format!("./{}.txt",
                            cpuid_asm::get_trim_proc_name().replace(" ", "_")
                        ),
                    };
                    break;
                },
                _ => eprintln!("Unknown option: {}", args[i]),
            }
        }

        return opt;
    }
}

fn main() {
    let opt = MainOpt::parse();

    if opt.save {
        save_file(opt.save_path);
    } else if opt.raw && opt.dump_all {
        raw_dump_all();
    } else if opt.raw {
        raw_dump();
    } else if opt.dump_all {
        println!("CPUID Dump");
        dump_all();
    }

    if opt.raw || opt.dump_all || opt.save { return; }

    println!("CPUID Dump");
    dump();
}
