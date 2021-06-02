//  © 2020 Erik Rigtorp <erik@rigtorp.se>
//  SPDX-License-Identifier: MIT
//  c2c_bench.rs was ported from c2clat:
//      https://github.com/rigtorp/c2clat
//
//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

/*
#[cfg(target_os = "linux")]
extern crate libc;
*/
#[cfg(target_os = "linux")]
use libc::{cpu_set_t, CPU_SET, CPU_ISSET, CPU_ZERO, sched_setaffinity, sched_getaffinity};

#[cfg(target_os = "windows")]
use kernel32::{GetCurrentThread, SetThreadAffinityMask};

extern crate cpuid_asm;

use std::{mem, thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};

macro_rules! pin_thread { ($cpu: expr) => {
        #[cfg(target_os = "linux")]
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
        #[cfg(target_os = "windows")]
        unsafe {
            SetThreadAffinityMask(GetCurrentThread(), 1 << $cpu);
        }
}}

fn print_matrix  (title: &str, result: Vec<Vec<u128>>,
                cpu_list: &Vec<usize>, ncpu: usize, opt: &Opt) {

    macro_rules! md_table { ($opt: expr) => {
        if $opt { " |" } else { "" };
        }
    }

    if opt.plot {
        println!("set title \"Inter-core one-way data latency between CPU cores [{}]\"", title);
        println!("set xlabel \"CPU\"");
        println!("set ylabel \"CPU\"");
        println!("set cblabel \"Latency (ns)\"");
        println!("$data << EOD");
    } else {
        println!("\n{}[{} (ns)]", if opt.md { "#### " } else { "" }, title );
    }

    print!(" CPU{}", md_table!(opt.md));
    for n in cpu_list {
        print!("{:>5}{}", n, md_table!(opt.md) );
    }
    println!();
    if opt.md {
        print!(" --: | ");
        for _n in 0..ncpu {
            print!(" --: | ");
        }
        println!();
    }
    for i in 0..ncpu {
        print!("{:>4}{}", i, md_table!(opt.md) );
        for j in 0..ncpu {
            print!("{:>5}{}", result[i][j], md_table!(opt.md));
        }
        println!();
    }

    if opt.plot {
        println!("EOD");
        print!("plot '$data' matrix rowheaders columnheaders using 2:1:3 ");
        println!("with image");
    }
    println!();
}

struct Opt {
    md:     bool,
    plot:   bool,
}

fn main() {
    let mut NSAMPLES: isize= 1_000;

    let mut opt = Opt { md: false, plot: false, };

    let opt_args: Vec<String> = std::env::args().collect();

    let mut i = 1;
    for _ in 1..opt_args.len() {
        if opt_args.len() <= i {
            break;
        }

        let v = &opt_args[i];

        if v == "-md" {
            opt.md   = true;
            opt.plot = false;
        } else if v == "-p" {
            opt.md   = false;
            opt.plot = true;
        } else if v == "-n" {
            NSAMPLES = opt_args[i+1].parse::<isize>()
                .expect("Please number");

            if NSAMPLES <= 1 {
                return;
            }
            i += 2;
            continue;
        } else {
            return;
        }
        i += 1;
    }

    let ncpu = cpuid_asm::CpuCoreCount::get();
    let mut cpus: Vec<usize> = Vec::new();

    #[cfg(target_os = "linux")]
    unsafe {
        let mut set = mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut set);

        let status = sched_getaffinity(0, mem::size_of::<cpu_set_t>(), &mut set);
        if status == -1 {
            eprintln!("sched_getaffinity failed");
            return;
        }

        for i in 0..(ncpu.total_thread) as usize {
            if CPU_ISSET(i, &set) {
                cpus.push(i);
            }
        }
    }
    #[cfg(target_os = "windows")]
    for i in 0..(ncpu.total_thread) as usize {
        cpus.push(i);
    }

    let ncpu: usize = cpus.len();
    
    let mut min_result: Vec<Vec<u128>> = vec![vec![0; ncpu]; ncpu];
    let mut avg_result: Vec<Vec<u128>> = vec![vec![0; ncpu]; ncpu];

    
    // TODO: align for cache line
    #[derive(Clone)]
    #[repr (C, align(64))]
    struct Seq {
        v: Arc<AtomicIsize>,
        _pad: Vec<Arc<AtomicIsize>>,
    }
    impl Seq {
        fn set() -> Seq {
            let c = cpuid_asm::CacheInfo::get();
            let line = c.l1d_line;

            return Seq {
                v: Arc::new(AtomicIsize::new(-1)),
                _pad: vec![Arc::new(AtomicIsize::new(0));
                            line as usize / mem::size_of::<isize>() - 1],
            }
        }
    }
    let seq1 = Seq::set();
    let seq2 = Seq::set();

    for i in 0..(ncpu) {
        for j in (i+1)..(ncpu) {

            let _seq1 = seq1.clone();
            let _seq2 = seq2.clone();

            let c = cpus[i];

            let t = thread::spawn(move || {
                pin_thread!(c);
                for _m in 0..100 {
                    for n in 0..NSAMPLES {
                        while _seq1.v.load(Ordering::Acquire) != n {}
                        _seq2.v.store(n, Ordering::Release);
                    }
                }
            });

            let mut perf: u128;
            let mut tmp: u128 = u128::MAX;
            let mut avg: u128 = 0;

            pin_thread!(cpus[j]);
            for _m in 0..100 {
                seq1.v.store(-1, Ordering::Release);
                seq2.v.store(-1, Ordering::Release);

                let start = time::Instant::now();

                for n in 0..NSAMPLES {
                    seq1.v.store(n, Ordering::Release);
                    while seq2.v.load(Ordering::Acquire) != n {}
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

    if opt.plot {
        println!("reset");
        println!("unset key");
        println!("set auto noextend");
        println!("set multiplot layout 2,1");
        println!("set size ratio 1");
        println!("set palette color negative");
    }
    
    print_matrix("min", min_result, &cpus, ncpu, &opt);
    print_matrix("avg", avg_result, &cpus, ncpu, &opt);

    if opt.plot {
        println!("unset multiplot");
    }
}

