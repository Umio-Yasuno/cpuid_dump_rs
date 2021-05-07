pub static _AX: u32 = 0x8000_0000;

pub fn line() {
    for _i in 0..75 {
        print!("=");
    }
    println!();
}

pub fn dump() {
    println!("CPUID Dump");
    line();

    let mut a: [u32; 4] = [0; 4];

    for i in 0x0..=0x10 {
        unsafe {
            asm!("cpuid",
                inout("eax") i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }

        println!(" 0x{:08X}:  eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
            i, a[0], a[1], a[2], a[3]);
    }

    println!();

    for i in 0x0..=0x20 {
        unsafe {
            asm!("cpuid",
                inout("eax") _AX + i => a[0],
                lateout("ebx") a[1],
                lateout("ecx") a[2],
                lateout("edx") a[3]
            );
        }

        println!(" 0x{:08X}:  eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
            _AX + i, a[0], a[1], a[2], a[3]);
    }
    println!();
}
