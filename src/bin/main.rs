extern crate cpuid_asm;
use cpuid_asm::*;

fn main() {
    println!();
/*
    let a: [u32; 4] = [0; 4];
    let _AX: u32 = 0x80000000;
    
    cpuid_dump();
*/
    let vendor_name = get_vendor_name();
    println!("Vendor: {}", vendor_name);

    let cpu_name = get_processor_name();
    println!("Processor Name: {}", cpu_name);

    let cpu_info = FamModStep::get();
    println!("Family: 0x{0:X} ({0}), Model: 0x{1:X} ({1}), Stepping: {2}",
        cpu_info.syn_fam, cpu_info.syn_mod, cpu_info.step);

    println!();
    let core_count = CpuCoreCount::get();
    println!("HTT/SMT: {}", core_count.has_htt);
    println!("{}-Core/{}-Thread (Thread per Core: {})",
        core_count.phy_core, core_count.total_thread, core_count.thread_per_core);
    println!("Core ID: {}", core_count.core_id);

    println!();
    let cpu_feature = CpuFeature::get();
    println!(" x86_64_v1: {}\n x86_64_v2: {}\n x86_64_v3: {}\n x86_64_v4: {}",
        cpu_feature.x86_64_v1, cpu_feature.x86_64_v2, cpu_feature.x86_64_v3, cpu_feature.x86_64_v4);

/*
    CacheInfo::get(cpu_info.syn_fam);
    cache_info_amd();
    cache_info();
    cpu_feature();
*/
}

