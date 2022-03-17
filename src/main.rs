extern crate cpuid_asm;
use cpuid_asm::*;

fn main() {
    println!("Level: {}", MicroArchLevel::level_u8());
}
