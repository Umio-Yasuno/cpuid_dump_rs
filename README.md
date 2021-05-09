## ToDo

 * [x] Cache info of Intel CPU 
 * [x] Core Affinity

## Document
### Rust
 * [asm - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html)
### CPUID
 * [Intel® Architecture Instruction Set Extensions Programming Reference](https://software.intel.com/content/www/us/en/develop/download/intel-architecture-instruction-set-extensions-programming-reference.html)
 * [Developer Guides, Manuals & ISA Documents - AMD](https://developer.amd.com/resources/developer-guides-manuals/)
 * [CPUID - Wikipedia](https://en.wikipedia.org/wiki/CPUID)
 * <https://gitlab.com/x86-psABIs/x86-64-ABI>

### x86-64-v1..4

| Level Name | CPU Feature | Example Instruction |
| :-- | :--: | :--: |
| v1/(baseline) | CMOV  | cmov |
|               | CX8   | cmpxchg8b |
|               | FPU   | fld |
|               | FXSR  | fxsave |
|               | MMX   | emms |
|               | OSFXSR | fxsave |
|               | SCE | syscall |
|               | SSE | cvtss2si |
|               | SSE2 | cvtpi2pd |
| x86-64-v2     | CMPXCHG16B    | cmpxchg16b |
|               | LAHF-SAHF     | lahf |
|               | POPCNT        | popcnt |
|               | SSE3          | addsubpd |
|               | SSE4_1        | blendpd |
|               | SSE4_2        | pcmpestri |
|               | SSSE3         | phaddd |
| x86-64-v3     | AVX       | vzeroall |
|               | AVX2      | vpermd |
|               | BMI1      | andn |
|               | BMI2      | bzhi |
|               | F16C      | vcvtph2ps |
|               | FMA       | vfmadd132pd |
|               | LZCNT     | lzcnt |
|               | MOVBE     | movbe |
|               | OSXSAVE   | xgetbv |
| x86-64-v4     | AVX512F   | kmovw |
|               | AVX512BW  | vdbpsadbw |
|               | AVX512CD  | vplzcntd |
|               | AVX512DQ  | vpmullq |
|               | AVX512VL  | |
