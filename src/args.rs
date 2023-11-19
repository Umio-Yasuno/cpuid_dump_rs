use std::io;
use crate::TOTAL_WIDTH;
use crate::load_aida64_log;
use crate::{cpuid, CpuidDump, dump_all_threads, leaf_pool, CpuVendor, RawCpuid};

const LEAF_HEAD: &str = "       [Leaf.Sub]";
const LEAF_LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; LEAF_HEAD.len()]) };
const LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; TOTAL_WIDTH]) };

fn hex_head() -> String {
    const EAX: &str = "  [EAX]   ";
    const EBX: &str = "  [EBX]   ";
    const ECX: &str = "  [ECX]   ";
    const EDX: &str = "  [EDX]   ";

    format!("\
        {LEAF_HEAD}  {EAX} {EBX} {ECX} {EDX}\n\
        {LINE}\
    \n")
}

fn bin_head() -> String {
    const OUTPUT_LEN: usize = 35; // 32 [bits] + '_' * 3
    const PAD_LEN: usize = (OUTPUT_LEN - "[EAX / ECX]".len()) / 2;

    const PAD: &str = unsafe { std::str::from_utf8_unchecked(&[b' '; PAD_LEN-1]) };
    const OUTPUT_LINE: &str = unsafe { std::str::from_utf8_unchecked(&[b'='; OUTPUT_LEN]) };

    format!("\
        {LEAF_HEAD}  {PAD} [EAX / ECX] {PAD} \
        {PAD}  [EBX / EDX]\n\
        {LEAF_LINE}  {OUTPUT_LINE}  {OUTPUT_LINE}\
    \n")
}

pub fn dump_write(pool: &[u8]) -> io::Result<()> {
    use std::io::{Write, stdout};
    let mut out = stdout().lock();

    out.write_all(pool)?;
    Ok(())
}

fn default_name() -> String {
    let proc_name = libcpuid_dump::ProcName::get_trim_name().replace(' ', "_");
    /* Family, Model, Stepping */
    let fms = cpuid!(0x1, 0x0).eax;

    /* like "AMD_Ryzen_5_5600G_with_Radeon_Graphics_00A50F00.txt" */
    format!("{proc_name}_{fms:08X}.txt")
}

fn help_msg() {
    const MSG: &str = concat!(
        "CPUID Dump ", env!("CARGO_PKG_VERSION"), "\n",
        "https://github.com/Umio-Yasuno/cpuid_dump_rs\n",
        "\n",
        "USAGE:\n",
        "    cargo run -- [options ..] or <cpuid_dump> [options ..]\n",
        "\n",
        "FLAGS:\n",
        "    -a, -all\n",
        "        Display result for all threads.\n",
        "    -r, -raw\n",
        "        Display raw/hex result.\n",
        "    -bin\n",
        "        Display binary result.\n",
        "    -c, -compat\n",
        "        Display the same format as `cpuid -r` (cpuid by Todd Allen)\n",
        "    -full\n",
        "        Combine \"-disp-zero\" and \"-no-diff\"\n",
        "    -disp-zero\n",
        "        Display result even if E[ABCD]X are zero.\n",
        "    -no-diff\n",
        "        Do not omit diff when all threads execution\n",
        "\n",
        "OPTIONS:\n",
        "    --l <u32>, --leaf <u32>\n",
        "        Display result only for the specified value, the value is Leaf/InputEAX <u32>.\n",
        "        e.g. --leaf 1, --leaf 0x8000_0008,\n",
        "    --sub_leaf <u32>, --subleaf <u32>\n",
        "        Display result only for the specified value, the value is Sub_Leaf/InputECX <u32>.\n",
        "    --s <path/filename>, --save <path/filename>\n",
        "        Save dump result to text file.\n",
        "        If there is no path/filename argument, will be used \"./<processor_name>\".\n",
        "    --aida64 <path/filename>\n",
    );

    println!("{MSG}")
}

#[derive(Debug, Copy, Clone)]
pub enum DumpFormat {
    Raw,
    Binary,
    Parse,
    CompatCpuid,
    Debug,
}

impl DumpFormat {
    pub fn head_fmt(&self) -> String {
        match self {
            Self::Binary => bin_head(),
            Self::Debug |
            Self::CompatCpuid => "".to_string(),
            _ => hex_head(),
        }
    }

    pub fn rawcpuid_fmt_func(&self) -> fn(&RawCpuid, &CpuVendor) -> String {
        match self {
            Self::Raw => RawCpuid::raw_fmt,
            Self::Binary => RawCpuid::bin_fmt,
            Self::Parse => RawCpuid::parse_fmt,
            Self::CompatCpuid => RawCpuid::compat_fmt,
            Self::Debug => RawCpuid::debug_fmt,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MainOpt {
    pub fmt: DumpFormat,
    pub dump_all: bool,
    pub save_path: Option<String>,
    pub leaf: Option<(u32, u32)>,
    pub skip_zero: bool,
    pub diff: bool,
    pub load_aida64: Option<String>,
}

impl Default for MainOpt {
    fn default() -> Self {
        Self {
            fmt: DumpFormat::Parse,
            dump_all: false,
            save_path: None,
            leaf: None,
            skip_zero: true,
            diff: true,
            load_aida64: None,
        }
    }
}

impl MainOpt {
    fn parse_value(raw_value: &str) -> u32 {
        /* for like "0x8000_0000" */
        let raw_value = raw_value.replace('_', "");

        if let Some(stripped) = raw_value.strip_prefix("0x") {
            u32::from_str_radix(stripped, 16).unwrap()
        } else {
            raw_value.parse::<u32>().unwrap()
        }
    }

    pub fn main_parse() -> Self {
        let mut opt = MainOpt::default();
        let mut skip = false;

        let args: Vec<String> = std::env::args().collect();

        for (idx, arg) in args.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            if !arg.starts_with('-') {
                // eprintln!("Unknown option: {}", args[i]);
                continue;
            }

            let arg = arg.trim_start_matches('-');

            match arg {
                "a" | "all" => {
                    opt.dump_all = true;
                },
                "r" | "raw" => {
                    opt.fmt = DumpFormat::Raw;
                    // opt.skip_zero = false;
                },
                "s" | "save" => {
                    use std::path::Path;
                    let mut path = default_name();

                    if let Some(v) = args.get(idx+1) {
                        if v.starts_with('-') { 
                            opt.save_path = Some(path);
                            continue;
                        }

                        path = if Path::new(v).is_dir() {
                            format!("{v}{path}")
                        } else {
                            v.to_string()
                        };
                    }

                    opt.save_path = Some(path);
                },
                "aida64" => {
                    opt.load_aida64 = args.get(idx+1).cloned();

                    if opt.load_aida64.is_none() {
                        std::process::exit(1);
                    }
                },
                "leaf" => {
                    opt.skip_zero = false;
                    opt.diff = false;

                    if let Some(v) = args.get(idx+1) {
                        let leaf = Self::parse_value(v);
                        opt.leaf = Some((leaf, 0x0));
                    } else {
                        eprintln!("missing argument <u32> to \"--leaf\"");
                    };
                },
                "subleaf" | "sub_leaf" | "sub-leaf" => {
                    if let (Some((leaf, _)), Some(sub_leaf)) = (opt.leaf, args.get(idx+1)) {
                        let sub_leaf = Self::parse_value(sub_leaf);
                        opt.leaf = Some((leaf, sub_leaf));
                    } else {
                        eprintln!("missing argument \"--sub_leaf <u32>\"");
                    };
                }
                "bin" => {
                    opt.fmt = DumpFormat::Binary;
                },
                "c" | "compat" => {
                    opt.dump_all = true;
                    opt.fmt = DumpFormat::CompatCpuid;
                    opt.skip_zero = false;
                    opt.diff = false;
                },
                "debug" => {
                    opt.fmt = DumpFormat::Debug
                },
                "h" | "help" => {
                    help_msg();
                    std::process::exit(0);
                },
                "disp-zero" => {
                    opt.skip_zero = false;
                },
                "no-diff" => {
                    opt.diff = false;
                },
                "full" => {
                    opt.skip_zero = false;
                    opt.diff = false;
                },
                _ => {
                    eprintln!("Unknown option: {}", arg);
                    help_msg();
                    std::process::exit(1);
                },
            }
        }

        opt
    }

    pub fn dump_pool(&self) -> Vec<u8> {
        let leaf_pool = leaf_pool();

        if self.dump_all {
            return dump_all_threads(&leaf_pool, self.skip_zero, self.fmt, self.diff).into_bytes();
        }

        let cpuid_dump = CpuidDump::new(&leaf_pool, self.skip_zero);

        cpuid_dump.top_disp(self.fmt).into_bytes()
    }

    pub fn only_leaf(&self, leaf: u32, sub_leaf: u32) -> io::Result<()> {
        let tmp = if self.dump_all {
            dump_all_threads(&[(leaf, sub_leaf)], self.skip_zero, self.fmt, self.diff)
        } else {
            let cpuid_dump = CpuidDump::new(&[(leaf, sub_leaf)], self.skip_zero);
            cpuid_dump.top_disp(self.fmt)
        };

        dump_write(&tmp.into_bytes())?;

        Ok(())
    }

    pub fn save_file(&self, save_path: &str) -> io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let pool = self.dump_pool();

        let mut f = File::create(save_path)?;

        f.write_all(&pool)?;
        println!("Output to \"{save_path}\"");

        Ok(())
    }

    pub fn load_aida64(&self, path: &String) -> io::Result<()> {
        let log = std::fs::read_to_string(path)?;
        let mut vec_cpuid_dump = load_aida64_log::parse_aida64(&log);
        let mut cpuid_dump_iter = vec_cpuid_dump.iter_mut();
        let first = cpuid_dump_iter.next().unwrap();

        let s = first.top_disp(self.fmt);
        let ss: String = cpuid_dump_iter.map(|cpuid_dump| {
            if self.diff {
                let mut first_rawcpuid_pool = first.rawcpuid_pool.iter();

                cpuid_dump.rawcpuid_pool.retain(|sub| {
                    let Some(first) = first_rawcpuid_pool.next() else { return false };
                    first != sub
                });
            }

            cpuid_dump.disp(self.fmt)
        }).collect();

        dump_write(&format!("{s}{ss}").into_bytes())?;

        Ok(())
    }
}
