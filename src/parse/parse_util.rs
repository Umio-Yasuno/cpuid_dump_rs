use crate::{INPUT_WIDTH, OUTPUT_WIDTH, PARSE_WIDTH};

const PAD_WIDTH: usize = INPUT_WIDTH + OUTPUT_WIDTH + 1;
// pub const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; PAD_WIDTH]) };

const fn ln_pad() -> [u8; PAD_WIDTH+1] {
    let mut tmp = [b' '; PAD_WIDTH+1];
    tmp[0] = b'\n';

    tmp
}

pub const LN_PAD: &str = unsafe { std::str::from_utf8_unchecked(&ln_pad()) };

#[macro_export]
macro_rules! lnpad {
    () => {
        LN_PAD.to_string()
    };
}

fn u32_to_bool_array(reg: u32) -> [bool; 32] {
    let mut flags = [false; 32];

    for (pos, flag) in flags.iter_mut().enumerate() {
        *flag = ((reg >> pos) & 0b1) != 0;
    }

    flags
}

pub(crate) fn str_detect_ftr(reg: u32, ftr_str: &[&str]) -> Vec<String> {
    let flags = u32_to_bool_array(reg);
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for (flag, ftr) in flags.iter().zip(ftr_str.iter()) {
        if *flag && !ftr.is_empty() {
            buff.push(ftr.to_string());
        }
    }

    buff
}

pub(crate) fn align_mold_ftr(ftrs: &[String]) -> String {
    let mut rest: usize = PARSE_WIDTH;
    let mut mold = String::with_capacity(ftrs.len() * 48);

    const DECO_LEN: usize = "[] ".len();
    
    for f in ftrs {
        let len = f.len() + DECO_LEN;

        if len <= rest {
            rest -= len;
        } else {
            mold += LN_PAD;
            rest = PARSE_WIDTH.saturating_sub(len);
        }

        [ "[", f, "] " ].map(|s| mold.push_str(s));
    }

    mold
}
