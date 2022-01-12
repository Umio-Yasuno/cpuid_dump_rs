extern crate cpuid_asm;
use cpuid_asm::*;
use std::arch::x86_64::CpuidResult;

fn main() {
/*
    println!("{}", get_proc_name());
    let tmp_0bh = CpuidResult {
        eax: 0x00000001,
        ebx: 0x00000018,
        ecx: 0x00000201,
        edx: 0x0000004C,
    };
    let tmp: u32 = 0x001F8FF7;
    println!("HFI: {}, EHFI: {}",
        (tmp >> 19) & 1,
        (tmp >> 23) & 1,
    );
*/
    println!("Test Leaf: 0xB");

    let cpuid_0bh = CpuidResult {
        eax: 0x00000001,
        ebx: 0x00000002,
        ecx: 0x00000100,
        edx: 0x00000000,
    };
    let cpuid_0bh = CpuidResult {
        eax: 0x00000007,
        ebx: 0x00000018,
        ecx: 0x00000201,
        edx: 0x00000000,
    };
    println!("Uniq topo: {}", cpuid_0bh.eax & 0xF);
    println!("Number of logical processors at this level type: {}", cpuid_0bh.ebx & 0xFFFF);
    println!("Level num (same ecx): {}", cpuid_0bh.ecx & 0xFF);
    println!("Level type: {}", (cpuid_0bh.ecx >> 8) & 0xFF);
    println!("x2APIC ID: {}", cpuid_0bh.edx);
}
