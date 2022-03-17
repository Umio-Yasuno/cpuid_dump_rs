//  Copyright (c) 2021 Umio Yasuno
//  SPDX-License-Identifier: MIT

#[path = "./codename/codename.rs"]
mod codename;
pub use codename::*;

#[path = "./codename/codename_amd.rs"]
mod codename_amd;
pub use codename_amd::*;

#[path = "./codename/codename_intel.rs"]
mod codename_intel;
pub use codename_intel::*;
