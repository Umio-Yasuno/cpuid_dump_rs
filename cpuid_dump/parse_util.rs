use crate::*;

#[macro_export]
macro_rules! print_cpuid {
    ($in_eax: expr, $in_ecx: expr, $cpuid: expr) => {
        print!(
            "  0x{:08X}_x{:1X}:  0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X} ",
            $in_eax, $in_ecx, $cpuid.eax, $cpuid.ebx, $cpuid.ecx, $cpuid.edx
        )
    };

    ($out: expr, $in_eax: expr, $in_ecx: expr, $cpuid: expr) => {
        write!(
            $out,
            "    0x{:08X} 0x{:1X}: eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ",
            $in_eax, $in_ecx, $cpuid.eax, $cpuid.ebx, $cpuid.ecx, $cpuid.edx
        ).unwrap()
    };
}

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

#[macro_export]
macro_rules! push {
    ($buff: expr, $str: expr) => {
        $buff.push($str.to_string())
    };
}

#[macro_export]
macro_rules! flag {
    ($pos: expr, $reg: expr) => {
        $pos & $reg != 0
    };
}

#[macro_export]
macro_rules! pad {
    () => {
        " ".repeat(62)
    };
}

#[macro_export]
macro_rules! padln {
    () => {
        format!("\n{}", pad!())
    };
}

pub struct Reg(u32);
impl Reg {
    pub fn new(reg: u32) -> Reg {
        Reg(reg)
    }

    pub fn to_bitvec(&self) -> Vec<u8> {
        let mut bit = [0u8; 32];
        for i in 0..32 {
            bit[i] = ((self.0 >> i) & 1) as u8;
        }
        return bit.to_vec();
    }

    pub fn to_boolvec(&self) -> Vec<bool> {
        self.to_bitvec().iter().map(|&x| x == 1 ).collect()
    }
}

pub fn cpu_name(tmp: &CpuidResult) -> String {
    let mut name = Vec::with_capacity(48);
    let reg = [tmp.eax, tmp.ebx, tmp.ecx, tmp.edx];

    reg.iter().for_each(
        |&val| name.extend(&val.to_le_bytes())
    );

    return String::from_utf8(name).unwrap();
}

pub fn to_vstring(src: Vec<&str>) -> Vec<String> {
    src.iter().map( |v| v.to_string() ).collect::<Vec<String>>()
}

pub fn detect_ftr(reg: u32, ftr_str: Vec<String>) -> Vec<String> {
    let len = if 32 < ftr_str.len() {
        32
    } else {
        ftr_str.len()
    };
    let reg = Reg::new(reg).to_boolvec();
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for id in 0..len {
        if !reg[id] || ftr_str[id].len() < 1 {
            continue;
        }
        push!(buff, ftr_str[id]);
    }

    return buff;
}

pub fn concat_string(src: Vec<String>) -> String {
    let mut dst = String::new();

    src.iter().for_each(
        |val| dst.push_str(val.as_str())
    );

    return dst;
}

pub fn mold_ftr(buff: Vec<String>) -> String {
    let total_len = buff.len();
    let mut mold = String::new();

    for (c, v) in buff.iter().enumerate() {
        let c = c + 1;
        let long_str = 9 < v.len();
        let line = 
            if (c % 3) == 0 && c != total_len
            && !long_str {
                padln!()
            } else {
                "".to_string()
            };

        let inner = if long_str {
            format!(" [{}]{}", v, padln!())
        } else {
            format!(" [{}]", v)
        };

        mold.push_str(inner.as_str());
        mold.push_str(line.as_str());
    }
    return mold;
}
