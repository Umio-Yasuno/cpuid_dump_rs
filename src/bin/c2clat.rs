//  © 2020 Erik Rigtorp <erik@rigtorp.se>
//  SPDX-License-Identifier: MIT
//  c2c_bench.rs was ported from c2clat:
//      https://github.com/rigtorp/c2clat

#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
use libc::{cpu_set_t, CPU_SET, CPU_ISSET, CPU_ZERO, sched_setaffinity, sched_getaffinity};

extern crate cpuid_asm;

use std::{env, mem, thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};

macro_rules! pin_thread { ($cpu: expr) => {
     unsafe {
        let mut set = mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);
        CPU_SET($cpu, &mut set);

        let status = sched_setaffinity(0, mem::size_of::<cpu_set_t>(), &set);
        if status == -1 {
            eprintln!("sched_setaffinity failed");
            return;
        }
    }
}}

fn print_table  (title: &str, result: Vec<Vec<u128>>,
                cpu_list: &Vec<usize>, ncpu: usize, opt_md: bool) {

        macro_rules! md_table { ($opt: expr) => {
            if $opt { " |" } else { "" };
            }
        }

    println!("\n{}{}", if opt_md {"#### "} else {""}, title );
    print!(" CPU{}", md_table!(opt_md));
    for n in cpu_list {
        print!("{:>5}{}", n, md_table!(opt_md) );
    }
    println!();
    if opt_md {
        print!(" --: | ");
        for _n in 0..ncpu {
            print!(" --: | ");
        }
        println!();
    }
    for i in 0..ncpu {
        print!("{:>4}{}", i, md_table!(opt_md) );
        for j in 0..ncpu {
            print!("{:>5}{}", result[i][j], md_table!(opt_md));
        }
        println!();
    }
    println!();
}

const NSAMPLES: isize = 1_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opt_md: bool = false;

    for opt in args {
        if opt == "-md" {
            opt_md = true;
        }
    }

    let ncpu = cpuid_asm::CpuCoreCount::get();
    let mut cpus: Vec<usize> = Vec::new();

    unsafe {
        let mut set = mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);

        let status = sched_getaffinity(0, mem::size_of::<cpu_set_t>(), &mut set);
        if status == -1 {
            eprintln!("sched_getaffinity failed");
            return;
        }

        for i in 0..(ncpu.total_thread) as usize {
            if CPU_ISSET(i, &mut set) {
                cpus.push(i);
            }
        }
    }

    let ncpu: usize = cpus.len();
    
    let mut min_result: Vec<Vec<u128>> = vec![vec![0; ncpu]; ncpu];
    let mut avg_result: Vec<Vec<u128>> = vec![vec![0; ncpu]; ncpu];

            let seq1 = Arc::new(AtomicIsize::new(-1));
            let _pad = Arc::new(AtomicIsize::new(-1));
            let seq2 = Arc::new(AtomicIsize::new(-1));

    for i in 0..(ncpu) {
        for j in (i+1)..(ncpu) {


            let seq_1 = seq1.clone();
            let seq_2 = seq2.clone();

            let c = cpus[i];

            let t = thread::spawn(move || {
                pin_thread!(c);
                for _m in 0..100 {
                    for n in 0..NSAMPLES {
                        while seq_1.load(Ordering::Acquire) != n {}
                        seq_2.store(n, Ordering::Release);
                    }
                }
            });

            let mut perf: u128;
            let mut tmp: u128 = u128::MAX;
            let mut avg: u128 = 0;

            pin_thread!(cpus[j]);
            for _m in 0..100 {
                seq1.store(-1, Ordering::Release);
                seq2.store(-1, Ordering::Release);

                let start = time::Instant::now();

                for n in 0..NSAMPLES {
                    seq1.store(n, Ordering::Release);
                    while seq2.load(Ordering::Acquire) != n {}
                }

                perf = start.elapsed().as_nanos();
                tmp = std::cmp::min(tmp, perf);
                if _m != 0 {  // pin_thread cost
                    avg += perf;
                }
            }

            t.join().unwrap();

            min_result[i][j] = tmp / NSAMPLES as u128 / 2;
            min_result[j][i] = tmp / NSAMPLES as u128 / 2;

            avg_result[i][j] = avg / NSAMPLES as u128 / (100-1) / 2;
            avg_result[j][i] = avg / NSAMPLES as u128 / (100-1) / 2;
        }
    }

    print_table("[min (ns)]", min_result, &cpus, ncpu, opt_md);
    print_table("[avg (ns)]", avg_result, &cpus, ncpu, opt_md);
}

