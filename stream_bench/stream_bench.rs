use std::{mem, time, ptr};

const NSAMPLE: usize = 50;
const MAX_ARRAY_SIZE: usize = 18;

macro_rules! msize {
    ($type: ty, $array: expr) => {
        println!("{:>4} MiB ({:>8} KiB)",
            mem::size_of::<$type>() * $array / (1 << 20),
            mem::size_of::<$type>() * $array / (1 << 10)
        );
    }
}

fn main() {
    println!("Copy (8-byte read + 8-byte write)");

    //  let mut src  = Vec::<f64>::with_capacity((1 << 18) * 1024);
    //  let mut _dst = Vec::<f64>::with_capacity((1 << 18) * 1024);

    for shift in 1..MAX_ARRAY_SIZE {
        let array: usize = (1 << shift)*1024;

        let mut src  = vec![0f64; array];
        let mut _dst = vec![1f64; array];
        
        for i in 0..array {
            src[i] = (i as f64) * 0.1;
        }

        let tmp_sec = time::Instant::now();
                //  _dst[..array].copy_from_slice(&src[..array]);
                let mut tmp = 0f64;
            for i in 0..NSAMPLE {
                for _i in &src {
                    tmp += _i;
                }
                unsafe {
                    //  memcpy
                    /*
                    ptr::copy_nonoverlapping(
                        src.as_ptr(),
                        _dst.as_mut_ptr(),
                        _dst.len()
                    );
                    */
                    //  memset
                    //  _dst.fill(0f64);

                }
            }
        let tmp_sec = tmp_sec.elapsed();
        let tmp_sec = tmp_sec.as_secs_f64();

        println!("{}", tmp);
    /*
        if shift == 1 {
            println!("{}", tmp_sec);
        }
    */
    /*
        let mut err_c = 0;

        for i in 0..array {
            if _dst[i] != src[i] {
                err_c += 1;
            }
        }
    
        if err_c != 0 {
            println!("Memory Error Count: {}", err_c);
        }
    */
        msize!(f64, array);

        let mbw = (mem::size_of::<f64>() *1 *array) as f64 / (1 << 20) as f64;
        let end_sec = tmp_sec / NSAMPLE as f64;

        println!("    {:>6.0} MiB/s ({:.8}s)",
            mbw / end_sec,
            end_sec
        );
    }
}
