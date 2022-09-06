use crate::*;
use std::fmt::Write;

const PAD_WIDTH: usize = INPUT_WIDTH + OUTPUT_WIDTH + 1;
// pub const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; PAD_WIDTH]) };

const fn ln_pad() -> [u8; PAD_WIDTH+1] {
    let mut tmp = [b' '; PAD_WIDTH+1];
    tmp[0] = b'\n';

    return tmp;
}

pub const LN_PAD: &str = unsafe { std::str::from_utf8_unchecked(&ln_pad()) };

#[macro_export]
macro_rules! lnpad {
    () => {
        LN_PAD.to_string()
    };
}

pub(crate) struct Reg(u32);
impl Reg {
    pub(crate) fn new(reg: u32) -> Self {
        Self(reg)
    }

    fn to_bit_array(&self) -> [u8; 32] {
        let mut array = [0u8; 32];

        for (pos, v) in array.iter_mut().enumerate() {
            *v = ((self.0 >> pos) & 1) as u8;
        }

        return array;
    }

    pub(crate) fn to_bool_array(&self) -> [bool; 32] {
        let mut array = [false; 32];

        for (bit, flag) in self.to_bit_array().iter().zip(array.iter_mut()) {
            *flag = *bit != 0;
        }

        return array;
    }
}

pub(crate) fn str_detect_ftr(reg: u32, ftr_str: &[&str]) -> Vec<String> {
    let reg = Reg::new(reg).to_bool_array();
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for (r, ftr) in reg.iter().zip(ftr_str.iter()) {
        if *r && 0 < ftr.len() {
            buff.push(ftr.to_string());
        }
    }

    return buff;
}

pub(crate) fn align_mold_ftr(buff: &[String]) -> String {
    let mut rest: usize = PARSE_WIDTH;
    let mut mold = String::with_capacity(buff.len() * 48);
    let mut inner: String;

    const DECO_LEN: usize = " []".len();
    
    for v in buff {
        let len = v.len() + DECO_LEN;

        if len <= rest {
            inner = format!(" [{v}]");
            rest -= len;
        } else {
            inner = format!("{LN_PAD} [{v}]");
            rest = if PARSE_WIDTH < len {
                0
            } else {
                PARSE_WIDTH - len
            };
        }

        write!(mold, "{inner}").unwrap();
    }

    return mold;
}

/*
pub(crate) fn ftr_variant_expand(base_name: &str, flag_str: &[(bool, &str)]) -> String {
    let mut base = format!("{base_name}{{");

    for (flag, name) in flag_str {
        if *flag {
            write!(base, "{name},").unwrap();
        }
    }
    
    base.pop();
    write!(base, "}}").unwrap();

    return base;
}
*/
