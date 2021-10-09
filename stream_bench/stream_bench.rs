#![feature(asm)]

use std::{mem, time, thread, ptr};
use std::cell::RefCell;

const array: usize = 134_217_728;

fn main() {
    println!("Stream");

    let mut src  = vec![0.0; array];
    let mut _dst = vec![0.0; array];

    for i in 0..array {
        src[i] = i as f64 * 0.1;
    }

for _i in 0..10 {
    let start = time::Instant::now();
        stream_copy(&src, &mut _dst);
    let end = start.elapsed();
    let end = end.as_secs_f64();

    print!("\t{} MiB\n", mem::size_of::<usize>() * array / (1 << 20) );
    let mbw: f64 = mem::size_of::<f64>() as f64 * array as f64 / (1 << 20) as f64;
    println!("ST: {:.3} MiB/s ({}s)", mbw / end, end);

    for i in 0..array {
        if _dst[i] != src[i] {
            println!("Memory Error");
            return;
        }
    }
}

}

#[inline(always)]
fn stream_copy(src: &Vec<f64>, _dst: &mut Vec<f64>) {
/*
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), _dst.as_mut_ptr(), src.len());
    }
*/
    _dst.copy_from_slice(&src[..]);
}
