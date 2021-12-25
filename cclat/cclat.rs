#![feature(asm)]

/*
use std::{mem, thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
*/

fn main() {
    //  println!("NOP");

    unsafe {
        let x = 0;
        let start: u64 = core::arch::x86_64::__rdtscp(&mut 0);
        asm!("mov {} {}",
            out(reg) _,
            in(reg) x,
        );
        println!("{}", start);
    }
    return;


for _ in 0..10 {
    unsafe {
        asm!("nop", "nop");
        asm!("nop", "nop");
        asm!("nop", "nop");
        asm!(
            "nop",
            "nop",
            "nop",
        );
    }
}
/*
    let x = Arc::new(AtomicIsize::new(-1));

    //  println!("{}", x.load(Ordering::Acquire));
    //  let mut tmp: u128 = u128::MAX;
    let mut tmp: u64 = 0u64;

    println!("TSC: {}",
        unsafe {
            core::arch::x86_64::__rdtscp(&mut 3)
        }
    );

    for _ in 0..10000 {
        //  let start = time::Instant::now();
    unsafe {
        let start = core::arch::x86_64::_rdtsc();

        let _ = x.load(Ordering::Acquire);

        //  let perf = start.elapsed();
        let end = core::arch::x86_64::_rdtsc();

        //  tmp = std::cmp::min(tmp, perf.as_nanos());
        tmp = end - start;
    }
    }
        println!("Load: {} ns", tmp);
*/
}
