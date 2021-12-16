extern crate cpuid_asm;
use cpuid_asm::*;

fn main() {
    println!("{}", get_processor_name());
}
