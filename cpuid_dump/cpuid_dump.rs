//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{CpuidResult, __cpuid_count};

extern crate cpuid_asm;
use cpuid_asm::{cpuid, Vendor, _AX};

mod parse;
use crate::parse::*;

use std::io::Write;
use std::{mem, thread};
//  use std::fmt::write;

/*
struct Opt {
    long: bool,
}

impl Opt {
    fn default() -> Opt {
        Opt {
            long: false,
        }
    }
    fn parse() -> Opt {
        let mut opt = Opt::default();
        let args: Vec<String> = std::env::args().collect();

        for v in &args[1..] {
            match v.as_str() {
                "-l" |
                "--long" => opt.long = true,
                _ => {},
            }
        }
        return opt;
    }
}
*/

struct VendorFlag {
    amd: bool,
    intel: bool,
}

impl VendorFlag {
    fn check() -> VendorFlag {
        let vendor = Vendor::get();
        let amd = vendor.check_amd();
        let intel = vendor.check_intel() && !amd;

        VendorFlag {
            amd,
            intel,
        }
    }
}

fn dump() {
    //  let opt = Opt::parse();
    use std::io::{BufWriter, Write, stdout};


    let mut parse_pool: Vec<u8> = Vec::new();

    parse_pool.extend(
        format!("   (in)EAX_xECX:  {:<10} {:<10} {:<10} {:<10}\n",
            "(out)EAX", "(out)EBX", "(out)ECX", "(out)EDX").into_bytes()
    );
    parse_pool.extend(
        format!("{}\n", "=".repeat(80)).into_bytes()
    );


    let cpuid_pool = cpuid_pool();
    
    for cpuid in cpuid_pool {
        let v = cpuid.parse();
        parse_pool.extend(
            cpuid.parse_fmt(v).into_bytes()
        );
    }
    parse_pool.extend(b"\n");

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    out.write(&parse_pool).unwrap();
}

fn dump_all() {
    let thread_count = cpuid_asm::CpuCoreCount::get().total_thread;

    for i in 0..(thread_count) as usize {
        thread::spawn(move || {
            cpuid_asm::pin_thread!(i);

            let id = cpuid_asm::CpuCoreCount::get().core_id;
            println!("Core ID: {:>3} / Thread: {:>3}", id, i);

            dump();
        })
        .join().unwrap();
    }
}

pub struct RawCpuid {
    pub leaf: u32, // in_eax
    pub sub_leaf: u32, // in_ecx
    pub result: CpuidResult,
}

impl RawCpuid {
    pub fn exe(leaf: u32, sub_leaf: u32) -> RawCpuid {
        let result = cpuid!(leaf, sub_leaf);

        RawCpuid {
            leaf,
            sub_leaf,
            result,
        }
    }
    pub fn result(&self, end_str: &str) -> String {
        format!("  0x{:08X}_x{:1X}:  0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X} {}",
            self.leaf, self.sub_leaf,
            self.result.eax, self.result.ebx, self.result.ecx, self.result.edx,
            end_str,
        )
    }
    pub fn raw_fmt(&self) -> String {
        self.result("\n")
    }
    pub fn parse_fmt(&self, parse_string: String) -> String {
        self.result(parse_string.as_str())
    }
    pub fn parse(&self) -> String {
        let vendor = VendorFlag::check();

        let tmp: String = match self.leaf {
            0x0 => format!(" [{}]", cpuid_asm::get_vendor_name() ),
            0x1 => {
                let v = vec![
                    info_00_01h(&self.result),
                    padln!().to_string(),
                    feature_00_01h(&self.result),
                ];
                concat_string(v)
            },
            0x7 => match self.sub_leaf {
                0x0 => feature_00_07h_x0(&self.result),
                0x1 => feature_00_07h_x1(&self.result.eax),
                _ => unreachable!(),
            },
            0xD => enum_amd_0dh(&self),
            0x1F => if vendor.intel {
                v2_ext_topo_intel_1fh(&self.result)
            } else {
                "".to_string()
            },
            0x8000_0001 => {
                let mut v = Vec::with_capacity(2);
                if vendor.amd {
                    v.push(pkgtype_amd_80_01h(&self.result.ebx));
                    v.push(padln!().to_string());
                }
                v.push(feature_80_01h(&self.result));
                concat_string(v)
            },
            0x8000_0002..=0x8000_0004 => format!(" [{}]", cpu_name(&self.result)),
            _ => if vendor.amd {
                match self.leaf {
                    0x8000_0005 => l1_amd_80_05h(&self.result),
                    0x8000_0006 => l2_amd_80_06h(&self.result),
                    0x8000_0007 => apmi_amd_80_07h(&self.result.edx),
                    0x8000_0008 => spec_amd_80_08h(&self.result.ebx),
                    0x8000_000A => rev_id_amd_80_0ah(&self.result),
                    0x8000_0019 => l1l2tlb_1g_amd_80_19h(&self.result),
                    0x8000_001A => fpu_width_amd_80_1ah(&self.result.eax),
                    0x8000_001B => ibs_amd_80_1bh(&self.result.eax),
                    0x8000_001D => cache_prop(&self.result),
                    0x8000_001E => cpu_topo_amd_80_1eh(&self.result),
                    0x8000_001F => secure_amd_80_1fh(&self.result.eax),
                    _ => "".to_string(),
                }
            } else if vendor.intel {
                match self.leaf {
                    0x4 => cache_prop(&self.result),
                    0x16 => clock_speed_intel_00_16h(&self.result),
                    0x1A => intel_hybrid_1ah(&self.result.eax),
                    _ => "".to_string(),
                }
            } else {
                "".to_string()
            },
        };

        return tmp + "\n";
    }
}

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

fn raw_pool() -> Vec<u8> {
    let mut pool: Vec<u8> = Vec::new();
    let cpuid_pool = cpuid_pool();

    for cpuid in cpuid_pool {
        let tmp = cpuid.raw_fmt().into_bytes();
        pool.extend(tmp);
    }

    return pool;
}

fn raw_dump() {
    use std::io::{BufWriter, Write, stdout};

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let pool = raw_pool();

    out.write(&pool).unwrap();
}

fn raw_dump_all() {
    let thread_count = cpuid_asm::CpuCoreCount::get().total_thread;

    for i in 0..(thread_count) as usize {
        thread::spawn(move || {
            cpuid_asm::pin_thread!(i);
            println!("\nCPU {:>3}:", i);
            raw_dump();
        })
        .join().unwrap();
    }
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
                        _ => format!("./cpuid_dump.txt"),
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

    if opt.raw && opt.dump_all {
        raw_dump_all();
    } else if opt.raw  {
        raw_dump();
    } else if opt.dump_all {
        println!("CPUID Dump");
        dump_all();
    }

    if opt.raw || opt.dump_all || opt.save { return; }

    println!("CPUID Dump");
    dump();
}
