use crate::*;

#[macro_export]
macro_rules! has_ftr {
    ($ftr_bool: expr, $name_str: expr) => {
        if $ftr_bool {
            $name_str
        } else {
            ""
        }
    };
    ($ftr_bool: expr, $name_str: expr, $else_ftr: expr, $else_name: expr) => {
        if $ftr_bool {
            $name_str
        } else if $else_ftr {
            $else_name
        } else {
            ""
        }
    };
}

/*
#[macro_export]
macro_rules! push {
    ($buff: expr, $str: expr) => {
        $buff.push($str.to_string())
    };
}
*/

#[macro_export]
macro_rules! pad {
    () => { " ".repeat(62) };
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
        let mut bit = [0u8; 32];
        for i in 0..32 {
            bit[i] = ((self.0 >> i) & 1) as u8;
        }
        return bit;
    }
    pub fn to_bool_array(&self) -> [bool; 32] {
        let mut array = [false; 32];
        for (idx, v) in self.to_bit_array().iter().enumerate() {
            array[idx] = *v == 1;
        }
        return array;
    }
    /*
    pub fn to_bitvec(&self) -> Vec<u8> {
        self.to_bit_array().to_vec()
    }
    pub fn to_boolvec(&self) -> Vec<bool> {
        self.to_bool_array().to_vec()
    }
    */
}

pub fn cpu_name(tmp: &CpuidResult) -> String {
    let mut name = Vec::with_capacity(64);

    [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx].iter().for_each(
        |val| name.extend(val.to_le_bytes().iter().map(
            // replace from \u0000..\u001F (<Control>) to \u0020 (space)
            |&byte| if byte <= 0x1F { 0x20 } else { byte }
        ))
    );

    return String::from_utf8(name).unwrap();
}

pub fn str_detect_ftr(reg: u32, ftr_str: &[&str]) -> Vec<String> {
    let len = ftr_str.len();
    let len = if 32 < len {
        32
    } else {
        len
    };

    let reg = Reg::new(reg).to_bool_array();
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for id in 0..len {
        if !reg[id] || ftr_str[id].len() < 1 {
            continue;
        }
        buff.push(ftr_str[id].to_string());
    }

    return buff;
}

pub fn detect_ftr(reg: u32, ftr_str: Vec<String>) -> Vec<String> {
    let len = ftr_str.len();
    let len = if 32 < len {
        32
    } else {
        len
    };

    let reg = Reg::new(reg).to_bool_array();
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for id in 0..len {
        if !reg[id] || ftr_str[id].len() < 1 {
            continue;
        }
        buff.push(ftr_str[id].to_string());
    }

    return buff;
}

/*
pub fn concat_string(src: Vec<String>) -> String {
    let mut dst = String::new();

    src.iter().for_each(
        |val| dst.push_str(val.as_str())
    );

    return dst;
}
*/

pub fn concat_string_from_slice(src: &[String]) -> String {
    let mut dst = String::new();

    src.iter().for_each(
        |val| dst.push_str(val.as_str())
    );

    return dst;
}

/*
pub fn packed_mold_ftr(buff: &[String]) -> String {
    let to_line = |buff: &[String]| -> String {
        let mut tmp = String::new();

        for v in buff {
            tmp.push_str(format!("[{}] ", v).as_str());
        }
        tmp
    };
        
    let post_mold = to_line(buff);
    let mut post_mold = post_mold.as_str();

    const LEN: usize = 32;
    let mut split_tmp = "";
    let mut post_result = String::new();

    loop {
        if LEN < post_mold.len() {
            (split_tmp, post_mold) = post_mold.split_at(LEN);
            post_result.push_str(format!(" {}{}", split_tmp, padln!()).as_str());
        } else {
            post_result.push_str(format!(" {}", post_mold).as_str());
            println!("Debug Result: {}", post_result);

            return post_result;
        }
    }
}
*/

pub fn align_mold_ftr(buff: &[String]) -> String {
    const WIDTH: usize = 32;
    let mut rest: usize = WIDTH;
    let mut len: usize;
    let mut mold = String::new();
    let mut _inner = String::new();
    
    for v in buff {
        len = v.len() + 3;

        if len <= rest {
            _inner = format!(" [{}]", v);
            rest -= len;
        } else {
            _inner = format!("{} [{}]{}", padln!(), v, "");

            rest = if WIDTH < len {
                0
            } else {
                WIDTH - len
            };
        }
        mold.push_str(_inner.as_str());
    }

    return mold;
}

/*
pub fn mold_ftr(buff: Vec<String>) -> String {
    return align_mold_ftr(&buff);
}
*/
