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
     -disp-zero
         Display result even if E[ABCD]X are zero.

OPTIONS:
     --l <u32>, --leaf <u32>
         Display result only for the specified value, the value is Leaf/InputEAX <u32>.
         e.g. --leaf 1, --leaf 0x8000_0008,
     --sub_leaf <u32>, --sub-leaf <u32>
         Display result only for the specified value, the value is Sub-Leaf/InputECX <u32>.
     --pin <usize>, --pin_threads <usize>
         Display result for the specified thread.
     --s <path/filename>, --save <path/filename>
         Save dump result to text file.
         If there is no path/filename argument, will be used "./<processor_name>".
```

## Dump Results
 * [AMD_Ryzen_5_2600_00800F82h](./dump_result/AMD_Ryzen_5_2600_00800F82h.txt)
 * [AMD_Ryzen_5_5600G_with_Radeon_Graphics](./dump_result/AMD_Ryzen_5_5600G_with_Radeon_Graphics.txt)
 * [AMD_Ryzen_5_5600G_with_Radeon_Graphics (bin fmt)](./dump_result/AMD_Ryzen_5_5600G_with_Radeon_Graphics_bin.txt)

## Document
### CPUID
 * [Intel® Architecture Instruction Set Extensions Programming Reference](https://software.intel.com/content/www/us/en/develop/download/intel-architecture-instruction-set-extensions-programming-reference.html)
 * [Developer Guides, Manuals & ISA Documents - AMD](https://developer.amd.com/resources/developer-guides-manuals/)
    * [AMD64 Architecture Programmer’s Manual, Volume 2: System Programming - 24593.pdf](https://www.amd.com/system/files/TechDocs/24593.pdf)
 * [CPUID - Wikipedia](https://en.wikipedia.org/wiki/CPUID)
 * <https://gitlab.com/x86-psABIs/x86-64-ABI>
