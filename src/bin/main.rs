//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

#![feature(asm)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

extern crate cpuid_asm;
use cpuid_asm::*;
use cpuid_asm::feature_detect::*;

fn main() {

    let vendor_name = get_vendor_name();
    println!("Vendor: {}", vendor_name);

    let cpu_name = get_processor_name();
    println!("Processor Name: {}", cpu_name);

    let cpu_info = FamModStep::get();
    println!("Family: {0:#X} ({0}), Model: {1:#X} ({1}), Stepping: {2}",
        cpu_info.syn_fam, cpu_info.syn_mod, cpu_info.step);
    println!("Code Name: {}",
        codename::get_codename(cpu_info.syn_fam, cpu_info.syn_mod, cpu_info.step));
    println!();

    let cpu_feature = x86_64_abi::get();
    println!(" x86_64_v1: {}\n x86_64_v2: {}\n x86_64_v3: {}\n x86_64_v4: {}",
        cpu_feature.v1, cpu_feature.v2,
        cpu_feature.v3, cpu_feature.v4);
    println!();

    let cpu_cache = CacheInfo::get();
    println!(" L1d Cache: {} KiB, {}-byte line, {}-way",
        cpu_cache.l1d_size, cpu_cache.l1d_line, cpu_cache.l1d_way);
    println!(" L1i Cache: {} KiB, {}-byte line, {}-way",
        cpu_cache.l1i_size, cpu_cache.l1i_line, cpu_cache.l1i_way);
    println!(" L2 Cache: {} KiB, {}-byte line, {}-way",
        cpu_cache.l2_size, cpu_cache.l2_line, cpu_cache.l2_way);
    println!(" L3 Cache: {} MiB, {}-byte line, {}-way",
        cpu_cache.l3_size, cpu_cache.l3_line, cpu_cache.l3_way);
    println!();

}

