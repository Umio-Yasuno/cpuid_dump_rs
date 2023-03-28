## Usage
```
USAGE:
     cargo run -- [options ..] or <cpuid_dump> [options ..]

FLAGS:
     -a, -all
         Display result for all threads.
     -r, -raw
         Display raw/hex result.
     -bin
         Display binary result.
     -c, -compat
         Display the same format as `cpuid -r` (cpuid by Todd Allen)
     -full
         Combine "-disp-zero" and "-no-diff"
     -disp-zero
         Display result even if E[ABCD]X are zero.
     -no-diff
         Do not omit diff when all threads execution

OPTIONS:
     --l <u32>, --leaf <u32>
         Display result only for the specified value, the value is Leaf/InputEAX <u32>.
         e.g. --leaf 1, --leaf 0x8000_0008,
     --sub_leaf <u32>, --subleaf <u32>
         Display result only for the specified value, the value is Sub_Leaf/InputECX <u32>.
     --s <path/filename>, --save <path/filename>
         Save dump result to text file.
         If there is no path/filename argument, will be used "./<processor_name>".
```

## Use as Library
```
[dependencies]
libcpuid_dump = { git = "https://github.com/Umio-Yasuno/cpuid_dump_rs" }
```

## Dump Results
 * [AMD_Ryzen_5_2600_00800F82h](./dump_result/AMD_Ryzen_5_2600_00800F82h.txt)
 * [AMD_Ryzen_5_5600G_with_Radeon_Graphics](./dump_result/AMD_Ryzen_5_5600G_with_Radeon_Graphics_00A50F00.txt)
 * [AMD_Ryzen_5_5600G_with_Radeon_Graphics (bin fmt)](./dump_result/AMD_Ryzen_5_5600G_with_Radeon_Graphics_00A50F00_bin.txt)

## Documents
### CPUID
 * [Intel® Architecture Instruction Set Extensions Programming Reference](https://software.intel.com/content/www/us/en/develop/download/intel-architecture-instruction-set-extensions-programming-reference.html)
 * [Developer Guides, Manuals & ISA Documents - AMD](https://developer.amd.com/resources/developer-guides-manuals/)
    * [AMD64 Architecture Programmer’s Manual, Volume 3: General-Purpose and System Instructions, 24594 - 24594.pdf](https://www.amd.com/system/files/TechDocs/24594.pdf)
 * [CPUID - Wikipedia](https://en.wikipedia.org/wiki/CPUID)
 * <https://gitlab.com/x86-psABIs/x86-64-ABI>
