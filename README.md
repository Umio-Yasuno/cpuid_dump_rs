## Usage

```
USAGE:
     cargo run -- [options ..] or <cpuid_dump> [options ..]

OPTIONS:
     -a, -all
         Display result for all threads.
     -r, -raw
         Display raw/hex result.
     --l <u32>, --leaf <u32>
         Display result only for the specified value, the value is Leaf/InputEAX <u32>.
         e.g. --leaf 1, --leaf 0x8000_0008,
                 --sub_leaf <u32>, --sub-leaf <u32>
         Display result only for the specified value, the value is Sub-Leaf/InputECX <u32>.
     -bin
         Display binary result, for --leaf/--sub_leaf option.
     --pin <usize>, --pin_threads <usize>
         Display result for the specified thread.
     --s <path/filename>, --save <path/filename>
         Save dump result to text file.
         If there is no path/filename argument, will be used "./<processor_name>".
```

## Document
### Rust
 * [asm - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html)
### CPUID
 * [Intel® Architecture Instruction Set Extensions Programming Reference](https://software.intel.com/content/www/us/en/develop/download/intel-architecture-instruction-set-extensions-programming-reference.html)
 * [Developer Guides, Manuals & ISA Documents - AMD](https://developer.amd.com/resources/developer-guides-manuals/)
 * [CPUID - Wikipedia](https://en.wikipedia.org/wiki/CPUID)
 * <https://gitlab.com/x86-psABIs/x86-64-ABI>
