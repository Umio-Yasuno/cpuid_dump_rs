//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

pub use core::arch::x86_64::CpuidResult;
// use crate::const_cpuid_dump::*;

#[path = "./parse/const_parse.rs"]
mod const_parse;
pub use const_parse::*;

#[path = "./parse/parse_util.rs"]
#[macro_use]
mod parse_util;
pub use parse_util::*;

#[path = "./parse/parse_generic.rs"]
mod parse_generic;
pub use parse_generic::*;

#[path = "./parse/parse_amd.rs"]
mod parse_amd;
pub use parse_amd::*;

#[path = "./parse/parse_intel.rs"]
mod parse_intel;
pub use parse_intel::*;
