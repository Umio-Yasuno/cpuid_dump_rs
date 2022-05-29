use crate::*;
use std::fmt::Write;

pub const PAD_WIDTH: usize = INPUT_WIDTH + OUTPUT_WIDTH + 1;

#[macro_export]
macro_rules! pad {
    () => { " ".repeat(PAD_WIDTH) };
}

#[macro_export]
macro_rules! padln {
    () => { format!("\n{}", pad!()) };
}

pub struct Reg(u32);
impl Reg {
    pub fn new(reg: u32) -> Reg {
        Reg(reg)
    }

    pub fn to_bit_array(&self) -> [u8; 32] {
        let mut array = [0u8; 32];

        for (pos, v) in array.iter_mut().enumerate() {
            *v = ((self.0 >> pos) & 1) as u8;
        }

        return array;
    }

    pub fn to_bool_array(&self) -> [bool; 32] {
        let mut array = [false; 32];

        for (bit, flag) in self.to_bit_array().iter().zip(array.iter_mut()) {
            *flag = *bit != 0;
        }

        return array;
    }
}

pub fn str_detect_ftr(reg: u32, ftr_str: &[&str]) -> Vec<String> {
    let reg = Reg::new(reg).to_bool_array();
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for (r, ftr) in reg.iter().zip(&ftr_str[..]) {
        if *r && 0 < ftr.len() {
            buff.push(ftr.to_string());
        }
    }

    return buff;
}

pub fn align_mold_ftr(buff: &[String]) -> String {
    let mut rest: usize = PARSE_WIDTH;
    // let mut len: usize;
    let mut mold = String::new();
    let mut inner: String;

    const DECO_LEN: usize = " []".len();
    
    for v in buff {
        let len = v.len() + DECO_LEN;

        if len <= rest {
            inner = format!(" [{}]", v);
            rest -= len;
        } else {
            inner = format!("{} [{}]", padln!(), v);
            if PARSE_WIDTH < len {
                /*
                inner = format!(" [{}{}  {}]",
                    &v[..rest], padln!(), &v[rest..]);
                rest = PARSE_WIDTH - (len - rest);
                */
                rest = 0;
            } else {
                // inner = format!("{} [{}]", padln!(), v);
                rest = PARSE_WIDTH - len;
            };
        }
        // mold.push_str(&inner);
        write!(mold, "{inner}").unwrap();
    }

    return mold;
}

pub fn ftr_variant_expand(base_name: &str, flag_str: &[(bool, &str)]) -> String {
    let mut base = format!("{base_name}{{");

    for (flag, name) in flag_str {
        if *flag {
            // base.push_str(&format!("{name},"));
            write!(base, "{name},").unwrap();
        }
    }
    
    base.pop();
    // base.push_str("}");
    write!(base, "}}").unwrap();

    return base;
}
