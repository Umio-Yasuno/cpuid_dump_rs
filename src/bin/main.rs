#![feature(asm)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

extern crate cpuid_asm;
use cpuid_asm::*;
use cpuid_asm::feature_detect::*;

#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
use libc::{cpu_set_t, CPU_SET, CPU_ZERO, sched_setaffinity};

use std::{mem, env, thread};

fn dump_all() {
    let core_count = CpuCoreCount::get();

    if cfg!(windows) {
        println!("dump_all func supports Linux only.");
        return;
    }

    for i in 0..(core_count.total_thread) as usize {
        thread::spawn( move || {
            unsafe {
                let mut set = mem::zeroed::<cpu_set_t>();
                CPU_ZERO(&mut set);
                CPU_SET(i, &mut set);

                sched_setaffinity(0,
                                  mem::size_of::<cpu_set_t>(),
                                  &set);
            }

            let id = CpuCoreCount::get();
            println!("Core ID: {:<3} / Thread: {:<3}",
                id.core_id, i);
            //cpuid_dump::dump();

        }).join().unwrap();
    }
}

fn main() {
    println!();

    let args: Vec<String> = env::args().collect();

    let mut opt_dump: bool      = false;
    let mut opt_dump_all: bool  = false;
    let mut opt_c2c: bool       = false;

    for opt in args {
        //  println!("{}", opt);
        if opt == "-d" || opt == "--dump" {
            opt_dump = true;
        } else if opt == "-a" || opt == "--all" {
            opt_dump_all = true;
        } else if opt == "-c2c" {
            opt_c2c = true;
        }
    }
    
    /*
    if opt_dump && opt_dump_all {
        dump_all();
        return;
    } else if opt_dump {
        cpuid_dump::dump();
        return;
    } else if opt_c2c {
        c2c_bench::c2c();
        return;
    }
    */
    let vendor_name = get_vendor_name();
    println!("Vendor: {}", vendor_name);

    let cpu_name = get_processor_name();
    println!("Processor Name: {}", cpu_name);

    let cpu_info = FamModStep::get();
    println!("Family: {0:#X} ({0}), Model: {1:#X} ({1}), Stepping: {2}",
        cpu_info.syn_fam, cpu_info.syn_mod, cpu_info.step);
    println!("Code Name: {}",
        codename::get_codename(cpu_info.syn_fam, cpu_info.syn_mod, cpu_info.step));
    println!();

    let cpu_feature = CpuFeature::get();
    println!(" x86_64_v1: {}\n x86_64_v2: {}\n x86_64_v3: {}\n x86_64_v4: {}",
        cpu_feature.x86_64_v1, cpu_feature.x86_64_v2,
        cpu_feature.x86_64_v3, cpu_feature.x86_64_v4);
    println!();

    let cpu_cache = CacheInfo::get(cpu_info.syn_fam);
    println!(" L1d Cache: {} KiB, {}-byte line, {}-way",
        cpu_cache.l1d_size, cpu_cache.l1d_line, cpu_cache.l1d_way);
    println!(" L1i Cache: {} KiB, {}-byte line, {}-way",
        cpu_cache.l1i_size, cpu_cache.l1i_line, cpu_cache.l1i_way);
    println!(" L2 Cache: {} KiB, {}-byte line, {}-way",
        cpu_cache.l2_size, cpu_cache.l2_line, cpu_cache.l2_way);
    println!(" L3 Cache: {} MiB, {}-byte line, {}-way",
        cpu_cache.l3_size, cpu_cache.l3_line, cpu_cache.l3_way);
    println!();

}

