//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

use core::arch::x86_64::{CpuidResult, __cpuid_count};

extern crate cpuid_asm;
use cpuid_asm::{bitflag, cpuid, Vendor, _AX};

mod parse;
use crate::parse::*;

/*
#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
use libc::{cpu_set_t, CPU_SET, CPU_ZERO, sched_setaffinity};

#[cfg(target_os = "windows")]
use kernel32::{GetCurrentThread, SetThreadAffinityMask};
*/

use std::io::Write;
use std::{mem, thread};
//  use std::fmt::write;

fn dump() {
    println!(
        "   (in)EAX_xECX:  {:<10} {:<10} {:<10} {:<10}",
        "(out)EAX", "(out)EBX", "(out)ECX", "(out)EDX"
    );

    let mut buff = String::new();
    for _i in 0..80 {
        buff.push_str("=");
    }
    println!("{}", buff);

    let vendor_amd   = Vendor::check_amd();
    let vendor_intel = Vendor::check_intel();

    for i in 0..=0x20 {
        if vendor_amd {
            if (0x2 <= i && i <= 0x4)
                || (0x8 <= i && i <= 0xA)
                || (0xC == i)
                || (0xE == i)
                || (0x11 <= i)
            {
                continue;
            } else if i == 0xD {
                enum_amd_0dh();
                continue;
            }
        }
        if vendor_intel && i == 0x4 {
            cache_prop(0x4);
            continue;
        } else if i == 0x7 {
            feature_00_07h_x0();
            if vendor_intel {
                feature_00_07h_x1();
            }
            continue;
        } else if i == 0xB {
            for j in 0..=1 {
                let tmp = cpuid!(i, j);
                print_cpuid!(i, j, tmp);
                println!();
            }
            continue;
        }

        let tmp = cpuid!(i, 0);
        print_cpuid!(i, 0, tmp);

        if i == 0 {
            print!(" [{}]", cpuid_asm::get_vendor_name());
        } else if i == 0x1 {
            info_00_01h(tmp.eax, tmp.ebx);
            feature_00_01h(tmp.ecx, tmp.edx);
        } else if vendor_intel && i == 0x16 {
            clock_speed_intel_00_16h(tmp);
        } else if vendor_intel && i == 0x1A {
            intel_hybrid_1ah(tmp.eax);
        }
        println!();
    }

    println!();

    for i in 0x0..=0x21 {
        if vendor_amd {
            if 0xB <= i && i <= 0x18 {
                continue;
            } else if i == 0x1D {
                cache_prop(_AX + 0x1D);
                continue;
            }
        }

        let tmp = cpuid!(_AX + i, 0);
        print_cpuid!(_AX + i, 0, tmp);

        if i == 0x1 {
            if vendor_amd {
                pkgtype_amd_80_01h(tmp.ebx);
            }
            feature_80_01h(tmp.ecx, tmp.edx);
        } else if 0x2 <= i && i <= 0x4 {
            print!(" [{}]", cpu_name(tmp));
        }

        if vendor_amd {
            match i {
                0x5  => l1_amd_80_05h(tmp),
                0x6  => l2_amd_80_06h(tmp),
                0x7  => apmi_amd_80_07h(tmp.edx),
                0x8  => spec_amd_80_08h(tmp.ebx),
                0x19 => l1l2tlb_1g_amd_80_19h(tmp.eax, tmp.ebx),
                0x1A => fpu_width_amd_80_1ah(tmp.eax),
                0x1E => cpu_topo_amd_80_1eh(tmp.ebx, tmp.ecx),
                0x1F => secure_amd_80_1fh(tmp.eax),
                _ => {}
            }
        }
        println!();
    }
    println!();
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

macro_rules! raw {
    ($dst: expr, $in_eax: expr, $in_ecx: expr) => {
        let tmp = cpuid!($in_eax, $in_ecx);

        print_cpuid!($dst, $in_eax, $in_ecx, tmp);
        writeln!($dst).unwrap();
    }
}

fn raw_dump() {
    let out = std::io::stdout();
    let mut out = out.lock();

    for i in 0x0..=0xD {
        if i == 0xD {
            for ecx in [0x0, 0x1, 0x2, 0x9, 0xB, 0xC] {
                raw!(out, i, ecx);
            }
            continue;
        }
        raw!(out, i, 0x0);
    }

    for i in 0x0..=0x21 {
        if i == 0x1D {
            for ecx in 0x0..=0x4 {
                raw!(out, _AX + i, ecx);
            }
            continue;
        }
        raw!(out, _AX + i, 0x0);
    }
    out.flush().unwrap();
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

fn main() {
    for opt in std::env::args() {
        println!("{}", opt);
        if opt == "-a" || opt == "--all" {
            println!("CPUID Dump");
            dump_all();
            return;
        } else if opt == "-r" || opt == "--raw" {
            raw_dump_all();
            return;
        }
    }
    println!("CPUID Dump");
    dump();
}
