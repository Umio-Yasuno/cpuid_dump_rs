## ToDo

 * [x] Cache info of Intel CPU 
 * [x] Core Affinity
    * [ ] Support for Windows OS

## Usage
### cpuid_dump

    $ cargo run --bin cpuid_dump
    // all thread
    $ cargo run --bin cpuid_dump -- -a

### c2clat (Original: [rigtorp/c2clat](https://github.com/rigtorp/c2clat))

    $ cargo build --release --bin c2clat
    $ ./target/release/c2clat
    // Output Markdown table
    $ ./target/release/c2clat -md
    // gnuplot
    $ ./target/release/c2clat -p | gnuplot -p

## Document
### Rust
 * [asm - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html)
### CPUID
 * [Intel® Architecture Instruction Set Extensions Programming Reference](https://software.intel.com/content/www/us/en/develop/download/intel-architecture-instruction-set-extensions-programming-reference.html)
 * [Developer Guides, Manuals & ISA Documents - AMD](https://developer.amd.com/resources/developer-guides-manuals/)
 * [CPUID - Wikipedia](https://en.wikipedia.org/wiki/CPUID)
 * <https://gitlab.com/x86-psABIs/x86-64-ABI>

