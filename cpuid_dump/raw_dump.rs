fn raw_pool() -> Vec<u8> {
    let mut pool: Vec<u8> = Vec::new();
    let cpuid_pool = cpuid_pool();

    for cpuid in cpuid_pool {
        let tmp = cpuid.raw_fmt().into_bytes();
        pool.extend(tmp);
    }

    return pool;
}

pub fn raw_dump() {
    use std::io::{BufWriter, Write, stdout};

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let pool = raw_pool();

    out.write(&pool).unwrap();
}

pub fn raw_dump_all() {
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
